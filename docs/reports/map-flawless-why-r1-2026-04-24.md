# map-flawless-why — Round 1 Report
**Date:** 2026-04-24 | **Session:** 1 | **Status:** COMPLETE

---

## 1. Round Overview

**Mission:** Forensic map of why `/xbgst` + `xask` + delegation works flawlessly. Grounding incident: `c725c2a` (gpt-5.5 rewire reverted and reset to `2d16ad0`).

**Axes (8):**

| ID | Axis | Direction |
|----|------|-----------|
| G | Gate Layering | understand why the 4-layer xask gate catches regressions before they ship |
| D | Delegation Routing | map how dispatch decisions land on the right model tier |
| R | Role Cast | characterize how teammate role definitions produce correct behavior |
| E | Evidence Schema | explain why the evidence gate suppresses low-confidence moves |
| S | SSoT / Drift Resistance | diagnose how single-source-of-truth survives config churn |
| P | Pareto Economics | model why the filter accepts only moves that improve ≥1 axis and regress none |
| M | Memory / Learning Loop | trace how findings persist across sessions and accumulate signal |
| F | Failure Modes | enumerate bypass paths and latent collapse vectors |

**Axis coverage by move:** G(M01), D(M02+M03), S(M04+M05), F(M06), P(M07). **Axes R, M, E have no direct move.** E + F axis coverage was substituted via M07 (evidence-gate-structural) + M06 + M02 cross-axis. R and M axes are uncovered — flagged as R1 gap.

**Team dispatched (11 members):**

| Teammate | Role | xask Target |
|----------|------|-------------|
| the-judge | orchestrator | opus · xhigh (CC native) |
| the-planner | Phase 0 / WWKD | CC native · Layer-0 wwkd skill |
| g-scout-priorart-r1 | research / prior art | `xask --effort medium gemini` |
| g-connector-crossaxis-r1 | cross-axis patterns | `xask --effort high gemini` |
| cdx-reviewer-driftcheck-r1 | correctness / code review | `xask -R codex` |
| cdx-sentinel-attacksurface-r1 | security / adversarial | `xask -R codex` *(did not return)* |
| cdx-critic-design-r1 | adversarial design | `xask -R codex` · Layer-0 heuer-planning |
| cdx-revenger-protocol-r1 | reverse engineering | `xask -R -F codex` |
| ccs-labrat-breakmodes-r1 | empirical probes | `xask --spark codex` |
| ccs-simplifier-r1 | deletion / YAGNI | CC native |
| ccs-distiller | synthesis / dedup | CC native |
| ccs-scribe-r1 | audit trail | CC native (filter-exempt) |

**Wall time:** ~13 minutes (Phase 0 planner + 8 specialists parallel + distiller + cross-pollination cycles)

**Evidence audit:** `EVIDENCE AUDIT: 7 moves with evidence, 0 moves without, 0 dropped, 1 spoof_flagged`

> **Audit note:** 4 of 7 moves have `evidence: none` — this is correct per protocol. Research (`g-scout`), cross-axis (`g-connector`), reverse-engineering (`cdx-revenger`), and adversarial-design (`cdx-critic`) `axis_family` entries are explicitly evidence-exempt in xbreed-shared.md Pareto Filter Evidence Schema. The evidence gate applies the `evidence:` block requirement only to correctness/empirical axis families.

---

## 2. Per-Teammate Proposals (M01–M07)

*Evidence fields are verbatim from ccs-distiller SYNTHESIS_READY (received post-initial-commit via SendMessage relay). Report updated in addendum commit.*

---

### M01-priorart — Gate Grounding as Capacity-Agnostic Mechanism
**SOURCE:** g-scout-priorart-r1
**AXIS:** G — grounding / capacity-agnostic gate mechanism
**CLAIM:** xbreed's 4-layer gate is a runtime descendant of MetaGPT's rigid SOP pattern, elevated beyond context-window SOP into env-reality grounding (CLI stdout) — making it capacity-agnostic: low-effort / verbosity-pathological models cannot hallucinate passing Layer-2 raw-quote because the literal CLI artifact must exist in the real environment.
**EVIDENCE:** `evidence: none — research axis_family`
**LINCHPIN:** Layer-2 raw-quote gate as semantic primitive (no reviewed system ties evidence gate to literal shell output); `scripts/xask:37` flag-order gate as Layer-3 source-tagging defense
**REJECTED-ALTERNATIVE:** "Circuit breaker" framing — rejected: circuit breakers react post-failure; xbreed gate is prophylactic (rejects before scoring pipeline)
**CONFIDENCE:** MED — single source, research axis, no cross-model corroboration; structural framing novel and internally consistent but not empirically probed

---

### M02-compose — Cross-Axis Composition Failure (c725c2a Anatomy)
**SOURCE:** g-connector-crossaxis-r1
**AXIS:** D — cross-axis composition / delegation integrity
**CLAIM:** xbreed protocol has no runtime enforcement that co-rotating axes stay in-tier simultaneously — memory locks are Protocol-tier conventions, not Build/Runtime assertions; `c725c2a` is the canonical composition failure where each axis passed its own invariant check but cross-axis effect = systemic regression.
**EVIDENCE:** `evidence: none — cross-axis axis_family`
**LINCHPIN:** `c725c2a` as composition failure (not single-axis model-selection error); no CI assertion covers multi-axis tier invariants simultaneously
**REJECTED-ALTERNATIVE:** "Model selection error" (single-axis) — rejected: correcting only model ID leaves effort-tier collapse + double-godspeed idempotence regression active
**CONFIDENCE:** MED — structural pattern confirmed; connector explicitly flagged hallucinated specifics in raw output (own dissent); composition matrix cells (G×D, E×P, M×S) analytically derived, not file-verified
**⚠ SPOOF_FLAGGED (partial):** Connector self-reported hallucination of specifics in raw output. Structural pattern (composition failure, cross-axis unguarded) holds per independent convergence with M03/M04/M05. Matrix cells are analytically derived, not file-verified. See Section 6.

---

### M03-driftaudit — Delegation Drift: Canonical vs Mirror Table
**SOURCE:** cdx-reviewer-driftcheck-r1
**AXIS:** D — delegation drift, canonical vs mirror table
**CLAIM:** The "one xask invocation locked across canonical + mirror + slash-command" spine claim does NOT hold: all 13 specialist rows differ between `shared.md` and `the-judge.md` on ≥1 cell; model-tier mismatches (revenger sonnet-med/shared → opus-med/judge; planner sonnet-med/shared → opus-high/judge) are behavioral divergences, not cosmetic elaborations; `verify-docs.sh` covers only connector row → false-clean on 12/13.
**EVIDENCE:** `evidence: xask -R codex raw output, [obs strong] on all drift findings; severity flag from post-DESPAWN: -R flag omission in judge = blocker; live break-mode (judge reads its own file as authoritative).`
**LINCHPIN:** `xbreed-shared.md:104` (revenger=sonnet·medium) vs `the-judge.md:29` (revenger=opus 4.7 medium); `scripts/verify-docs.sh` connector-only grep; live break-mode: judge reads its own file as authoritative
**REJECTED-ALTERNATIVE:** "Intentional mirror elaborations" — rejected: model-tier mismatches (sonnet→opus, sonnet→opus-high) cannot be explained as elaboration; elaboration explains wording, not routing to different cost/capability tier
**CONFIDENCE:** HIGH — primary-source verified by distiller (`shared.md:104`, `the-judge.md:29`, `shared.md:106`, `the-judge.md:31`); cross-confirmed by M04 (revenger SSoT audit); 3-way convergence with M04+M05 on enforcement gap

---

### M04-recon — SSoT Enforcement Gap: Build/CI Covers Only 1 of 13 Rows
**SOURCE:** cdx-revenger-protocol-r1
**AXIS:** S — SSoT enforcement / tier-gap map
**CLAIM:** System hardening is structurally asymmetric: `src/protocol.rs` `include_str!` + content-sentinel tests enforce heading EXISTENCE at Build/CI tier, but routing prose (which model/effort each role delegates to) lives exclusively at Protocol tier for 12/13 roles — `verify-docs.sh` covers only connector row; three role files (reviewer, sentinel, critic) have drifted `--effort` values that pass CI silently.
**EVIDENCE:** `evidence: none — reverse-engineering axis_family. Anchor: xask raw output + src/protocol.rs include_str! observation + scripts/verify-docs.sh connector-only grep confirmed.`
**ADDENDA (post-DESPAWN):** 7 stale templates + 4 canonical-only references confirmed; no Build/CI guard on sync.
**LINCHPIN:** `src/protocol.rs` `include_str!` (Build/CI heading-only guard); `scripts/verify-docs.sh` connector-only grep (1 of 13 rows); routing prose for 12/13 roles = Protocol-tier only, zero Build/CI enforcement
**REJECTED-ALTERNATIVE:** "verify-docs.sh is sufficient second line of defense" — rejected: only extracts/compares ONE row (connector), leaving 12/13 unguarded at Build/CI
**CONFIDENCE:** HIGH — cross-model (codex revenger + claude simplifier + claude reviewer = 3-way convergence); distiller verified mutation-tester threshold conflict (`shared.md:108` ≥3 vs `the-judge.md:33` ≥5); `verify-docs.sh` connector-only confirmed by direct observation

---

### M05-yagni — Highest-Blast-Radius Stale Block: Gemini Labrat Universal (the-judge.md:81)
**SOURCE:** ccs-simplifier-r1
**AXIS:** S — SSoT gap, highest-blast-radius stale block
**CLAIM:** `the-judge.md` "Gemini labrat swarm (universal)" block (lines 81–85) actively contradicts SSoT — instructs ANY agent to fire `xask gemini` for labrat probes, but 2026-04-18 migration locked codex-spark as sole labrat channel; block never updated; `session-handout-2026-04-18` records migration as completed; deletion = zero functional regression.
**EVIDENCE:** `evidence: grep -n "Gemini labrat swarm" ~/.claude/agents/the-judge.md → 81:**Gemini labrat swarm (universal):**; grep on shared.md:L152 confirms codex-spark sole; session-handout-2026-04-18.md:81 records migration. evidence: none — deletion axis (informational sweep, no actual deletion).`
**LINCHPIN:** `~/.claude/agents/the-judge.md:81` = "**Gemini labrat swarm (universal):** ANY agent role can fire a Gemini labrat swarm."; `xbreed-shared.md:152` = "Codex-spark is the sole labrat channel (user directive 2026-04-18)"; `xbreed-shared.md:156` = "Gemini fanout is mutation-tester-only."
**REJECTED-ALTERNATIVE:** Flag judge sub-role table drifts for DELETION — rejected: judge reads this table to route sub-roles; drift = mis-routing → correct fix is UPDATE (align to SSoT), not deletion
**CONFIDENCE:** HIGH — grep-verified by distiller (`the-judge.md:81` literal match); anchored against `shared.md:152+156`; cross-confirmed by labrat (live break-mode #1) + reviewer (severity-blocker flag)

---

### M06-gpt55probe — H-Series Failure Modes: c725c2a Smoking Gun + Live Break-Modes
**SOURCE:** ccs-labrat-breakmodes-r1
**AXIS:** F — failure modes of c725c2a + live break-modes
**CLAIM (corrected):** H1L (reasoning-budget suppression at low effort, NOT verbosity-expansion) is the primary c725c2a smoking gun; H2L (medium-effort precision loss) severity UPGRADED (false-positives that clear Pareto are worse than clean misses); H3L (double-godspeed) unconfirmed; TWO live break-modes discovered via peer DMs; Goodhart silent-pass is the most dangerous unprobed F-axis scenario.
**EVIDENCE:** `evidence: 3 H/M/R triples — H1L: identical 3-file diff at -e low vs -e high gpt-5.5, top-3 findings compared / -e low showed {{QUERY}}{{QUERY}} citation artifact, missed behavioral ordering bug; -e high clean. H2L: same cross-axis prompt at high vs medium gemini, links counted/classified / count equal (8 vs 8); high-effort more structurally anchored. H3L: 4 variants --gs/| godspeed combos / duplicate activation ambiguity; transport unconfirmed. Confidence [xask dry] across all three.`
**SCOPE CORRECTION (post-DESPAWN):** `c725c2a` fully reverted to `2d16ad0`; H1L/H2L/H3L probes describe "what would have failed"; active live break-mode = `the-judge.md` drift reading its own file as authoritative (per M03)
**SPEC GAP:** No documented effort-tier baseline for correctness lanes (reviewer/sentinel/critic) → `-e low` wasn't flagged as regression in `c725c2a` rewrite because no baseline existed to violate
**LINCHPIN:** H1L: `-e low` showed `{{QUERY}}{{QUERY}}` citation artifact + missed behavioral ordering bug (`-e high` clean); H2L: count parity (8 vs 8) but structural anchoring depth diverges; live break-mode #1 = `the-judge.md:81` (M05); live break-mode #2 = drifted `--effort` role file → wrong ThinkingBudget at runtime (M04)
**REJECTED-ALTERNATIVE:** H2L as primary (link-count loss) — probes showed count parity at both effort tiers; failure was precision/anchoring depth not wholesale second-order blindness
**CONFIDENCE:** MED — H1L label corrected via cross-model peer (scout on `a373168`); H2L strengthened by connector; H3L remains LOW (transport blocked, execution-level unconfirmed); Goodhart addendum = unprobed gap; `[xask dry]` marker throughout

---

### M07-ach — Evidence Schema Gate as Primary Immune System (Pareto Adversarial)
**SOURCE:** cdx-critic-design-r1
**AXIS:** P — Pareto adversarial / evidence gate structural
**CLAIM:** H3C (evidence-schema gate) is the structural immune system, NOT H1C (axis-filter) — H1C is derivative of H3C's output; worst unaddressed scenario is Goodhart evidence-complete decoy: genuine evidence + genuine local axis improvement + real defect off-axis → passes ALL gates; 4-round cap terminates at wrong frontier unresolved.
**EVIDENCE:** `evidence: none — adversarial-design axis_family.`
**LINCHPIN:** `xbreed-shared.md:L194+L203` evidence gate (spot-check literal-substring `rg -F`); H3C drops unverified moves BEFORE axis-eval cost; distiller filters by stated axes → structurally blind to off-axis defect (distiller-layer amplification)
**REJECTED-ALTERNATIVE:** H4C (small-N papers over latent ambiguity) — rejected: `xbrd-R2` reaching 105/105 tests across 5 R2 moves is past toy-N shape; no run has hit Round 4 without explicit halt condition
**CONFIDENCE:** MED — adversarial-design axis, `evidence:none`; H3C structural primacy well-argued and consistent with `xbrd-vs-hvm4-R2` data (2 spoof-flagged upstream of 5 accepted R2 moves); Goodhart scenario = unprobed attack surface; distiller-layer bias amplification is self-referential but structurally valid

---

**Naming normalization (distiller canonical):**
- **Labrat H-series:** H1L = reasoning-budget suppression at low effort | H2L = medium-effort precision loss | H3L = double-godspeed activation ambiguity (unconfirmed)
- **Critic H-series:** H1C = axis-filter (derivative) | H2C = audit/commit revertibility | H3C = evidence-schema gate (PRIMARY) | H4C = small-N (REJECTED)

---

## 3. Cross-Model Conflicts

Five verified conflicts with judge resolution:

### Conflict (i) — the-revenger model tier
**Claim contested:** Model tier of `the-revenger`
**`shared.md:104`:** `sonnet · medium`
**`the-judge.md:29`:** `the-revenger (opus 4.7 medium)`
**Status:** VERIFIED DRIFT — mirror has not been updated to track shared.md collapse to sonnet-medium (unified scheme per `feedback_unified_tier_scheme.md`).
**Judge resolution:** shared.md is SSoT; sonnet · medium is operative. `the-judge.md:29` is stale.

---

### Conflict (ii) — the-planner model tier
**Claim contested:** Model tier of `the-planner`
**`shared.md:106`:** `sonnet · medium · Layer-0 wwkd skill load`
**`the-judge.md:31`:** `the-planner (opus 4.7 high · Layer-0 wwkd skill)`
**Status:** VERIFIED DRIFT — same unified-scheme collapse as (i).
**Judge resolution:** shared.md is SSoT; sonnet · medium is operative. `the-judge.md:31` is stale.

---

### Conflict (iii) — mutation-tester fanout threshold
**Claim contested:** Minimum target count for gemini fanout activation
**`shared.md:108`:** `≥3 targets / breadth` (Axis → Profile Mapping table)
**`shared.md:43`:** `≥3 or for breadth discovery` (inline mutation-tester brief text)
**`shared.md:156`:** `≥5-target mutation generation` (Gemini fanout scope section)
**`the-judge.md:33`:** `≥5 targets` (table)
**`AGENTS.md:24`:** `≥5 targets or breadth discovery`
**Status:** VERIFIED CONFLICT — internal contradiction within shared.md (≥3 at lines 43/108 vs ≥5 at line 156), AND external conflict vs the-judge.md + AGENTS.md (both ≥5). Distiller verified.
**Judge resolution:** Unresolved at R1. Operative value for this session: ≥5 (majority of documents + AGENTS.md canonical row agree). Escalation noted for judge.

---

### Conflict (iv) — H1L mechanism label
**Claim contested:** Mechanism label for high-vs-low-effort behavioral delta
**Original label (multiple teammates):** "verbosity-expansion" — more tokens = richer output
**Scout cross-check (commit `a373168`):** "reasoning-budget suppression / System-1 heuristics" — high effort allocates budget to internal chain-of-thought; visible token count may decrease while reasoning quality increases
**Status:** RESOLVED IN SYNTHESIS — "reasoning-budget suppression" is the accurate label; "verbosity-expansion" was a surface observation misidentified as mechanism.
**Judge resolution:** Scout's commit-anchored cross-check accepted; labrat H1L probe (`{{QUERY}}{{QUERY}}` artifact at `-e low`, clean at `-e high`) confirmed asymmetry.

---

### Conflict (v) — "SSoT chain intact" vs drift verified
**Claim contested:** Whether the SSoT chain (shared.md → mirror docs) is operationally intact
**Planner Phase 0 baseline:** SSoT chain intact; mirrors synchronized.
**Reviewer + revenger + distiller convergence:** Drift is real — 6+ cells verified stale across `the-judge.md` and `AGENTS.md`.
**Status:** RESOLVED — drift is real; planner Phase 0 baseline was optimistic. Mirror sync has not been enforced since unified-scheme collapse (2026-04-18).
**Judge resolution:** Drift acknowledged. Remediation is informational (see Section 5c); user decides action.

---

## 4. Pareto Verdict Per Move

| Move | Verdict | Axis | Rationale |
|------|---------|------|-----------|
| M01-priorart | **ACCEPT** | G | Capacity-agnostic gate framing passes evidence gate; structurally novel, internally consistent; MED confidence noted |
| M02-compose | **ACCEPT** | D | Composition failure framing intact via 3-way convergence despite connector spoof; MED confidence flagged |
| M03-driftaudit | **ACCEPT** | D | Drift verified primary-source (`shared.md:104`, `the-judge.md:29`); 3-way convergence; HIGH confidence |
| M04-recon | **ACCEPT** | S | `src/protocol.rs` + `verify-docs.sh` connector-only confirmed by direct observation; HIGH confidence |
| M05-yagni | **ACCEPT** | S | `the-judge.md:81` grep-verified; anchored against `shared.md:152+156`; HIGH confidence |
| M06-gpt55probe | **ACCEPT** | F | 3 H/M/R triples with `[xask dry]` markers; H1L label corrected via peer cross-check; MED confidence |
| M07-ach | **ACCEPT** | P | H3C structural primacy well-argued; consistent with upstream R2 spoof data; MED confidence |

**All 7 moves ACCEPTED. Zero regressions. Zero drops.**

---

## 5. Optimization Routes Surveyed

Surfaced by team; NOT executed. Informational per `feedback_no_safety_theater.md`. User decides action.

**(a) Extend `audit_hash` to sign Layer-1 verbatim xask string**
Proposed: `hash(Layer-1_string + proposal_content)` — closes effort-attestation gap in brief; does NOT close DEBUG-trap degradation (separate failure path). Source: g-scout fix proposal.

**(b) Extend `verify-docs.sh` from connector-only to all 13 role rows**
Would catch the 6+ drift cells at Build/CI tier. Source: multi-teammate convergence (reviewer + revenger + distiller independently surfaced drift; connector noted `verify-docs.sh` gap).

**(c) Update `the-judge.md` to align mirror with `shared.md`**
Primary deletion candidates: L81–L85 stale labrat block; model-tier mismatches (the-revenger, the-planner); fanout threshold inconsistency; stale gate strings. Source: revenger surface enum + distiller synthesis.

**(d) Add `effort_tier_baseline:` documentation to correctness-axis roles**
Roles: reviewer, sentinel, critic. Purpose: future rewires of the `-e` flag have a baseline to violate, making regressions detectable. Source: revenger spec-gap insight (M06 spec-gap note).

**(e) Document two confirmed evidence-gate bypass paths in `shared.md`**
- Bypass path 1: **prose burial** — evidence embedded in narrative prose, not in `evidence:` block; passes structural gate, evades content check
- Bypass path 2: **post-proposal file mint** — proposer mints a file after claim, timestamps it, cites as prior evidence; distiller cannot distinguish pre-existing from minted evidence without hash-at-proposal-time (CRIT per sentinel addendum)
Source: cdx-critic RETHINK + sentinel F-axis addendum (3-gap taxonomy, Section 9).

---

## 6. Spoof Flags

**M02-compose partial** — `g-connector-crossaxis-r1` self-disclosed hallucinated specifics in raw output during cross-axis composition matrix enumeration. Connector named composition matrix cells (G×D, E×P, M×S) as file-verified; these are analytically derived, not verified. Structural pattern (composition failure claim; cross-axis tier invariants unguarded at Build/CI) is intact via convergence: cdx-reviewer-driftcheck-r1 and cdx-revenger-protocol-r1 independently corroborated the unguarded-invariant claim. Distiller flagged connector output as `spoof_flagged: partial` in SYNTHESIS_READY. M02 confidence downgraded from HIGH to MED.

No other spoof flags in R1.

---

## 7. Audit Hash

```
66aee9e30bc47c7fe7caad69c778358919c7c7ef5a51c5616e31f5bbf3974677
```

Covers: M01–M07 proposals + evidence audit line + conflict resolution state at SYNTHESIS_READY.

---

## 8. Commit Delta

- Initial commit: `ec048ac` — report skeleton with RELAY-PENDING evidence (pre-distiller relay)
- Addendum commit: follows this rewrite — verbatim evidence inserted, move anatomy corrected

---

## 9. Coverage Gap — Sentinel Late Arrival + Uncovered Axes

**Sentinel did not return:** `cdx-sentinel-attacksurface-r1` did not return before SYNTHESIS_READY. E+F axis coverage was substituted via M07 (evidence gate structural, P-axis) + M06 (failure modes, F-axis) + M02 (composition failure cross-axis, D-axis).

**Axes with zero direct moves:** R (Role Cast) and M (Memory/Learning Loop) received no direct proposal in R1. These are candidate axes for R2 if judge determines continued value.

**Sentinel R1 addendum — 3-gap taxonomy (F-axis):**
1. **Solo pipeline bypass** — a single teammate operating without peer DMs can propose and self-cite evidence; no cross-check gate triggers
2. **Post-timestamp provenance race** (CRIT) — proposer mints a file post-claim and timestamps it; distiller cannot distinguish pre-existing from minted evidence without hash-at-proposal-time
3. **Effort-level attestation gap** (HIGH) — DEBUG trap injects effort tier but there is no in-proposal attestation that the correct tier was operative; a degraded-effort run produces a proposal indistinguishable from a full-effort run

Gaps 2 and 3 are reflected in Optimization Routes 5a and 5e above. No moves added to Pareto batch retroactively.

**Distiller dedup note:** 12 cross-move references collapsed to 7 unique moves.

---

## Gate (Scribe-owned pre-commit verification)

```bash
cargo clippy && cargo test && cargo fmt --check
```

Expected: zero errors, zero warnings (or pre-existing warnings only), all tests pass.
Actual (initial commit run): `Finished dev profile` · `test result: ok. 0 passed` · fmt clean

> No source code changed in this milestone. Gate run for pre-commit hygiene confirmation only.

---

## Standard Report Fields

### Does
Produced the Round 1 forensic report for `map-flawless-why-0424`, documenting 7 accepted Pareto moves across axes G/D/D/S/S/F/P (R, E, M uncovered), 5 verified cross-model conflicts with resolutions, 5 optimization routes surveyed (informational), 1 spoof flag (M02-compose partial), and the sentinel late-arrival + uncovered-axes coverage gap.

### Touches
- `docs/reports/map-flawless-why-r1-2026-04-24.md` — created then updated with verbatim distiller relay

### Out-of-scope
- Axes R (Role Cast) and M (Memory/Learning Loop) — no direct moves in R1 (candidate axes for R2)
- E-axis direct coverage — substituted via M07 (P-axis evidence-gate sub-focus) and M02 (D-axis cross-axis)
- Sentinel proposals excluded from main Pareto batch (covered in Section 9 addendum; 3-gap taxonomy informational only)
- No source code changes in this round

### Links
- Plan: N/A (documentation/forensic mission; no plan doc in `docs/plans/`)
- Commits: `ec048ac` (skeleton) → addendum commit follows
- Next: R2 (judge-gated; candidate axes R, M, E for fresh coverage)
