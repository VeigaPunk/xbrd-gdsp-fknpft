use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Policy {
    pub version: u32,
    pub mode: String,
    #[serde(default)]
    pub pinned_for: HashMap<String, String>,
    #[serde(default)]
    pub deny_bash_patterns: Vec<String>,
    #[serde(default)]
    pub prompt_bash_patterns: Vec<String>,
    #[serde(default)]
    pub deny_tools: Vec<String>,
    #[serde(default)]
    pub prompt_tools: Vec<String>,
}

impl Policy {
    pub fn load(path: &Path) -> Result<Self> {
        let s = std::fs::read_to_string(path)?;
        let p: Policy = serde_yaml::from_str(&s)?;
        Ok(p)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Models {
    pub claude: ClaudeModel,
    pub codex: CodexModel,
    pub gemini: GeminiModel,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ClaudeModel {
    pub default: String,
    pub effort: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CodexModel {
    pub default: String,
    pub reasoning_effort: String,
    /// Codex feature flags to enable via `-c features.<name>=true` at launch.
    /// The v0.4 default pins `fast_mode: true` (the real codex feature flag;
    /// `service_tier=fast` from the original spec was empirically confirmed
    /// silently-ignored by codex's TOML parser).
    #[serde(default)]
    pub features: HashMap<String, bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GeminiModel {
    pub default: String,
}

impl Models {
    pub fn load(path: &Path) -> Result<Self> {
        let s = std::fs::read_to_string(path)?;
        let m: Models = serde_yaml::from_str(&s)?;
        Ok(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn loads_minimal_policy() {
        let mut f = NamedTempFile::new().unwrap();
        writeln!(f, "version: 1\nmode: deny_list_only").unwrap();
        let p = Policy::load(f.path()).unwrap();
        assert_eq!(p.version, 1);
        assert_eq!(p.mode, "deny_list_only");
        assert!(p.deny_bash_patterns.is_empty());
    }

    #[test]
    fn loads_full_policy_with_patterns() {
        let mut f = NamedTempFile::new().unwrap();
        writeln!(f, r#"version: 1
mode: deny_list_only
deny_bash_patterns:
  - 'rm -rf /'
prompt_bash_patterns:
  - 'git push --force'
deny_tools: []
prompt_tools: []"#).unwrap();
        let p = Policy::load(f.path()).unwrap();
        assert_eq!(p.deny_bash_patterns, vec!["rm -rf /".to_string()]);
        assert_eq!(p.prompt_bash_patterns, vec!["git push --force".to_string()]);
    }

    #[test]
    fn fails_on_missing_required_fields() {
        let mut f = NamedTempFile::new().unwrap();
        writeln!(f, "deny_bash_patterns:\n  - 'rm -rf /'").unwrap();
        // Missing required `version` and `mode`.
        let result = Policy::load(f.path());
        assert!(result.is_err(), "expected load to fail when version/mode are missing");
    }

    #[test]
    fn loads_models_yaml() {
        let mut f = NamedTempFile::new().unwrap();
        writeln!(f, r#"claude:
  default: opus
  effort: max
codex:
  default: gpt-5.4
  reasoning_effort: xhigh
  features:
    fast_mode: true
gemini:
  default: gemini-3.1-pro-preview"#).unwrap();
        let m = Models::load(f.path()).unwrap();
        assert_eq!(m.claude.default, "opus");
        assert_eq!(m.codex.default, "gpt-5.4");
        assert_eq!(m.codex.reasoning_effort, "xhigh");
        assert_eq!(m.codex.features.get("fast_mode"), Some(&true));
        assert_eq!(m.gemini.default, "gemini-3.1-pro-preview");
    }
}
