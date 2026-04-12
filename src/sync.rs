use anyhow::Result;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

pub fn materialize_claude_settings(policy_path: &Path) -> Value {
    let mut settings = json!({
        "hooks": {
            "PreToolUse": [{
                "matcher": "Bash",
                "hooks": [{
                    "type": "command",
                    "command": format!("xbreed guard claude-code --policy '{}'", policy_path.display()),
                    "timeout_ms": 500
                }]
            }]
        }
    });

    if std::env::var("TMUX").is_ok() {
        settings["env"] = json!({
            "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1"
        });
        settings["preferences"] = json!({
            "teammateMode": "tmux"
        });
    }

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
        assert_eq!(entry["matcher"], "Bash");
        let inner = entry["hooks"].as_array().unwrap();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0]["type"], "command");
        assert!(inner[0]["command"].as_str().unwrap().contains("xbreed guard claude-code"));
        assert!(inner[0]["command"].as_str().unwrap().contains("/x/policy.yaml"));
        assert_eq!(inner[0]["timeout_ms"], 500);
    }

    #[test]
    fn materialize_includes_tmux_settings_when_tmux_set() {
        // SAFETY: test runs single-threaded (--test-threads=1)
        unsafe { std::env::set_var("TMUX", "/tmp/tmux-1000/default,12345,0") };
        let v = materialize_claude_settings(&PathBuf::from("/x/policy.yaml"));
        unsafe { std::env::remove_var("TMUX") };
        assert_eq!(v["env"]["CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS"], "1");
        assert_eq!(v["preferences"]["teammateMode"], "tmux");
    }

    #[test]
    fn materialize_excludes_tmux_settings_when_tmux_unset() {
        // SAFETY: test runs single-threaded (--test-threads=1)
        unsafe { std::env::remove_var("TMUX") };
        let v = materialize_claude_settings(&PathBuf::from("/x/policy.yaml"));
        assert!(v.get("env").is_none());
        assert!(v.get("preferences").is_none());
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
