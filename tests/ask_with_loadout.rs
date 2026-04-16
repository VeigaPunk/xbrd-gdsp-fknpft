use std::fs;
use std::io::Write as _;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use tempfile::tempdir;

/// Write a skill file into a fake ~/.agents/skills dir under `home`.
fn write_skill(home: &std::path::Path, name: &str, body: &str) {
    let dir = home.join(".agents").join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), body).unwrap();
}

/// Write a shell stub at `bin_dir/name` that logs its argv to `log_path` and exits 0.
fn write_stub(bin_dir: &std::path::Path, name: &str, log_path: &std::path::Path) {
    fs::create_dir_all(bin_dir).unwrap();
    let stub = bin_dir.join(name);
    let script = format!(
        "#!/bin/sh\nprintf '%s\\0' \"$@\" > \"{}\"\nexit 0\n",
        log_path.display()
    );
    let mut f = fs::File::create(&stub).unwrap();
    f.write_all(script.as_bytes()).unwrap();
    let mut perms = fs::metadata(&stub).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&stub, perms).unwrap();
}

/// Read a stub log and split on NUL into the argv list.
fn read_log(log_path: &std::path::Path) -> Vec<String> {
    let raw = fs::read_to_string(log_path).unwrap();
    raw.trim_end_matches('\0')
        .split('\0')
        .map(|s| s.to_string())
        .collect()
}

fn run_xbreed_ask(
    home: &std::path::Path,
    bin_dir: &std::path::Path,
    args: &[&str],
) -> std::process::Output {
    run_xbreed_ask_in_dir(home, home, bin_dir, args)
}

fn run_xbreed_ask_in_dir(
    cwd: &std::path::Path,
    home: &std::path::Path,
    bin_dir: &std::path::Path,
    args: &[&str],
) -> std::process::Output {
    let path = format!(
        "{}:{}",
        bin_dir.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    Command::new(env!("CARGO_BIN_EXE_xbreed"))
        .current_dir(cwd)
        .env("HOME", home)
        .env("PATH", path)
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn ask_codex_with_loadout_injects_developer_instructions_override() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let bin_dir = home.join("bin");
    let log = home.join("codex.log");

    write_skill(home, "godspeed", "GO FAST NOW");
    write_stub(&bin_dir, "codex", &log);

    let out = run_xbreed_ask(
        home,
        &bin_dir,
        &["ask", "codex", "--with", "godspeed", "say hi"],
    );
    assert!(out.status.success(), "xbreed ask failed: {:?}", out);

    let argv = read_log(&log);
    assert_eq!(argv[0], "exec");
    assert_eq!(argv[1], "--skip-git-repo-check");
    assert!(argv.contains(&"approval_policy=\"never\"".to_string())); // -c approval_policy
                                                                      // suppression flags present
    assert!(argv.contains(&"include_permissions_instructions=false".to_string()));
    assert!(argv.contains(&"include_apps_instructions=false".to_string()));
    assert!(argv.contains(&"include_environment_context=false".to_string()));
    // developer_instructions from loadout
    let dev_instr = argv
        .iter()
        .find(|a| a.starts_with("developer_instructions="))
        .expect("developer_instructions flag missing");
    assert!(dev_instr.contains("GO FAST NOW"));
    assert_eq!(*argv.last().unwrap(), "say hi");
}

#[test]
fn ask_gemini_with_loadout_prepends_to_prompt() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let bin_dir = home.join("bin");
    let log = home.join("gemini.log");

    write_skill(home, "godspeed", "GO FAST NOW");
    write_stub(&bin_dir, "gemini", &log);

    // Write a .env.local so gemini_auth_chain() has at least one ApiKey entry.
    // The stub exits 0 immediately so the key value doesn't need to be real.
    fs::write(home.join(".env.local"), "GEMINI_API_KEY=test-key\n").unwrap();

    let out = run_xbreed_ask_in_dir(
        home,
        home,
        &bin_dir,
        &["ask", "gemini", "--with", "godspeed", "say hi"],
    );
    assert!(out.status.success(), "xbreed ask failed: {:?}", out);

    let argv = read_log(&log);
    assert_eq!(argv[0], "-m");
    assert_eq!(argv[1], "gemini-3.1-pro-preview");
    assert_eq!(argv[2], "-p");
    let combined = &argv[3];
    assert!(combined.contains("GO FAST NOW"));
    assert!(combined.contains("## godspeed"));
    assert!(combined.ends_with("say hi"));
    assert!(combined.contains("\n---\n"));
}

#[test]
fn ask_without_with_flag_dispatches_cleanly() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let bin_dir = home.join("bin");
    let log = home.join("codex.log");

    write_stub(&bin_dir, "codex", &log);

    let out = run_xbreed_ask(home, &bin_dir, &["ask", "codex", "say hi"]);
    assert!(out.status.success(), "xbreed ask failed: {:?}", out);

    let argv = read_log(&log);
    assert_eq!(argv[0], "exec");
    assert_eq!(*argv.last().unwrap(), "say hi");
    assert!(!argv
        .iter()
        .any(|a| a.starts_with("developer_instructions=")));
}

#[test]
fn ask_with_missing_skill_errors_cleanly() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let bin_dir = home.join("bin");
    let log = home.join("codex.log");

    // No skill written.
    write_stub(&bin_dir, "codex", &log);

    let out = run_xbreed_ask(
        home,
        &bin_dir,
        &["ask", "codex", "--with", "nonexistent", "say hi"],
    );
    assert!(!out.status.success(), "expected failure for missing skill");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("skill not found"));
    assert!(stderr.contains("nonexistent"));
    // And the stub must NOT have been called.
    assert!(
        !log.exists(),
        "stub should not run when loadout resolution fails"
    );
}

#[test]
fn ask_with_multiple_skills_comma_separated() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let bin_dir = home.join("bin");
    let log = home.join("codex.log");

    write_skill(home, "godspeed", "GO FAST");
    write_skill(home, "librarian", "CURATE");
    write_stub(&bin_dir, "codex", &log);

    let out = run_xbreed_ask(
        home,
        &bin_dir,
        &["ask", "codex", "--with", "godspeed,librarian", "research"],
    );
    assert!(out.status.success());

    let argv = read_log(&log);
    let dev_instr = argv
        .iter()
        .find(|a| a.starts_with("developer_instructions="))
        .expect("developer_instructions missing");
    assert!(dev_instr.contains("GO FAST"));
    assert!(dev_instr.contains("CURATE"));
    let go_idx = dev_instr.find("GO FAST").unwrap();
    let cur_idx = dev_instr.find("CURATE").unwrap();
    assert!(go_idx < cur_idx, "godspeed should come before librarian");
}
