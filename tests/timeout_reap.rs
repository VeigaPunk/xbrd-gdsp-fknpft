use std::process::Command;
use std::time::Duration;
use xbreed::ask::execute_with_timeout;

/// M1.5 — the two reader threads must drain both pipes concurrently or the
/// child deadlocks when either pipe fills its kernel buffer. 131072 bytes
/// each is well past the typical 65536-byte pipe capacity.
#[test]
fn execute_with_timeout_drains_both_pipes_without_deadlock() {
    let script = r#"python3 -c "import sys; sys.stdout.buffer.write(b'O'*131072); sys.stderr.buffer.write(b'E'*131072); sys.exit(0)""#;
    let mut cmd = Command::new("sh");
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
        r#"echo $$ > "{pidfile_str}"; exec python3 -c "
import sys, time
while True:
    sys.stdout.buffer.write(b'O'*1024); sys.stdout.flush()
    sys.stderr.buffer.write(b'E'*1024); sys.stderr.flush()
    time.sleep(0.01)
""#
    );
    let mut cmd = Command::new("sh");
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
