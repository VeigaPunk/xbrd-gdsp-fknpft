# Agent Roster

xbreed ships 14 perspective agents in `templates/agents/`. Copy them to
`~/.claude/agents/` for Claude Code to discover them as subagent types.

## Agents

| Agent | Model | Role | Delegation bias |
|-------|-------|------|-----------------|
| **the-judge** | opus 4.7 max | Orchestrator and arbiter. Names axes, dispatches specialists, applies Pareto filter, drafts implementation. Top of the stack. | In-session (spawns others, never spawned) |
| **scout** | sonnet | Research lens. Finds what exists outside the repo — libraries, docs, prior art, release notes. Prefers Gemini delegation with librarian loadout. | `xask --effort medium codex` *(gemini-rate-limited 2026-04-15; restore to `xask --effort medium gemini` with librarian loadout when canary in shared.md passes)* |
| **reviewer** | sonnet | Surgical code reviewer. Finds the bug that ships to prod. Delegates to Codex Always. | `xask --effort high codex` (default), escalate to `xhigh` only for deep architectural review |
| **labrat** | sonnet | Expendable single-shot probe. Tests one hypothesis cheap and fast. State nuked on despawn. | `xask --spark codex` (default), `xask gemini` for long-context |
| **connector** | sonnet | Cross-axis pattern matcher. Sees the whole table, calls out unusual connections and second-order effects. Breadth over depth. | `xask --effort medium codex` *(gemini-rate-limited 2026-04-15; restore to `xask gemini` for breadth when canary passes)*, `advisor()` for reasoning |
| **distiller** | sonnet | Deduplicates N parallel findings, flags contradictions, assigns confidence scores. Text synthesis with optional tool verification. Sits between workers and the-judge. | Spawned after peer DMs land, before judge Pareto filter; persistent across rounds |
| **executor** | sonnet | Writes code, runs tests, returns results. Stateless by default — scoped to one subtask. | `xask --spark codex` (Layer-1 gate) for bounded tasks, `xask --effort high codex` for refactors, `advisor()` for reasoning |
| **simplifier** | sonnet | YAGNI enforcer. Finds what to delete. If removing it passes all tests, it was dead. | Direct analysis + deletion verification |
| **the-revenger** | opus 4.7 max | Reverse engineering specialist. Maps behavior, infers intent, reproduces functionality. Godspeeded by default. 4-phase RECON/PROBE/MODEL/BUILD protocol. | `xask --effort medium codex` *(gemini-rate-limited 2026-04-15; restore to `xask gemini` for surface enumeration when canary passes)*, direct recon |
| **sentinel** | sonnet | Security auditor. Attacker-minded — hunts vulnerabilities, injection vectors, insecure configs, privilege escalation. Full tool access for scanning and remediation. | `xask --effort high codex` for exploit analysis, `xask gemini` for CVE prior art, `advisor()` for multi-hop chains |
| **critic** | sonnet | Approach-level adversarial reviewer. Challenges design decisions, architectural assumptions, and strategy choices. Distinct from reviewer (code bugs) and sentinel (security). | `xask --effort high codex` for deep design review, `xask gemini` for alternatives |
| **mutation-tester** | sonnet | Adversarial test suite validator. Generates code mutations, runs against tests, reports surviving mutants. | `xask --spark codex` (Layer-1 gate) for mutation generation, `xask gemini` for target discovery |
| **scribe** | sonnet | Writes Carpaccio milestone reports and executes git commits. One report + gate + commit per milestone — the auditable-trail anchor. Filter-exempt output role. | CC native (no xask gate) |
| **the-planner** | sonnet | Pre-execution planner. Owns Phase 0 data-walk and WWKD plan artifact generation. Dispatched by the-judge before executors. | CC native (no xask gate) |

## Dispatch table

The canonical Axis → Profile Mapping lives at `commands/references/xbreed-shared.md` (SSoT). This file previously carried a read-only copy; it was removed to eliminate drift. For routing rules, delegation targets, and xask effort tiers, consult shared.md directly.

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

## Escalation protocol

All sonnet teammates have two escalation paths:

| Method | When to use | Mechanism |
|--------|-------------|-----------|
| `advisor()` | In-session high-stakes reasoning, full-context review | CC-native, zero-params, blocks until opus 4.7 max responds. **Layer 0 — no xask gate.** |

`advisor()` forwards the teammate's entire conversation context to opus 4.7 max effort. Use it before committing to non-obvious architectural decisions, when stuck, or when a finding contradicts a peer. It is NOT subject to the xask 4-layer gate (it's CC-native, not cross-model).

## Swarm capabilities

- **Labrat swarm:** Up to 12 labrats in parallel. Fire-and-forget — no
  TaskCreate, they report via SendMessage + DESPAWN signal.
- **Gemini labrat swarm (universal):** Any agent role can fire a 1-call, 10-probe
  fan-out inside Gemini's context. Refire up to 2x (30 max probes).
