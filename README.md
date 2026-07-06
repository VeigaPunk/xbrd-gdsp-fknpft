# xbreed

Multi-model meta-launcher for Claude Code, Codex, and Gemini CLI with a
shared deny-list safety policy and judge-orchestrated agent teams.

See `docs/superpowers/specs/2026-04-10-xbreeder-design.md` for full design.
See [`AGENTS.md`](AGENTS.md) for the full agent roster and dispatch table.
See [`docs/command-flows.md`](docs/command-flows.md) for visual flow diagrams of every command.

**Trust Claude, godspeed, and let it rip.**

Pareto frontier chasing, no target. Improve one axis, harm none — that's the
antimetabole, and that's the only nudge the lead gets. Walk stops when nothing improves without a
tradeoff.

Core three: the orchestration schema, the runtime (`xask` on the native
mailbox), the godspeed directive.

## Setup

Paste this into your CLI:

    claude "git clone https://github.com/VeigaPunk/xbrd-gdsp-fknpft ~/repos/xbrd-gdsp-fknpft && execute ~/repos/xbrd-gdsp-fknpft/docs/SETUP.md top to bottom. Every step has a gate — verify each one, stop on failure. Done means 'make verify-install' prints OK and /xgs resolves in a fresh session."
