# xbrd-exec-bench — Round 1 Audit Trail
**Mission:** xbrd-exec-bench-0424 | **Round:** 1 | **Date:** 2026-04-24
**commit:** dc1cac9

---

## Round 1 Scope

### Axes Named
| Axis | Label | Question |
|---|---|---|
| C | Coverage | Which source-bench cells are reachable through xask? |
| O | Overhead | How much wrapper latency does xask/xbreed add over raw `codex exec`? |
| F | Fidelity | Can TTFT and output_tokens be extracted reliably from the JSONL stream? |
| T | Toolchain | Does the measurement harness compile and run cleanly inside the repo? |
| S | Schema | Does the output table schema match the source bench format? |
| CMP | Comparative | Are fast-on/fast-off Δ rows legible and numerically valid? |
| M-SELECT | Model selection | Which model/effort combination wins on each metric? |

### Planner's 3 Key Divergences (from Data Walk)
| # | Divergence | Impact |
|---|---|---|
| D1 | `gpt-5.5` is absent from xask entirely — no constant, no lane, no CLI flag path (`src/ask.rs` lines 122–137; `scripts/xask` MODEL whitelist = `gemini\|codex` only) | Source bench's 8 gpt-5.5 cells have **zero xask analog**; declared WONTFIX via raw `codex exec -m gpt-5.5` probe (M6) |
| D2 | Spark effort hardcoded `model_reasoning_effort=low` at `src/ask.rs:77` | Spark cells at medium/high/xhigh effort unmappable through xask; spark column = low-only; other cells declared `N/A (xask low-only)` |
| D3 | `--json` flag survives full xask→xbreed→codex exec chain (`xask:51,67`, `ask.rs:67-68`) | TTFT proxy IS extractable from JSONL stdout without src changes; first `thread.started` event → first `item.completed` = TTFT anchor |

*(D4 also noted: `bench-metrics.py` scrapes teammate TRANSCRIPT JSONLs — not applicable to single-call xask; separate harness required.)*

### Specialists Dispatched (Round 1)
| Teammate | Axis | Tool/Route |
|---|---|---|
| g-connector-schema-crosslinks | CMP + cross-axis | `xask --effort high gemini`, session `3203fbb9` |
| cdx-labrat-m1-smoke | F + T | codex smoke probe, M1 skeleton |
| ccs-critic-approach | M-SELECT + approach | `xask -R codex` read on plan + routing bench report |
| ccs-scout-xask-surface | S + C | CLI surface audit, `codex exec --help` |
| ccs-planner-r0 (Part A/B) | O + T | `src/ask.rs` augment + `src/bin/xbrd-bench.rs` render binary (~150 LOC, no external crates) |

*Note: cdx-reviewer-bench-harness was non-responsive (2 prompts, DM-only). O+T axis coverage declared filled by ccs-planner-r0 Part A/B.*

---

## Per-Specialist Findings

### 1. g-connector-schema-crosslinks — CMP × Cross-Axis
**Summary:** Connector named four second-order interaction breaks that can silently corrupt table legibility even when individual columns are valid. The `_≄` glyph adaptation addresses missing Cov entries without breaking column alignment.

**Evidence (verbatim DM):**
> "log four named second-order breaks — Max-Bar Fallacy (CMP×V), Typographical Cavitation (F×CMP), Truncated Frontier Projection (E×CMP), Confounding Variable Entanglement (O×F) — plus the Cov column adaptation (`_≄` glyph pattern). All four flagged by gemini at certain/strong confidence. Source: xask --effort high gemini, session 3203fbb9."

**Named breaks:**
| Name | Axes | Risk |
|---|---|---|
| Max-Bar Fallacy | CMP×V | Block-bar scaled to max visible value; outlier compresses all other bars |
| Typographical Cavitation | F×CMP | ±σ glyph renders as empty column in narrow terminals |
| Truncated Frontier Projection | E×CMP | Δ row baseline is wrong if fast-off arm uses different effort |
| Confounding Variable Entanglement | O×F | Wrapper overhead and TTFT proxy error are correlated; Δ inflated |

---

### 2. cdx-labrat-m1-smoke — F + T (Schema Probe)
**Summary:** M1 skeleton confirmed: XBREED_BENCH_LOG fires, JSONL event sequence verified, `output_tokens` field location confirmed at `turn.completed.usage.output_tokens`. No timestamp field in bench log entry; no reasoning events in stream.

**Evidence (verbatim DM):**
> "Verbatim JSONL: `{\"t_complete_iso\":\"2026-04-24T10:31:58-03:00\",\"cli\":\"codex\",\"wall_s\":4.998,\"exit_code\":0,\"effort\":\"default\",\"teammate\":\"null\"}`. Event sequence: thread.started → turn.started → item.completed → turn.completed. usage.output_tokens at turn.completed.usage.output_tokens. No timestamp field, no reasoning events. Probe 3 parity confirmed."

**Implications for harness:**
- TTFT proxy = wall time from process start to first `item.completed` event (not a JSONL field — process I/O timing)
- `output_tokens` reliably at `turn.completed.usage.output_tokens`
- `effort` field in bench log = `"default"` for unlabeled runs; harness must inject effort label explicitly
- No `TTFT_ms` field in bench log — must be computed externally

---

### 3. ccs-critic-approach — M-SELECT + Approach (ACH Matrix)
**Summary:** ACH matrix over H1/H2/H3 frames. H3 (reframe to xbreed-reachable sub-question) survives disconfirmation; H1 and H2 rejected as primary frames. Three critiques: two RETHINK, one CONSIDER.

**Evidence (verbatim DM):**
> "H3 (reframe to xbreed-reachable sub-question) is the correct framing — survives ACH disconfirmation. H1 rejected: upper effort tiers are real operational territory (+67% output tokens, +34% wall time per source routing bench), not corner cases. H2 rejected as primary framing: gap is already declared WONTFIX in repo; treating scoped benchmark as broken global one produces paralysis without new coverage. Three critiques filed: [1] RETHINK: Title lacks scope qualifier → over-claim risk downstream. [2] RETHINK: Gap rows must be required output schema, not optional → H3 reframe validity gate. [3] CONSIDER: n=3 insufficient at top reachable tier; xhigh out-of-scope (hardcoded unreachable); n≥5 recommended for top reachable tier if variance claims appear in table. Evidence base: docs/plans/xbrd-exec-bench-plan-0424.md + docs/reports/codex-routing-findings-2026-04-16.md, verified via codex xask -R read."

**ACH verdicts:**
| Hypothesis | Verdict | Key disconfirmation |
|---|---|---|
| H1: Source bench scope is correct target | REJECT | Upper effort tiers (+67% tok, +34% wall) are real ops territory, not corners |
| H2: Scoped bench = broken global bench | REJECT | Gap already WONTFIX; framing produces paralysis without new coverage |
| H3: Reframe to xbreed-reachable sub-question | ACCEPT | No disconfirming evidence; aligns with D1/D2 declared gaps |

---

### 4. ccs-scout-xask-surface — S + C (CLI Surface)
**Summary:** Highest-fidelity audit evidence for the fast-off arm is the verbatim `codex exec --help` flag list. CLI version drift can invalidate the entire fast-off arm design; flag names at lines 44 and 87–88 are the load-bearing contract.

**Evidence (verbatim DM):**
> "For the audit trail, the highest-fidelity evidence to capture is the verbatim `codex exec --help` flag list (lines 44 + 87–88) as a snapshot — the CLI version can drift and these flag names are the load-bearing contract for the entire fast-off arm."

**Implication:** `codex exec --help` output must be captured as a versioned artifact at M1 gate and re-verified if CLI is updated. Fast-off arm flag contract is a fragile dependency.

---

### 5. ccs-planner-r0 Part A/B — O + T (Rust Harness Design)
**Summary:** Planner produced two-part implementation filling the O+T axis gap left by non-responsive reviewer. Part A: `src/ask.rs` augmented with `Instant`-based TTFT capture and `serde_json` metric emission. Part B: `src/bin/xbrd-bench.rs` render binary for table output and JSONL parse. Combined ~150 LOC, no external crates.

**Evidence:** Plan artifact `docs/plans/xbrd-exec-bench-plan-0424.md` (ccs-planner-r0, wwkd posture); O+T axis declared filled by team-lead relay (judge Pareto summary).

---

## Distiller Synthesis — All 11 Moves
*(Source: ccs-distiller SYNTHESIS_READY, relayed verbatim via team-lead Pareto summary. Direct distiller DM not received by scribe — provenance: team-lead relay.)*

| Move | Title | Axes | Confidence |
|---|---|---|---|
| MOVE-1 | Δ split (fast-on/fast-off Δ row as required schema element) | CMP | — |
| MOVE-2 | Contamination flags (mark cells where TTFT proxy error and wrapper overhead are correlated) | O×F | — |
| MOVE-3 | Gap rows schema required (gap rows must appear in output; not optional) | S + M-SELECT | — |
| MOVE-4 | TTFT anchor = `item.completed` (not `thread.started`; empirically confirmed by M1 probe) | F | — |
| MOVE-5 | n≥5 at top reachable tier; xhigh declared OOS (hardcoded unreachable in ask.rs) | M-SELECT | — |
| MOVE-6 | Composite validity gate (TTFT+output_tokens+wall_s all non-null = row validity) | T | — |
| MOVE-7 | 18/28 baseline (xask-reachable cell count declared as 18, not ~10 as originally estimated) | C | — |
| MOVE-8 | Substrate divergence footnote (WSL2 /tmp is ext4 warm-cache; drain pattern affects all cells equally) | T | — |
| MOVE-9 | Cov 3-glyph (`_≄` pattern for missing Cov entries) | CMP | single-source |
| MOVE-10 | Δ placeholder (Δ row rendered as `—` when only one arm measured, not omitted) | CMP | — |
| MOVE-11 | TTFT spawn-latency bias (xbreed Rust binary spawn ~5–15ms; model-dependent; empirical measurement needed) | O | empirical-needed |

*Dropped in distiller (not included in MOVE list): not specified in relay. See Rejected Alternatives below.*

---

## Judge's Pareto-Filter Verdict

**Axes net: C+1, O+1, F+1, T+2, S+1, CMP+3, M-SELECT+2. No axis regressed.**

### ACCEPT — Land in R2 (9 moves)
| Move | Reason |
|---|---|
| MOVE-1 (Δ split) | Required schema — legibility gate for fast-on/fast-off comparison |
| MOVE-2 (contamination flags) | Prevents silent correlation between O and F error sources |
| MOVE-3 (gap rows required) | H3 reframe validity gate; gap rows must be schema, not optional |
| MOVE-4 (TTFT anchor = item.completed) | Empirically confirmed by M1 probe; replaces invention-risk assumption |
| MOVE-5 (n≥5 top tier + xhigh OOS) | Critic CONSIDER promoted; xhigh structurally unreachable (ask.rs:77) |
| MOVE-6 (composite validity gate) | Prevents partial rows from silently corrupting table |
| MOVE-7 (18/28 baseline) | Corrects planner's conservative ~10 estimate; widens reachable coverage |
| MOVE-8 (substrate divergence footnote) | WSL2 ext4 warm-cache bias applies equally to all cells; must be documented |
| MOVE-10 (Δ placeholder) | Table rendering robustness; partial-arm runs must not omit Δ row |

### DEFER to M_final (1 move)
| Move | Reason |
|---|---|
| MOVE-9 (Cov 3-glyph `_≄`) | Single-source (connector only); implementation cost non-trivial; defer until render pass |

### PARK as M2 Probe (1 move)
| Move | Reason |
|---|---|
| MOVE-11 (TTFT spawn-latency bias model-dependent) | Empirical data needed before this can be quantified; block on M2 measurement |

---

## Rejected Alternatives

*(Moves dropped by distiller or not raised to Pareto; reasoning reconstructed from specialist inputs and judge relay.)*

| Alternative | Why dropped |
|---|---|
| H1 framing (source bench scope as target) | Rejected by critic ACH: upper effort tiers are real ops territory; scoping to source bench loses actionable signal |
| H2 framing (scoped = broken global) | Rejected by critic ACH: gap already WONTFIX; paralysis without new coverage |
| bench-metrics.py scraper approach | D4: scrapes teammate TRANSCRIPT JSONLs — not applicable to single-call xask; no transcript written by xask |
| Python subprocess wrapper (original M2 plan) | Superseded by Rust Part A/B (src/ask.rs Instant + src/bin/xbrd-bench.rs); no external crates, stays in-tree |
| TOML -c feature flags for fast-mode toggle | Replaced by first-class `--disable-fast` flag approach in harness design; TOML features non-discoverable at CLI level |
| xhigh effort for spark in bench matrix | Structurally unreachable (ask.rs:77 hardcodes `model_reasoning_effort=low`); declared N/A, not measured |
| Cov 3-glyph at R2 (MOVE-9 as immediate) | Single-source, implementation cost; deferred to M_final render pass |

---

## Optimization Routes Surveyed

### Wrapper Overhead Measurement
| Option | Decision |
|---|---|
| Python subprocess wrapper (original plan) | SUPERSEDED — bench-xask.py → xbrd-bench render binary; Python adds its own spawn overhead, confounding measurement |
| xask source modification (add Instant timer) | Part A: `src/ask.rs` Instant+serde_json augment — in-process, avoids subprocess layering |
| Raw `codex exec --json` direct call | Fast-off control arm: raw `codex exec` bypasses xask entirely; Δ between raw and xask = wrapper overhead |

### Fast-Off Control Arm Design
| Option | Decision |
|---|---|
| `codex exec --json -m <model>` direct | Selected: load-bearing contract at `codex exec --help` lines 44+87–88 (scout finding); must be versioned |
| `xask --fast off` toggle | Does not exist: `fast_mode` applied unconditionally in ask.rs:75–88; no CLI flag |
| Separate effort ladder for raw arm | Not needed: fast-off arm runs same effort as fast-on baseline for Δ validity |

### Benchmark Script Architecture
| Option | Decision |
|---|---|
| bench-xask.py (~200 LOC Python) | Original plan artifact (plan:87–99); superseded by Rust binary |
| src/bin/xbrd-bench.rs (~150 LOC Rust) | Selected: Part B from ccs-planner-r0; render+parse in-tree, no external crates, cargo build gated |
| JSONL accumulator at scratch/ | Retained: `scratch/bench-xask-results.jsonl` (gitignored); render reads accumulator |

### TTFT Proxy Design
| Option | Decision |
|---|---|
| Wall time to first stdout line (original) | Revised: anchor is `item.completed` event (not first stdout line) per M1 probe |
| Network first-byte measurement | Unobservable from xask outer; documented as methodological ceiling |
| Spawn-latency subtraction | PARK: model-dependent; requires M2 empirical data (MOVE-11) |

---

## Open Items Carried Forward to R2

| Item | Status | Owner |
|---|---|---|
| Apply 9 ACCEPT moves to harness design | R2 executor | cdx-executor-r2 |
| M1 gate: capture `codex exec --help` flag snapshot | R2 pre-work | cdx-executor-r2 |
| M2 probe: measure xbreed spawn latency for MOVE-11 | M2 gate | cdx-executor-r2 |
| cdx-reviewer-bench-harness non-responsive | Axis O+T filled by planner Part A/B; reviewer gap noted | — |
| MOVE-9 Cov glyph | DEFER to M_final render pass | M_final executor |

---

## Links
- **Plan:** `docs/plans/xbrd-exec-bench-plan-0424.md`
- **Connector session:** `3203fbb9`
- **Evidence doc:** `docs/reports/codex-routing-findings-2026-04-16.md` (critic H1 baseline)
- **Next:** R2 (`cdx-executor-r2` applies 9 ACCEPT moves + produces report artifact)
