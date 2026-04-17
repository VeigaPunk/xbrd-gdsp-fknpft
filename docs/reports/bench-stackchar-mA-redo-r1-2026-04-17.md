# Bench Stack-Characterization — Mission A REDO Round 1

**Date:** 2026-04-17
**Team:** bgst-stackbench-redo-0417
**Parent run:** bgst-stackbench-0417 (halted by user as failure signal)
**Stack:** sonnet-medium + godspeed-mode + templates/ RESTORED (f3882aa)
**Bench dir:** /tmp/xbgst-bench-20260417-161538
**Round gate:** PASS

---

## Why REDO

- R1 halted by user: "this round was a failure signal"
- `templates/` premature delete — original intent commit 324402d removed the dir; reverted by f3882aa (restored 25 files in `templates/dispatch/`)
- `xask` "broken" claim was R1 labrat hallucination — binary works; 3 post-restore probes all exit 0
- Distiller stalled on 8-way synthesis — ccs-distiller-docrw-mA never delivered after 3 idle notifications
- R2 executor write was also reverted (stale partial diff discarded per user directive)
- Full post-halt remediation captured in commit f3882aa

---

## Methodology deltas vs original R1

| Delta | Original R1 | REDO R1 |
|-------|-------------|---------|
| Team size | 9 teammates | 5 teammates (drop scout, critic, simplifier, distiller) |
| Pareto filter | Distiller synthesis → judge | Judge applies filter directly — no distiller gate |
| Labrat probes | 1 run per probe (hallucination risk) | 2 runs per probe, diff outputs; flag NON_DETERMINISTIC |
| Reviewer method | cdx-reviewer via xask -R (multi-call) | ccs-reviewer CC-native (Read+Grep only) — no xask |
| Mutation-tester | Runs once, logs mutations.jsonl | Runs pre+post + self-tests its own fixture recursively |
| Connector | Gemini high LOCKED (✓) | Gemini high LOCKED (✓) — unchanged |

---

## Axes (7)

| Axis | Label | Direction | Observable |
|------|-------|-----------|------------|
| F | Factual accuracy | ↑ | All doc claims match live codebase state |
| C | Completeness | ↑ | All dispatch paths + probe tables present and correct |
| S | Size | ↓ | Lines removed from bloated/dead sections |
| R | Redundancy | ↓ | Duplicate content / copy-paste artefacts removed |
| T | Throughput | ↓ | Wall-clock time from bench_init to r1_pareto_filter |
| H | Hallucination coverage | ↓ | Refuted false claims; latent hallucinations surfaced |
| E | Evidence schema | ↑ | Evidence-block citations present in accepted moves |

---

## Telemetry

- **Teammates spawned:** 5 — all at `2026-04-17T16:15:48-03:00`
  - `ccs-executor-redo-mA`
  - `g-connector-redo-mA`
  - `ccs-reviewer-redo-mA`
  - `cdx-labrat-redo-mA`
  - `ccs-mutation-tester-redo-mA`
- **REDO R1 wall clock:** 4m 48s (`bench_redo_init` 16:15:38 → `redo_r1_pareto_filter` 16:20:26, UTC-3)
- **Original R1 wall clock:** 7m 47s (9 teammates, distiller stalled)
- **Speedup:** 467s / 288s = **1.62x** (rounds.jsonl estimate: "1.7x+")
- **Proposals:** 5/5 landed, 0 rejected, 0 partial (vs original R1: 8/9, distiller stalled)
- **0 regressions** — all 5 accepted by Pareto filter
- **per-teammate.jsonl coverage:** SPARSE — only `ccs-executor-redo-mA` has a completion record; other 4 teammates' completion timestamps absent (same instrumentation gap as original R1)
- **mutations.jsonl:** 1 blank line — mutation-tester findings captured in rounds.jsonl `"H"` field instead of dedicated file

---

## Hallucinations (from hallucinations.jsonl, verbatim — 6 entries)

### H1-RESOLVED

```json
{"id":"H1-RESOLVED","claim":"templates/dispatch/<model>.md does not exist","location":"docs/swarm-test-flow.md:101-103","live_state":"templates/dispatch/ EXISTS with gemini.md, codex.md, claude.md, README.md; xask:82 TEMPLATE_DIR points correctly","severity":"resolved","axis":"H"}
```

**Status:** Resolved by commit f3882aa (templates/ restored). Original R1 H1 was valid at time of labrat probe; now closed.

---

### H2

```json
{"id":"H2","claim":"Judge runs at 'opus 4.7 max'","location":"docs/swarm-test-flow.md:22","live_state":"~/.claude/agents/the-judge.md frontmatter: effort=xhigh (not max); the-judge.md:39 confirms 'opus 4.7 · xhigh'","severity":"medium","axis":"H"}
```

**Status:** Still holds. `max` vs `xhigh` distinction is a doc factual error.

---

### H3

```json
{"id":"H3","claim":"xask --direct codex '10*10' passes (exit 0)","location":"docs/swarm-test-flow.md:51","live_state":"scripts/xask flag parser lines 37-48 has NO --direct case; hits '*' catchall -> exit 1 with 'unknown flag --direct'","severity":"blocker","axis":"F"}
```

**Status:** Still holds. Triangulation confirms both runs exit 1 at 3ms each. BLOCKER.

---

### H4

```json
{"id":"H4","claim":"dispatch('claude', ...) -> claude -p '<prompt>' --append-system-prompt '<loadout>'","location":"docs/swarm-test-flow.md:117-119","live_state":"ask.rs:496: `other => anyhow::bail!(\"unknown cli: {other} (expected codex|gemini)\")` — claude arm does NOT exist; xbreed ask claude bails at runtime","severity":"blocker","axis":"H"}
```

**Status:** Still holds. `src/ask.rs:496` confirms claude dispatch bails regardless of templates/ state. BLOCKER. Note: HC_connector_redo_1 below shows connector initially confused templates/ restore with H4 resolution — refuted by judge primary-source.

---

### H5

```json
{"id":"H5","claim":"Claude swarm 5/5 probes pass (AXES FINAL STATE L83)","location":"docs/swarm-test-flow.md:54-62,83","live_state":"Claude swarm table has rows #1,2,3,5 only — row #4 MISSING (4 rows in table vs 5/5 claim). ALSO: all probes use 'xbreed ask claude' which fails per H4","severity":"high","axis":"F"}
```

**Status:** Still holds. Two compounding errors: missing row #4, and all probe commands route through dead `claude` arm.

---

### HC_connector_redo_1

```json
{"id":"HC_connector_redo_1","claim":"H2 claude dispatch resolved by templates/ restore","source":"g-connector-redo-mA","refuted_by":"judge direct Read src/ask.rs:496","live_state":"other => anyhow::bail!(\"unknown cli: {other} (expected codex|gemini)\") — still bails on claude regardless of template file existence","severity":"medium","axis":"F_hallucination_by_connector_not_doc","pattern":"critic_hallucination_extended_to_connector"}
```

**Status:** Connector hallucinated mid-round that H4 was resolved by restoring templates/. Judge primary-sourced `src/ask.rs:496` and refuted. Connector self-corrected. Pattern: `critic_hallucination_extended_to_connector` — confirms the prior finding (even strong models conflate file-existence with code-path existence).

---

## Triangulation data (from triangulation.jsonl, verbatim — 10 rows)

**Labrat: 5 probes × 2 runs = 10 rows, 0 NON_DETERMINISTIC**

```json
{"teammate":"cdx-labrat-redo-mA","probe":1,"run":1,"cmd":"xask --spark codex \"say only PROBE1\"","exit":0,"wall_ms":4337,"stdout_head":"PROBE1 | godspeed","stderr_head":""}
{"teammate":"cdx-labrat-redo-mA","probe":1,"run":2,"cmd":"xask --spark codex \"say only PROBE1\"","exit":0,"wall_ms":6333,"stdout_head":"PROBE1 | godspeed","stderr_head":""}
{"teammate":"cdx-labrat-redo-mA","probe":2,"run":1,"cmd":"xask -R codex \"say only REVIEW1\"","exit":0,"wall_ms":6187,"stdout_head":"REVIEW1 | godspeed","stderr_head":""}
{"teammate":"cdx-labrat-redo-mA","probe":2,"run":2,"cmd":"xask -R codex \"say only REVIEW1\"","exit":0,"wall_ms":4983,"stdout_head":"REVIEW1 | godspeed","stderr_head":""}
{"teammate":"cdx-labrat-redo-mA","probe":3,"run":1,"cmd":"xask --direct codex \"2+2\"","exit":1,"wall_ms":3,"stdout_head":"","stderr_head":"xask: unknown flag '--direct'"}
{"teammate":"cdx-labrat-redo-mA","probe":3,"run":2,"cmd":"xask --direct codex \"2+2\"","exit":1,"wall_ms":3,"stdout_head":"","stderr_head":"xask: unknown flag '--direct'"}
{"teammate":"cdx-labrat-redo-mA","probe":4,"run":1,"cmd":"xask gemini \"say only GEM1\"","exit":0,"wall_ms":12932,"stdout_head":"GEM1 | godspeed","stderr_head":""}
{"teammate":"cdx-labrat-redo-mA","probe":4,"run":2,"cmd":"xask gemini \"say only GEM1\"","exit":0,"wall_ms":9190,"stdout_head":"GEM1 | godspeed","stderr_head":""}
{"teammate":"cdx-labrat-redo-mA","probe":5,"run":1,"cmd":"xbreed ask codex --spark --with godspeed \"say WORKS1\"","exit":0,"wall_ms":5994,"stdout_head":"WORKS1","stderr_head":""}
{"teammate":"cdx-labrat-redo-mA","probe":5,"run":2,"cmd":"xbreed ask codex --spark --with godspeed \"say WORKS1\"","exit":0,"wall_ms":6256,"stdout_head":"WORKS1","stderr_head":""}
```

**Triangulation findings:**
- `--spark codex`: 4337ms / 6333ms — both exit 0. Binary works.
- `-R codex`: 6187ms / 4983ms — both exit 0. **REFUTES original R1 labrat's "exit 1 unknown flag"** — original R1 labrat hallucinated the exit code.
- `--direct codex`: 3ms / 3ms — both exit 1 with identical stderr. Flag genuinely absent. H3 confirmed blocker.
- `gemini`: 12932ms / 9190ms — both exit 0. OAuth dispatch functional.
- `xbreed ask --spark --with godspeed`: 5994ms / 6256ms — both exit 0. Full stack functional post-restore.

**Methodology finding:** Original R1 labrat hallucinated exit codes for `-R codex` (claimed unknown flag; actual exit 0). Two-run triangulation with diff-outputs mandate is the correct control.

---

## Cross-model convergence

Three independent sources confirmed `--direct` dead and H3 blocker status:

| Source | Model | Method | Finding |
|--------|-------|--------|---------|
| `cdx-labrat-redo-mA` | codex (spark) | empirical 2×run | exit 1, wall_ms 3, stderr "xask: unknown flag '--direct'" |
| `ccs-reviewer-redo-mA` | Claude | Read+Grep scripts/xask:37-48 | `*` catchall → exit 1, no `--direct` case |
| `g-connector-redo-mA` | Gemini (high) | cross-axis pattern | reinforced labrat+reviewer signal on `--direct`; flags dead probe tables |

Three-layer agreement → H3 blocker is not a labrat artifact. Whole probe table column (--direct exit codes) may be fabricated; connector requested full exit-code sweep in R2.

---

## Moves (per teammate — 5/5 ACCEPT)

### M1 — ccs-executor-redo-mA: Structural rewrite plan

**Axes:** C (+4), S (-5 lines)

**Proposal:** Plan the R2 write to close H1-RESOLVED path corrections, H3 `--direct` removal, H4 `claude` arm removal, H5 probe table row-4 addition + command corrections. Minimum-diff discipline: 7 line changes + 5 line deletions, bounded scope.

**Evidence (from per-teammate.jsonl — only record in REDO):**

```json
{"teammate":"ccs-executor-redo-mA","role":"executor","axes":["C","S"],"round":1,"t_complete_iso":"2026-04-17T19:18:22-03:00","wall_s":142,"evidence_present":true,"proposal_words":187}
```

Note: `t_complete_iso` reflects R2 write completion (~3h after init); R1 proposal wall time captured as `executor_60s` in rounds.jsonl T field.

**Pareto verdict:** ACCEPT (M1_executor_structural_redo)

---

### M2 — g-connector-redo-mA: Exit-code sweep hypothesis + HC catch

**Axes:** F (cross-axis), H (self-hallucination caught)

**Proposal:** Reinforced H3+H4 blocker status via cross-model triangulation. Raised hypothesis that `--direct` H3 may be symptomatic — other probe table exit codes may also be fabricated (not just `--direct`). Requested full exit-code column sweep in R2. Also surfaced HC_connector_redo_1 (initially claimed H4 resolved by templates/ restore) — self-corrected after judge primary-sourced `src/ask.rs:496`.

**Evidence (reconstructed from rounds.jsonl — per-teammate record absent):**

```
rounds.jsonl: "connector_self_hallucination_caught":"H2_claude_dispatch_resolved_by_restore_was_wrong_connector_self_corrected_after_judge_primary_source_rebut"
rounds.jsonl: "cross_model_convergence":"labrat+reviewer+connector_3_layer_on_--direct_dead"
rounds.jsonl: "new_methodology_finding":"swarm-test-flow_probe_tables_may_be_fabricated_not_just_--direct_R2_full_exit_code_sweep_requested"
hallucinations.jsonl HC_connector_redo_1: refuted_by="judge direct Read src/ask.rs:496"
```

**Pareto verdict:** ACCEPT (M2_connector_exit_code_sweep_hypothesis)

---

### M3 — ccs-reviewer-redo-mA: 3 live blockers confirmed CC-native

**Axes:** F (+3 confirmed), H

**Proposal:** Primary-source Read+Grep verification of 6 doc claims. Confirmed H3 (`scripts/xask:37-48` — no `--direct` case), H4 (`src/ask.rs:496` — `claude` bails), H5 (probe table has 4 rows not 5; all commands route dead claude arm). No xask delegation — CC-native Read+Grep only.

**Evidence (reconstructed from original per-teammate record which carries method description):**

```json
{"teammate":"cdx-reviewer-docrw-mA","role":"factual-verifier","axes":["F","H"],"claims_verified":6,"claims_refuted":3,"method":"Read+Grep primary-source, no xask needed","verdict":"fail","timestamp":"2026-04-17"}
```

Note: Original run record cited for method; REDO reviewer (ccs-reviewer-redo-mA) applied identical method. Reviewer wall time: 90s per rounds.jsonl T field.

**Pareto verdict:** ACCEPT (M3_reviewer_blockers_confirmed_redo)

**Stack finding:** CC-native reviewer at ~90s was faster than original R1 codex-R reviewer (xask multi-call round-trips, time unrecorded). Validates CC-native primary-source posture.

---

### M4 — cdx-labrat-redo-mA: Triangulation — 0 NON_DETERMINISTIC

**Axes:** H (original R1 hallucinations refuted), empirical

**Proposal:** 5 probes × 2 runs each. Diff outputs; flag any NON_DETERMINISTIC exits. Key finding: `-R codex` exits 0 on both runs — REFUTES original R1 labrat's claimed "exit 1 unknown flag". `--direct` exits 1 on both runs at wall_ms 3 — CONFIRMS H3 as genuine (not noise). Full table verbatim in triangulation.jsonl.

**Evidence (from triangulation.jsonl, byte-verbatim — all 10 rows above in Triangulation section).**

Wall time: 51s per rounds.jsonl T field.

**Pareto verdict:** ACCEPT (M4_labrat_triangulation_0_nondeterministic)

---

### M5 — ccs-mutation-tester-redo-mA: Fixture gate + self-test

**Axes:** H (fixture coverage), test robustness

**Proposal:** Ran `tests/swarm_doc_fixtures.sh` against original doc — 4 real failures caught (validates fixture detects live stale-doc issues). Applied 3 mutations to fixture: 2/3 survived (brittle gate flagged — 2 mutation classes unguarded). Mutation-tester tested its own fixture recursively (R1 self-test). R2 mandate: harden fixture to catch all 3 mutation classes (expected: 3/3 catch after R2).

**Evidence (from rounds.jsonl axis_deltas_final H field — per-teammate record absent):**

```
rounds.jsonl: "H":"fixture_gate_4_real_failures_caught_+_2_fixture_mutations_survived_brittle_gate_flagged"
```

Wall time: 60s per rounds.jsonl T field.

**Pareto verdict:** ACCEPT (M5_mutation_tester_fixture_self_test)

---

## Rejected / deferred

- **Cross-doc cleanups** for `docs/command-flows.md` and `docs/xask-protocol.md` (stale "templates/ removed" claims introduced post-324402d) → **DEFERRED to R3** — not in Mission A scope, no regression risk until those docs are referenced in tests.
- **Mission B** (benchmark mailbox round-trip latency stack) → **DEFERRED** to subsequent team per parent plan.
- **Mission C** (validate godspeed skill loading perf) → **DEFERRED** to subsequent team per parent plan.
- **Full exit-code column sweep** (connector hypothesis: all probe table exit codes may be fabricated) → **DEFERRED to R2** — ccs-reviewer-redo-mA assigned.
- **Fixture hardening** (2 surviving mutation classes) → **DEFERRED to R2** — ccs-mutation-tester-redo-mA assigned.

---

## Stack health findings

1. **Fixture (tests/swarm_doc_fixtures.sh) is real:** Catches 4 genuine stale-doc issues on original doc. Not a toy artifact — live signal.

2. **Fixture robustness gap:** 1 of 3 mutations caught in R1 → R2 target is 3/3. Two brittle paths flagged by self-test. Mutation-tester's recursive self-probe is a methodology win — found its own weak spots before R2 reviewer sees them.

3. **Executor minimum-diff discipline:** Proposal bounded to 7 line changes + 5 line deletions. No scope creep from original R1's broader structural rewrite plan.

4. **Connector self-corrected on HC:** HC_connector_redo_1 was flagged by judge, connector immediately accepted primary-source evidence and updated its cross-axis summary. Good signal — connector is a sound axis when judge maintains primary-source authority.

5. **CC-native reviewer faster than xask-R:** ~90s wall time for 6 verified claims. Original R1 codex-R reviewer used xask -R (multi-call round-trips, time not recorded but structurally slower). For primary-source verification tasks, CC-native Read+Grep is the right default.

6. **Original R1 labrat hallucinated two exit codes:** `-R codex` ("exit 1 unknown flag" — actual exit 0) and original `--direct` single-run probe (only marginally different — both confirm dead, but the `-R` fabrication shows single-run empirical probes are unreliable). Triangulation mandate now mandatory for all empirical labrat work.

7. **per-teammate.jsonl still sparse in REDO:** Only executor has a completion record. Other 4 teammates have no timestamps. Bench instrumentation gap persists from original R1; carry to R3 backlog.

8. **mutations.jsonl empty in REDO:** Mutation-tester logged to rounds.jsonl aggregate, not per-run records. mutation-tester's findings are in rounds.jsonl `H` field only. Instrumentation gap.

---

## Round 1 → Round 2 handoff

- **ccs-executor-redo-mA:** Writing corrected `docs/swarm-test-flow.md` per accepted moves M1–M5. Minimum-diff discipline: close H1-RESOLVED paths, remove H3 `--direct` row, remove H4 `claude` arm, fix H5 probe table row #4 + command corrections. R2 write already in progress (per-teammate.jsonl t_complete 19:18:22).
- **ccs-reviewer-redo-mA (R2):** Full exit-code column sweep (connector hypothesis — probe tables may be broadly fabricated, not just `--direct`). Primary-source Read+Grep only; no xask.
- **ccs-mutation-tester-redo-mA (R2):** Fixture self-test harden — guard 2 surviving mutation classes identified in R1 self-probe. Target: 3/3 mutations caught after R2.
- **ccs-scribe-redo-mA (you):** This report writing in parallel with R2 execution per amended plan.
- **Pending gate conditions for mission close:**
  - H3, H4, H5 closed in doc (reviewer R2 confirms)
  - Exit-code column sweep clean or new hallucinations logged
  - `bash tests/swarm_doc_fixtures.sh` passes with ≥5 assertions
  - `wc -l docs/swarm-test-flow.md` < pre-REDO count (S-axis: -5 lines minimum)
  - Fixture mutation catch rate ≥ 3/3 (up from R1's 1/3)

---

## Pareto filter (from rounds.jsonl, verbatim)

```json
{"ts":"2026-04-17T16:20:26-03:00","event":"redo_r1_pareto_filter","mission":"A_redo","round":1,"proposals":5,"accepted":5,"rejected":0,"partial":0,"distiller":"skipped_by_design_judge_direct_Pareto","wall_total_min":"<4","comparison_to_r1":"R1=7m47s_9teammates_distiller_stalled vs REDO_R1=<4m_5teammates_judge_direct → 1.7x+ speedup","axis_deltas_final":{"F":"+4 real doc contradictions (H3 --direct, H4 claude arm, H5 claude count) + 2 R1_labrat_hallucinations_refuted + 1 resolved (H1)","C":"+4 changes planned","S":"-5 lines planned","R":"-1 dup","T":"executor_60s_connector_90s_reviewer_90s_labrat_51s_mutation_60s","H":"fixture_gate_4_real_failures_caught_+_2_fixture_mutations_survived_brittle_gate_flagged","E":"5/5_evidence_fields_present"},"connector_self_hallucination_caught":"H2_claude_dispatch_resolved_by_restore_was_wrong_connector_self_corrected_after_judge_primary_source_rebut","cross_model_convergence":"labrat+reviewer+connector_3_layer_on_--direct_dead","new_methodology_finding":"swarm-test-flow_probe_tables_may_be_fabricated_not_just_--direct_R2_full_exit_code_sweep_requested"}
```

---

## Links

- Plan: `docs/swarm-test-flow.md` (target doc)
- Parent run report: `docs/reports/bench-stackchar-mA-r1-2026-04-17.md`
- Original telemetry: `/tmp/xbgst-bench-20260417-155932/`
- REDO telemetry: `/tmp/xbgst-bench-20260417-161538/`
- templates/ restoration commit: f3882aa
- Next: REDO R2 — executor write + reviewer exit-code sweep + fixture harden
