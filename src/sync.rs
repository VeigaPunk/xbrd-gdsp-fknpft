use anyhow::Result;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

pub fn materialize_claude_settings(policy_path: &Path) -> Value {
    let mut settings = json!({
        "hooks": {
            "PreToolUse": [{
                "matcher": "*",
                "hooks": [{
                    "type": "command",
                    "command": format!("xbreed guard claude-code --policy '{}'", policy_path.display().to_string().replace('\'', "'\\''")),
                    "timeout_ms": 2000
                }]
            }]
        }
    });

    // xbreed is built around multi-teammate orchestration with live peer DMs;
    // those DMs only work when each teammate runs as its own CC session (tmux
    // pane or iTerm2 native pane). `teammateMode: "auto"` picks `in-process`,
    // which reduces teammates to fire-and-forget subagents — SendMessage to a
    // peer that already returned is a silent no-op, breaking cross-DM critique.
    // Always emit the flags so the setup is independent of whether `$TMUX` was
    // set at sync time.
    settings["env"] = json!({
        "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1"
    });
    settings["teammateMode"] = json!("tmux");

    settings
}

pub fn write_claude_settings(out_dir: &Path, policy_path: &Path) -> Result<PathBuf> {
    std::fs::create_dir_all(out_dir)?;
    let out_path = out_dir.join("claude-settings.json");
    let v = materialize_claude_settings(policy_path);
    std::fs::write(&out_path, serde_json::to_string_pretty(&v)?)?;
    Ok(out_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn materialize_produces_pretooluse_hook() {
        let v = materialize_claude_settings(&PathBuf::from("/x/policy.yaml"));
        let hooks = v.get("hooks").unwrap();
        let pre = hooks.get("PreToolUse").unwrap().as_array().unwrap();
        assert_eq!(pre.len(), 1);
        let entry = &pre[0];
        assert_eq!(entry["matcher"], "*", "wildcard matcher fires on all tools");
        let inner = entry["hooks"].as_array().unwrap();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0]["type"], "command");
        assert!(inner[0]["command"]
            .as_str()
            .unwrap()
            .contains("xbreed guard claude-code"));
        assert!(inner[0]["command"]
            .as_str()
            .unwrap()
            .contains("/x/policy.yaml"));
        assert_eq!(inner[0]["timeout_ms"], 2000);
    }

    #[test]
    fn materialize_always_emits_tmux_teammate_mode() {
        let v = materialize_claude_settings(&PathBuf::from("/x/policy.yaml"));
        assert_eq!(v["env"]["CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS"], "1");
        assert_eq!(
            v["teammateMode"], "tmux",
            "teammateMode is a top-level setting — CC reads it via J8().teammateMode, not from a `preferences` sub-object"
        );
        assert!(
            v.get("preferences").is_none(),
            "no `preferences` wrapper — CC's settings schema does not nest teammateMode"
        );
    }

    #[test]
    fn write_settings_creates_file_and_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let out_dir = tmp.path().join("generated");
        let policy = PathBuf::from("/home/user/policy.yaml");
        let path = write_claude_settings(&out_dir, &policy).unwrap();
        assert!(path.exists());
        let contents = std::fs::read_to_string(&path).unwrap();
        assert!(contents.contains("PreToolUse"));
        assert!(contents.contains("xbreed guard claude-code"));
    }
}
