use std::process::Command;
use tempfile::tempdir;

#[test]
fn sync_writes_claude_settings_json() {
    let tmp = tempdir().unwrap();
    let policy = tmp.path().join("policy.yaml");
    std::fs::write(&policy, "version: 1\nmode: deny_list_only\n").unwrap();
    let out = tmp.path().join("generated");

    let status = Command::new(env!("CARGO_BIN_EXE_xbreed"))
        .arg("sync")
        .arg("--policy")
        .arg(&policy)
        .arg("--out")
        .arg(&out)
        .status()
        .unwrap();
    assert!(status.success());

    let settings = out.join("claude-settings.json");
    assert!(settings.exists(), "claude-settings.json was not written");
    let contents = std::fs::read_to_string(&settings).unwrap();
    assert!(contents.contains("PreToolUse"));
    assert!(contents.contains("xbreed guard claude-code"));
}
