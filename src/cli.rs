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
        /// Enter the review lane (codex only). By default routes to
        /// gpt-5.4-mini. Combine with --full/-F to route to full gpt-5.5
        /// (the-revenger-class RECON work where the 1.05M context window
        /// earns the cost). Mutually exclusive with --spark; --spark wins.
        #[arg(long, short = 'R')]
        review: bool,
        /// Escape hatch for the review lane: route -R to full gpt-5.5
        /// (1.05M context) instead of the default gpt-5.4-mini (400K context).
        /// Reserved for the-revenger RECON tasks stitching codebase-scale
        /// evidence. No effect without --review/-R.
        #[arg(long, short = 'F')]
        full: bool,
        /// Route to gpt-5.5 (codex only) with fast_mode enabled. Supports all
        /// effort levels (low/medium/high/xhigh via -e). Added 2026-04-24 for
        /// xbrd-exec bench — enables xask-arm measurement of gpt-5.5 so the
        /// comparison of 5.5 via raw codex exec vs via xbreed wrapper becomes
        /// expressible. Mutually exclusive with --spark (spark wins); orthogonal
        /// to --review/--full (those route to 5.5 family).
        #[arg(long = "gpt55")]
        gpt55: bool,
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
    /// Run preflight checks before spawning a team
    Precheck {
        #[command(subcommand)]
        check: PrecheckAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum PrecheckAction {
    /// Check if the tmux window has enough room for the requested team size.
    /// Exits 0 if safe, 1 if over cap, 0 with a notice if tmux is unavailable.
    PaneCap {
        /// Number of panes (teammates) about to be spawned
        #[arg(long, short = 'n')]
        team_size: u32,
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
