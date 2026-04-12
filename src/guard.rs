use crate::config::Policy;
use anyhow::Result;
use regex::RegexSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Decision {
    Allow,
    Deny,
    Prompt,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DecisionOut {
    pub decision: Decision,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
}

impl DecisionOut {
    pub fn allow() -> Self {
        Self { decision: Decision::Allow, reason: None, question: None }
    }
    pub fn deny(reason: impl Into<String>) -> Self {
        Self { decision: Decision::Deny, reason: Some(reason.into()), question: None }
    }
    pub fn prompt(question: impl Into<String>) -> Self {
        Self { decision: Decision::Prompt, reason: None, question: Some(question.into()) }
    }
}

pub struct Engine {
    deny_bash: RegexSet,
    prompt_bash: RegexSet,
    deny_tools: Vec<String>,
    prompt_tools: Vec<String>,
}

impl Engine {
    pub fn from_policy(p: &Policy) -> Result<Self> {
        Ok(Engine {
            deny_bash: RegexSet::new(&p.deny_bash_patterns)?,
            prompt_bash: RegexSet::new(&p.prompt_bash_patterns)?,
            deny_tools: p.deny_tools.clone(),
            prompt_tools: p.prompt_tools.clone(),
        })
    }

    pub fn evaluate(&self, tool: &str, args: &[String]) -> DecisionOut {
        // tool-level gates
        if self.deny_tools.iter().any(|t| t == tool) {
            return DecisionOut::deny(format!("tool {tool} is in deny_tools"));
        }
        if self.prompt_tools.iter().any(|t| t == tool) {
            return DecisionOut::prompt(format!("Allow tool {tool}?"));
        }
        // bash-level gates only apply when tool is Bash
        if tool == "Bash" {
            let cmdline = args.join(" ");
            if self.deny_bash.is_match(&cmdline) {
                return DecisionOut::deny(format!("matches deny_bash_patterns: {cmdline}"));
            }
            if self.prompt_bash.is_match(&cmdline) {
                return DecisionOut::prompt(format!("Allow: {cmdline}?"));
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
    let engine = Engine::from_policy(policy)?;
    let args = if input.tool_name == "Bash" {
        vec![input.tool_input.command.clone()]
    } else {
        vec![]
    };
    let mut out = engine.evaluate(&input.tool_name, &args);
    // headless trust: no TTY → allow on prompt (v0.1 always treats as headless)
    if out.decision == Decision::Prompt {
        out = DecisionOut::allow();
    }
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
            pinned_for: Default::default(),
            deny_bash_patterns: vec![],
            prompt_bash_patterns: vec![],
            deny_tools: vec![],
            prompt_tools: vec![],
        }
    }

    #[test]
    fn empty_policy_allows_everything() {
        let p = empty_policy();
        let engine = Engine::from_policy(&p).unwrap();
        assert_eq!(engine.evaluate("Bash", &["ls".to_string()]).decision, Decision::Allow);
        assert_eq!(engine.evaluate("Read", &["file.txt".to_string()]).decision, Decision::Allow);
    }

    fn policy_with_deny_bash(patterns: Vec<&str>) -> Policy {
        let mut p = empty_policy();
        p.deny_bash_patterns = patterns.into_iter().map(String::from).collect();
        p
    }

    fn policy_with_prompt_bash(patterns: Vec<&str>) -> Policy {
        let mut p = empty_policy();
        p.prompt_bash_patterns = patterns.into_iter().map(String::from).collect();
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
    fn prompt_bash_pattern_prompts_on_match() {
        let p = policy_with_prompt_bash(vec![r"\bgit\s+push\s+.*--force"]);
        let engine = Engine::from_policy(&p).unwrap();
        let d = engine.evaluate("Bash", &["git".into(), "push".into(), "--force".into()]);
        assert_eq!(d.decision, Decision::Prompt);
        assert!(d.question.is_some());
    }

    #[test]
    fn deny_beats_prompt_on_same_command() {
        let mut p = empty_policy();
        p.deny_bash_patterns = vec![r"\brm\b".into()];
        p.prompt_bash_patterns = vec![r"\brm\b".into()];
        let engine = Engine::from_policy(&p).unwrap();
        let d = engine.evaluate("Bash", &["rm".into(), "foo".into()]);
        assert_eq!(d.decision, Decision::Deny);
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
    fn prompt_tools_asks_for_named_tool() {
        let mut p = empty_policy();
        p.prompt_tools = vec!["Edit".into()];
        let engine = Engine::from_policy(&p).unwrap();
        let d = engine.evaluate("Edit", &[]);
        assert_eq!(d.decision, Decision::Prompt);
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
    fn evaluate_from_json_prompt_downgraded_to_allow_in_headless() {
        let mut p = empty_policy();
        p.prompt_bash_patterns = vec![r"\bgit push --force\b".into()];
        let input = r#"{"session_id":"s","transcript_path":"/tmp/t","cwd":"/tmp","hook_event_name":"PreToolUse","tool_name":"Bash","tool_input":{"command":"git push --force"}}"#;
        let out = evaluate_from_json(input, &p).unwrap();
        assert!(out.contains(r#""decision":"allow""#));
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
}
