# cco-Upgrade A/B Plan — 5-Axis Dedicated Round

**Team:** `cco-upgrade-ab-<future>` (to be instantiated)
**Round:** 0 (planning artifact — planner dispatched ahead of execution team)
**Date:** 2026-04-16
**Parent report:** `docs/reports/xbreed-superiority-r1-2026-04-16.md` §8 ("cco-Upgrade A/B Verdict — partial coverage")
**Author:** `cco-the-planner-cco-ab` (A6, team `ask-resilience-r2-0416`)

---

## TL;DR

- R1 produced an **N=2 selective verdict** (arch: convergent; novel: opus-better) from opportunistic mirrors layered onto an unrelated round. The verdict is credible but under-powered.
- This plan specifies a **dedicated 5-axis A/B run** — same brief, two harnesses (`model: "sonnet"` vs `model: "opus"`), five roles drawn from the R1-flagged "adversarial/synthesis" cluster: `distiller`, `critic`, `sentinel`, `reviewer` (on an adversarial brief), `connector`.
- Decision rule: **promote a role to `cco-` iff opus mirror catches ≥1 material brief error the sonnet mirror accepts, AND the material error would have shipped.** Convergent outputs keep sonnet.
- Assumed cost multiplier: **≈5× per-token for Opus 4.7 vs Sonnet 4.6** (documented as assumption; verify against current Anthropic pricing page before run — not pulled live by planner).
- N=5 with clear asymmetry (≥3 roles show material-error signal) is decisive; N=5 with mixed signal (1–2 roles) is ambiguous and triggers a Round-2 rerun of the ambiguous roles on a harder brief.

---

## 1. Data Walk (Phase 0)

### 1.1 Prior evidence surface (what exists)

From `docs/reports/xbreed-superiority-r1-2026-04-16.md` §8:

| Role (R1) | Sonnet harness output | Opus harness output | Signal |
|---|---|---|---|
| `ccs-reviewer-arch` vs `cco-reviewer-arch-mirror` | 70 transitive deps (corrected from 40) | 70 transitive deps (independent confirm) | **Convergent** — no opus signal on read-only enumeration |
| `ccs-reviewer-novel` vs `cco-reviewer-novel-mirror` | 7 verified novel claims | 4 verified + 2 partial + 2 rejected; **caught 3 factual errors in brief** post-commit | **Strong opus-better** on adversarial review |
| `cdx-labrat-cco-hypothesis` (theoretical probe) | n/a — zombied, never spawned | n/a | **Null** — cannot use |

**Verdict from R1:** selective upgrade, not blanket. Opus for adversarial/synthesis; sonnet for read-only/probes. Explicit recommendation (§8 final paragraph): *"running a 5-axis A/B in a dedicated future round before formalizing the upgrade."*

### 1.2 Harness model-parameter mechanism

- **Spawn-time control:** `Agent(subagent_type=<role>, model="opus"|"sonnet", ...)` — documented in the Agent tool schema (this conversation's system prompt).
- **Defaults:** `templates/agents/*.md` frontmatter currently binds each role to a default model (`sonnet` for most, `opus 4.7 max` for `the-revenger`, `the-judge`).
- **Override path:** the harness `model` parameter at spawn time takes precedence over template frontmatter. A/B trial does NOT require template edits — spawn-time override is sufficient.
- **Naming convention:** `{prefix}-{role}-{suffix}` where prefix encodes the model. For the A/B, sonnet-harness teammates use `ccs-` prefix, opus-harness teammates use `cco-` prefix. Same `{role}` and `{suffix}` across the pair.

### 1.3 Cost assumption

- **Anthropic pricing (assumption, NOT verified in-session):** Opus 4.7 ≈ 5× Sonnet 4.6 per million input+output tokens. This ratio has held across the Claude 4.x family and is the working assumption for this plan.
- **Verification hook:** before the run, maintainer pulls live prices from the Anthropic pricing page and substitutes the real ratio into the cost table below. A ratio in [3×, 7×] does not change the plan structure; outside that range, revisit the break-even threshold in §5.
- **Per-run budget estimate:** 5 roles × 2 harnesses × ≈10k output tokens/role ≈ 100k output tokens total. At 5× ratio, opus slice is ≈83% of the token cost of the run — absolute dollars negligible compared to the decision value.

### 1.4 What's missing (the reason this round exists)

- N=2 is too small to generalize. R1 mirrors were both `reviewer` variants on the same underlying brief corpus. Opus advantage on "catch brief errors" has not been stress-tested on roles where synthesis (distiller), design challenge (critic), or adversarial pattern-matching (sentinel, connector) dominate.
- No dedicated adversarial brief was used. R1's briefs were survey-style ("enumerate novel mechanisms"); error-rich briefs (with seeded factual errors) would exercise opus's catch-rate more aggressively.
- Cost/quality trade is informal — no explicit decision rule on when to promote.

### 1.5 Risks / blocking unknowns

- **R1:** If the seeded-error brief is the only differentiator, opus advantage may not generalize to organic briefs. Mitigation: also run one unseeded organic brief per role and compare.
- **R2:** Opus harness may be slower (longer reasoning traces), risking connector-stall-class issues (see memory: `feedback_connector_stall.md`). Mitigation: explicit reasoning cap in every opus brief, same cap as sonnet sibling.
- **R3:** Scorer bias — if a single judge scores both mirrors, judge may anchor on first-seen. Mitigation: blind-score against `move_id` (per existing Judge Blinding Protocol, `xbreed-shared.md:144-186`); distiller emits SYNTHESIS_READY without source labels.

---

## 2. WWKD Skeleton

### 2.1 What

Run a dedicated 5-axis A/B round where each axis dispatches a **sonnet-harness / opus-harness pair** on the SAME brief, capture per-pair deltas, and produce a per-role promotion/hold verdict plus a cost-aware aggregate recommendation.

**Success boundary:** we exit with one of three states per role — `PROMOTE cco-`, `HOLD ccs-`, or `AMBIGUOUS — rerun`. The round produces a commit-worthy report + a concrete edit to the Axis→Profile table in `commands/references/xbreed-shared.md` for any role that promotes.

### 2.2 Why

- R1 §8 is the direct trigger (recommendation #3 in R1 secondary follow-ups).
- The user's original hypothesis ("all sonnets → opus 4.7 with auto effort flag") was answered selectively by R1. Selective verdicts rest on weak evidence need to be either confirmed or overturned with N≥5 before a protocol edit ships.
- Silent drift: the fix pattern from R1 (promote distiller, reviewer-on-critical) has already leaked into the current round (this team — `cco-the-planner`, `cco-critic`, `cco-distiller`). Formalizing the rule converts ad-hoc per-round promotion into a documented convention.

### 2.3 Assumptions + Risks

- **A1:** 5× cost multiplier for opus vs sonnet (see §1.3).
- **A2:** Harness `model` parameter override is reliable and the opus-harness teammate behaves as opus 4.7 (not secretly downgraded). Validated in R1 (cco-reviewer-novel-mirror demonstrably caught errors sonnet missed — behavior consistent with opus, not sonnet masquerading).
- **A3:** The "adversarial/synthesis" cluster is the right focus. R1 already established read-only (arch, scout, executor) as convergent or null-signal. This round does not re-test read-only roles.
- **R (risk):** N=5 may be too small if signal is noisy. Pre-commit threshold: ≥3 of 5 roles show material-error asymmetry → promote cluster; ≤1 → hold cluster; 2 → rerun the ambiguous pair on a harder brief.

### 2.4 How (milestone dependency chain)

```
M1 (baseline review) → M2 (formalize brief template)
                         ↓
                   M3 (dispatch 5-axis A/B, parallel)
                         ↓
                   M4 (blind scoring)
                         ↓
                   M5 (source reveal + verdict)
                         ↓
                   Polish (aggregate methodology + protocol edit)
```

### 2.5 Escalation points

- **E1 (pre-dispatch):** If brief template has seeded errors that are too subtle (both harnesses miss) or too obvious (both harnesses catch), escalate to judge for brief recalibration before M3.
- **E2 (post-blind-score):** If per-role verdict is AMBIGUOUS on ≥3 of 5 axes, escalate — N=5 was insufficient, and the judge must decide whether to dispatch Round 2 immediately or defer.
- **E3 (post-reveal):** If SOURCE_MAP hash-check fails (Judge Blinding Protocol §Audit-commit handshake), round is invalid — rerun from M4.

---

## 3. Milestones

| # | Title | Gate command | Expected output | Executor |
|---|---|---|---|---|
| M1 | Baseline inventory from R1 | `grep -A3 "cco-reviewer" docs/reports/xbreed-superiority-r1-2026-04-16.md` | R1 N=2 data (arch convergent; novel opus-better) cited verbatim in M1 artifact | scribe (or planner-direct, CC-native) |
| M2 | Formalize A/B brief template | `test -f docs/briefs/cco-ab-template-<date>.md && wc -l docs/briefs/cco-ab-template-<date>.md` | Brief file exists, ≥1 seeded factual error documented in a `# seeded_errors` comment block, ≤150 lines | scribe (CC-native) |
| M3 | Dispatch 5-axis mirrored A/B (10 teammates) | `xbreed team list \| grep -c cco-ab-r1` returns 10 (or `TaskList` shows 10 Phase-2 tasks) | 5 sonnet + 5 opus teammates registered on team `cco-ab-r1`, all DM-posted within dispatch cycle | the-judge (orchestration); executor(s) dispatch |
| M4 | Blind-score mirrors (distiller SYNTHESIS_READY + audit_hash) | Distiller DM to judge contains `SYNTHESIS_READY` + `audit_hash: <64-hex>` | Per-move confidence + audit_hash posted; judge posts provisional Pareto scores citing hash, no source-map inspection | cco-distiller (per R1 verdict — synthesis is the role most needing opus) + the-judge |
| M5 | Reveal + per-role verdict | Distiller returns SOURCE_MAP; judge recomputes hash; verdict table emitted | 5-row verdict table: `role / sonnet_output / opus_output / material_delta / verdict (PROMOTE\|HOLD\|AMBIGUOUS)` | the-judge |
| Polish | Aggregate methodology + protocol edit | `grep -E "^\| cco-|^\| ccs-" commands/references/xbreed-shared.md` reflects new per-role verdicts; report at `docs/reports/cco-upgrade-r2-<date>.md` committed | (a) R2 report file written; (b) `xbreed-shared.md` Axis→Profile table updated for PROMOTE roles; (c) AGENTS.md + the-judge.md read-only copies synced (per SSoT rule, shared.md:55-56); (d) cargo test passes | scribe + reviewer (coherence-check per shared.md §Coherence Check) |

### 3.1 Milestone detail

**M1 — Baseline inventory.** Quote R1 §8 verbatim into the opening of the R2 report draft. Document which roles already have mirror data (reviewer-arch, reviewer-novel) and which are fresh (distiller, critic, sentinel, connector). Fresh roles carry the weight of this round's signal.

**M2 — Brief template.** One brief per role, each with these properties:
  - **Seeded errors:** ≥2 plausible-but-wrong factual claims embedded (example: a line:col citation that points to the wrong file, or a numerical stat off by a factor). Seeded errors are documented in a comment block at the brief foot so the judge can score catch-rate.
  - **Organic content:** the rest of the brief is a real task on this repo (e.g., "review `src/ask.rs` for W3 ghost-axis fixes" for the reviewer mirror). Output is usable even if the A/B is inconclusive.
  - **Same brief to both harnesses:** byte-identical. A/B is valid only if the only difference between pairs is the `model` parameter at spawn.
  - **Reasoning cap:** explicit `"Keep reasoning to ≤X tokens. Post finding directly."` — same cap for both harnesses (per memory `feedback_connector_stall.md`).

**M3 — Dispatch.** Spawn 10 teammates in one parallel batch. Naming: `ccs-{role}-ab` / `cco-{role}-ab`. Roles (and R1-relevance rationale):
  1. `distiller` — R1 §8 flagged synthesis as opus's strongest theoretical advantage; untested in R1.
  2. `critic` — adversarial/synthesis cluster core; R1 §8 explicitly recommends upgrade.
  3. `sentinel` — adversarial/security; R1 §8 recommends upgrade.
  4. `reviewer` (adversarial brief, not read-only) — differentiates from R1's convergent arch-review data point.
  5. `connector` — cross-axis pattern-matching; R1 did not cover; relevant because connector is already xask-locked to gemini high and the CC harness only frames the call (opus vs sonnet framing may or may not matter — worth testing).

**M4 — Blind scoring.** Follow Judge Blinding Protocol (`xbreed-shared.md:144-186`) exactly:
  - Distiller synthesizes all 10 outputs, emits SYNTHESIS_READY with per-`move_id` confidence + `audit_hash` (SHA-256 of sorted `[{move_id, source_prefix}]`).
  - Judge posts provisional Pareto verdicts against `move_id` — does NOT request SOURCE_MAP yet.
  - Scoring axes per pair: `material_error_catch_rate`, `framing_quality`, `brief_error_downgrades` (did opus catch a seeded error sonnet missed?), `cost` (tokens-out estimate), `time_to_post`.

**M5 — Reveal + verdict.** Judge requests SOURCE_MAP; recomputes audit_hash; verifies match; runs spot-check (random `move_id` → direct `confirm_model` DM to original proposer). On match + clean spot-check, per-role verdict emits:
  - **PROMOTE:** opus caught ≥1 seeded error sonnet missed AND opus framing was not materially worse on other axes. Role becomes `cco-{role}` default in Axis→Profile table.
  - **HOLD:** outputs convergent on the material axes. Sonnet is sufficient; no promotion.
  - **AMBIGUOUS:** mixed signal (opus better on one axis, worse on another). Triggers **exactly one** Round-2 rerun of that pair on a harder brief. **Termination bound:** if the rerun again returns AMBIGUOUS, the role is coerced to HOLD with a `hold_by_ambiguity` tag in the verdict table — not rerun further. No recursive rerun path; the loop is bounded to depth 1. Rationale: closes the non-terminating-rerun spec defect flagged by cco-critic-compile-gate pre-R3.

**Polish — Methodology + protocol edit.** A6 score aggregation:
  - **Unit of analysis (revised pre-R3, post cco-critic-compile-gate preview):** the statistical unit is the **brief** (A/B trial), NOT the axis. The 5 axes are *raters* per brief, returning {-1, 0, +1} each. Prior draft conflated the two; correction lands here.
  - Per-brief scores normalized to {-1, 0, +1} on each of 5 rater axes (negative = opus worse, 0 = convergent, +1 = opus better).
  - Per-brief sum is the "opus-net-benefit" score. Threshold: sum ≥ +2 per brief promotes; sum ≤ 0 per brief holds; sum = +1 is AMBIGUOUS.
  - **Known calibration gap:** the +2 threshold is heuristic, not power-calibrated. False-positive rate under null depends on inter-rater reliability ρ; not simulated in this plan. R3 critic pass is expected to land a power-analysis patch (likely via `xask --effort high codex` on unit-correct framing: k = brief count, 5 raters per brief, detect Δ_MDE as function of ρ).
  - Cluster verdict: if ≥3 of 5 **roles** (not briefs) PROMOTE across their respective briefs, update `xbreed-shared.md` Axis→Profile table cluster-wide for the adversarial/synthesis family. Else, per-role updates only.
  - Protocol edit lands as a PR-style commit that also syncs `AGENTS.md` + `the-judge.md` read-only copies (SSoT rule, `xbreed-shared.md:55-56`).

---

## 4. Dependencies

- M1 → M2 (need baseline to calibrate seeded-error severity).
- M2 → M3 (brief template must exist before dispatch).
- M3 → M4 (all 10 teammates must post before synthesis).
- M4 → M5 (blind scoring must commit before reveal, per Judge Blinding Protocol).
- M5 → Polish (verdicts must exist before protocol edit).

**No parallel shortcuts.** The blinding protocol specifically forbids early source reveal — M4/M5 ordering is structural, not optional.

---

## 5. Expected Cost / Quality Multiplier

### 5.1 Per-run economics (assuming 5× opus-to-sonnet cost ratio)

| Item | Sonnet-only | A/B run (sonnet + opus) | Δ |
|---|---|---|---|
| Roles dispatched | 5 | 10 | +5 |
| Effective tokens (relative units) | 5 | 5 + (5 × 5) = 30 | +25 (+500%) |
| Expected catch-rate on seeded errors | baseline (~60% per R1 inference) | baseline + opus delta | +Δ |
| Round duration | 1× | ~1× (parallel dispatch, opus per-role slower but clock-parallelized) | flat |

### 5.2 Break-even decision rule

Per-role promotion to `cco-` is worth it iff:

```
Δ_quality × decision_value_per_round ≥ 4 × baseline_cost_per_role
```

where `Δ_quality` is the marginal catch-rate improvement (0–1 scale) and `decision_value_per_round` is the cost of shipping a brief-error-tainted round (R1's three factual errors shipped in the first commit; errata cost ≈ one additional commit + audit-trail damage).

**Practical threshold:** if opus catches ≥1 material brief error per 3 rounds that sonnet would have shipped, the 5× cost is justified for that role. Any less, the cheap path wins.

### 5.3 Not-re-tested axes

Explicitly out of scope (R1 already settled):
- `scout`, `executor`, `labrat`, `mutation-tester` — sonnet stays. R1 §8: read-only/probe cluster, null signal from opus.
- `the-revenger`, `the-judge` — already opus 4.7 per `xbreed-shared.md:68,70`. Not part of the A/B.
- `the-planner`, `scribe`, `simplifier` — CC-native, no xask gate; default sonnet. Planner (this artifact's author) was dispatched on opus as a one-off experiment for this specific task, not a protocol change.

---

## 6. Success Criteria

- **Primary:** a decisive (N=5) per-role verdict table committed to `docs/reports/cco-upgrade-r2-<date>.md` with inline seeded-error catch evidence for each pair.
- **Secondary:** `commands/references/xbreed-shared.md` Axis→Profile table + AGENTS.md + `the-judge.md` updated for any PROMOTE roles, tests green (`cargo test && cargo fmt --check && cargo clippy`).
- **Tertiary:** explicit break-even cost rule documented in the new report so future A/B rounds have a calibration reference.
- **Exit guard:** if N=5 produces <3 PROMOTE AND <3 HOLD (i.e., ≥3 AMBIGUOUS), do NOT ship a protocol edit; escalate to the-judge for Round-2 brief redesign.

---

## 7. Risks

1. **Under-powered verdict (k=5 briefs is still small).** Opus advantage may be real but noisy. The statistical unit is the brief (k), not the axis; 5 axes are raters per brief. Threshold (sum ≥ +2 per brief) is heuristic — false-positive rate under null depends on inter-rater reliability ρ, not simulated here. R3 critic pass is the designated surface for power-analysis patching. Mitigation: decision rules (§5.2) with explicit AMBIGUOUS escape hatch and **depth-1-bounded** rerun path (per §3.1 M5 revision — rerun-of-rerun coerces to HOLD, no unbounded loop).
2. **Opus-as-planner anchoring.** This plan is authored by a cco-harness planner, so the plan itself is A/B-contaminated. The plan structure (seeded-errors, blind scoring, cost table) is observable — if a sonnet-harness planner would have produced a materially weaker plan, that's itself A/B signal (but unverified by this round's design). Mitigation: note in-report that the plan artifact is A/B-biased and does not count toward the 5 axes.
3. **Brief recycling contamination.** Using R1's leftover briefs risks opus having seen the sonnet failure mode via conversation compression. Mitigation: all M2 briefs are fresh, unrelated to R1 corpus.
4. **Judge-scoring halo-leak.** If the judge reads source labels before provisional scoring, blind scoring is broken. Mitigation: Judge Blinding Protocol hash-commit enforces ordering; round invalidates if hash diverges post-reveal.
5. **5× cost ratio misassumption.** If Opus pricing has changed (pulled live before run), break-even rule (§5.2) adjusts automatically — coefficient `4` in the inequality tracks ratio minus one.
6. **Sonnet-4.6 ≠ Sonnet at R1 time.** If Sonnet is updated between R1 and this round, the A/B isn't strictly replicating R1's sonnet baseline. Mitigation: pin harness versions in the run log; declare explicit baseline drift if it occurs.

---

## 8. Return Format (executor handoff)

```markdown
# Plan — 5-axis cco-upgrade A/B
**Session:** ask-resilience-r2-0416 | **Dispatched by:** the-judge (via cco-the-planner-cco-ab) | **Date:** 2026-04-16

## Phase 0 — State map
- Exists: R1 §8 N=2 evidence (arch convergent; novel opus-better); harness `model` param override; `templates/agents/*.md` frontmatter defaults
- Missing: dedicated 5-axis brief template with seeded errors; per-role verdict on distiller/critic/sentinel/connector (reviewer has R1 data); break-even cost rule
- Risk: N=5 still small; opus-planner anchoring in this artifact; 5× cost ratio assumed not live-verified

## WWKD
1. What: 5-axis sonnet-vs-opus A/B with seeded-error briefs; per-role PROMOTE|HOLD|AMBIGUOUS verdict; protocol edit on PROMOTE
2. Why: R1 §8 explicit recommendation; selective verdict needs N≥5 before formalization; silent drift already leaking into current rounds
3. Assumptions/Risks: 5× cost ratio; adversarial/synthesis cluster is correct focus; N=5 may be too small
4. How: M1 baseline → M2 brief → M3 dispatch → M4 blind score → M5 reveal/verdict → Polish methodology + protocol edit
5. Escalation points: brief calibration (E1); ≥3 AMBIGUOUS (E2); hash divergence (E3)

## Milestones
(see §3 table above)

## Dependencies
M1 → M2 → M3 → M4 → M5 → Polish. No parallel shortcuts — blinding protocol enforces M4/M5 ordering.
```

---

## 9. Bottom Line

A 5-axis dedicated A/B round produces a defensible per-role verdict for the adversarial/synthesis cluster (`distiller`, `critic`, `sentinel`, `reviewer`, `connector`) by running same-brief paired dispatches under sonnet and opus harnesses, blind-scoring against `move_id`, and applying a cost-aware break-even rule. Per-role outcomes of PROMOTE/HOLD/AMBIGUOUS drive direct edits to the `xbreed-shared.md` Axis→Profile table (or rerun on AMBIGUOUS ≥3). Total cost is ≈6× a single-harness round at the 5× opus ratio assumption — cheap insurance against silently shipping brief-error-tainted adversarial rounds.

**Handoff:** this plan is advisory per planner posture. Executors may proceed with `[planner-gate: advisory, risks-open]` if the-judge does not respond within one dispatch cycle. R1 §8 recommendation now has a concrete execution blueprint.

evidence: none — planning artifact
