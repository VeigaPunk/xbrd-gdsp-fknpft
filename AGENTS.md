# Agent Roster

xbreed uses 14 perspective agents. Canonical definitions live in
`~/.claude/agents/` (user-managed). Claude Code discovers them there as
subagent types. The former `templates/agents/` mirror was removed 2026-04-17
to kill source-of-truth ambiguity; recover a historical snapshot via
`git show 0ac5571:templates/agents/<name>.md` if needed.

## Agents

| Agent | Model | Role | Delegation bias |
|-------|-------|------|-----------------|
| **the-judge** | **opus 4.7 · high** (orchestrator exception — user directive 2026-04-17: every other teammate is sonnet-medium, judge stays opus for intermediation depth; downgraded from xhigh 2026-04-19) | Orchestrator and arbiter. Names axes, dispatches specialists, applies Pareto filter, drafts implementation. Top of the stack. | In-session (spawns others, never spawned) |
| **scout** | sonnet · medium | Research lens. Finds what exists outside the repo — libraries, docs, prior art, release notes. Prefers Gemini delegation with librarian loadout. | `xask --effort medium gemini` (LOCKED default, librarian loadout, `# ThinkingBudget: 4096`; codex fallback only on gemini 429 with `[xask dry]` marker per Layer 3) |
| **reviewer** | sonnet · medium | Surgical code reviewer. Finds the bug that ships to prod. Delegates to Codex always. | `xask -R codex` (review lane — gpt-5.4-mini; add `-F` for full gpt-5.4 / 1.05M ctx on large-diff review) |
| **labrat** | sonnet · medium | Expendable single-shot probe. Tests one hypothesis cheap and fast. State nuked on despawn. | `xask --spark codex` (default), `xask gemini` for long-context |
| **connector** | sonnet · medium | Cross-axis pattern matcher. Sees the whole table, calls out unusual connections and second-order effects. Breadth over depth. | `xask --effort high gemini` (LOCKED — no codex fallback even on 429), `advisor()` for reasoning |
| **distiller** | sonnet · medium | Deduplicates N parallel findings, flags contradictions, assigns confidence scores. Text synthesis with optional tool verification. Sits between workers and the-judge. | Spawned after peer DMs land, before judge Pareto filter; persistent across rounds |
| **executor** | sonnet · medium | Writes code, runs tests, returns results. Stateless by default — scoped to one subtask. | `xask --spark codex` (Layer-1 gate), `advisor()` for reasoning |
| **simplifier** | sonnet · medium | YAGNI enforcer. Finds what to delete. If removing it passes all tests, it was dead. | Direct analysis + deletion verification |
| **the-revenger** | sonnet · medium | Reverse engineering specialist. Maps behavior, infers intent, reproduces functionality. Godspeeded by default. 4-phase RECON/PROBE/MODEL/BUILD protocol. | `xask -R -F codex` for RECON (full gpt-5.4 / 1.05M ctx); `xask -R codex` (mini) for narrow surface enum; direct recon fallback |
| **sentinel** | sonnet · medium | Security auditor. Attacker-minded — hunts vulnerabilities, injection vectors, insecure configs, privilege escalation. Full tool access for scanning and remediation. | `xask -R codex` for exploit analysis, `xask gemini` for CVE prior art, `advisor()` for multi-hop chains |
| **critic** | sonnet · medium + Layer-0 `Skill('heuer-planning')` | Approach-level adversarial reviewer. Challenges design decisions, architectural assumptions, and strategy choices. Distinct from reviewer (code bugs) and sentinel (security). | `xask -R codex` for deep design review, `xask gemini` for alternatives |
| **mutation-tester** | sonnet · medium | Adversarial test suite validator. Generates code mutations, runs against tests, reports surviving mutants. | DUAL: `xask --spark codex` (single mutation, ≤4 targets — fast spot-check) OR `xask --effort low gemini` 10-probe fanout (≥5 targets or breadth discovery, `# ThinkingBudget: 512`) — see shared.md Layer-1 gate for selection rule |
| **scribe** | sonnet · medium | Writes Carpaccio milestone reports and executes git commits. One report + gate + commit per milestone — the auditable-trail anchor. Filter-exempt output role. | CC native (no xask gate) |
| **the-planner** | sonnet · medium + Layer-0 `Skill('wwkd')` | Pre-execution planner. Owns Phase 0 data-walk and WWKD plan artifact generation. **Dispatched FIRST by the-judge at Phase 0** to map the skeleton with a defensible baseline before specialist dispatch. | CC native (no xask gate); wwkd skill auto-loaded at spawn |

## Dispatch table

The canonical Axis → Profile Mapping lives at `commands/references/xbreed-shared.md` (SSoT). This file previously carried a read-only copy; it was removed to eliminate drift. For routing rules, delegation targets, and xask effort tiers, consult shared.md directly.

## Naming convention

Teammates use `{prefix}-{role}-{suffix}` naming:

| Prefix | Model/CLI |
|---|---|
| `g-` | Gemini (via `xask gemini`) |
| `ccs-` | Claude Sonnet — standing teammate prefix (all teammates run sonnet medium post-2026-04-17) |
| `cco-` | Claude Opus 4.7 — reserved for `the-judge` only (orchestrator exception); no teammate currently uses this prefix |
| `cdx-` | Codex (via `xbreed ask codex`) |

Examples: `ccs-scout-docs`, `g-labrat-probe`, `cdx-reviewer-auth`, `ccs-executor-tests`

## Inter-Model Communication Protocol v0.2

All agents produce structured output using only the blocks appropriate to their
role. Minimal valid message = `[GOAL]` + one other block. See individual agent
files in `~/.claude/agents/` for per-role return formats.

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
