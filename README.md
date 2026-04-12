# xbreed

Multi-model meta-launcher for Claude Code, Codex, and Gemini CLI with a
shared deny-list safety policy and judge-orchestrated agent teams.

See `docs/superpowers/specs/2026-04-10-xbreeder-design.md` for full design.
See [`AGENTS.md`](AGENTS.md) for the full agent roster and dispatch table.
See [`docs/command-flows.md`](docs/command-flows.md) for visual flow diagrams of every command.

## Install

    cargo install --path .

## Usage

### v0.1 — guard, sync, launch, ask

    xbreed guard <cli>             # reads JSON from stdin, writes decision to stdout
    xbreed sync                    # regenerates per-CLI config from policy.yaml
    xbreed claude [args]           # launch Claude Code in max-power mode
    xbreed ask <cli> <prompt>      # headless one-shot

### v0.2 — ask with skill loadouts

    xbreed ask <cli> --with godspeed,the-librarian <prompt>

`--with` resolves comma-separated skill names against `~/.agents/skills/`,
`~/.claude/skills/`, then `~/.config/xbreed/skills/`, concatenates each skill's
`SKILL.md`, and injects via the target CLI's native system-prompt mechanism
(`claude --append-system-prompt`, `codex -c developer_instructions=…`, gemini
prompt-prepend fallback).

### v0.3 — godspeed teams (configuration release)

v0.3 enables Claude Code's experimental native agent teams and ships a small
set of perspective subagent definitions plus a `godspeed-team` meta-skill that
turns the lead session into a Tier-1 Pareto-walking orchestrator.

**Setup:**

1. Add `"CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1"` to the `env` block of `~/.claude/settings.json`
2. Copy the perspective agents:

       mkdir -p ~/.claude/agents
       cp templates/agents/*.md ~/.claude/agents/

3. Copy the skills:

       for s in godspeed godspeed-team xbreed xbreed-team; do
         mkdir -p ~/.agents/skills/$s
         cp templates/skills/$s/SKILL.md ~/.agents/skills/$s/SKILL.md
       done

4. Restart Claude Code (env var only takes effect on new sessions)

**Use:**

Inside a Claude Code session, say:

    godspeed-team: <your task>

The lead (the-judge, opus) names 2-3 optimization axes, spawns teammates
from the 8-agent roster — scout, reviewer, labrat, connector, distiller,
executor, simplifier — runs them in parallel, applies a Pareto filter, and
reports the surviving frontier. Teammates delegate to other CLIs via
`xbreed ask <cli> --with <skill> "<prompt>"`. See [`AGENTS.md`](AGENTS.md)
for the full roster.

**What v0.3 does NOT include:**

- New Rust code (binary stays at v0.2)
- An `xbreed team init` scaffolder (deferred to v0.3.1)
- Heterogeneous team-native peers (codex/gemini still go through `xbreed ask`)
- Per-task persistent memory across team runs
- Telemetry / scoring loop (lives in the godspeed-iter3 project, not here)

See `docs/superpowers/specs/2026-04-10-xbreeder-v0.3-team-design.md` for the
full design.

### v0.4 — effort flags, protocol v0.2, xask dispatch

v0.4 is a configuration + protocol release. The binary version is `0.4.0`.

**Effort/thinking flags per model:**

    config/models.yaml:
      claude:  effort: max
      codex:   reasoning_effort: xhigh
      gemini:  (thinkingBudget via xask --effort)

`xbreed claude` passes `--effort <level>` to the Claude Code CLI.
`xbreed ask` routes effort to each CLI's native flag via `xask --effort`.

**Agent roster redesign:** 9 agents consolidated to 8 (79% token reduction).
All agents now use Inter-Model Communication Protocol v0.2 with structured
`# State` / `# Unknowns` / `# Artifact` blocks. See [`AGENTS.md`](AGENTS.md).

**xask dispatch wrapper** (`scripts/xask`): unified dispatch to claude, codex,
and gemini with `--effort`, `--rich`/`--direct` modes, `--scope`, and
`--with` loadout pass-through. Contamination-aware: Gemini gets clean mode
by default (no dir tree injection) unless `--rich` is explicit.

**Other v0.4 changes:**

- Team member cap raised from 4 to 8 per round
- Dual OAuth switch + 429 auto-fallback for Gemini
- Gemini labrat swarm as universal agent capability (any role can fan out)
- Protocol v0.2 Handoff blocks for recursive sub-lead dispatch
- Non-interactive mode (`v0.38.0` harness compat)
- Godspeed posture inherited by default for gemini + codex dispatches
