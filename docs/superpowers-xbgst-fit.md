# Superpowers × /xbgst — Fit Analysis (DRAFT)

> Produced via `/xbgst` Round 1 — team `superpowers-fit-gst-1744545000`.
> 6 axes, 6 teammates (3 gemini / 3 codex / mix), 1 distiller, Pareto-filtered.
>
> **Round 2 (2026-04-13, team `sp-xbgst-impl-1744545600`):** M1-M5 re-synthesised with 4 teammates + distiller. All five moves survive with amendments — evidence schema shipped as atomic bundle with distiller passthrough + closed-enum exempt allowlist; M2 reformulated to test-output pair (no SHA ceremony) with executor.md rewrite as prerequisite pair; M4 demoted from "canonical" to "reference" to preserve xbreed-shared.md SSoT; M5 taxonomy split into three buckets (10 cuts, not 11).

## AXES JUDGED
integration-surface · correctness-fit · complexity-subset · adversarial-breakage · empirical-probe · cross-axis-callsites

## Core finding

**Don't embed the Superpowers pipeline inside subagent briefs.** The two systems solve the same problem ("too fast, too shallow") with opposite strategies — Superpowers serializes with HARD-GATES, /xbgst parallelizes with Pareto. Forcing both onto one subagent silently drops the Superpowers gates (recency bias wins) and produces frontier pollution by incommensurable artifacts (specs vs. candidate moves).

The fit is **structural, not instructional.** Superpowers contributes *principles* to /xbgst's filter schema; it does not contribute *skills* to subagent briefs.

## SYNTHESIS — five surviving moves

### M1 · Evidence-field Pareto gate · [high confidence, 5-source convergence]
**Source principle:** `verification-before-completion`
**Change:** Phase 3 Pareto filter schema requires an `evidence:` field on every proposed move. Empty is allowed only for non-executable axes (docs, coordination, research). For executor / simplifier / mutation-tester moves, evidence = fresh verification output (test suite exit code, diff, SHA pair). Moves without required evidence are **dropped, not scored** — the verification discipline is enforced by the filter, not by the agent's willingness to comply.
**Why selective, not blanket:** Labrat falsified the universal framing — exit 0 ≠ semantic completeness, and docs/coordination tasks have no runnable command. The schema is task-aware.

### M2 · Executor-lane `|godspeed-impl` variant · [moderate]
**Source principle:** Superpowers TDD red-green-refactor, scoped
**Change:** Replace the raw `|godspeed` suffix for executor spawns only with `|godspeed-impl`. Preserves no-clarifying-questions and parallel-tool-calls directives; adds: *"A move is a complete TDD cycle — failing-test commit SHA + implementation commit SHA + passing CI output. Incomplete cycles are not Pareto-eligible."* All other teammate roles inherit plain `|godspeed`; TDD is out-of-scope for research/critique lanes by domain, not waived.

### M3 · Pre-flight brainstorming as user ritual, not pipeline · [high, 3-source]
**Source principle:** Superpowers `brainstorming` HARD-GATE
**Change:** Document in `/xbgst` that **Superpowers brainstorming runs before `/xbgst` is invoked**, producing the approved spec that becomes `/xbgst`'s input prompt. Never inside the walk. This eliminates the "stop asking clarifying questions" contradiction at the architecture level.

### M4 · `dispatching-parallel-agents` as canonical reference · [moderate]
**Source principle:** `dispatching-parallel-agents`
**Change:** Cite `superpowers:dispatching-parallel-agents` in `xbreed-shared.md` Phase 2 as the canonical pattern for crafted-brief + isolated-context + concurrent-dispatch. This is already what /xbgst does; naming it reduces drift and gives teammates a shared mental model.

### M5 · Explicit cut-list · [moderate]
Skip 10 Superpowers skills, grouped by reason:

- **Serialization-incompatible** (serial human-gate checkpoints contradict godspeed): `systematic-debugging`, `executing-plans`, `receiving-code-review`.
- **Lifecycle-position** (pre-`/xbgst` ceremony or post-`/xbgst` cleanup — neither phase exists inside the runtime loop): `writing-plans`, `finishing-a-development-branch`, `requesting-code-review`, `using-git-worktrees`, `using-superpowers`.
- **Domain/superseded** (addressed by /xbgst's own primitives): `subagent-driven-development`, `writing-skills`.

`test-driven-development` is kept selectively via M2 (red-before-green encoded in executor-lane evidence). YAGNI everywhere else.

## CONFLICTS

- **C1 (resolved against connector):** *Distiller + TDD makes distillation executable spec* (gemini / connector) vs. *cut TDD entirely* (simplifier). Resolution: M1's `evidence:` field achieves the empirical-falsifiability property without re-importing the TDD iron law. Reject connector's variant; keep the end-state.
- **C2 (resolved via reviewer's structural bridge):** *Embed verification gate inside the walk* (gemini / scout) vs. *Defer gate entirely to judge post-frontier* (codex / critic). Resolution: gate lives in the **filter schema** (judge-side), teammates only **attach** evidence (subagent-side). Both halves satisfied; neither "embedded in brief" nor "deferred past frontier."
- **C3 (resolved by labrat):** Universal high-leverage claim for `verification-before-completion` FALSIFIED — selective gate is correct framing. Encoded in M1.

## IMPLEMENTATION SKETCH

- **files:**
  - `commands/references/xbreed-shared.md` — add "Pareto filter evidence schema" section; cite `dispatching-parallel-agents` under Phase 2.
  - `commands/xbgst.md` — document pre-flight brainstorming expectation; replace Phase 3 description with evidence-gated filter.
  - `templates/agents/executor.md` — add `|godspeed-impl` variant block.
- **sequencing:** M3 + M4 (documentation, no behavior change) → M1 (schema change, one test) → M2 (executor-only, lane-scoped). M5 is the absence-change: no imports land.
- **tests:** one fixture — a malformed move (executor-lane proposal with no `evidence:` field) reaches the Pareto filter; expected: dropped with reason, not scored.

## OPEN QUESTIONS

- Distiller output schema — should SYNTHESIS_READY carry the `evidence:` field upstream, or is it attached only at teammate-proposal granularity?
- Non-executable axes (scout, connector) need a **negative evidence convention** (e.g., `evidence: none — research axis`) or the filter will silently reject them.

## AXES FINAL STATE
| Axis | Improved? | Note |
|---|---|---|
| integration-surface | ✅ | M1+M4 name the clean joinpoints |
| correctness-fit | ✅ | C1+C2 structurally resolved |
| complexity-subset | ✅ | M5 rejects 11 skills |
| adversarial-breakage | ✅ | M3 dissolves the recency-bias failure mode |
| empirical-probe | ✅ | labrat's falsification preserved as "selective gate" |
| cross-axis-callsites | ◻️ | matrix emitted but connector's top pick was Pareto-rejected |

Frontier stable. Round 1 exits.
