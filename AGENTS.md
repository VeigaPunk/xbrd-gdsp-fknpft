# Agent Roster

xbreed ships 8 perspective agents in `templates/agents/`. Copy them to
`~/.claude/agents/` for Claude Code to discover them as subagent types.

## Agents

| Agent | Model | Role | Delegation bias |
|-------|-------|------|-----------------|
| **the-judge** | opus | Orchestrator and arbiter. Names axes, dispatches specialists, applies Pareto filter, drafts implementation. Top of the stack. | In-session (spawns others, never spawned) |
| **scout** | sonnet | Research lens. Finds what exists outside the repo — libraries, docs, prior art, release notes. Read-only. | `xask gemini` with librarian loadout |
| **reviewer** | sonnet | Surgical code reviewer. Finds the bug that ships to prod. Read-only — finds problems, does not fix them. | `xask codex` for deep reviews |
| **labrat** | haiku | Expendable single-shot probe. Tests one hypothesis cheap and fast. State nuked on despawn. | Direct bash or `xask gemini` (512 thinkingBudget, godspeed always loaded) |
| **connector** | sonnet | Cross-axis pattern matcher. Sees the whole table, calls out unusual connections and second-order effects. Breadth over depth. | `xask gemini` for breadth, `xask claude` for reasoning |
| **distiller** | sonnet | Deduplicates N parallel findings, flags contradictions, assigns confidence scores. Pure text synthesis — no tool calls. | In-session text synthesis only |
| **executor** | sonnet | Writes code, runs tests, returns results. Stateless by default — scoped to one subtask. | `xask codex` for refactors, `xask claude` for reasoning |
| **simplifier** | sonnet | YAGNI enforcer. Finds what to delete. If removing it passes all tests, it was dead. | Direct analysis + deletion verification |

## Dispatch table (the-judge reference)

| Axis family | Agent | Delegation |
|---|---|---|
| Research, prior art, outside-world | `scout` | `xask gemini` |
| Correctness, bugs, code review | `reviewer` | `xask codex` |
| Empirical probes, dry-runs | `labrat` (haiku) | direct bash or `xask gemini` |
| Code execution, implementation | `executor` | `xask codex` or `xask claude` |
| Cross-axis patterns, breadth | `connector` | `xask gemini` |
| Findings synthesis, dedup | `distiller` | in-session text synthesis |
| Complexity reduction, YAGNI | `simplifier` | direct analysis |

## Naming convention

Teammates use `{prefix}-{role}-{suffix}` naming:

| Prefix | Model/CLI |
|---|---|
| `g-` | Gemini (via `xask gemini`) |
| `ccs-` | Claude Sonnet |
| `cco-` | Claude Opus |
| `cdx-` | Codex (via `xbreed ask codex`) |

Examples: `ccs-scout-docs`, `g-labrat-probe`, `cdx-reviewer-auth`, `ccs-executor-tests`

## Inter-Model Communication Protocol v0.2

All agents produce structured output using only the blocks appropriate to their
role. Minimal valid message = `[GOAL]` + one other block. See individual agent
files in `templates/agents/` for per-role return formats.

## Swarm capabilities

- **Labrat swarm:** Up to 8 haiku labrats in parallel. Fire-and-forget — no
  TaskCreate, they report via SendMessage + DESPAWN signal.
- **Gemini labrat swarm (universal):** Any agent role can fire a 1-call, 10-probe
  fan-out inside Gemini's context. Refire up to 2x (30 max probes).
