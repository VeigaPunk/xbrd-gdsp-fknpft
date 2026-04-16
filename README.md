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

       for s in godspeed godspeed-team xbreed xbreed-team xgs xbgst; do
         mkdir -p ~/.agents/skills/$s
         cp templates/skills/$s/SKILL.md ~/.agents/skills/$s/SKILL.md
       done

4. Restart Claude Code (env var only takes effect on new sessions)

**Use:**

Inside a Claude Code session, say:

    godspeed-team: <your task>

The lead (the-judge, opus 4.7 max) names 2-3 optimization axes, spawns teammates
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

### v0.4 — effort flags, protocol v0.2, xask dispatch, 4-command split

v0.4 is a configuration + protocol release. The binary version is `0.4.0`.

**Four orchestration commands:**

| Command | Mode | Cross-model | Iteration | Speed |
|---------|------|:-----------:|-----------|:-----:|
| `/xbreed` (`/xb`) | Solo judge | xask gemini/codex | Single turn | Fast |
| `/xbt` (`/xbreed-team`) | Deliberative team | xask gemini/codex | Judge-driven rounds (5 cap) | Slow |
| `/xgs` | Godspeed Pareto team | None (all-Claude) | Pareto walk (4 rounds) | Fast |
| `/xbgst` | Godspeed Pareto + cross-model | xask gemini/codex | Pareto walk (4 rounds) | Medium |

**xask protocol** — cross-model delegation with 4-layer enforcement:

1. **Structural gate**: teammate's first tool call must be `xask [model]`
2. **Raw-quote gate**: verbatim CLI output in `<raw_output>` tags
3. **Fallback tiers**: `[xask dry]` marker on failure, no deadlock
4. **Confidence rule**: provenance marker, not quality demotion

**Judge protocol** — DRAFT format with CONFLICTS block:

- CONFLICTS mandatory when cross-model sources produce opposite verdicts
- Model labels (`gemini`/`codex`) in cross-model modes, teammate labels in all-Claude
- Judge weights contradicting quotes higher than confirming quotes
- Falsification probe: one targeted xask on highest-divergence unchallenged claim

**Effort/thinking flags per model:**

    config/models.yaml:
      claude:  effort: max
      codex:   reasoning_effort: xhigh
      gemini:  (thinkingBudget via xask --effort — warning: silently ignored by CLI)

`xbreed claude` passes `--effort <level>` to the Claude Code CLI.
`xbreed ask` routes effort to each CLI's native flag. Gemini emits a
stderr warning since it has no native `--effort` flag.

**Empirical timing** (from live labrat probes):

| Operation | Wall time |
|-----------|-----------|
| `xask gemini` | ~14s (OAuth cascade) |
| `xask codex` | ~6s (structured) / ~1s (`--direct`) |
| Full xbgst round | ~29s |

**Agent roster:** 8 agents using Inter-Model Communication Protocol v0.2
with structured `# State` / `# Unknowns` / `# Artifact` blocks and
epistemic role constraint (at most one non-obvious claim per proposal).
See [`AGENTS.md`](AGENTS.md).

**xask dispatch wrapper** (`scripts/xask`): unified dispatch to claude, codex,
and gemini with `--effort`, `--rich`/`--direct` modes, `--scope`, and
`--with` loadout pass-through. Contamination-aware: Gemini gets clean mode
by default (no dir tree injection) unless `--rich` is explicit.

**v0.4 install:**

    cargo install --path .
    for s in godspeed godspeed-team xbreed xbreed-team xgs xbgst; do
      mkdir -p ~/.agents/skills/$s
      cp templates/skills/$s/SKILL.md ~/.agents/skills/$s/SKILL.md
    done
    cp templates/agents/*.md ~/.claude/agents/

**Other v0.4 changes:**

- 4-phase spawn protocol for godspeed modes (axes → names → spawn → rounds)
- Deliberative rounds for `/xbt` with judge challenge→refine loop (5-round soft cap)
- CONFLICTS block in DRAFT protocol (cross-model contradiction surfacing)
- Codex effort flag argv ordering fix (prompt now correctly last positional arg)
- PID-namespaced temp dir for `--rich` mode (race condition fix)
- awk gsub `&` injection fix in template substitution
- Gemini `--effort` stderr warning (silently dropped → now visible)
- Dual OAuth switch + 429 auto-fallback for Gemini
- Gemini labrat swarm as universal agent capability
- Protocol v0.2 Handoff blocks for recursive sub-lead dispatch
- Non-interactive mode (`v0.38.0` harness compat)
- Godspeed posture inherited by default for gemini + codex dispatches

See [`docs/command-flows.md`](docs/command-flows.md) for visual flow diagrams
including the xbgst sequence diagram with empirical timing annotations.
