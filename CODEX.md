# xbreed — Codex CLI Project Instructions

You are dispatched to a Rust multi-model orchestrator. Respond concisely. Execute directly.

## Build & Test

```bash
cargo build              # compile
cargo test               # run all tests
cargo test <name>        # run specific test
cargo clippy             # lint (treat warnings as work-to-do)
cargo fmt --check        # format check
cargo fmt                # auto-format
```

**Verify loop after any code change:** `cargo clippy && cargo test && cargo fmt --check`

## Tech Stack

- Rust 2021 edition
- CLI: clap 4 (derive)
- Serialization: serde + serde_json + serde_yaml
- Errors: anyhow (no custom error types — use `anyhow::Result`)
- Binary: `xbreed` (src/main.rs + src/lib.rs)

## Architecture

Multi-model meta-launcher orchestrating Claude Code, Codex CLI, and Gemini CLI with a shared deny-list safety policy (`config/policy.yaml`).

- `src/` — Rust source (lib.rs + main.rs + modules)
- `config/` — policy.yaml, models.yaml, per-CLI config templates
- `scripts/` — shell helpers (xask, xbreed wrappers)
- `templates/agents/` — perspective agent definitions
- `templates/dispatch/` — inter-model dispatch templates (codex.md, gemini.md)
- `commands/` — CC slash command symlinks
- `docs/` — design specs, flow diagrams
- `tests/` — integration tests

## Style Constraints

- All public functions must have doc comments
- `anyhow::Result` only — no custom error types
- CLI subcommands: lowercase, no hyphens (`guard`, `sync`, `ask`)
- Agent templates: YAML frontmatter + markdown body
- `config/policy.yaml` is the single source of truth for deny-list rules
- `config/models.yaml` is the single source of truth for model defaults

## When Dispatched by xbreed

You are invoked via `codex exec -p xbreed` (autonomous, workspace-write) or `codex exec -p xbreed_review` (read-only, xhigh reasoning). Inter-Model Protocol v0.2 expects minimal blocks: `# Goal` + `# State` + `# Action` + `# Artifact: <type>`. Use inline status tags: `obs:`, `inf:`, `asm:`, `risk:` with confidence (certain/strong/moderate/weak/speculative).

**Tell me what to do, not what to think.** Prefer concrete diffs/commands over philosophical reasoning.

## References

- [AGENTS.md](AGENTS.md) — xbreed agent roster and dispatch table (for xbreed internal use, not Codex self-instructions)
- [CLAUDE.md](CLAUDE.md) — equivalent for Claude Code
- [docs/cli-contamination.md](docs/cli-contamination.md) — CLI context injection reference
