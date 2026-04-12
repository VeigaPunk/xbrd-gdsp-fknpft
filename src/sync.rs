use anyhow::Result;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

pub fn materialize_claude_settings(policy_path: &Path) -> Value {
    json!({
        "hooks": {
            "PreToolUse": [{
                "matcher": { "tool_name": "Bash" },
                "command": format!("xbreed guard claude-code --policy \"{}\"", policy_path.display()),
                "timeout_ms": 500
            }]
        }
    })
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
        let hook = &pre[0];
        assert_eq!(hook["matcher"]["tool_name"], "Bash");
        assert!(hook["command"].as_str().unwrap().contains("xbreed guard claude-code"));
        assert!(hook["command"].as_str().unwrap().contains("/x/policy.yaml"));
        assert_eq!(hook["timeout_ms"], 500);
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
