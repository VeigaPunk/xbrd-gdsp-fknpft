# xbreed harness mission — R2 report

**Mission:** `/xbgst /wwkd | godspeed` against `docs/reports/xbreed-harness-charter-2026-04-17.md`
**Team:** `xhc-r2-0417` (R2 of xbreed harness charter; R1 team crashed mid-session, recovered)
**Date:** 2026-04-17
**Scribe:** `ccs-scribe-harness-r2` (teammate-mode, dispatched concurrent with Pareto walk)
**Audit_hash:** `03e0dd46f12e5e1e0203a5633e23295661e4a28ce7c948dbcbfd1af9036569a5` [recomputed MATCH]
**R1 ref:** `docs/reports/xbreed-harness-r1-2026-04-17.md` (commit `aa4aa78`)

---

## 1. Mission scope

### Charter items (3)

| Item | Axis | Topic | R1 status |
|---|---|---|---|
| E | effort | Effort-tier override precedence (settings.json xhigh beats frontmatter medium) | DEFERRED — repro gate open (M-E1-AMENDED) |
| T | xask | xask as first-class native CC tool (not MCP) | ACCEPTED R1 (M-T1 — out-of-scope fold, HIGH) |
| B | batch | True-concurrent batch dispatch / tmux pane-cap | ACCEPTED R1 (M-B1 — cap formula, HIGH); implementation deferred to R2 |

### R1 outcomes carried into R2

- **M-B1 (ACCEPTED HIGH):** Dynamic cap formula `practical_cap = WIN_H − (current_panes + team_size − 1)`, fail threshold `MIN_ROWS=8`. R2 dispatch: implement preflight in xbreed binary (red→green TDD).
- **M-T1 (ACCEPTED HIGH):** xask-native-tool is out-of-scope (no user-space non-MCP registration surface in CC 2.1.112). Fold into xbreed-shared.md:92 out-of-scope paragraph. Rolled into R1 commit.
- **M-E1-AMENDED (DEFERRED LOW):** Cannot close without mechanical observation of effective effort from within CC 2.1.112 teammate. R2 dispatch: `ccs-labrat-effort-mechobs-r2`.
- **M-BE1 (DEFERRED MED, same-model capped):** Effort-weighted pane accounting — structural argument only; no cross-model validation. R2 dispatch: `cdx-labrat-crossmodel-be1-r2`.
- **M-D1 (DEFERRED LOW):** Labrat-N batch exemption from pane-cap gate — forward recommendation, no R2 redispatch.

---

## 2. R2 scope

Three R2 teammates dispatched in parallel:

1. `ccs-labrat-effort-mechobs-r2` — axis E: mechanical observation of effective effort in CC 2.1.112 teammate
2. `cdx-executor-preflight-cap-r2` — axis B: implement `xbreed precheck pane-cap --team-size N` (red→green TDD)
3. `cdx-labrat-crossmodel-be1-r2` — axis B×E: cross-model validation of M-BE1 effort-weighted pane accounting

---

## 3. Phase 2 teammate reports

### 3.1 Proposal 1 — `ccs-labrat-effort-mechobs-r2` (axis E, M-E1-R2)

**HYPOTHESIS:** Can a CC 2.1.112 teammate mechanically observe its own effective effort tier from within the running process?

**METHOD:** 6 parallel mechanical probes:
1. `printenv` — scan for CLAUDE_CODE_EFFORT_LEVEL or similar env vars
2. `/proc/$$/environ` — low-level process environment scan
3. Team config member entry inspection — check if "effort" field propagates
4. Frontmatter check — confirm own agent `.md` frontmatter effort: field
5. `~/.claude/settings.json` — read global effortLevel
6. `xask --spark codex` for CC docs — query codex-spark on documented teammate propagation

**RESULT:** Observation IMPOSSIBLE. Verdict: **epistemic-not-ergonomic ceiling.**

Env vars confirmed present: `CLAUDECODE=1`, `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`
Env var confirmed absent: `CLAUDE_CODE_EFFORT_LEVEL` (not set, not propagated)
Team config member entry: NO "effort" field
labrat.md frontmatter: NO "effort:" field
`~/.claude/settings.json:45`: `effortLevel=xhigh` (session-wide default)

**Codex-spark raw output (verbatim):**

```
No, there is no documented mechanism that lets a running subagent/teammate read its own
effective effort level from process state (env, /proc, or MCP introspection). For
teammate mode: docs do not list effort in the subagent fields that propagate to
teammates (tools + model only); therefore, frontmatter effort is not documented as
winning there. Teammates are treated as their own regular sessions, so effort comes
from that session context (/effort, settings/env) rather than teammate-specific subagent
effort override.
```

**Hypothesis resolution (3-way):**

- **(a) CC bug — frontmatter should win per docs:** REJECTED. Documented behavior — effort is not in the propagated teammate fields (tools + model only propagate).
- **(b) Teammate-mode ≠ subagent-delegation spawn path:** CONFIRMED. Subagent delegation (Agent tool with non-teammate semantics) honors frontmatter effort; teammate mode uses session effort. These are architecturally distinct paths.
- **(c) Prior labrat methodology error:** CONFIRMED as contributing. The R1 labrat null result was correct; the "out-of-scope" verdict conflated the two spawn paths and prematurely called closure.

**Operative effective effort for ALL current teammates:** `xhigh` (session setting, `~/.claude/settings.json:45`).

**Reachable fix (not yet probed):** `CLAUDE_CODE_EFFORT_LEVEL=medium` env var pre-spawn — if this cascades into teammate processes, it provides the ergonomic override without session-wide downgrade. Designated M-E2-R2 (gap finding, optional).

---

### 3.2 Proposal 2 — `cdx-executor-preflight-cap-r2` (axis B, M-B1-R2)

**HYPOTHESIS:** The M-B1 cap formula is implementable as `xbreed precheck pane-cap --team-size N` with red→green TDD cycle, clean clippy, and exit-code semantics.

**METHOD:** Red-before-green TDD. Write failing test first (`error[E0432]: unresolved import xbreed::precheck`), implement public API to green, validate edge cases.

**RESULT:** Implementation shipped. 10/10 tests passing.

**New files:**

- `src/precheck.rs` — public API: `MIN_ROWS=8` constant, `CapResult` enum `{Ok, Fail{panes_in_use, cap, team_size}, TmuxUnavailable}`, `compute_cap(win_h, current_panes, team_size) -> CapResult` pure function, `run(team_size) -> anyhow::Result<CapResult>` live-tmux function
- `tests/precheck_pane_cap.rs` — 10 tests covering: `constants_correct`, `zero_team_size_always_ok_nonempty_window`, `fresh_session_single_pane_ok`, `window_already_too_small_fails_regardless`, `exactly_at_cap_boundary_ok`, `one_below_cap_boundary_fails`, `fail_carries_team_size`, `fail_carries_panes_in_use`, `large_team_in_normal_window_fails`, `reasonable_team_in_normal_window_ok`

**Modified files:**

- `src/lib.rs` — `pub mod precheck;` added
- `src/cli.rs` — `Precheck { check: PrecheckAction }` subcommand + `PrecheckAction::PaneCap { team_size: u32 }` added
- `src/main.rs` — `Commands::Precheck` match arm dispatching to `xbreed::precheck::run()` with exit-code semantics

**Cap formula (implemented):**

```
practical_cap = WIN_H − (current_panes + team_size − 1)
Fail when: practical_cap < MIN_ROWS (= 8)
```

Edge cases verified by tests:
- `team_size=0`: no-op via `saturating_sub` arithmetic (always Ok)
- exactly-at-boundary (`WIN_H=9, current=1, team=1`): `practical_cap=8=MIN_ROWS` → Ok
- one-below-boundary (`WIN_H=8, current=1, team=1`): `practical_cap=7<8` → Fail

**Exit-code semantics (judge-verified at runtime):**
- `CapResult::Ok` → exit 0, stdout: `"pane-cap ok: team_size=N fits"`
- `CapResult::TmuxUnavailable` → exit 0, stdout: `"tmux not detected, cap check skipped"` (fail-open)
- `CapResult::Fail` → exit 1, stderr: `"X panes in use, cap Y, cannot spawn Z — shutdown idle teammates first"`

**RED gate:** `error[E0432]: unresolved import xbreed::precheck` — confirmed pre-implementation failure.
**GREEN gate:** `cargo test precheck` → 10/10 passing.
**Lint:** `cargo clippy` clean. `cargo fmt --check` clean.

**Rejected alternative:** Extending `xbreed guard` with `--batch-size` flag. Guard reads from stdin hook (different protocol — UserPromptSubmit hook injection path). Preflight is a stand-alone pre-spawn gate with its own invocation context. Architecturally distinct; extending guard would conflate two separate enforcement points.

---

### 3.3 Proposal 3 — `cdx-labrat-crossmodel-be1-r2` (axis B×E, M-BE1-R2)

**HYPOTHESIS:** M-BE1 "xhigh-effort teammates are meaningfully longer-lived, warranting effort-weighted pane accounting" — cross-model validation via codex-spark.

**METHOD:** Single `xask --spark codex` query framed to invite contradiction of the M-BE1 structural claim.

**RESULT:** PARTIAL CONCUR with REFRAME.

**Codex-spark raw output (verbatim):**

```
The counter-argument is important and mostly correct as first-order framing. Tool
latency + idle-before-despawn are effort-independent baselines. But effort tier is not
ignorable: higher-effort models can spend more wall-clock in reasoning loops, retries,
and extra tool calls. Right model: pane-lifetime = fixed overhead + variable
compute+reasoning time — only the variable part should scale by effort tier. So
effort-tier weighting is not a category error if applied as a multiplier on estimated
active compute, not as the whole capacity unit. Suggested policy: weighted occupancy
model — base + effort_factor × EWMA(active_runtime); revisit factors after 50–100
samples.
```

**Verdict:** Does NOT promote to HIGH. Stays MED.

Structural reframe accepted: `pane-lifetime = fixed overhead + variable compute×effort_factor`. The raw-headcount formula (M-B1) remains correct for R2 — it operates on the fixed-overhead model, which is accurate as a conservative gate. The effort-weighted refinement is telemetry-gated future work (50–100 samples before EWMA factors are empirically stable). M-BE1 is not a contradiction of M-B1; it is a structural addendum that requires measurement before implementation.

---

## 4. Distiller synthesis

**Distiller:** `ccs-distiller-harness-r2`
**Audit_hash:** `03e0dd46f12e5e1e0203a5633e23295661e4a28ce7c948dbcbfd1af9036569a5` [recomputed MATCH]

4 moves (3 primary + 1 optional gap):

| Move | Axis | Confidence | Status |
|---|---|---|---|
| M-E1-R2 | E | HIGH (cross-model confirmed) | Hypothesis B confirmed; epistemic-not-ergonomic ceiling; operative effort xhigh session-wide |
| M-B1-R2 | B | MED (single-source verified) | Preflight pane-cap implementation shipped; 10/10 tests; formula verified |
| M-BE1-R2 | B×E | MED (same-model cap) | Cross-model reframe: pane-lifetime = fixed overhead + variable compute; stays MED; telemetry-gated |
| M-E2-R2 | E | MED (gap, optional) | `CLAUDE_CODE_EFFORT_LEVEL=medium` env pre-spawn as reachable ergonomic fix; unprobed; labrat gate required before R3 execution |

**Same-model cap applied:** M-BE1-R2 uses both cdx-prefix sources (cdx-executor + cdx-labrat). Cap holds at MED.
**Cross-model confirmation:** M-E1-R2 codex-spark quote corroborates CC docs on teammate propagation fields.

---

## 5. EVIDENCE AUDIT (R2 — pre-Pareto)

```
EVIDENCE AUDIT: 4 moves with evidence, 0 moves without, 0 dropped, 0 spoof_flagged
```

audit_hash `03e0dd46f12e5e1e0203a5633e23295661e4a28ce7c948dbcbfd1af9036569a5` recomputed MATCH with distiller's committed hash — round auditable, no SPOOF_SUSPECT.

Evidence citations:
- M-E1-R2: `~/.claude/settings.json:45` effortLevel=xhigh [literal]; CLAUDE_CODE_EFFORT_LEVEL absent [printenv]; codex-spark raw_output [verbatim quote above]; CC docs teammate propagation fields (tools + model only) [evidence_unverified: external]
- M-B1-R2: `src/precheck.rs` (10/10 tests pass) [local file]; `~/.claude/hooks/adaptive-panes.sh:23` MIN_ROWS=8 [literal — inherited from R1 verification]; cargo clippy/fmt clean [toolchain output]
- M-BE1-R2: codex-spark raw_output [verbatim quote above]
- M-E2-R2: Forward gap — evidence_schema_exempt (no gate yet run)

---

## 6. Pareto filter verdicts

### ACCEPTED

#### M-E1-R2 — Effort-tier ceiling: epistemic-not-ergonomic (axis E) — HIGH

- **Claim:** CC 2.1.112 teammate-mode does not propagate effort from Agent tool spawn args or frontmatter. CLAUDE_CODE_EFFORT_LEVEL absent from env. Team config member entry has no effort field. Effective effort = session setting (xhigh). This is documented behavior, not a CC bug (hypothesis B confirmed; hypothesis A rejected).
- **Significance:** All aspirational effort-tier memories are NO-OP in teammate-mode. The only ergonomic fix is session-wide env pre-spawn (M-E2-R2, unprobed) or accepting xhigh-for-all baseline.
- **Axis E frontier:** CLOSES — ceiling is epistemic (architectural, documented), not ergonomic (misconfigured).

#### M-B1-R2 — Preflight pane-cap implementation (axis B) — MED

- **Claim:** `xbreed precheck pane-cap --team-size N` implemented and shipped. Formula `practical_cap = WIN_H − (current_panes + team_size − 1)`, fail threshold MIN_ROWS=8. 10/10 tests pass including all edge cases. CLI wired in src/main.rs with correct exit-code semantics.
- **Evidence:** `src/precheck.rs`, `tests/precheck_pane_cap.rs`, `src/cli.rs:71-86`, `src/main.rs:69-90` — all primary-source verified in this session.
- **MED (not HIGH):** Single implementation pass; no integration-level chaos test (N=10 batch) yet. Pure-function tests are comprehensive; live-tmux path tested only at CLI level (judge runtime verification).

#### M-BE1-R2 — Structural reframe: pane-lifetime model (axis B×E) — MED (ACCEPT-as-refinement)

- **Claim:** The raw-headcount formula (M-B1) is correct for the current gate. The structural refinement `pane-lifetime = fixed overhead + variable compute×effort_factor` is not a contradiction — it's a more precise model that requires 50–100 samples of EWMA(active_runtime) before effort-weighting factors can be calibrated. Same-model cap holds at MED.
- **Action:** No implementation change to M-B1 formula. M-BE1 telemetry-gated for future round after real-world samples accumulate.

### DEFERRED to R3

#### M-E2-R2 — `CLAUDE_CODE_EFFORT_LEVEL=medium` env pre-spawn probe (axis E) — MED (gap)

- **Claim:** If `CLAUDE_CODE_EFFORT_LEVEL=medium` is set in the parent-session environment before spawning teammates, it may cascade into teammate processes (env propagation) and provide a session-level ergonomic override.
- **Evidence:** None yet — gap identified from M-E1-R2 findings; labrat probe required.
- **R3 gate:** Spawn a teammate with `CLAUDE_CODE_EFFORT_LEVEL=medium` pre-set. Teammate runs `printenv | grep CLAUDE`. If env var is present → confirm it actually influences model effort selection. If positive → R3 executor ships thin env-prefix wrapper or documents the env-prefix pattern in xbreed briefs. If negative → R3 closes with second ceiling note.

---

## 7. CONFLICTS (R2)

None. All 3 teammate proposals were mutually consistent and addressed distinct move slots. No CONFLICTS_RELAY items.

---

## 8. Memory invalidations (protocol win)

The M-E1-R2 finding reveals that 3 aspirational effort-tier memories are NO-OP in teammate-mode. These memories document INTENDED behavior that is architecturally unreachable via Agent tool teammate spawn path (CC 2.1.112):

| Memory file | Tier claim | Status |
|---|---|---|
| `feedback_sonnet_effort_tiers.md` | distiller/simplifier/scribe at medium | NO-OP in teammate-mode |
| `feedback_cco_opus_high.md` | cco- subagents at role-specific effort (max/xhigh/high) | NO-OP in teammate-mode |
| `feedback_the_planner_wwkd.md` | the-planner opus 4.7 high + wwkd Layer 0 | effort tier NO-OP in teammate-mode |

**Clarification:** The memories are NOT wrong as aspirational targets or as documentation of INTENDED effort routing. They correctly document what the session operator wants. The gap is architectural: `effort:` frontmatter and Agent tool `effort` args do not propagate to teammates in CC 2.1.112. Operative effort for all teammates = `xhigh` (session setting).

**Recommended action (not in scribe scope):** Update each memory to add a `[teammate-mode: NO-OP — use CLAUDE_CODE_EFFORT_LEVEL env pre-spawn per M-E2-R2]` annotation. Judge or operator owns this update.

---

## 9. R3 charter

**Rationale:** M-E2-R2 is labrat-probeable in one fast xask probe. Round-2-always-runs invariant satisfied (R2 improved axes E + B from R1 baseline). R3 is justified IFF M-E2-R2 env probe returns positive.

### R3 dispatch plan

**Phase 2 (parallel):**

- `ccs-labrat-effort-env-r3` — probe: does `CLAUDE_CODE_EFFORT_LEVEL=medium` in parent env cascade into teammate processes? Method: team config + `printenv | grep CLAUDE` inside spawned teammate.

**Conditional on positive probe result:**

- `cdx-executor-env-prefix-r3` — implement thin env-prefix pattern: document or wrap teammate spawn to include `CLAUDE_CODE_EFFORT_LEVEL=<tier>` based on agent template frontmatter. Delivery: either a shell wrapper or a documented xbreed brief directive + xbreed-shared.md section update.

**R3 exit condition:**
- Positive probe → executor implements + R3 commit ships env-prefix pattern
- Negative probe → R3 scribe documents second ceiling note (env path also blocked) → final DRAFT

**R3 scribe:** Dispatch `ccs-scribe-harness-r3` concurrent with Pareto walk, per `feedback_scribe_per_round.md`.

---

## 10. Commit plan (R2)

Single R2 commit staged (1:1:1 invariant — one milestone, one report, one commit):

- `docs/reports/xbreed-harness-r2-2026-04-17.md` (this report)
- `src/precheck.rs` (new — M-B1-R2 implementation)
- `tests/precheck_pane_cap.rs` (new — 10 tests)
- `src/lib.rs` (modified — `pub mod precheck;` added)
- `src/cli.rs` (modified — `Precheck` subcommand + `PrecheckAction::PaneCap`)
- `src/main.rs` (modified — `Commands::Precheck` match arm)

Pre-commit checklist:
- [ ] `git status` — verify staged files match above list
- [ ] `git diff --cached --stat` — confirm no stray changes
- [ ] `cargo clippy && cargo test && cargo fmt --check` — verify loop (CLAUDE.md mandate)
- [ ] Commit message: `M-B1-R2 preflight pane-cap + R2 report — gate: cargo test 10/10 pass`

Commit message format: `feat(harness): M-B1-R2 preflight pane-cap + R2 report — gate: cargo test 10/10 pass`

---

## 11. Provenance + recovery note

### Provenance

- **R2 session:** `xhc-r2-0417` — fresh team spawned after R1 crash recovery
- **R1 crash:** Prior session crashed after distiller SYNTHESIS_READY + critic AMENDMENT, before scribe/commit. 8 orphan processes force-killed via `xbreed-cleanup --force`.
- **R1 recovery:** team-lead in-session scribe (post-crash exception per `feedback_scribe_per_round.md`). inbox 621 lines (70+ messages); audit_hash `850e17e1` recomputed MATCH.
- **Lesson documented:** Snapshot inbox before `xbreed-cleanup --force`. Inbox history is destroyed by cleanup; recovered content was in team-lead's in-context DM buffer (lucky). Next crash: copy `.claude/teams/<id>/inbox` before running cleanup.

### Session-crash → new-team recovery pattern

The R1→R2 handoff worked cleanly via on-disk state:
- R1 reports committed to git (durable)
- R2 team spawned fresh with full R1 context in brief
- Only loss: team inbox history (transient; not load-bearing after synthesis complete)

This validates the `feedback_scribe_per_round.md` + commit-per-round discipline as the crash-recovery anchor. Git history is the real persistence layer; teammate DM history is ephemeral.

---

*Report written by `ccs-scribe-harness-r2` (teammate-mode, CC-native, filter-exempt per documentation axis). Concurrent with judge's Pareto walk per `feedback_scribe_per_round.md`.*
