# xbreed — Claude Code Project Config

## Build & Test

```bash
cargo build              # compile
cargo test               # run all tests
cargo test <name>        # run specific test
cargo clippy             # lint
cargo fmt --check        # format check
cargo fmt                # auto-format
cargo install --path .   # install binary
```

## Verify Loop

After any code change, run: `cargo clippy && cargo test && cargo fmt --check`

## Tech Stack

- **Language:** Rust 2021 edition
- **CLI framework:** clap 4 (derive)
- **Serialization:** serde + serde_json + serde_yaml
- **Error handling:** anyhow
- **Binary:** `xbreed` (src/main.rs + src/lib.rs)

## Architecture

Multi-model meta-launcher orchestrating Claude Code, Codex CLI, and Gemini CLI with a shared deny-list safety policy (`config/policy.yaml`).

- `src/` — Rust source (lib.rs + main.rs + modules)
- `config/` — policy.yaml and per-CLI config templates
- `scripts/` — shell helpers (xask, xbreed wrappers)
- `~/.claude/agents/` — canonical perspective agent definitions (user-managed)
- `templates/` — 25-file repo mirror of agent/dispatch/skill templates (restored 2026-04-17 by commit f3882aa after Mission A/bgst-stackbench-0417 surfaced a `scripts/xask` dispatch dependency; kept in-tree for reference)
- `commands/` — CC slash command symlinks
- `docs/` — design specs, flow diagrams
- `tests/` — integration tests

## Style Constraints

- All public functions must have doc comments
- Use `anyhow::Result` for error returns, not custom error types
- CLI subcommands: lowercase, no hyphens (e.g. `guard`, `sync`, `ask`)
- Agent templates use YAML frontmatter + markdown body
- Keep policy.yaml as single source of truth for deny-list rules

## Agent Roster

See [AGENTS.md](AGENTS.md) for the full dispatch table and naming conventions.
