# Bench Stack-Characterization — Mission A Round 1

**Date:** 2026-04-17
**Team:** bgst-stackbench-0417
**Mission:** scoped rewrite of docs/swarm-test-flow.md
**Stack:** sonnet-medium + godspeed-mode + mandatory-connector
**Bench dir:** /tmp/xbgst-bench-20260417-155932
**Round gate:** PARTIAL — executor writing, fixture gate pending, F-contradictions open

---

## Hypothesis

Is sonnet-medium + godspeed-mode ≥2x faster than opus-medium at comparable directional correctness?

Round 1 wall clock: **7m 47s** (bench_init 15:59:32 → r1_pareto_filter 16:07:19, UTC-3).
Directional correctness verdict deferred to R2 post-write verify. Speed datum captured.

---

## Axes (7)

| Axis | Label | Direction | Observable |
|------|-------|-----------|------------|
| F | Factual accuracy | ↑ | All doc claims match live codebase state |
| C | Completeness | ↑ | All dispatch paths + all probe tables present and correct |
| S | Size | ↓ | Lines removed from bloated/dead sections |
| R | Redundancy | ↓ | Duplicate content / copy-paste artefacts removed |
| T | Throughput | ↓ | Wall-clock time from bench_init to r1_pareto_filter |
| H | Hallucination coverage | ↓ | Refuted false claims; latent hallucinations surfaced |
| E | Evidence schema | ↑ | Evidence-block citations present in accepted moves |

---

## Telemetry summary

- **Teammates spawned:** 9 — all at 2026-04-17T16:01:15-03:00
  - ccs-executor-docrw-mA
  - g-connector-docrw-mA
  - cdx-reviewer-docrw-mA
  - cdx-labrat-docrw-mA
  - g-scout-docrw-mA
  - ccs-mutation-tester-docrw-mA
  - cdx-critic-docrw-mA
  - ccs-simplifier-docrw-mA
  - ccs-distiller-docrw-mA
- **Round 1 wall clock:** 7m 47s (bench_init → r1_pareto_filter)
- **Proposals landed:** 8/9 (ccs-distiller-docrw-mA stalled — 3 idle notifications after close-window nudge; judge applied Pareto filter directly)
- **Hallucinations refuted:** 3 (H1, H2, H3 — see Hallucinations section)
- **Mutations applied:** 5 — **5/5 survived** (0% test-suite coverage on target doc)
- **Pre-existing latent hallucination:** 1 — Claude swarm table has 4 rows (#1,#2,#3,#5; probe #4 missing) while Pareto filter claims 5/5 (H3, surfaced by ccs-mutation-tester-docrw-mA)
- **per-teammate.jsonl coverage:** SPARSE — only cdx-reviewer-docrw-mA has a completion record; all other teammates' completion timestamps are absent from per-teammate.jsonl

---

## Hallucinations (from hallucinations.jsonl, verbatim)

### H1

```json
{"id":"H1","claim":"loads templates/dispatch/<model>.md + substitutes {{QUERY}},{{CONTEXT}},{{SCOPE_BOUNDARY}}","location":"docs/swarm-test-flow.md:101-103","live_state":"templates/ dir does not exist; removed 2026-04-17. scripts/xask line 82-83 tombstone comment confirms removal.","severity":"blocker","axis":"F"}
```

### H2

```json
{"id":"H2","claim":"dispatch(\"claude\", ...) -> claude -p \"<prompt>\" --append-system-prompt \"<loadout>\"","location":"docs/swarm-test-flow.md:117-119","live_state":"src/ask.rs:496 match arm: other => bail!(expected codex|gemini). 'claude' is not valid. xask script lines 72-74 also only accepts gemini|codex.","severity":"blocker","axis":"F+H"}
```

### H3

```json
{"id":"H3","claim":"Claude swarm 5/5 probes pass","location":"docs/swarm-test-flow.md:54-61 + line 83","live_state":"Claude swarm table has 4 rows (#1,#2,#3,#5 — probe #4 missing). Pareto filter still claims 5/5. Count is wrong.","severity":"high","axis":"F"}
```

---

## Mutations (from mutations.jsonl, verbatim)

```json
{"mut_n":1,"change":"opus 4.7 max → opus 4.7 xhigh in Judge node","survived":true,"which_test":"NONE — no script reads swarm-test-flow.md"}
{"mut_n":2,"change":"xbreed ask gemini → xbreed ask gemma (model name swap)","survived":true,"which_test":"NONE — verify-docs.sh checks connector.md/AGENTS.md only"}
{"mut_n":3,"change":"gemini-3.1-pro-preview → gemini-4.0-pro (fake model id)","survived":true,"which_test":"NONE — no test validates model ids in docs"}
{"mut_n":4,"change":"Loadout::resolve([\"godspeed\"]) → Loadout::resolve([\"nosuchskill\"]) in dispatch chain","survived":true,"which_test":"NONE — doc code block is not compiled; cargo test has no doc-fixture runner"}
{"mut_n":5,"change":"Deleted entire Round 2 Roundtable section (lines 63-76)","survived":true,"which_test":"NONE — no section-completeness check exists for this file"}
```

**Finding:** 5/5 mutations survived. The target doc has zero test-suite coverage. All structural invariants (section presence, model names, command syntax, loadout names) are unverifiable by any currently-wired test.

---

## Moves (per teammate)

### M1 — ccs-executor-docrw-mA: Structural rewrite

**Proposed:** Full structural rewrite of docs/swarm-test-flow.md — correct dispatch chain (remove `claude` arm, update xask paths), fix probe tables, relabel mermaid nodes to reflect xask routing not xbreed ask.

**Evidence block:** No evidence: block in per-teammate.jsonl (completion record absent). Move reconstructed from r1_pareto_filter event and H1/H2 data.

**Pareto verdict:** ACCEPT (M1_executor_structural)

**Rationale:** Directly closes H1+H2 blockers; mermaid relabel aligns with connector's axis-blind-spot catch on L17-19 (xbreed ask → xask).

---

### M2 — g-connector-docrw-mA: Delete L95-121 (dead dispatch chain block)

**Proposed:** Delete lines 95-121 of swarm-test-flow.md — the `[claude]` arm of the dispatch chain code block is both factually wrong (H2) and dead code path; the entire `templates/` substitution narrative (H1) is in this section. Connector flagged that executor's relabel of L17-19 mermaid nodes (xbreed ask gemini/codex/claude → xask gemini/xask codex) would be incoherent unless this block was also removed or corrected.

**Evidence block:** No evidence: block in per-teammate.jsonl (completion record absent). Cross-axis catch reconstructed from r1_pareto_filter event M2_connector_delete_L95-121 and r1_proposals_in axes_signaled S:-26/R:-70.

**Pareto verdict:** ACCEPT (M2_connector_delete_L95-121)

**Rationale:** Mandatory-connector rule paid off — executor's move alone would have left mermaid L17-19 incoherent with the retained dispatch block. Connector caught the whole-table coherence gap.

---

### M3 — cdx-reviewer-docrw-mA: 3 factual blockers refuted

**Proposed:** Flag H1, H2, H3 as blockers requiring correction before any ship decision. Method: Read+Grep primary-source, no xask.

**Evidence block (from per-teammate.jsonl, verbatim):**

```json
{"teammate":"cdx-reviewer-docrw-mA","role":"factual-verifier","axes":["F","H"],"claims_verified":6,"claims_refuted":3,"method":"Read+Grep primary-source, no xask needed","verdict":"fail","timestamp":"2026-04-17"}
```

**Pareto verdict:** ACCEPT (M3_reviewer_3_refuted)

**Rationale:** Three blocker-severity facts wrong in the doc; two are implementation-visible (src/ask.rs:496, scripts/xask:72-74). Codex reviewer did primary-source verification without xask delegation — lower cost than planned.

---

### M4 — cdx-labrat-docrw-mA: L114 spark flag

**Proposed:** Line 114 of swarm-test-flow.md documents `codex exec --skip-git-repo-check -c model_reasoning_effort=<effort>` but omits `--spark` flag used in actual invocations. Flag as gap; propose adding `--spark` annotation.

**Evidence block:** No evidence: block in per-teammate.jsonl (completion record absent). Move reconstructed from r1_pareto_filter event M4_labrat_L114_spark.

**Pareto verdict:** ACCEPT (M4_labrat_L114_spark)

**Rationale:** Direct factual gap in documented codex --direct invocation; --spark is a real flag used in routing.

---

### M5 — g-scout-docrw-mA: Schema format (partial)

**Proposed:** Full protocol-schema rewrite — add YAML frontmatter schema block at top of doc defining dispatch fields, model IDs, and expected outputs for each probe type. Provide normative field definitions for `{{QUERY}}`, `{{CONTEXT}}`, `{{SCOPE_BOUNDARY}}` as schema (even if templates/ removed, schema documents intent).

**Evidence block:** No evidence: block in per-teammate.jsonl (completion record absent). Move reconstructed from r1_pareto_filter event M5_scout_schema_format_only and rounds.jsonl E:+schema axis signal.

**Pareto verdict:** PARTIAL (M5_scout_schema_format_only — format section only, not full schema rewrite)

**Rationale:** Full schema rewrite expands scope beyond doc repair; format-section addition (axis E) is in-scope. Full protocol-schema deferred (see Rejected Alternatives).

---

### M6 — ccs-mutation-tester-docrw-mA: Grep fixture

**Proposed:** Write `tests/swarm_doc_fixtures.sh` — bash fixture asserting section completeness, model name validity, and command syntax presence in swarm-test-flow.md. Motivated directly by 5/5 mutation survival rate (zero coverage finding).

**Evidence block (reconstructed from mutations.jsonl verbatim data — no per-teammate.jsonl record):**

All 5 mutations survived with `"which_test":"NONE"` — the coverage gap is the proposal's evidence base.

**Pareto verdict:** ACCEPT (M6_mutation_grep_fixture)

**Rationale:** 5/5 survival rate with zero existing coverage is a hard gap. Fixture addresses H axis (prevents future latent hallucinations slipping through). Writing deferred to R2 (mutation-tester writes tests/swarm_doc_fixtures.sh).

---

### M7 — cdx-critic-docrw-mA: CONVERT-TO-SCRIPT

**Proposed:** Do not repair the doc in-place. Convert swarm-test-flow.md from a narrative document into an executable validation script (`scripts/validate-swarm-flow.sh`) that actually runs the probes. Documentation becomes generated output. Eliminates drift permanently.

**Evidence block:** No evidence: block in per-teammate.jsonl (completion record absent). Move reconstructed from r1_pareto_filter event M7_critic_CONVERT (rejected).

**Pareto verdict:** REJECT (M7_critic_CONVERT)

**Rationale:** Scope expansion — Mission A is doc repair, not architecture change. CONVERT-TO-SCRIPT is a valid long-term option but changes the artifact type and requires probe harness infra not currently present. See Rejected Alternatives.

---

### M8 — ccs-simplifier-docrw-mA: Cut mermaid + 2 dead sections

**Proposed (3 cuts):**
- cut1: Remove duplicate "All clear 15/15 probes pass 3/3 models agree" from mermaid Done node (redundant with Pareto filter section below).
- cut2: Delete entire mermaid flowchart (L8-30) — diagram duplicates the prose tables; YAGNI, adds maintenance surface.
- cut3: Remove "## Dispatch chain verified end-to-end" section (L95-121) — superseded by H1+H2 corrections; executor's structural rewrite covers this.

**Evidence block:** No evidence: block in per-teammate.jsonl (completion record absent). Cuts reconstructed from r1_pareto_filter event M8_simplifier_cut1+cut3_only (partial) and M8_simplifier_cut2_mermaid (rejected).

**Pareto verdict:** PARTIAL (cut1 + cut3 ACCEPT; cut2 mermaid REJECT)

**Rationale for cut2 rejection:** Intra-claude conflict — executor (M1) and connector (M2) both proposed relabeling mermaid nodes to fix xbreed→xask routing labels. Simplifier's delete would drop the structural context the relabel was correcting. Judge applied KEEP+RELABEL over DELETE. See Conflicts section.

---

### M9 — ccs-distiller-docrw-mA: STALLED

**Proposed:** N/A — no proposal delivered.

**Evidence block:** N/A — 3 idle notifications after close-window nudge. No per-teammate.jsonl completion record.

**Pareto verdict:** N/A — judge applied Pareto filter directly without synthesis.

**Note:** Bench finding — distiller stalled on 8-way synthesis with sonnet-medium. Role may be too broad for this model tier at this proposal volume. Flag for Mission B R2.

---

## Conflicts

### Cross-model: Gemini vs Codex on repair strategy

- **Gemini-side (g-scout-docrw-mA, M5):** REWRITE-IN-PLACE with schema — repair the narrative doc, add a schema block for dispatch field definitions, keep the document format.
- **Codex-side (cdx-critic-docrw-mA, M7):** CONVERT-TO-SCRIPT — abandon the doc format entirely, generate documentation from an executable script.

**Judge resolution:** M5 partial ACCEPT (format section only), M7 REJECT. Repair-in-place is the mission scope. Schema full rewrite and CONVERT-TO-SCRIPT are both deferred to optimization routes.

---

### Intra-Claude: Simplifier CUT mermaid vs Executor+Connector KEEP+RELABEL mermaid

- **ccs-simplifier-docrw-mA (M8 cut2):** Delete mermaid entirely — YAGNI, maintenance surface, diagram is redundant.
- **ccs-executor-docrw-mA (M1):** Structural rewrite includes relabeling mermaid L17-19 nodes (xbreed ask gemini/codex/claude → xask gemini/xask codex).
- **g-connector-docrw-mA (M2):** Caught that mermaid relabel and L95-121 deletion must be coherent — validated executor's KEEP+RELABEL approach.

**Judge resolution:** KEEP+RELABEL wins. Simplifier's cut2 rejected. Mermaid L17-19 relabeled per M1; L95-121 block deleted per M2 (which makes cut3 redundant, but cut3 was already ACCEPT so executor's structural pass covers it).

---

## Rejected alternatives (explicit record)

| Move | Teammate | Rejection rationale |
|------|----------|---------------------|
| M7: CONVERT-TO-SCRIPT | cdx-critic-docrw-mA | Scope expansion beyond Mission A; changes artifact type; requires executable probe harness not yet built. Revisit when scripts/validate-swarm-flow.sh infra exists. |
| M5 (full): Protocol-schema rewrite | g-scout-docrw-mA | Full YAML frontmatter schema with normative field definitions exceeds repair scope; {{QUERY}} etc. are tombstoned (templates/ removed); schema would document removed intent, not current behavior. Revisit if dispatch schema is formally specified. |
| M8 cut2: Delete mermaid | ccs-simplifier-docrw-mA | Executor+connector consensus on KEEP+RELABEL overrides YAGNI argument; mermaid xbreed→xask relabel is the factual correction for R2 routing, not dead content. Revisit if mermaid adds no new information after relabel. |

---

## Optimization routes surveyed (not taken)

1. **Full CONVERT-to-script** (M7 origin): Replace docs/swarm-test-flow.md with scripts/validate-swarm-flow.sh; documentation auto-generated from test output. Eliminates drift permanently. Blocked by: Mission A scope, no probe harness infra. Revisit: Mission B or C if doc drift recurs post-R2.

2. **Scout's full protocol-schema rewrite** (M5 origin): Add YAML frontmatter schema defining dispatch field contract — model ids, flag names, expected exit codes, template substitution variables. Blocked by: templates/ dir removed; schema would document tombstoned API. Revisit: if xask dispatch contract is re-formalized.

3. **Simplifier's full mermaid delete** (M8 cut2 origin): YAGNI pruning of the flowchart entirely. Not taken because executor+connector's relabel corrects the mermaid's factual content (xbreed→xask routing labels). Revisit: post-relabel, if mermaid is still redundant with prose tables.

4. **Distiller synthesis before Pareto** (standard flow): Judge bypassed due to distiller stall. Planned: distiller reads all 8 proposals, deduplicates, flags contradictions, assigns confidence before judge filters. Actual: judge applied filter directly from raw proposals. Revisit: Mission B — consider tighter distiller brief or lower proposal count.

---

## R1 findings for stack itself (meta)

1. **Distiller stalled → bench finding:** ccs-distiller-docrw-mA received 8 proposals and stalled after 3 idle-notification nudges. Sonnet-medium may be under-resourced for 8-way synthesis at this proposal complexity. Action: R2 of Mission B — either scope distiller brief tighter or reduce concurrent proposal count.

2. **Connector (gemini high LOCKED) delivered axis-blind-spot catch:** g-connector-docrw-mA caught that executor's mermaid relabel (L17-19: xbreed ask → xask) would be incoherent unless the dispatch chain block (L95-121) was also corrected. This is a whole-table coherence gap that no focused specialist caught. The mandatory-connector rule (per memory `feedback_connector_every_round.md`) paid off in R1.

3. **Reviewer (codex R) found 3 factual blockers via Read+Grep only:** cdx-reviewer-docrw-mA verified 6 claims and refuted 3 using primary-source Read+Grep. No xask delegation needed. Cost lower than planned (no xask = no OAuth dispatch). This validates the reviewer's "primary-source first" posture.

4. **Mutation-tester found latent hallucination that no specialist caught:** ccs-mutation-tester-docrw-mA's independent probe found H3 (Claude swarm table has 4 rows, Pareto filter claims 5/5) — a count discrepancy that reviewer (checking claim truthfulness) and executor (checking dispatch paths) both missed. The independent-probe methodology found a gap all focused specialists passed over.

5. **per-teammate.jsonl is sparse:** Only cdx-reviewer has a completion record. 8 other teammates have no completion timestamps logged. This is a bench instrumentation gap — R2 should wire completion-event logging for all roles.

6. **5/5 mutation survival = zero doc coverage:** No wired test reads swarm-test-flow.md. Model names, section structure, command syntax, loadout names — all unguarded. M6 (grep fixture) directly addresses this but writing is deferred to R2.

---

## Round 1 → Round 2 handoff

- **Executor (ccs-executor-docrw-mA):** Writing the corrected docs/swarm-test-flow.md per accepted moves M1+M2+M3+M4+M8(cut1+cut3).
- **Mutation-tester (ccs-mutation-tester-docrw-mA):** Writing tests/swarm_doc_fixtures.sh per M6 — grep-based fixture asserting section completeness, model name validity, command syntax.
- **Fresh reviewer:** Spawn for R2 post-write verify — confirm H1/H2/H3 closed, H3 probe table count corrected, xask routing labels in mermaid corrected.
- **Pending gate conditions:**
  - All 4 F-axis contradiction entries close (H1, H2, H3 + M4 L114 spark correction)
  - tests/swarm_doc_fixtures.sh exists and passes `bash tests/swarm_doc_fixtures.sh` with ≥ 5 assertions
  - `wc -l docs/swarm-test-flow.md` < pre-R1 count (S-axis: -26 lines minimum)
- **Distiller:** Revisit role brief for R2 — current brief too broad for sonnet-medium on 8-way synthesis.
- **Bench instrumentation:** Wire per-teammate completion events for all 9 roles before R2 spawns.

---

## Pareto filter summary (from rounds.jsonl, verbatim)

```json
{"ts":"2026-04-17T16:07:19-03:00","event":"r1_pareto_filter","mission":"A","round":1,"accepted":["M1_executor_structural","M2_connector_delete_L95-121","M3_reviewer_3_refuted","M4_labrat_L114_spark","M6_mutation_grep_fixture"],"partial":["M5_scout_schema_format_only","M8_simplifier_cut1+cut3_only"],"rejected":["M7_critic_CONVERT","M8_simplifier_cut2_mermaid"],"distiller_drop":"ccs-distiller-docrw-mA stalled — 3 idle nofications after close-window nudge; judge applied filter directly","axis_moves":{"F":"+4_pending_close","C":"+4_sections_to_add","S":"-26_lines_cut","R":"-27_dup_lines_cut","H":"gate_added_via_fixture","E":"evidence_schema_in_moves_only"}}
```
