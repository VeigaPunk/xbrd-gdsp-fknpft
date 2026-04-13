use anyhow::Result;
use std::path::Path;
use std::process::{Command, ExitStatus};

pub(crate) fn build_claude_launcher(
    model: &str,
    effort: &str,
    settings_path: &Path,
    passthrough: &[String],
) -> Command {
    let mut c = Command::new("claude");
    c.arg("--model")
        .arg(model)
        .arg("--effort")
        .arg(effort)
        .arg("--dangerously-skip-permissions")
        .arg("--settings")
        .arg(settings_path)
        .args(passthrough);
    c
}

pub fn launch_claude(
    model: &str,
    effort: &str,
    settings_path: &Path,
    passthrough: &[String],
) -> Result<ExitStatus> {
    let mut cmd = build_claude_launcher(model, effort, settings_path, passthrough);
    Ok(cmd.status()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn launcher_has_max_power_flags() {
        let settings = PathBuf::from("/tmp/settings.json");
        let cmd = build_claude_launcher("opus", "max", &settings, &[]);
        let args: Vec<&std::ffi::OsStr> = cmd.get_args().collect();
        let args_str: Vec<String> = args
            .iter()
            .map(|a| a.to_string_lossy().to_string())
            .collect();
        assert!(args_str.contains(&"--model".to_string()));
        assert!(args_str.contains(&"opus".to_string()));
        assert!(args_str.contains(&"--effort".to_string()));
        assert!(args_str.contains(&"max".to_string()));
        assert!(args_str.contains(&"--dangerously-skip-permissions".to_string()));
        assert!(args_str.contains(&"--settings".to_string()));
    }

    #[test]
    fn launcher_appends_passthrough_args() {
        let settings = PathBuf::from("/tmp/settings.json");
        let cmd = build_claude_launcher(
            "opus",
            "max",
            &settings,
            &["-p".to_string(), "hello".to_string()],
        );
        let args: Vec<String> = cmd
            .get_args()
            .map(|a| a.to_string_lossy().to_string())
            .collect();
        let hello_idx = args.iter().position(|a| a == "hello").unwrap();
        let p_idx = args.iter().position(|a| a == "-p").unwrap();
        assert!(p_idx < hello_idx);
    }
}
