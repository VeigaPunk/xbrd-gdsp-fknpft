use std::process::Command;
use std::time::Duration;
use xbreed::ask::execute_with_timeout;

/// M1.5 — the two reader threads must drain both pipes concurrently or the
/// child deadlocks when either pipe fills its kernel buffer. 131072 bytes
/// each is well past the typical 65536-byte pipe capacity.
#[test]
fn execute_with_timeout_drains_both_pipes_without_deadlock() {
    let script = r#"printf "%*s" 131072 "" | tr " " "O" > /dev/stdout
printf "%*s" 131072 "" | tr " " "E" > /dev/stderr"#;
    let mut cmd = Command::new("bash");
    cmd.arg("-c").arg(script);

    let out = execute_with_timeout(cmd, Duration::from_secs(10))
        .expect("chatty child should complete within 10s");

    assert!(out.status.success(), "child exited non-zero: {:?}", out);
    assert_eq!(out.stdout.len(), 131072, "stdout byte count mismatch");
    assert_eq!(out.stderr.len(), 131072, "stderr byte count mismatch");
    assert!(
        out.stdout.iter().all(|b| *b == b'O'),
        "stdout contains non-'O' bytes"
    );
    assert!(
        out.stderr.iter().all(|b| *b == b'E'),
        "stderr contains non-'E' bytes"
    );
}

/// M2 — grandchild regression. A bash child that spawns a `sleep` grandchild
/// must have BOTH processes killed on timeout. Without process-group kill, the
/// direct bash child dies but the grandchild sleep gets reparented to PID 1 and
/// leaks. This test captures the grandchild PID independently and asserts it
/// is gone after `execute_with_timeout` returns.
#[test]
fn execute_with_timeout_kills_grandchild_on_timeout() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let grandchild_pidfile = tmp.path().join("grandchild.pid");
    let gf_str = grandchild_pidfile.display().to_string();

    // bash (direct child) spawns `sleep 300` (grandchild), writes grandchild PID,
    // then waits. On process-group kill both bash and sleep receive SIGKILL.
    let script = format!(r#"sleep 300 & echo $! > "{gf_str}"; wait"#);
    let mut cmd = Command::new("bash");
    cmd.arg("-c").arg(&script);

    let r = execute_with_timeout(cmd, Duration::from_secs(1));
    assert!(r.is_err(), "expected timeout Err, got Ok");
    let msg = format!("{:?}", r.err().unwrap());
    assert!(
        msg.contains("xask-timeout"),
        "error not marked as xask-timeout: {msg}"
    );

    // Poll up to 1 s for the grandchild pidfile (bash writes it quickly after
    // forking sleep, but the poll covers slow CI environments).
    let mut grandchild_pid: u32 = 0;
    for _ in 0..20 {
        if let Ok(s) = std::fs::read_to_string(&grandchild_pidfile) {
            if let Ok(p) = s.trim().parse::<u32>() {
                grandchild_pid = p;
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    assert!(
        grandchild_pid > 0,
        "grandchild PID was never written to '{}' — bash did not fork sleep within poll window",
        grandchild_pidfile.display()
    );

    // Settle: give the kernel a brief window to unlink /proc/<pid> after kill+wait.
    for _ in 0..20 {
        if !std::path::Path::new(&format!("/proc/{grandchild_pid}")).exists() {
            return; // grandchild reaped — test passes
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    panic!(
        "/proc/{grandchild_pid} still exists — grandchild sleep leaked past timeout \
         (process-group kill regression)"
    );
}

/// M1 (codex variant) — regression trap for the W3 ghost-leak fix. A child
/// that loops forever writing to both pipes must be killed AND reaped on
/// timeout; `/proc/<pid>` must be gone after `execute_with_timeout` returns.
#[test]
fn execute_with_timeout_kills_chatty_child_on_timeout() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let pidfile = tmp.path().join("child.pid");
    let pidfile_str = pidfile.display().to_string();

    // Record own pid, then loop forever writing to stdout AND stderr.
    // `exec` replaces the shell so the recorded pid IS the process we kill.
    let script = format!(
        r#"echo $$ > "{pidfile_str}"; while true; do
  printf "%*s" 1024 "" | tr " " "O" > /dev/stdout
  printf "%*s" 1024 "" | tr " " "E" > /dev/stderr
  sleep 0.01
done"#
    );
    let mut cmd = Command::new("bash");
    cmd.arg("-c").arg(&script);

    let r = execute_with_timeout(cmd, Duration::from_secs(1));
    assert!(r.is_err(), "expected timeout Err, got Ok");
    let msg = format!("{:?}", r.err().unwrap());
    assert!(
        msg.contains("xask-timeout"),
        "error not marked as xask-timeout: {msg}"
    );

    let pid_str = std::fs::read_to_string(&pidfile).expect("pidfile written");
    let pid: u32 = pid_str.trim().parse().expect("pidfile parses as u32");

    // Give the kernel a short settle window — kill() + wait() returns quickly
    // but `/proc/<pid>` unlink is asynchronous on some kernels.
    for _ in 0..20 {
        if !std::path::Path::new(&format!("/proc/{pid}")).exists() {
            return;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    panic!("/proc/{pid} still exists — child leaked past timeout");
}
