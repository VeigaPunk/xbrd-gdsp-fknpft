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
    // M11: codex prompt always ends in "| godspeed" (user directive, structural
    // guarantee in ask.rs::dispatch).
    assert_eq!(*argv.last().unwrap(), "say hi | godspeed");
}

#[test]
fn ask_gemini_with_loadout_prepends_to_prompt() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let bin_dir = home.join("bin");
    let log = home.join("gemini.log");

    write_skill(home, "godspeed", "GO FAST NOW");
    write_stub(&bin_dir, "gemini", &log);

    // Create a fake OAuth creds file so default_gemini_oauth_exists() returns
    // true (xbreed reads only ~/.gemini/oauth_creds.json — single-path as of
    // 2026-04-19). The stub gemini binary exits 0 immediately so the creds
    // don't need to be valid — only the file's existence is checked.
    fs::create_dir_all(home.join(".gemini")).unwrap();
    fs::write(home.join(".gemini/oauth_creds.json"), "{}\n").unwrap();

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
    // M11: godspeed suffix guarantee holds even without --with.
    assert_eq!(*argv.last().unwrap(), "say hi | godspeed");
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

/// M6 (codex #7) — end-to-end codex yolo contract through `xbreed ask codex`.
/// Asserts argv contains: `exec`, `--skip-git-repo-check`, the adjacent pair
/// `--sandbox` + `danger-full-access`, `approval_policy="never"`,
/// `model_reasoning_effort=high`, and the trailing prompt.
///
/// Guards against any future refactor silently dropping one of the yolo
/// flags. The yolo routing is a user-locked policy
/// (feedback_yolo_routing.md) and lives in three layers here:
/// comment, frontmatter, and this test.
#[test]
fn ask_codex_route_preserves_full_unlock_contract() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let bin_dir = home.join("bin");
    let log = home.join("codex.log");

    write_stub(&bin_dir, "codex", &log);

    let out = run_xbreed_ask(
        home,
        &bin_dir,
        &["ask", "codex", "--effort", "high", "say hi"],
    );
    assert!(out.status.success(), "xbreed ask codex failed: {:?}", out);

    let argv = read_log(&log);
    assert_eq!(argv[0], "exec");
    assert!(
        argv.iter().any(|a| a == "--skip-git-repo-check"),
        "missing --skip-git-repo-check in argv: {argv:?}"
    );

    // Adjacency: --sandbox must be immediately followed by danger-full-access.
    let sandbox_idx = argv
        .iter()
        .position(|a| a == "--sandbox")
        .expect("missing --sandbox flag");
    assert_eq!(
        argv.get(sandbox_idx + 1).map(String::as_str),
        Some("danger-full-access"),
        "--sandbox not immediately followed by danger-full-access: {argv:?}"
    );

    assert!(
        argv.contains(&"approval_policy=\"never\"".to_string()),
        "missing approval_policy=\"never\" in argv: {argv:?}"
    );
    assert!(
        argv.contains(&"model_reasoning_effort=high".to_string()),
        "missing model_reasoning_effort=high in argv: {argv:?}"
    );

    // M11: codex prompt always ends in "| godspeed" (user directive).
    assert_eq!(
        argv.last().map(String::as_str),
        Some("say hi | godspeed"),
        "prompt must be the final argv element with godspeed suffix: {argv:?}"
    );

    // M7 (mutation sentinels): --ephemeral and features.fast_mode=true must
    // survive any refactor. Ephemeral ensures no session bleed across headless
    // dispatches. fast_mode=true is the non-spark performance path for gpt-5.4
    // family. Mutation-r1 confirmed both are live kill-switch targets.
    assert!(
        argv.iter().any(|a| a == "--ephemeral"),
        "missing --ephemeral in argv: {argv:?}"
    );
    assert!(
        argv.contains(&"features.fast_mode=true".to_string()),
        "missing features.fast_mode=true in argv: {argv:?}"
    );

    // M10 (explicit codex default model pin): the default (non-spark, non-review)
    // dispatch lane must include -m gpt-5.4-mini so a codex version bump that
    // shifts the default model is caught at argv audit time, not silently.
    // Pairs with ask.rs constant CODEX_MINI_MODEL and ~/.codex/config.toml SSoT.
    // User directive 2026-04-17: mini is the standing default; review lane
    // (`xask -R codex`) is the path to gpt-5.4-mini.
    let m_idx = argv.iter().position(|a| a == "-m");
    assert!(
        m_idx.is_some(),
        "missing -m flag in default-lane argv: {argv:?}"
    );
    assert_eq!(
        argv[m_idx.unwrap() + 1],
        "gpt-5.4-mini",
        "default (non-spark, non-review) path must pin -m gpt-5.4-mini: {argv:?}"
    );
}

/// M6 (codex #6) — gemini argv asserts budget stays prompt-side and yolo stays
/// CLI-side. The gemini CLI has no native --effort flag, so the flag must NOT
/// appear in argv; the budget must be embedded in the prompt text, and
/// `--approval-mode yolo` must survive as an adjacent argv pair.
#[test]
fn ask_gemini_uses_yolo_and_no_native_effort_flag() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let bin_dir = home.join("bin");
    let log = home.join("gemini.log");

    write_stub(&bin_dir, "gemini", &log);
    fs::create_dir_all(home.join(".gemini")).unwrap();
    fs::write(home.join(".gemini/oauth_creds.json"), "{}\n").unwrap();

    let out = run_xbreed_ask_in_dir(
        home,
        home,
        &bin_dir,
        &["ask", "gemini", "--effort", "high", "say hi"],
    );
    assert!(out.status.success(), "xbreed ask gemini failed: {:?}", out);

    let argv = read_log(&log);

    // Adjacency: --approval-mode yolo.
    let approval_idx = argv
        .iter()
        .position(|a| a == "--approval-mode")
        .expect("missing --approval-mode flag");
    assert_eq!(
        argv.get(approval_idx + 1).map(String::as_str),
        Some("yolo"),
        "--approval-mode not immediately followed by yolo: {argv:?}"
    );

    // No bare --effort token may appear in gemini argv; the budget routes
    // through the prompt template, not the CLI.
    assert!(
        !argv.iter().any(|a| a == "--effort"),
        "gemini argv must not contain bare --effort token: {argv:?}"
    );
}

#[test]
fn ask_with_multiple_skills_comma_separated() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let bin_dir = home.join("bin");
    let log = home.join("codex.log");

    write_skill(home, "godspeed", "GO FAST");
    write_skill(home, "curator", "CURATE");
    write_stub(&bin_dir, "codex", &log);

    let out = run_xbreed_ask(
        home,
        &bin_dir,
        &["ask", "codex", "--with", "godspeed,curator", "research"],
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
    assert!(go_idx < cur_idx, "godspeed should come before curator");
}

/// M8 (codex --json plumbing) — end-to-end contract: when `xbreed ask codex --json`
/// is invoked, `--json` must appear in the codex argv, positioned before the prompt.
///
/// Guards the three-layer plumb: cli.rs (clap field) → main.rs (destructure + pass)
/// → ask.rs (build_codex_ask_with_loadout emits --json when json=true). Any silent
/// removal at any layer breaks orchestrator pipelines that rely on structured output.
#[test]
fn ask_codex_json_flag_reaches_codex_argv() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let bin_dir = home.join("bin");
    let log = home.join("codex.log");

    write_stub(&bin_dir, "codex", &log);

    let out = run_xbreed_ask(home, &bin_dir, &["ask", "codex", "--json", "say hi"]);
    assert!(
        out.status.success(),
        "xbreed ask codex --json failed: {:?}",
        out
    );

    let argv = read_log(&log);

    // --json must appear in argv
    assert!(
        argv.iter().any(|a| a == "--json"),
        "--json flag missing from codex argv: {argv:?}"
    );

    // --json must appear before the prompt (before the final positional arg).
    // M11: prompt now carries the "| godspeed" suffix (user directive).
    let json_idx = argv.iter().position(|a| a == "--json").unwrap();
    let prompt_idx = argv.iter().position(|a| a == "say hi | godspeed").unwrap();
    assert!(
        json_idx < prompt_idx,
        "--json must precede the prompt in codex argv: {argv:?}"
    );

    // All other yolo/suppression flags must still be present (no regression)
    assert!(
        argv.iter().any(|a| a == "--skip-git-repo-check"),
        "missing --skip-git-repo-check: {argv:?}"
    );
    assert!(
        argv.contains(&"approval_policy=\"never\"".to_string()),
        "missing approval_policy: {argv:?}"
    );

    // Without --json, --json must NOT appear
    let log2 = home.join("codex_nojson.log");
    write_stub(&bin_dir, "codex", &log2);
    let out2 = run_xbreed_ask(home, &bin_dir, &["ask", "codex", "say hi"]);
    assert!(
        out2.status.success(),
        "xbreed ask codex (no --json) failed: {:?}",
        out2
    );
    let argv2 = read_log(&log2);
    assert!(
        !argv2.iter().any(|a| a == "--json"),
        "--json must not appear when flag absent: {argv2:?}"
    );
}

/// M9 (codex -o / --output-last-message plumbing) — end-to-end contract: when
/// `xbreed ask codex --output-last-message <FILE>` is invoked, `-o <FILE>` must
/// appear in the codex argv, positioned before the prompt.
///
/// Guards the three-layer plumb: cli.rs (clap field) → main.rs (destructure +
/// as_deref) → ask.rs (build_codex_ask_with_loadout emits -o <path> when
/// output_last_message is Some). Any silent removal breaks artifact capture for
/// mailboxing, logs, and downstream reducers.
#[test]
fn ask_codex_output_last_message_flag_reaches_codex_argv() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let bin_dir = home.join("bin");
    let log = home.join("codex.log");
    let out_file = home.join("last_msg.txt");

    write_stub(&bin_dir, "codex", &log);

    let out = run_xbreed_ask(
        home,
        &bin_dir,
        &[
            "ask",
            "codex",
            "--output-last-message",
            out_file.to_str().unwrap(),
            "say hi",
        ],
    );
    assert!(
        out.status.success(),
        "xbreed ask codex --output-last-message failed: {:?}",
        out
    );

    let argv = read_log(&log);

    // -o must appear in argv
    assert!(
        argv.iter().any(|a| a == "-o"),
        "-o flag missing from codex argv: {argv:?}"
    );

    // the path argument must immediately follow -o
    let o_idx = argv.iter().position(|a| a == "-o").unwrap();
    assert_eq!(
        argv[o_idx + 1],
        out_file.to_str().unwrap(),
        "-o must be followed by the output path: {argv:?}"
    );

    // -o must appear before the prompt (before the final positional arg).
    // M11: prompt carries "| godspeed" suffix (user directive).
    let prompt_idx = argv.iter().position(|a| a == "say hi | godspeed").unwrap();
    assert!(
        o_idx < prompt_idx,
        "-o must precede the prompt in codex argv: {argv:?}"
    );

    // All other yolo/suppression flags must still be present (no regression)
    assert!(
        argv.iter().any(|a| a == "--skip-git-repo-check"),
        "missing --skip-git-repo-check: {argv:?}"
    );
    assert!(
        argv.contains(&"approval_policy=\"never\"".to_string()),
        "missing approval_policy: {argv:?}"
    );

    // Without --output-last-message, -o must NOT appear
    let log2 = home.join("codex_no_output.log");
    write_stub(&bin_dir, "codex", &log2);
    let out2 = run_xbreed_ask(home, &bin_dir, &["ask", "codex", "say hi"]);
    assert!(
        out2.status.success(),
        "xbreed ask codex (no -o) failed: {:?}",
        out2
    );
    let argv2 = read_log(&log2);
    assert!(
        !argv2.iter().any(|a| a == "-o"),
        "-o must not appear when flag absent: {argv2:?}"
    );
}

/// M12 (-R -F escape hatch plumb) — end-to-end contract: when
/// `xbreed ask codex --review --full` is invoked, `-m gpt-5.4` (not
/// `gpt-5.4-mini`) must reach codex argv. Without --full, -R routes to
/// gpt-5.4-mini default.
///
/// Guards the three-layer plumb: cli.rs (--full/-F field) → main.rs
/// (destructure + pass) → ask.rs (review && full branch pins CODEX_FULL_MODEL).
/// User directive 2026-04-18: the-revenger RECON needs 1.05M context; -F
/// escape hatch is the reachable path.
#[test]
fn ask_codex_review_full_flag_routes_to_full_model() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let bin_dir = home.join("bin");

    // Case 1: -R --full → -m gpt-5.4 (full)
    let log_full = home.join("codex_full.log");
    write_stub(&bin_dir, "codex", &log_full);
    let out_full = run_xbreed_ask(
        home,
        &bin_dir,
        &["ask", "codex", "--review", "--full", "recon this"],
    );
    assert!(
        out_full.status.success(),
        "xbreed ask codex -R --full failed: {:?}",
        out_full
    );
    let argv_full = read_log(&log_full);
    let m_idx = argv_full
        .iter()
        .position(|a| a == "-m")
        .expect("missing -m flag in -R -F argv");
    assert_eq!(
        argv_full[m_idx + 1],
        "gpt-5.4",
        "-R --full must pin -m gpt-5.4 (full): {argv_full:?}"
    );

    // Case 2: -R without --full → -m gpt-5.4-mini (review lane default)
    let log_mini = home.join("codex_mini.log");
    write_stub(&bin_dir, "codex", &log_mini);
    let out_mini = run_xbreed_ask(home, &bin_dir, &["ask", "codex", "--review", "review this"]);
    assert!(out_mini.status.success(), "xbreed ask codex -R failed");
    let argv_mini = read_log(&log_mini);
    let m_idx_mini = argv_mini
        .iter()
        .position(|a| a == "-m")
        .expect("missing -m in -R argv");
    assert_eq!(
        argv_mini[m_idx_mini + 1],
        "gpt-5.4-mini",
        "-R without --full must pin -m gpt-5.4-mini (review default 2026-04-18): {argv_mini:?}"
    );

    // Case 3: --full without -R is a no-op → mini (not full)
    let log_noop = home.join("codex_noop.log");
    write_stub(&bin_dir, "codex", &log_noop);
    let out_noop = run_xbreed_ask(home, &bin_dir, &["ask", "codex", "--full", "hello"]);
    assert!(out_noop.status.success(), "xbreed ask codex --full failed");
    let argv_noop = read_log(&log_noop);
    let m_idx_noop = argv_noop
        .iter()
        .position(|a| a == "-m")
        .expect("missing -m in --full-only argv");
    assert_eq!(
        argv_noop[m_idx_noop + 1],
        "gpt-5.4-mini",
        "--full without -R must route to mini (default lane): {argv_noop:?}"
    );
}

#[test]
fn dispatch_codex_full_flag_threads_through() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let bin_dir = home.join("bin");
    let log = home.join("codex_full_threads.log");

    write_stub(&bin_dir, "codex", &log);

    let out = run_xbreed_ask(
        home,
        &bin_dir,
        &["ask", "codex", "--review", "--full", "thread this"],
    );
    assert!(
        out.status.success(),
        "xbreed ask codex -R -F failed: {:?}",
        out
    );

    let argv = read_log(&log);
    let m_idx = argv
        .iter()
        .position(|a| a == "-m")
        .expect("missing -m flag in -R -F argv");
    assert_eq!(
        argv[m_idx + 1],
        "gpt-5.4",
        "-R --full must pin -m gpt-5.4 and thread it through dispatch: {argv:?}"
    );
}

#[test]
fn ask_codex_spark_wins_over_review_and_full() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let bin_dir = home.join("bin");
    let log = home.join("codex_spark.log");

    write_stub(&bin_dir, "codex", &log);

    let out = run_xbreed_ask(
        home,
        &bin_dir,
        &["ask", "codex", "--spark", "--review", "--full", "probe"],
    );
    assert!(
        out.status.success(),
        "xbreed ask codex --spark --review --full failed: {:?}",
        out
    );

    let argv = read_log(&log);
    let m_idx = argv
        .iter()
        .position(|a| a == "-m")
        .expect("missing -m in spark argv");
    assert_eq!(
        argv[m_idx + 1],
        "gpt-5.3-codex-spark",
        "spark must win over review/full and pin spark model: {argv:?}"
    );
    assert!(
        argv.contains(&"model_reasoning_effort=low".to_string()),
        "model_reasoning_effort=low must be present on spark lane: {argv:?}"
    );
    assert!(
        !argv.contains(&"features.fast_mode=true".to_string()),
        "features.fast_mode=true must be absent on spark lane: {argv:?}"
    );
}

/// M11 (godspeed inheritance guarantee) — user directive: codex ALWAYS inherits
/// the godspeed posture through xask in its purest form. Structural guarantee
/// in ask.rs::dispatch — the codex prompt always ends with "| godspeed",
/// regardless of --with skill selection. Idempotent: if scripts/xask already
/// appended the suffix (SKILL=godspeed default), the Rust layer doesn't
/// duplicate it.
///
/// Guards against a future refactor that removes the suffix-append in
/// dispatch() or that changes the idempotence check.
#[test]
fn ask_codex_always_inherits_godspeed_suffix() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let bin_dir = home.join("bin");

    write_stub(&bin_dir, "codex", &home.join("codex.log"));

    // Case 1: no --with flag — godspeed suffix must still be appended
    let log1 = home.join("codex1.log");
    write_stub(&bin_dir, "codex", &log1);
    let out1 = run_xbreed_ask(home, &bin_dir, &["ask", "codex", "prompt one"]);
    assert!(out1.status.success(), "case 1 failed: {out1:?}");
    let argv1 = read_log(&log1);
    assert_eq!(
        *argv1.last().unwrap(),
        "prompt one | godspeed",
        "no-skill codex path must carry godspeed suffix: {argv1:?}"
    );

    // Case 2: --with curator (non-godspeed skill) — godspeed suffix still appended
    write_skill(home, "curator", "CURATE WISELY");
    let log2 = home.join("codex2.log");
    write_stub(&bin_dir, "codex", &log2);
    let out2 = run_xbreed_ask(
        home,
        &bin_dir,
        &["ask", "codex", "--with", "curator", "prompt two"],
    );
    assert!(out2.status.success(), "case 2 failed: {out2:?}");
    let argv2 = read_log(&log2);
    assert_eq!(
        *argv2.last().unwrap(),
        "prompt two | godspeed",
        "non-godspeed skill codex path must still carry godspeed suffix: {argv2:?}"
    );
    // curator loadout is still injected via developer_instructions (additive, not replaced)
    let dev_instr = argv2
        .iter()
        .find(|a| a.starts_with("developer_instructions="))
        .expect("curator developer_instructions missing");
    assert!(
        dev_instr.contains("CURATE WISELY"),
        "curator loadout missing despite godspeed suffix: {dev_instr}"
    );

    // Case 3: idempotence — if caller already appended "| godspeed", no double-suffix
    let log3 = home.join("codex3.log");
    write_stub(&bin_dir, "codex", &log3);
    let out3 = run_xbreed_ask(home, &bin_dir, &["ask", "codex", "prompt three | godspeed"]);
    assert!(out3.status.success(), "case 3 failed: {out3:?}");
    let argv3 = read_log(&log3);
    assert_eq!(
        *argv3.last().unwrap(),
        "prompt three | godspeed",
        "idempotence failed — double godspeed suffix: {argv3:?}"
    );
    // Specifically: there must be exactly ONE occurrence of "| godspeed" in the prompt
    let count = argv3.last().unwrap().matches("| godspeed").count();
    assert_eq!(
        count,
        1,
        "godspeed suffix must appear exactly once: {}",
        argv3.last().unwrap()
    );
}
