# Agent Roster

xbreed ships 8 perspective agents in `templates/agents/`. Copy them to
`~/.claude/agents/` for Claude Code to discover them as subagent types.

## Agents

| Agent | Model | Role | Delegation bias |
|-------|-------|------|-----------------|
| **the-judge** | opus | Orchestrator and arbiter. Names axes, dispatches specialists, applies Pareto filter, drafts implementation. Top of the stack. | In-session (spawns others, never spawned) |
| **scout** | sonnet | Research lens. Finds what exists outside the repo — libraries, docs, prior art, release notes. Prefers Gemini delegation with librarian loadout. | `xask gemini` with librarian loadout |
| **reviewer** | sonnet | Surgical code reviewer. Finds the bug that ships to prod. Delegates to Codex for deep reviews. | `xask codex` for deep reviews |
| **labrat** | sonnet | Expendable single-shot probe. Tests one hypothesis cheap and fast. State nuked on despawn. | `xask --spark codex` (default), `xask gemini` for long-context |
| **connector** | sonnet | Cross-axis pattern matcher. Sees the whole table, calls out unusual connections and second-order effects. Breadth over depth. | `xask gemini` for breadth, `advisor()` for reasoning |
| **distiller** | sonnet | Deduplicates N parallel findings, flags contradictions, assigns confidence scores. Text synthesis with optional tool verification. Sits between workers and the-judge. | Spawned after peer DMs land, before judge Pareto filter; persistent across rounds |
| **executor** | sonnet | Writes code, runs tests, returns results. Stateless by default — scoped to one subtask. | `xask --spark codex` (Layer-1 gate) for bounded tasks, `xask --effort high codex` for refactors, `advisor()` for reasoning |
| **simplifier** | sonnet | YAGNI enforcer. Finds what to delete. If removing it passes all tests, it was dead. | Direct analysis + deletion verification |
| **the-revenger** | opus | Reverse engineering specialist. Maps behavior, infers intent, reproduces functionality. Godspeeded by default. 4-phase RECON/PROBE/MODEL/BUILD protocol. | `xask gemini` for surface enumeration, direct recon |
| **sentinel** | sonnet | Security auditor. Attacker-minded — hunts vulnerabilities, injection vectors, insecure configs, privilege escalation. Full tool access for scanning and remediation. | `xask --effort xhigh codex` for exploit analysis, `xask gemini` for CVE prior art, `advisor()` for multi-hop chains |
| **critic** | sonnet | Approach-level adversarial reviewer. Challenges design decisions, architectural assumptions, and strategy choices. Distinct from reviewer (code bugs) and sentinel (security). | `xask --effort high codex` for deep design review, `xask gemini` for alternatives |
| **mutation-tester** | sonnet | Adversarial test suite validator. Generates code mutations, runs against tests, reports surviving mutants. | `xask --spark codex` (Layer-1 gate) for mutation generation, `xask gemini` for target discovery |

<!-- SYNC: read-only copy — source of truth is ~/.claude/commands/references/xbreed-shared.md Axis → Profile Mapping -->
## Dispatch table (the-judge reference)

| Axis family | Agent | Delegation |
|---|---|---|
| Research, prior art, outside-world | `scout` | `xask gemini` |
| Correctness, bugs, code review | `reviewer` | `xask codex` or `xreview` |
| Empirical probes, dry-runs | `labrat` (sonnet) | `xask --spark codex` (default), `xask gemini` for long-context |
| Code execution, implementation | `executor` | `xask --spark codex` |
| Cross-axis patterns, breadth | `connector` | `xask gemini` |
| Findings synthesis, dedup | `distiller` | spawned after peer DMs land, before Pareto filter |
| Deletion, YAGNI | `simplifier` | direct analysis |
| Reverse engineering, intent reconstruction | `the-revenger` | direct recon + `xask gemini` for surface enumeration |
| Security auditing, adversarial analysis | `sentinel` | `xask --effort xhigh codex` + `xask gemini` for CVEs |
| Pre-executor design, implementation planning | `Plan` (CC built-in) | CC native — no xask gate |
| Adversarial design, approach review | `critic` | `xask --effort high codex` or `xask gemini` |
| Test validation, mutation testing | `mutation-tester` | `xask --spark codex` |

## Umwelt Doctrine

Each specialist operates from its own umwelt — xask routes to the competence, not the role.

Every agent's xask dispatch profile IS its umwelt specification: the perceptual world it actually inhabits. Scout's umwelt is discovery and breadth (gemini-librarian). Reviewer's umwelt is surgical precision (codex-xhigh, temp 0.1–0.3). Labrat's umwelt is cheap-fast single-hypothesis probing (codex-spark). Connector's umwelt is cross-axis pattern matching (gemini-breadth). These are not job titles — they are perceptual profiles. Dispatching an agent outside its umwelt produces the tick-testing-for-vision failure: the agent appears incompetent because the task is outside what it can perceive, not because the capability is absent.

**The anti-pattern** is role-routing: assigning by job title rather than perceptual fit. LangGraph node labels are job titles. xask targets are site-fitness checks. Homogeneous model deployment behind a role-router (GPT-4-everywhere, Claude-everywhere) is the monoculture failure — Norway spruce assigned by timber-role regardless of site conditions. xbreed arrived at site-fitness routing through practice before having vocabulary for it; this section crystallizes that practice to prevent regression.

**Cross-umwelt dispatch is forbidden.** If a task requires a perceptual profile the assigned agent doesn't have, reassign — don't stretch.

**The axis schema is the legibility tax.** Specialists don't share a perceptual world; the only shared interface is the axis names in round briefings. Keep axes thin: minimum fields that enable coordination, not a complete ontology.

Each agent entry in the roster carries:

```
umwelt:
  model: <xask target>        # operative — determines routing
  blind_spots: [<list>]       # operative — what this agent cannot perceive; blocks wrong dispatch
```

`blind_spots` is the only required sub-field. It is what prevents mis-dispatch. `model`, `scope`, and `strengths` are advisory. A umwelt schema that grows mandatory enumerations and sub-typed scopes replicates the scientific-forestry failure at the roster layer — high-modernist imposition on agents whose perceptual world doesn't fit the taxonomy. Keep the schema thinner than what it describes.

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

## Project-scoped agents

These agents are installed in `~/.claude/agents/` but are **NOT judge-dispatchable** without
an explicit `scope_boundary` and domain brief. They are project-specific specialists.

| Agent | Model | Domain | Requires |
|-------|-------|--------|----------|
| **data-reconciler** | sonnet | Inventory reconciliation pipeline (Omie ERP, Python) | `scope_boundary` + domain brief with fixture paths |
| **pipeline-runner** | sonnet | Pipeline end-to-end test runner (inv-hxh fixtures) | `scope_boundary` + domain brief with test fixtures |

## Escalation protocol

All sonnet teammates have two escalation paths:

| Method | When to use | Mechanism |
|--------|-------------|-----------|
| `advisor()` | In-session high-stakes reasoning, full-context review | CC-native, zero-params, blocks until opus-max responds. **Layer 0 — no xask gate.** |
| `xask claude` *(deprecated — prefer `advisor()`)* | Legacy cross-model dispatch | Shell subprocess, 4-layer gate applies |

`advisor()` forwards the teammate's entire conversation context to opus-max effort. Use it before committing to non-obvious architectural decisions, when stuck, or when a finding contradicts a peer. It is NOT subject to the xask 4-layer gate (it's CC-native, not cross-model).

## Swarm capabilities

- **Labrat swarm:** Up to 12 labrats in parallel. Fire-and-forget — no
  TaskCreate, they report via SendMessage + DESPAWN signal.
- **Gemini labrat swarm (universal):** Any agent role can fire a 1-call, 10-probe
  fan-out inside Gemini's context. Refire up to 2x (30 max probes).
