use crate::config::Policy;
use anyhow::Result;
use regex::RegexSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Decision {
    Allow,
    Deny,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DecisionOut {
    pub decision: Decision,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl DecisionOut {
    pub fn allow() -> Self {
        Self {
            decision: Decision::Allow,
            reason: None,
        }
    }
    pub fn deny(reason: impl Into<String>) -> Self {
        Self {
            decision: Decision::Deny,
            reason: Some(reason.into()),
        }
    }
}

pub struct Engine {
    mode: String,
    deny_bash: RegexSet,
    deny_tools: Vec<String>,
    allow_tools: Vec<String>,
}

impl Engine {
    pub fn from_policy(p: &Policy) -> Result<Self> {
        if p.mode != "deny_list_only" && p.mode != "allow_list" {
            anyhow::bail!(
                "unknown policy mode: '{}' (expected 'deny_list_only' or 'allow_list')",
                p.mode
            );
        }
        Ok(Engine {
            mode: p.mode.clone(),
            deny_bash: RegexSet::new(&p.deny_bash_patterns)?,
            deny_tools: p.deny_tools.clone(),
            allow_tools: p.allow_tools.clone(),
        })
    }

    pub fn evaluate(&self, tool: &str, args: &[String]) -> DecisionOut {
        // Allow-list mode: default deny unless tool is explicitly allowed.
        if self.mode == "allow_list" {
            let allowed = self.allow_tools.iter().any(|t| t == tool);
            if !allowed {
                return DecisionOut::deny(format!(
                    "tool {tool} not in allow_tools (mode: allow_list)"
                ));
            }
            // Allowed tools still go through bash pattern checks below.
        }
        // Deny-list gates (run in both modes)
        if self.deny_tools.iter().any(|t| t == tool) {
            return DecisionOut::deny(format!("tool {tool} is in deny_tools"));
        }
        if tool == "Bash" {
            let cmdline = args.join(" ");
            if self.deny_bash.is_match(&cmdline) {
                return DecisionOut::deny(format!("matches deny_bash_patterns: {cmdline}"));
            }
        }
        DecisionOut::allow()
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct ToolInput {
    #[serde(default)]
    pub command: String,
}

#[derive(Debug, Deserialize)]
pub struct GuardInput {
    pub tool_name: String,
    #[serde(default)]
    pub tool_input: ToolInput,
    // other Claude Code fields (session_id, transcript_path, cwd, hook_event_name, etc.)
    // are accepted but ignored via serde's default deny_unknown_fields=false
}

pub fn evaluate_from_json(input_json: &str, policy: &Policy) -> Result<String> {
    let input: GuardInput = serde_json::from_str(input_json)?;
    // Fast path: non-Bash tools only need deny_tools + allow_tools check (Vec scan).
    // Skip the expensive RegexSet compile for deny_bash_patterns.
    if input.tool_name != "Bash" {
        let out = if policy.mode == "allow_list"
            && !policy.allow_tools.iter().any(|t| t == &input.tool_name)
        {
            DecisionOut::deny(format!(
                "tool {} not in allow_tools (mode: allow_list)",
                input.tool_name
            ))
        } else if policy.deny_tools.iter().any(|t| t == &input.tool_name) {
            DecisionOut::deny(format!("tool {} is in deny_tools", input.tool_name))
        } else {
            DecisionOut::allow()
        };
        return Ok(serde_json::to_string(&out)?);
    }
    let engine = Engine::from_policy(policy)?;
    let args = vec![input.tool_input.command.clone()];
    let out = engine.evaluate(&input.tool_name, &args);
    Ok(serde_json::to_string(&out)?)
}

pub fn run_from_stdin(policy_path: &std::path::Path) -> Result<()> {
    use std::io::Read;
    let policy = Policy::load(policy_path)?;
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    let out = evaluate_from_json(&buf, &policy)?;
    println!("{out}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_policy() -> Policy {
        Policy {
            version: 1,
            mode: "deny_list_only".to_string(),
            deny_bash_patterns: vec![],
            deny_tools: vec![],
            allow_tools: vec![],
        }
    }

    #[test]
    fn empty_policy_allows_everything() {
        let p = empty_policy();
        let engine = Engine::from_policy(&p).unwrap();
        assert_eq!(
            engine.evaluate("Bash", &["ls".to_string()]).decision,
            Decision::Allow
        );
        assert_eq!(
            engine.evaluate("Read", &["file.txt".to_string()]).decision,
            Decision::Allow
        );
    }

    fn policy_with_deny_bash(patterns: Vec<&str>) -> Policy {
        let mut p = empty_policy();
        p.deny_bash_patterns = patterns.into_iter().map(String::from).collect();
        p
    }

    #[test]
    fn deny_bash_pattern_blocks_match() {
        let p = policy_with_deny_bash(vec![r"\brm\s+-rf\s+/"]);
        let engine = Engine::from_policy(&p).unwrap();
        let d = engine.evaluate("Bash", &["rm".into(), "-rf".into(), "/".into()]);
        assert_eq!(d.decision, Decision::Deny);
        assert!(d.reason.is_some());
    }

    #[test]
    fn deny_bash_pattern_allows_non_match() {
        let p = policy_with_deny_bash(vec![r"\brm\s+-rf\s+/"]);
        let engine = Engine::from_policy(&p).unwrap();
        let d = engine.evaluate("Bash", &["ls".into()]);
        assert_eq!(d.decision, Decision::Allow);
    }

    #[test]
    fn non_bash_tool_ignores_bash_patterns() {
        let p = policy_with_deny_bash(vec![r"rm"]);
        let engine = Engine::from_policy(&p).unwrap();
        let d = engine.evaluate("Read", &["rm-something.txt".into()]);
        assert_eq!(d.decision, Decision::Allow);
    }

    #[test]
    fn deny_tools_blocks_named_tool() {
        let mut p = empty_policy();
        p.deny_tools = vec!["WebFetch".into()];
        let engine = Engine::from_policy(&p).unwrap();
        let d = engine.evaluate("WebFetch", &[]);
        assert_eq!(d.decision, Decision::Deny);
    }

    #[test]
    fn tool_gate_takes_precedence_over_bash_pattern() {
        let mut p = empty_policy();
        p.deny_tools = vec!["Bash".into()];
        p.deny_bash_patterns = vec![r"foo".into()];
        let engine = Engine::from_policy(&p).unwrap();
        let d = engine.evaluate("Bash", &["bar".into()]);
        assert_eq!(d.decision, Decision::Deny);
    }

    #[test]
    fn evaluate_from_json_allows_benign() {
        let p = empty_policy();
        let input = r#"{"session_id":"s","transcript_path":"/tmp/t","cwd":"/tmp","hook_event_name":"PreToolUse","tool_name":"Bash","tool_input":{"command":"ls -la"}}"#;
        let out = evaluate_from_json(input, &p).unwrap();
        assert!(out.contains(r#""decision":"allow""#));
    }

    #[test]
    fn evaluate_from_json_denies_match() {
        let mut p = empty_policy();
        p.deny_bash_patterns = vec![r"\brm\s+-rf\s+/".into()];
        let input = r#"{"session_id":"s","transcript_path":"/tmp/t","cwd":"/tmp","hook_event_name":"PreToolUse","tool_name":"Bash","tool_input":{"command":"rm -rf /"}}"#;
        let out = evaluate_from_json(input, &p).unwrap();
        assert!(out.contains(r#""decision":"deny""#));
    }

    #[test]
    fn evaluate_from_json_accepts_real_claude_code_shape() {
        let mut p = empty_policy();
        p.deny_bash_patterns = vec![r"\brm\s+-rf\s+/".into()];
        let input = r#"{
            "session_id": "abc-123",
            "transcript_path": "/home/user/.claude/projects/x/y.jsonl",
            "cwd": "/home/user/proj",
            "hook_event_name": "PreToolUse",
            "tool_name": "Bash",
            "tool_input": {
                "command": "rm -rf /",
                "description": "cleanup"
            }
        }"#;
        let out = evaluate_from_json(input, &p).unwrap();
        assert!(out.contains(r#""decision":"deny""#));
    }

    #[test]
    fn allow_list_mode_denies_unlisted_tool() {
        let mut p = empty_policy();
        p.mode = "allow_list".to_string();
        p.allow_tools = vec!["Read".into(), "Bash".into()];
        let engine = Engine::from_policy(&p).unwrap();
        assert_eq!(engine.evaluate("Read", &[]).decision, Decision::Allow);
        assert_eq!(engine.evaluate("Write", &[]).decision, Decision::Deny);
        assert_eq!(
            engine.evaluate("Bash", &["ls".into()]).decision,
            Decision::Allow
        );
    }

    #[test]
    fn allow_list_empty_denies_everything() {
        let mut p = empty_policy();
        p.mode = "allow_list".to_string();
        let engine = Engine::from_policy(&p).unwrap();
        assert_eq!(engine.evaluate("Read", &[]).decision, Decision::Deny);
        assert_eq!(
            engine.evaluate("Bash", &["ls".into()]).decision,
            Decision::Deny
        );
    }

    #[test]
    fn allow_list_still_enforces_deny_bash() {
        let mut p = empty_policy();
        p.mode = "allow_list".to_string();
        p.allow_tools = vec!["Bash".into()];
        p.deny_bash_patterns = vec![r"\brm\s+-rf\s+/".into()];
        let engine = Engine::from_policy(&p).unwrap();
        assert_eq!(
            engine.evaluate("Bash", &["ls".into()]).decision,
            Decision::Allow
        );
        assert_eq!(
            engine.evaluate("Bash", &["rm -rf /".into()]).decision,
            Decision::Deny
        );
    }

    #[test]
    fn non_bash_fast_path_respects_allow_list() {
        let mut p = empty_policy();
        p.mode = "allow_list".to_string();
        p.allow_tools = vec!["Read".into()];
        let input = r#"{"tool_name":"Write","tool_input":{}}"#;
        let out = evaluate_from_json(input, &p).unwrap();
        assert!(out.contains(r#""decision":"deny""#));
        let input2 = r#"{"tool_name":"Read","tool_input":{}}"#;
        let out2 = evaluate_from_json(input2, &p).unwrap();
        assert!(out2.contains(r#""decision":"allow""#));
    }

    #[test]
    fn malformed_policy_yaml_fails_closed() {
        use std::io::Write;
        let mut f = tempfile::NamedTempFile::new().unwrap();
        writeln!(f, "not: valid: yaml: {{{{").unwrap();
        let result = crate::config::Policy::load(f.path());
        assert!(
            result.is_err(),
            "malformed policy.yaml must fail, not silently allow"
        );
    }
}
