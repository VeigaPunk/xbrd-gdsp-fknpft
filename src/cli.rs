use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "xbreed",
    version,
    about = "Multi-model meta-launcher with shared safety policy"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Evaluate a tool call against policy (reads JSON from stdin)
    Guard {
        /// CLI name the tool call originates from
        cli: String,
        /// Path to policy.yaml
        #[arg(long, default_value = "~/.config/xbreed/policy.yaml")]
        policy: PathBuf,
    },
    /// Regenerate per-CLI config/hook files from policy.yaml
    Sync {
        #[arg(long, default_value = "~/.config/xbreed/policy.yaml")]
        policy: PathBuf,
        #[arg(long, default_value = "~/.config/xbreed/generated")]
        out: PathBuf,
    },
    /// Launch Claude Code in max-power mode with guard wired
    Claude {
        /// Arguments passed through to claude
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Headless one-shot dispatch to a named CLI, optionally with a skill loadout
    Ask {
        /// One of: claude, codex, gemini
        cli: String,
        /// Prompt to send
        prompt: String,
        /// Skill/directive names to load (comma-separated, repeatable).
        /// Searched in ~/.agents/skills, ~/.claude/skills, ~/.config/xbreed/skills
        #[arg(short = 'w', long = "with", value_delimiter = ',')]
        with: Vec<String>,
        /// Effort/reasoning level to pass to the target CLI.
        /// Maps to: claude --effort, codex -c model_reasoning_effort=, gemini thinkingBudget (future)
        #[arg(short = 'e', long = "effort")]
        effort: Option<String>,
        /// Use the fast codex-spark model with low effort (codex only).
        /// Equivalent to: -m gpt-5.3-codex-spark + model_reasoning_effort=low
        #[arg(long)]
        spark: bool,
        /// Emit raw JSON from codex exec responses (codex only).
        /// Passes --json to codex exec; no-op for gemini.
        #[arg(long)]
        json: bool,
        /// Write the final assistant message to FILE (codex only).
        /// Passes -o <FILE> to codex exec; no-op for gemini.
        #[arg(long = "output-last-message", short = 'o')]
        output_last_message: Option<PathBuf>,
    },
    /// Initialize a team workspace
    Team {
        #[command(subcommand)]
        action: TeamAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum TeamAction {
    /// Initialize the team directory
    Init,
    /// File-based side-channel mailbox for fast-path signals
    Mailbox {
        #[command(subcommand)]
        subaction: MailboxAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum MailboxAction {
    /// Write an event to the mailbox
    Write {
        #[arg(long)]
        from: String,
        #[arg(long)]
        kind: String,
        #[arg(long)]
        payload: String,
    },
    /// Drain all events from the mailbox and print as JSON
    Drain {
        /// Output in Claude Code hook JSON format
        /// (hookSpecificOutput.additionalContext) for UserPromptSubmit
        /// hook injection. Default output is a plain JSON array.
        #[arg(long)]
        inject: bool,
    },
    /// Compact the mailbox: fold low-signal events into a digest
    Compact {
        /// Event types to keep verbatim (heuristic attention proxy)
        #[arg(
            long,
            value_delimiter = ',',
            default_value = "concern,diff,discovery,shutdown-ack"
        )]
        keep_types: Vec<String>,
        /// Fold events older than this many seconds into the digest
        #[arg(long, default_value = "60")]
        digest_older_than: u64,
    },
}
