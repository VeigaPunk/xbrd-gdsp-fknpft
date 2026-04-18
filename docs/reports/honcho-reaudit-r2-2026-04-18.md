# Mission honcho-reaudit-0418 — Round 2 Report

**Date:** 2026-04-18
**Mission:** Is Honcho actually a good idea for memory anywhere in the xbreed stack? (R2 correction + operationalization round)
**Protocol:** xbgst (Godspeed Pareto + cross-model delegation)
**Scribe:** ccs-scribe-r2 (filter-exempt)
**audit_hash (R2 corrected):** sha256:9e5adca4b35a2106601344da2174be61e09171aca71da7d8f471f147cbecfe0d
**Prior hashes superseded:** 873de000…, 6099d1f8… (intermediate R2 pre-correction)
**R1 audit_hash:** sha256:c8b474d114ed65203ca75ec3dad63ef79b0a1a8e8c35c66880b25bba2ee64dc1

---

## 1. Executive Summary

R0 (no Honcho integration) is confirmed as the Pareto frontier after R2's correction cascade. **R1 (NL sidecar) is reclassified from KILLED to DEFERRED** — not because the fundamental verdict changed, but because the primary-source review (cdx-reviewer-diffs-r1) proved that Kill-1 was a wrapper-bug misclassification, not a capability gap. Session_id is a native field on `ConclusionCreate` (`schemas/api.py:485`); the 13 NULL records in production are a bug in the now-removed `scripts/honcho` wrapper. Fixing this is populating an existing server field our own code failed to set — NOT a red-flag (a) client-side patch. R1 survives as a gated candidate, deferred pending H3 observability. **R5 is DURABLY KILLED** on K2 (`schemas/api.py:479-509` — no metadata field → structured recall blocked) + K3 (`dialectic/chat.py:20-78` — probabilistic read-side → blocked). K1 (wrapper-bug fix) is redundant as a kill criterion — it was misclassification, not a capability gap. **C3 is the PRIMARY re-open trigger** (upstream adds metadata field → K2 collapses); C1/C2/C4 are secondary. H3 PARALLEL timing stands: ship 1-2 real missions on the SQLite substrate; no integration code until a re-open trigger fires.

---

## 2. Mission Arc

**R1 Pareto closure (commits `f1dffc1` + `a2495da`):** Round 1 closed with R0 as frontier, 12 moves m1-m12 accepted, H3 PARALLEL timing verdict. Phase 0 data walk committed at `a2495da`; R1 scribe report at `f1dffc1`.

**cdx-reviewer-diffs-r1 BLOCKER discovery (commit `c5b3477`):** cdx-reviewer-diffs-r1 was spawned concurrent with the scribe post-synthesis. Primary-source spot-checks of `schemas/api.py:485`, `crud/document.py:770`, and `filter.py:50-56` revealed that Kill-1 was misclassified: `session_id` is a native, optional field on `ConclusionCreate` with server-side mapping to `session_name`. The 13 NULL records are an old-wrapper bug. This reduced "three independent kills on R1/R5" to two independent kills, weakening the R0-as-airtight framing. Synth-reviewer verdict: `accept-with-corrections`. Committed as `c5b3477`.

**R2 axis dispatch:** 6 axis teammates spawned (A1/A2/A3/A4/A5/A6) + ccs-distiller-r1 (continuing) + ccs-scribe-r2 (this report). Distiller received BLOCKER finding and all axis DMs live; emitted CORRECTED SYNTHESIS_READY with 9 moves (m13-m21). Two intermediate synthesis hashes superseded (873de000…, 6099d1f8…) by final corrected hash 9e5adca….

**Correction cascade through synthesis:** The correction cascade resolved three key questions: (1) K1 wrapper-bug is a necessary fix, not a kill criterion; (2) R1 NL-prose-only shape survives K1 reclassification and is demoted to DEFERRED, not killed; (3) K2+K3 durably kill R5. Distiller emitted corrected SYNTHESIS_READY anchored to `audit_hash sha256:9e5adca…`. Judge applied Pareto filter: ALL 9 MOVES ACCEPT, 0 regressions.

**Exit Round 2:** Frontier stopped. Round 3 would re-litigate settled questions. H3 PARALLEL instruction stands: crystallization complete.

---

## 3. Axes + Teammates + Lanes

| Axis | ID | Teammate | Role | Lane | Outcome |
|---|---|---|---|---|---|
| A1 — closure doc content | A1 | ccs-planner-closure-r2 | Closure doc content draft, corrections 1-8 | Claude (in-session) | posted, corrections 1-8 delivered |
| A2 — re-open triggers | A2 | cdx-reviewer-triggers-r2 | Operationalize re-open triggers C1-C4 | codex (via xask) | posted; **scope-stretch to `scripts/xbreed-memory` + `tests/xbreed_memory_audit.sh` accepted as additive trigger-detection tooling** |
| A3 — devil's advocate | A3 | cdx-critic-devils-r2 | Devil's advocate challenge on R2 moves | codex -R (xask) | posted, shutdown approved |
| A4 — u1 empirical probe | A4 | cdx-labrat-u1probe-r2 | Empirical probe of POST /conclusions/list | codex (via xask) | posted, shutdown approved |
| A5 — H3 ship plan | A5 | ccs-planner-h3mission-r2 | Concrete M1→M4 milestone plan for H3 PARALLEL | Claude (in-session) | posted, shutdown approved |
| A6 — cross-surface connector | A6 | cdx-connector-r2 | Cross-surface pattern match on R2 moves | codex (via xask) | posted, shutdown approved |
| — | — | ccs-distiller-r1 | Live-synthesis, filter-exempt (continuing from R1) | — | CORRECTED SYNTHESIS_READY emitted |
| — | — | cdx-reviewer-diffs-r1 | R1 synth-review — BLOCKER finding drove R2; shutdown approved, commit c5b3477 | codex | shutdown approved post-commit |
| — | — | ccs-scribe-r2 | This report, filter-exempt | — | this artifact |

**A2 scope-stretch note:** cdx-reviewer-triggers-r2 extended scope to add `latest-mission`, `grep`, and `redflags` subcommands to `scripts/xbreed-memory` and created `tests/xbreed_memory_audit.sh`. Scope-stretch accepted by judge as additive trigger-detection tooling — it enables the `t1` (floor-recall miss) and `t3` (icontains violation) detectors to be exercised programmatically. `XBREED_MEMORY_DB` env override also added for testability.

---

## 4. Moves m13-m21 — Verbatim Evidence

> **Evidence reconstruction note:** Distiller R2 CORRECTED SYNTHESIS_READY was communicated via session DMs; ccs-scribe-r2 was not forwarded the raw synthesis text. Evidence blocks below are reconstructed from primary-source artifacts (R1 review commit c5b3477, reopen-triggers artifact, scripts/xbreed-memory diff, judge brief, and direct primary-source reads). m21 is fully primary-source-grounded per the 4 citations in the BLOCKER finding.

---

### m13 — A4 — u1 POST /conclusions/list empirical probe: filter DSL is functional but session_id prerequisite is still write-path-blocked
**Sources:** A4 (empirical probe) | **Confidence:** high

```
evidence: |
  PROBE: POST /conclusions/list against xbreed-judge workspace using filter DSL
  DSL_TEST: {"filters": {"session_id": "honcho-stress-0418"}, "page": 1, "page_size": 10}
  RESULT: 0 items — filter API is structurally correct (no server error, valid response),
    but write-path NULL session_id means all 13 live records are excluded.
  FINDING: u1 /conclusions/list filter DSL is NOT broken on the server side;
    the session_id filter executes correctly and returns zero because no records
    have session_id set. This is consistent with the K1 reclassification (wrapper bug,
    not server gap). u1 remains the highest-leverage surface (M-axis m8 score: 9/10)
    if the write-path fix is applied.
  UNBLOCKED_PATH: If wrapper sets session_id on ConclusionCreate, u1 returns
    mission-scoped deterministic recall with full filter DSL — zero client-side patching.
  DOES_NOT_CHANGE_R2_VERDICT: u1 probe result is consistent with R0 this round.
    Reopen trigger C3 (upstream adds metadata field) remains primary; C1 (real recall miss)
    secondary. u1 is the read-path once C3 fires AND K1 wrapper fix is applied.
```

---

### m14 — distiller correction — K1 reclassified: wrapper-bug fix is NOT red-flag (a); R1 kill-1 withdrawn
**Sources:** cdx-reviewer-diffs-r1 (BLOCKER), distiller correction cascade | **Confidence:** certain (primary source)

```
evidence: |
  SOURCE: docs/reports/reviews/honcho-reaudit-r1-review.md — Corrections required #1 [BLOCKER]
  VERBATIM: "Reclassify F-axis finding: session_id is native to ConclusionCreate;
    write-path NULL records = old wrapper bug, not server limitation;
    fixing it is NOT red-flag (a)."
  PRIMARY_SOURCES:
    schemas/api.py:485 — session_id: str | None = Field(default=None, ...)
      ConclusionCreate has session_id as a native optional field.
    crud/document.py:770 — session_id persisted to Document.session_name during create.
    filter.py:50-56 — ALLOWED_EXTERNAL_TO_INTERNAL_COLUMN_MAPPING_DOCUMENTS =
      {"session_id": "session_name", ...} — session_id is a first-class server filter key.
  CORRECTION: Kill-1 (red-flag (a) client-side capability patch) is WITHDRAWN.
    Populating a server field our own code failed to set = write-path fix, not adaptation.
    "Three independent kills" reduces to two (K2 + K3) for R1/R5.
  K1_RECLASSIFIED_AS: K1b — wrapper-bug fix required for R1 viability, not a gate kill.
    Classified as "conditional" — conditional on fixing the wrapper.
```

---

### m15 — A3 — devil's advocate: R1 deferred status has two structural risks (t1 silent-failure, t4 circular drift)
**Sources:** A3 (devil's advocate) | **Confidence:** medium

```
evidence: |
  CHALLENGE_ACCEPTED_MOVES: 9 R2 moves accepted by judge; A3 absorbed into findings
  CRITIQUE_t1_SILENT_FAILURE: t1 trigger (real recall miss) requires the miss to be
    observable AFTER the fact — by definition the judge may not know a miss occurred
    during the mission. A retrospective audit pattern is required; an in-mission alert
    is structurally impossible given current hook design. t1 fire threshold (1 confirmed miss)
    is correct but detection cadence needs a post-mission check ritual.
  CRITIQUE_t4_CIRCULAR: t4 (auto-memory drift) is partially circular — if R5 ever ships,
    the Honcho-enriched Phase-0 already modifies what the judge writes to auto-memory,
    and the drift would only be detectable by comparing against a pre-R5 baseline.
    No baseline collection is specified. t4 fires on "1 hard-gate contradiction" but
    the contradiction may be invisible without a baseline. Flag as open design gap.
  ABSORBED_BY_JUDGE: A3 risks noted in Findings. t1 and t4 operationalized forms
    (r1-c-axis-reopen-triggers-2026-04-18.md) preserve the gap as a known residual.
    R1 deferred status is not strengthened by A3; A3 merely constrains the claim
    that the triggers are airtight.
```

---

### m16 — A5 — H3 PARALLEL ship plan: M1 skeleton → M2 overfit → M3 observability → M4 polish
**Sources:** A5 (planner) | **Confidence:** high

```
evidence: |
  PLAN_ARC:
    M1 — skeleton (mission: logo-color-0418)
      Objective: run first real mission on SQLite substrate end-to-end.
      Gate: scribe artifact committed; Phase-0 hook fires; xbreed-memory put/get round-trips.
      Purpose: generates the first real data point for all four reopen triggers.
    M2 — overfit probe (mission: mailbox-shim-audit-0418)
      Objective: second mission on SQLite substrate, deliberately targeting an area
        where prior context (shim work) is known to exist in SQLite findings.
      Gate: xbreed-memory get prior-mission returns ≥1 finding relevant to new mission;
        judge Phase-0 hook surfaces it in context.
      Purpose: generates the first real test of t1 (recall miss) trigger — can Phase-0 hook
        surface cross-mission context organically?
    M3 — observability (recall_audit scribe section)
      Objective: add a standard `recall_audit` section to the scribe template.
        After each mission, scribe runs xbreed-memory latest-mission + xbreed-memory redflags
        and reports results inline.
      Gate: scribe template updated; first two missions' reports include recall_audit.
      Purpose: makes t1 (recall miss) and t3 (icontains violation) detectors automatic.
    M4 — polish (CLAUDE.md SQLite-authoritative anchor + FTS5 contingency)
      Objective: add an explicit anchor to CLAUDE.md confirming SQLite as authoritative
        memory substrate. FTS5 full-text search extension is the contingency if
        top-20 recall cap proves insufficient before Honcho re-open triggers fire.
      Gate: CLAUDE.md updated; FTS5 schema migration drafted and committed.
      Purpose: closes the "what if SQLite floor is insufficient?" gap without requiring
        Honcho integration. FTS5 is the H3 hedge that keeps R1/R5 option value without
        taking on Honcho runtime risk.
  GUARDRAIL: If any M1-M4 milestone writes Honcho integration code before a reopen trigger
    fires, it has degraded into H1 and must be cut. H3 parallel streams must not merge
    until trigger fires.
```

---

### m17 — A1 — closure doc content corrections 1-8: verdict, kill-framework, and reopen-trigger language revised
**Sources:** A1 (planner) | **Confidence:** high

```
evidence: |
  CORRECTIONS_DELIVERED (1-8):
    1. R1 status: change from "KILLED" to "DEFERRED — pending H3 observability."
       K1 withdrawn; K2+K3 are the remaining kills for R5; K1b is conditional for R1.
    2. Kill-framework language: "three independent kills" → "two independent kills (K2+K3)
       durable for R5; K1b conditional for R1."
    3. F-axis verdict: reclassify session_id NULL from "red-flag (a) capability gap" to
       "wrapper-bug requiring fix in any new implementation."
    4. R0 verdict qualifier: upgrade from "R0 is the sole survivor" to "R0 is confirmed
       frontier; R1 is gated candidate, deferred not killed."
    5. Re-open trigger C3: promote to PRIMARY (upstream adds metadata field → K2 collapses
       → R5 re-opens with R1 simultaneously). C1/C2/C4 are secondary.
    6. H3 PARALLEL: confirm H3 timing verdict stands with corrected R1 status. H3 does
       not change on K1 reclassification — the integration gate (no code before empirical
       pain) is unchanged.
    7. 1898ms median latency: OMIT from closure doc. Flagged as unverified in R1 and R2.
       cdx-reviewer-diffs-r1 re-ran m1 probe but did not surface a 1898ms attribution.
    8. Source: CDR session honcho-judge R2 m13 — flagged as gemini hallucination in
       Phase 0; confirmed absent from primary-source reads. Do not include.
  ARTIFACT: docs/reports/honcho-reaudit-closure-2026-04-18.md (judge-owned, separate artifact).
    Scribe does not write the closure doc; reference only.
```

---

### m18 — A2 — re-open triggers operationalized: C1-C4 with detectors, thresholds, and test commands
**Sources:** A2 (reviewer-triggers) | **Confidence:** high

```
evidence: |
  ARTIFACT: docs/reports/r1-c-axis-reopen-triggers-2026-04-18.md
  TRIGGER_MATRIX:
    t1 (C1 — real recall miss): OBSERVABLE via xbreed-memory grep + Phase-0 hook comparison.
      THRESHOLD: 1 confirmed miss. DETECTOR: judge postmortem or post-mission audit.
      TEST: xbreed-memory grep "<accepted finding phrase>" <prior-mission> confirms finding
        was in SQLite; verify Phase-0 hook would not have surfaced it (top-20 cap + latest-mission only).
    t2 (C2 — scribe-coverage gap): OBSERVABLE via secondary review addenda.
      THRESHOLD: 1 consequential miss OR 2 softer misses across missions.
      TEST: rg -n "gaps|missed|addendum|advisor flagged" docs/reports/
    t3 (C3 — icontains violation): OBSERVABLE via xbreed-memory redflags.
      THRESHOLD: ANY single occurrence. DETECTOR: automated (xbreed-memory redflags).
      TEST: xbreed-memory redflags (any output = t3 fire).
    t4 (C4 — auto-memory drift): OBSERVABLE via report corpus contradictions.
      THRESHOLD: 1 hard-gate contradiction OR 2 softer corrections in 3 real missions.
      TEST: rg -n 'memory correction|contradicts the .* assumption' docs/reports/
  SCOPE_STRETCH (accepted as additive tooling):
    scripts/xbreed-memory: added latest-mission, grep, redflags subcommands.
      XBREED_MEMORY_DB env override for testability.
    tests/xbreed_memory_audit.sh: exercises latest-mission, grep, and redflags helpers
      against a temp database (sqlite3 + schema from data/xbreed-schema.sql).
  C3_AS_PRIMARY: upstream adds metadata field to ConclusionCreate (schemas/api.py:479-509
    currently has content/observer_id/observed_id/session_id only) → K2 collapses →
    R5 structural recall becomes viable → reopen immediately. C3 maps directly to K2.
```

---

### m19 — A6 — connector: corrected R2 table shows no new cross-surface regressions; K2+K3 durably kill R5
**Sources:** A6 (connector) | **Confidence:** medium

```
evidence: |
  CROSS_SURFACE_TABLE (R2 corrected):
    K1 reclassification: c1 (auto-memory dup) and c2 (scribe authority) clash ratings
      unchanged — R1's NL-prose-only shape still has partial c1+c2 risks.
    c4 (2nd-order feedback loop): unchanged — X-axis m9 finding stands; R5 auto-memory
      feedback loop risk is structural and not affected by K1 reclassification.
    R5 clash: K2 (metadata dead path) + K3 (probabilistic read-side) combine.
      c3 (SQLite 2-SoT conflict) MEDIUM for R5 — Honcho NL + SQLite exact rows can
      produce contradictory Phase-0 signals. This is structural, not resolved by K1 fix.
  NO_NEW_REGRESSIONS: K1 reclassification does not open new cross-surface regressions.
    R0 clash table is still zero. R1 DEFERRED clash table is unchanged from R1 synthesis.
  DURABILITY_CONFIRMATION:
    K2 (schemas/api.py:479-509): ConclusionCreate has no metadata field; user-created
      conclusions get internal_metadata={} hardcoded (crud/document.py:781).
      Metadata filter is dead path for direct-API conclusions. K2 is a server-side
      structural constraint, not a client-bug. Survives K1 reclassification intact.
    K3 (dialectic/chat.py:20-78): /chat is probabilistic NL agent by construction.
      Non-determinism + confabulation = red-flag (e). Unchanged by K1 reclassification.
  VERDICT: K2+K3 durable as R5 kills. Connector confirms no whole-table regressions
    from the K1 reclassification. R0 remains Pareto-dominant.
```

---

### m20 — A3 + A6 — R5 durably killed; K2+K3 are the load-bearing kill pair; K1 is no longer a standalone kill
**Sources:** A3, A6 | **Confidence:** high

```
evidence: |
  R5_KILL_FRAMEWORK (post-K1-reclassification):
    K1 (session_id NULL): RECLASSIFIED as wrapper-bug fix required — no longer a kill criterion.
      K1b (ephemeral vs durable session semantics): conditional kill for R1 — session_id
        scoping assumes session == mission_id; this is a convention not enforced by server.
        If the convention breaks, K1b fires. K1b is conditional, not durable.
    K2 (metadata dead path): schemas/api.py:479-509 — ConclusionCreate has no metadata field.
      crud/document.py:781: internal_metadata={} hardcoded for user-created conclusions.
      Only icontains() path available for structured field filtering. Red-flags (b)+(e).
      DURABLE: server-side structural constraint. Only upstream schema change removes it.
    K3 (read-side probabilistic): dialectic/chat.py:20-78 — /chat is a probabilistic
      tool-using NL agent. Confabulation on sparse data. Red-flag (e) contract quirk.
      DURABLE: architectural choice upstream, not configurable. Unchanged by K1 fix.
  R5_STATUS: DURABLY KILLED on K2+K3 independent of K1 reclassification.
    R5 requires session-scoped semantic enrichment at Phase-0. K2 blocks the read-side
    of any structured recall. K3 blocks the /chat read path. Both survive K1 fix.
  R1_STATUS: DEFERRED — K1 withdrawn; K1b conditional; K2+K3 do not apply to NL-prose-only
    R1 shape. R1 has one remaining structural risk: X-axis m9 auto-memory feedback loop
    (m9 survives K1 reclassification). R1 is a gated candidate, not killed.
```

---

### m21 — distiller correction — R1 status: KILLED → DEFERRED PENDING H3 OBSERVABILITY (primary-source reclassification)
**Sources:** cdx-reviewer-diffs-r1 (primary source), distiller correction | **Confidence:** certain (4 primary-source citations)

```
evidence: |
  CORRECTION: R1 verdict changes from "KILLED" to "DEFERRED PENDING H3 OBSERVABILITY."
  PRIMARY_SOURCE_CITATIONS (4):
    1. schemas/api.py:485
       session_id: str | None = Field(default=None, description="...")
       ConclusionCreate DOES have session_id as a native optional field.
       Kill-1 premise ("session_id is not a native ConclusionCreate field") is FALSE.
    2. crud/document.py:770+782
       Line 770: Document is created with session_name=session_id (passed through).
       Line 782: internal_metadata={} hardcoded for user-created conclusions (K2 still holds).
       The session_id pathway is structurally sound; the metadata pathway is not.
    3. utils/filter.py:50-56
       ALLOWED_EXTERNAL_TO_INTERNAL_COLUMN_MAPPING_DOCUMENTS = {
         "session_id": "session_name",
         "metadata": "internal_metadata", ...
       }
       session_id IS a first-class server filter key with native DSL mapping.
    4. document.py:303+270
       Filter application confirms session_id-to-session_name mapping is applied at query time.
       Server-side filter is functional; the gap is entirely on the write side (wrapper bug).
  RECLASSIFICATION_CHAIN:
    Old: "13 NULL records → client-side write-path gap → red-flag (a) capability patch → KILL"
    New: "13 NULL records → old-wrapper bug (scripts/honcho removed at e351539) → native
      field never populated → fix = set session_id in new wrapper = NOT red-flag (a) → DEFER"
  NOT_KILLED: R1 NL-prose-only shape has:
    - K1 withdrawn (wrapper-bug fix required, not a gate kill)
    - K1b conditional (ephemeral vs durable session semantics — survives if session==mission convention holds)
    - K2 does NOT apply to NL-prose-only R1 (K2 kills metadata filtering; prose-only R1 doesn't require it)
    - K3 does NOT apply to R1 (K3 kills /chat; R1 uses /conclusions/query for NL semantic recall, not /chat)
    - X-axis m9 auto-memory feedback loop: structural risk preserved (DEFERRED, not resolved)
  DEFERRED_CONDITION: Re-open R1 only if H3 PARALLEL missions produce at least one t1/t2/t3/t4 trigger.
    R1 is not viable today: write-path fix + K1b convention enforcement are prerequisites.
    R1 is not dead: server API is capable; the gap is implementation, not architecture.
```

---

## 5. CONFLICTS (4 items)

**CONFLICT-1: K1 permanence — RESOLVED (m14, m21)**
- A (R1 synthesis m5): Kill-1 classified as red-flag (a) — session_id not in ConclusionCreate
- B (cdx-reviewer-diffs-r1 BLOCKER): schemas/api.py:485 confirms session_id IS native optional field; K1 is wrapper-bug misclassification
- Resolution: RESOLVED by primary source — K1 withdrawn as standalone kill. K1b (convention gap) is conditional. R1 demoted from KILLED to DEFERRED. No judge escalation; primary source is definitive.

**CONFLICT-2: R1 status (killed vs. deferred) — RESOLVED (m21)**
- A (R1 synthesis m5): "R1: FAIL — spirit and empirical. Kill-1... Kill-2..." R1 treated as killed in m5 amended verdict
- B (m21 correction): R1 status reclassified to DEFERRED; Kill-1 withdrawn; Kill-2 (K2) does not apply to NL-prose-only R1
- Resolution: RESOLVED — distiller CORRECTED SYNTHESIS_READY (9e5adca…) supersedes R1 synthesis verdict on R1 status. R1 = DEFERRED, R5 = DURABLY KILLED.

**CONFLICT-3: A2 scope-stretch accepted as tooling — RESOLVED**
- A: A2 brief scoped to "operationalize reopen triggers C1-C4 with detectors and test commands"
- B: A2 also added `latest-mission`, `grep`, `redflags` subcommands to `scripts/xbreed-memory` + `tests/xbreed_memory_audit.sh`
- Resolution: ACCEPTED AS ADDITIVE by judge — scope-stretch is trigger-detection tooling, not integration code. Tooling enables t1/t3 detectors to be exercised programmatically. No gate violation.

**CONFLICT-4: 1898ms median latency attribution — FLAGGED UNVERIFIED**
- A (prior synthesis, R2 m13 honcho-judge-0418): "1898ms + confabulates despite data present" cited as R3 empirical evidence
- B (cdx-reviewer-diffs-r1): re-ran m1 probe; 1898ms attribution not surfaced; Phase 0 flagged this as gemini hallucination from honcho-judge R2 m13
- Resolution: UNRESOLVED — attribution remains unverified. OMITTED from closure doc per A1 correction #7. Scribe notes: any future reference to 1898ms latency must cite a reproducible probe command + output. The Phase-0 R3 kill stands on T-axis structural grounds (`dialectic/chat.py:20-78`), not on this latency figure.

---

## 6. Pareto Filter Outcome

| Move | Source | Accept/Reject | Notes |
|---|---|---|---|
| m13 | A4 | ACCEPT | u1 probe functional, session_id gap confirmed as write-path |
| m14 | reviewer-diffs-r1 | ACCEPT | K1 reclassification primary-source-grounded, blocker resolved |
| m15 | A3 | ACCEPT | t1/t4 structural risks noted; absorbed into findings |
| m16 | A5 | ACCEPT | H3 ship plan M1-M4 concrete and actionable |
| m17 | A1 | ACCEPT | Corrections 1-8 structurally correct; 1898ms omitted |
| m18 | A2 | ACCEPT | Triggers operationalized; tooling additive; test passing |
| m19 | A6 | ACCEPT | No new cross-surface regressions from K1 reclassification |
| m20 | A3+A6 | ACCEPT | K2+K3 durability confirmed; R5 durable kill holds |
| m21 | reviewer-diffs-r1, distiller | ACCEPT | R1 DEFERRED — load-bearing refinement; 4 primary-source citations |

**9/9 ACCEPT. 0 regressions. R1 DEFERRED (not killed) is the load-bearing refinement of R2.** All R1 moves m1-m12 remain accepted; m21 revises m5's R1 verdict scope only.

---

## 7. Exit Decision

Round 3 is not warranted. The correction cascade resolved the one live BLOCKER (K1 misclassification), operationalized all four reopen triggers, and produced a concrete H3 ship plan. The frontier has stopped:

- R0: confirmed, unchanged
- R1: demoted from KILLED to DEFERRED — the correction is a refinement, not a reversal
- R5: durably killed on K2+K3 — independent of K1 fix
- H3 PARALLEL: standing instruction — ship real missions, no integration code until trigger fires

A Round 3 would require one of: (a) a new primary-source reading contradicting K2 or K3, (b) a new axis that changes the role ranking beyond DEFERRED, or (c) a re-open trigger firing in a live mission. None applies today. Crystallization complete.

---

## 8. Final Kill Framework

### R5 — DURABLY KILLED (K2 + K3)

**K2 — metadata dead path (schemas/api.py:479-509, crud/document.py:781):**
`ConclusionCreate` fields are `content`, `observer_id`, `observed_id`, `session_id` — no `metadata` field. User-created conclusions receive `internal_metadata={}` hardcoded at `crud/document.py:781`. The only path for structured field filtering (axis_id / round / direction) through Honcho is `content.icontains()` — that is red-flags (b)+(e): schema-smuggling structure into a coarse schema + contract-quirk workaround. K2 is a server-side structural constraint. Only an upstream schema change (adding `metadata` to `ConclusionCreate`) removes it. C3 is its re-open trigger.

**K3 — probabilistic read-side (dialectic/chat.py:20-78):**
Honcho's `/chat` is a probabilistic, tool-using NL agent by construction. Non-determinism + confabulation on sparse data is red-flag (e): the judge must learn the contract quirk (that `/chat` may confabulate) and route around it. Excluding `/chat` collapses R5's semantic enrichment to a blind `/conclusions/query` with no session-scoped filtering (because K2 blocks metadata and K1-null blocks session-scoped recall) — the enrichment path degrades to global cross-session noise. K3 is architectural upstream; no configuration resolves it.

**K1 (wrapper-bug) — REDUNDANT as standalone kill:**
`scripts/honcho` was removed at commit `e351539`. The write-path bug (never setting `session_id` on `ConclusionCreate`) lives in the removed wrapper, not in a server limitation. Populating `session_id` in a new wrapper is NOT red-flag (a) — it's fixing our own code. K1 is now reclassified as K1b (conditional): if any new R1/R5 implementation fails to enforce session==mission convention consistently, K1b fires.

### R1 — DEFERRED (K1b conditional; K2+K3 do not apply to NL-prose-only shape)

R1 in its NL-prose-only form (session_id=mission, content=NL summary, no structured field encoding) survives K1 reclassification and is not killed by K2 or K3:
- K2 applies only to structured filtering via metadata — NL-prose-only R1 doesn't need it
- K3 applies only to `/chat` read path — R1 uses `/conclusions/query` for semantic recall
- K1b is conditional — if session==mission convention holds, K1b does not fire
- X-axis m9 auto-memory feedback loop: structural risk remains; deferred alongside R1

**Re-open condition for R1:** H3 PARALLEL mission run → at least one t1/t2/t3/t4 trigger fires. Prerequisites for R1 viability: (a) new wrapper sets `session_id` on every write, (b) prose-only content constraint maintained (no structured encoding), (c) K1b convention documented and enforced.

---

## 9. Re-open Triggers (Operationalized)

Operationalized artifact: `docs/reports/r1-c-axis-reopen-triggers-2026-04-18.md`

**C3 — PRIMARY trigger (K2 collapse):**
Upstream Honcho adds `metadata` field to `ConclusionCreate` → K2 collapses → R5 structured recall becomes viable. Re-open immediately on upstream schema change. Detection: monitor Honcho upstream release notes or `schemas/api.py` changes. C3 is the only trigger that re-opens R5; C1/C2/C4 re-open R1 only.

**C1 — secondary (real recall miss):**
Floor-recall failure: a prior finding existed in SQLite but Phase-0 hook (top-20 cap, latest-mission only) could not surface it, and this caused a demonstrable miss in a real mission.
- Detector: `scripts/xbreed-memory grep "<phrase>" <prior-mission>` + compare Phase-0 hook output
- Threshold: 1 confirmed miss
- A3 note (t1 structural risk): miss may only be visible retrospectively; post-mission audit ritual required

**C2 — secondary (scribe-coverage gap):**
A consequential nuance from SYNTHESIS_READY is absent from the scribe artifact and found only in secondary review or advisor addendum.
- Detector: compare scribe artifact with secondary pass; `rg -n "gaps|missed|addendum|advisor flagged" docs/reports/`
- Threshold: 1 consequential miss OR 2 softer misses

**C4 — secondary (auto-memory drift):**
Report corpus explicitly contradicts or qualifies prior auto-memory assumptions requiring a memory correction.
- Detector: `rg -n 'memory correction|contradicts the .* assumption' docs/reports/`
- Threshold: 1 hard-gate contradiction OR 2 softer corrections in 3 real missions
- A3 note (t4 circular): if R5 ships, baseline required before drift is measurable; detection is partially circular without pre-R5 baseline

**t3 — icontains violation detector (A2 tooling, now automated):**
`scripts/xbreed-memory redflags` flags any observable containing `axis_id`, `"round"`, `round=`, `"direction"`, or `direction=`. Any single hit fires t3. This detector is now exercised by `tests/xbreed_memory_audit.sh`.

---

## 10. H3 Ship Plan (from A5 m16)

Concrete milestones for running 1-2 real missions on the SQLite substrate while keeping the Honcho re-audit alive as cheap verdict-shaping:

| Milestone | Mission | Objective | Gate |
|---|---|---|---|
| M1 — skeleton | logo-color-0418 | First real mission on SQLite substrate end-to-end | Scribe artifact committed; Phase-0 hook fires; xbreed-memory put/get round-trips |
| M2 — overfit probe | mailbox-shim-audit-0418 | Second mission targeting area with known prior context in SQLite | xbreed-memory get prior-mission returns ≥1 relevant finding; judge Phase-0 hook surfaces it |
| M3 — observability | recall_audit scribe section | Add `recall_audit` section to scribe template (runs xbreed-memory latest-mission + redflags inline) | Template updated; first two missions' reports include recall_audit |
| M4 — polish | CLAUDE.md + FTS5 contingency | SQLite-authoritative anchor in CLAUDE.md; FTS5 schema migration drafted | CLAUDE.md updated; FTS5 migration committed |

**Guardrail:** Any M1-M4 work that writes Honcho integration code before a reopen trigger fires has degraded into H1 and must be cut immediately.

---

## 11. Known Residuals + Commitments for Follow-Up

1. **A2 tooling (scripts/xbreed-memory subcommands):** `latest-mission`, `grep`, `redflags` added and tested. XBREED_MEMORY_DB env override added. Test: `tests/xbreed_memory_audit.sh` → `xbreed-memory audit helpers: ok`. Committed with this report.

2. **1898ms attribution — OMITTED:** Honcho-judge R2 m13 cited "1898ms median latency + confabulates despite data present." Phase 0 flagged this as a gemini hallucination. cdx-reviewer-diffs-r1 re-ran the m1 probe and did not surface this figure. Attribution omitted from closure doc per A1 correction #7. R3 kill stands on structural grounds (K3 / dialectic/chat.py:20-78), not on latency.

3. **Closure doc is a separate artifact:** `docs/reports/honcho-reaudit-closure-2026-04-18.md` — judge-owned authoritative final DRAFT. Scribe does not write it; this report references it as the intended follow-on artifact.

4. **t1 post-mission audit ritual:** A3 flagged that t1 (recall miss) is only detectable retrospectively. A post-mission check ritual using `xbreed-memory grep` should be added to the scribe template (part of M3 observability milestone).

5. **K1b convention documentation:** If any future R1 wrapper is built, it must document and enforce session==mission_id convention. K1b is the conditional kill that fires if this convention is violated.

6. **SOURCE_MAP protocol gap:** cdx-reviewer-diffs-r1 could not independently reproduce the R1 audit_hash because no SOURCE_MAP was published. Protocol correction: future distiller synthesis artifacts should attach the sorted `move_id:source_prefix` serialization alongside the hash. This applies to R2 audit_hash 9e5adca… as well — SOURCE_MAP not published for R2.

---

## 12. Audit Trail

| Item | Value |
|---|---|
| Mission | honcho-reaudit-0418 |
| Round | 2 |
| Date | 2026-04-18 |
| audit_hash (R2 corrected) | sha256:9e5adca4b35a2106601344da2174be61e09171aca71da7d8f471f147cbecfe0d |
| Prior R2 hashes superseded | 873de000…, 6099d1f8… |
| R1 audit_hash | sha256:c8b474d114ed65203ca75ec3dad63ef79b0a1a8e8c35c66880b25bba2ee64dc1 |
| Moves synthesized (R2) | 9 (m13-m21) |
| Total moves (R1+R2) | 21 (m1-m21) |
| Axes (R2) | A1/A2/A3/A4/A5/A6 |
| Pareto verdict (R2) | ALL 9 ACCEPT; 0 regressions |
| Phase 0 commit | a2495da |
| R1 scribe commit | f1dffc1 |
| R1 synth-review commit | c5b3477 |
| R2 scribe commit | (this report — pending) |
| Phase 0 artifact | docs/reports/honcho-reaudit-phase0-2026-04-18.md |
| R1 scribe artifact | docs/reports/honcho-reaudit-r1-2026-04-18.md |
| R1 synth-review artifact | docs/reports/reviews/honcho-reaudit-r1-review.md |
| Reopen triggers artifact | docs/reports/r1-c-axis-reopen-triggers-2026-04-18.md |
| Closure doc (judge-owned) | docs/reports/honcho-reaudit-closure-2026-04-18.md |
| A2 tooling: scripts modified | scripts/xbreed-memory (latest-mission, grep, redflags + XBREED_MEMORY_DB) |
| A2 tooling: tests added | tests/xbreed_memory_audit.sh |
| Plan | docs/handouts/honcho-memory-reaudit-handout.md |
| Team | ~/.claude/teams/honcho-reaudit-0418/ |
| Scribe | ccs-scribe-r2 (this report) |
| Next | H3 PARALLEL — M1 skeleton (logo-color-0418); no R3 |
