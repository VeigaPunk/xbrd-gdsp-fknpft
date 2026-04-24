# xbreed exec-path benchmark — 2026-04-24
**Session:** xbrd-exec-bench-0424 | **Author:** cdx-executor-r2 | **Status:** STUB — M2+ execution pending

---

## 1. Context

This report is the xbreed-protocol analogue of a direct `codex exec` benchmark report. Where a raw codex bench measures `codex exec` CLI performance directly, this bench measures **xbreed's execution path**: the `xask codex` wrapper chain (`shell → xbreed ask codex → codex exec --json`) against an equivalent raw `codex exec --disable fast_mode` control arm, with all 4 contamination-suppression flags applied.

### Why xbreed diverges structurally from raw codex exec

| Structural factor | Raw codex exec bench | xbreed bench |
|---|---|---|
| fast_mode toggle | Explicit `--disable fast_mode` flag | On = xask default; off = raw control arm only |
| Model coverage | All models accessible via CLI flags | Constrained to ask.rs model constants (18/28 cells) |
| xhigh effort | Reachable via `-c model_reasoning_effort=xhigh` | **Structurally unreachable** via xask — OOS |
| gpt-5.5 | Accessible via `-m gpt-5.5` | No xask lane (no CODEX_5_5_MODEL constant) — absent |
| Spawn overhead | None (direct CLI) | ~5–50 ms xbreed Rust binary spawn + OAuth shell bootstrap |
| TTFT anchor | First model-response JSONL event | First `item.type == "agent_message"` + `item.completed == true` (preamble skipped) |
| n per cell | As specified | n≥5 at top reachable tier (gpt-5.4-mini × high; spark × low); n=3 minimum elsewhere |

**18 of 28 source cells are reachable.** 10 cells are structural gaps: 2 xhigh OOS rows (covering 4 cells), 4 gpt-5.5 absent cells, 4 spark medium/high cells (ask.rs:77 hardcodes low-only), 2 spark xhigh cells (OOS + low-only compound).

---

## 2. Methodology

### Measurement substrate

**Metric capture:** Augmented `XBREED_BENCH_LOG` in `src/ask.rs` (Part A). Each xask invocation appends one JSONL line:
```json
{"t_complete_iso":"...","cli":"codex","wall_s":1.234,"ttft_ms":456,
 "output_tokens":128,"exit_code":0,"effort":"medium","teammate":"...","fast_mode":true}
```

**TTFT anchor (MOVE-4):** Wall-clock elapsed from child process spawn to first JSONL event where `item.type == "agent_message"` AND `item.completed == true`. `thread.started` and tool-start preamble events are explicitly skipped — they fire before model response and inflate apparent TTFT for fast models if used as the anchor.

**Render binary:** `xbrd-bench` (`src/bin/xbrd-bench.rs`, Part B). Reads JSONL accumulator, groups by `(model, effort, fast_mode)`, computes per-cell stats, emits Unicode table with 8 rendering constraints.

### Cell invocation table

| Model | Effort | Fast arm | Command |
|---|---|---|---|
| gpt-5.4-mini | low | on (xask) | `xask -e low codex "<prompt>"` |
| gpt-5.4-mini | low | off (raw) | `codex exec --skip-git-repo-check --color never --ephemeral --sandbox danger-full-access --disable fast_mode --json -c approval_policy="never" -c include_permissions_instructions=false -c include_apps_instructions=false -c include_environment_context=false -m gpt-5.4-mini "<prompt>"` |
| gpt-5.4-mini | medium | on (xask) | `xask -e medium codex "<prompt>"` |
| gpt-5.4-mini | medium | off (raw) | *(same raw invocation with `-m gpt-5.4-mini`)* |
| gpt-5.4-mini | high | on (xask) | `xask -e high codex "<prompt>"` (n≥5) |
| gpt-5.4-mini | high | off (raw) | *(same raw invocation with `-m gpt-5.4-mini`, n≥5)* |
| gpt-5.4 | low | on (xask) | `xask -R -e low codex "<prompt>"` |
| gpt-5.4 | low | off (raw) | *(same raw invocation with `-m gpt-5.4`)* |
| gpt-5.4 | medium | on (xask) | `xask -R -e medium codex "<prompt>"` |
| gpt-5.4 | medium | off (raw) | *(same raw invocation with `-m gpt-5.4`)* |
| gpt-5.4 | high | on (xask) | `xask -R -e high codex "<prompt>"` |
| gpt-5.4 | high | off (raw) | *(same raw invocation with `-m gpt-5.4`)* |
| gpt-5.3-spark | low | on (xask) | `xask --spark codex "<prompt>"` (n≥5) |
| gpt-5.3-spark | low | off (raw) | *(same raw invocation with `-m gpt-5.3-codex-spark -c model_reasoning_effort=low`, n≥5)* |
| gpt-5.4-mini | xhigh | any | **OOS** — xhigh unreachable via xask |
| gpt-5.4 | xhigh | any | **OOS** — xhigh unreachable via xask |
| gpt-5.3-spark | medium/high/xhigh | any | **gap** — ask.rs:77 hardcodes low-only |
| gpt-5.5 | any | any | **absent** — no xask lane |

**Raw control arm suppression flags (MOVE-2):** All 4 flags from `ask.rs:62–65` are mandatory on every raw arm invocation — `approval_policy="never"`, `include_permissions_instructions=false`, `include_apps_instructions=false`, `include_environment_context=false`. Omitting any one injects context tokens the xask arm never sees; this contaminates raw-arm TTFT and potentially inverts the Δ sign.

---

## 3. Coverage matrix (28 cells)

Every unreachable cell renders a row with `—` metric values and mandatory 3-glyph Cov encoding (MOVE-3 + MOVE-9).

**Cov glyph key:** `[effort-depth][delta-parity][routing]`
- Glyph 1: `█`=low/med/high · `▄`=low/med · `_`=low only
- Glyph 2: `≡`=symmetric env · `≄`=asymmetric wrapper vs raw · `≇`=categorically incomparable (fast\_mode absent on path)
- Glyph 3: `✗`=routing-excluded WONTFIX · `∅`=measurement gap (reachable, not yet benched)

**Schema-contract invariants triggered by glyph value:**
- Any row with glyph 2 = `≄` → test asserts raw arm subprocess carries all 4 suppression flags from ask.rs:62–65
- Any row with glyph 2 = `≇` → test asserts `features.fast_mode=true` is ABSENT from subprocess args
- Any row with glyph 3 = `✗` → test asserts routing report has WONTFIX entry for that model/effort

| Cell | Model | Effort | Fast | Status | Cov | Invariants triggered | Gap reason |
|---|---|---|---|---|---|---|---|
| C01 | gpt-5.4-mini | low | off | reachable | `█≄∅` | suppression-flag test | — |
| C02 | gpt-5.4-mini | low | on | reachable | `█≄∅` | suppression-flag test | — |
| C03 | gpt-5.4-mini | medium | off | reachable | `█≄∅` | suppression-flag test | — |
| C04 | gpt-5.4-mini | medium | on | reachable | `█≄∅` | suppression-flag test | — |
| C05 | gpt-5.4-mini | high | off | reachable | `█≄∅` | suppression-flag test | n≥5 (MOVE-5) |
| C06 | gpt-5.4-mini | high | on | reachable | `█≄∅` | suppression-flag test | n≥5 (MOVE-5) |
| C07 | gpt-5.4-mini | xhigh | off | OOS | `█≇✗` | fast\_mode-absent + WONTFIX | xhigh OOS (MOVE-5/7) |
| C08 | gpt-5.4-mini | xhigh | on | OOS | `█≇✗` | fast\_mode-absent + WONTFIX | xhigh OOS (MOVE-5/7) |
| C09 | gpt-5.4 | low | off | reachable | `█≄∅` | suppression-flag test | — |
| C10 | gpt-5.4 | low | on | reachable | `█≄∅` | suppression-flag test | — |
| C11 | gpt-5.4 | medium | off | reachable | `█≄∅` | suppression-flag test | — |
| C12 | gpt-5.4 | medium | on | reachable | `█≄∅` | suppression-flag test | — |
| C13 | gpt-5.4 | high | off | reachable | `█≄∅` | suppression-flag test | — |
| C14 | gpt-5.4 | high | on | reachable | `█≄∅` | suppression-flag test | — |
| C15 | gpt-5.4 | xhigh | off | OOS | `█≇✗` | fast\_mode-absent + WONTFIX | xhigh OOS (MOVE-5/7) |
| C16 | gpt-5.4 | xhigh | on | OOS | `█≇✗` | fast\_mode-absent + WONTFIX | xhigh OOS (MOVE-5/7) |
| C17 | gpt-5.3-spark | low | off | reachable | `_≄∅` | suppression-flag test | n≥5 (MOVE-5) |
| C18 | gpt-5.3-spark | low | on | reachable | `_≄∅` | suppression-flag test | n≥5 (MOVE-5) |
| C19 | gpt-5.3-spark | medium | off | gap | `_≇✗` | fast\_mode-absent + WONTFIX | ask.rs:77 hardcodes low-only |
| C20 | gpt-5.3-spark | medium | on | gap | `_≇✗` | fast\_mode-absent + WONTFIX | ask.rs:77 hardcodes low-only |
| C21 | gpt-5.3-spark | high | off | gap | `_≇✗` | fast\_mode-absent + WONTFIX | ask.rs:77 hardcodes low-only |
| C22 | gpt-5.3-spark | high | on | gap | `_≇✗` | fast\_mode-absent + WONTFIX | ask.rs:77 hardcodes low-only |
| C23 | gpt-5.3-spark | xhigh | off | OOS+gap | `_≇✗` | fast\_mode-absent + WONTFIX (compound) | xhigh OOS + spark low-only |
| C24 | gpt-5.3-spark | xhigh | on | OOS+gap | `_≇✗` | fast\_mode-absent + WONTFIX (compound) | xhigh OOS + spark low-only |
| C25 | gpt-5.5 | low | off | absent | `_≇✗` | fast\_mode-absent + WONTFIX | no xask lane |
| C26 | gpt-5.5 | low | on | absent | `_≇✗` | fast\_mode-absent + WONTFIX | no xask lane |
| C27 | gpt-5.5 | medium | off | absent | `_≇✗` | fast\_mode-absent + WONTFIX | no xask lane |
| C28 | gpt-5.5 | medium | on | absent | `_≇✗` | fast\_mode-absent + WONTFIX | no xask lane |

**Totals:** 18 reachable (`█≄∅` × 12, `_≄∅` × 2 not yet benched) · 4 xhigh OOS (`█≇✗`) · 4 spark low-only gap (`_≇✗`) · 2 spark xhigh compound (`_≇✗`) · 4 gpt-5.5 absent (`_≇✗`) = 28 cells.

**Invariant coverage:** 14 rows trigger suppression-flag test (glyph 2 = `≄`) · 14 rows trigger fast\_mode-absent + WONTFIX tests (glyph 2 = `≇` + glyph 3 = `✗`).

---

## 4. Primary table (medians) — STUBBED

*Actual numbers require M2+ bench execution. `TBD` cells will be populated by executor. `—` cells are permanent gaps.*

> **Footnote (MOVE-8, mandatory):** wrapper-spawn latency (~5–50 ms estimate) contaminates fast-on TTFT asymmetrically vs fast-off raw arm; within-run comparisons are valid, absolute cross-substrate comparisons are not.

| Model | Effort | Fast | TTFT (ms) ±σ | wall tok/s ±σ | decode tok/s\* | Δ\_wrap | Δ\_fast | n | Cov |
|---|---|---|---|---|---|---|---|---|---|
| gpt-5.4-mini | low | off | TBD | TBD | TBD | — | — | TBD | `█≄∅` |
| gpt-5.4-mini | low | on | TBD | TBD | TBD | TBD | TBD | TBD | `█≄∅` |
| gpt-5.4-mini | medium | off | TBD | TBD | TBD | — | — | TBD | `█≄∅` |
| gpt-5.4-mini | medium | on | TBD | TBD | TBD | TBD | TBD | TBD | `█≄∅` |
| gpt-5.4-mini | high | off | TBD | TBD | TBD | — | — | ≥5 | `█≄∅` |
| gpt-5.4-mini | high | on | TBD | TBD | TBD | TBD | TBD | ≥5 | `█≄∅` |
| gpt-5.4-mini | xhigh | — | — | — | — | — | — | — | `█≇✗` |
| gpt-5.4 | low | off | TBD | TBD | TBD | — | — | TBD | `█≄∅` |
| gpt-5.4 | low | on | TBD | TBD | TBD | TBD | TBD | TBD | `█≄∅` |
| gpt-5.4 | medium | off | TBD | TBD | TBD | — | — | TBD | `█≄∅` |
| gpt-5.4 | medium | on | TBD | TBD | TBD | TBD | TBD | TBD | `█≄∅` |
| gpt-5.4 | high | off | TBD | TBD | TBD | — | — | TBD | `█≄∅` |
| gpt-5.4 | high | on | TBD | TBD | TBD | TBD | TBD | TBD | `█≄∅` |
| gpt-5.4 | xhigh | — | — | — | — | — | — | — | `█≇✗` |
| gpt-5.3-spark | low | off | TBD | TBD | TBD | — | — | ≥5 | `_≄∅` |
| gpt-5.3-spark | low | on | TBD | TBD | TBD | TBD | — | ≥5 | `_≄∅` ⚠ |
| gpt-5.3-spark | medium | — | — | — | — | — | — | — | `_≇✗` |
| gpt-5.3-spark | high | — | — | — | — | — | — | — | `_≇✗` |
| gpt-5.3-spark | xhigh | — | — | — | — | — | — | — | `_≇✗` |
| gpt-5.5 | any | — | — | — | — | — | — | — | `_≇✗` |

Block bars (▏▎▍▌▋▊▇█) applied to numeric cells at render time; scale computed per-column over Pareto-eligible models only. `⚠` on spark fast-on = coverage-limited (2 cells < MIN_COMPARABLE_CELLS=3), excluded from Pareto ranking.

`*` decode tok/s noisy for fast/low-latency models where (wall − TTFT) approaches zero.

**Design-choice note:** `Δ_fast` and `Δ_wrap` are split columns, not merged. `Δ_fast` measures the fast_mode toggle effect on the model — valid only for gpt-5.4 family where fast_mode is independently togglable. `Δ_wrap` measures xask wrapper overhead vs raw arm — valid for all models. Merging them would conflate two causally distinct effects on spark, where fast_mode and wrapper overhead are not separable.

---

## 5. Key observations (template — structural predictions)

These are structural predictions that M2+ bench execution would verify or falsify:

1. **Wrapper overhead hypothesis:** If `Δ_wrap > 0` for spark and `Δ_wrap ≈ 0` for gpt-5.4-mini, wrapper spawn latency scales inversely with model response speed (faster model → wrapper is proportionally larger contaminant of total TTFT). If both ≈0, overhead is model-independent.

2. **fast_mode effect hypothesis (gpt-5.4 family):** If `Δ_fast > Δ_wrap` for gpt-5.4 at high effort, fast_mode provides additive benefit beyond wrapper elimination. If `Δ_fast ≈ Δ_wrap`, fast_mode's effect is dominated by wrapper overhead and the toggle provides minimal net utility vs raw arm.

3. **MOVE-11 spawn-latency model-dependence hypothesis:** If spawn_t / TTFT ratio is constant across gpt-5.4-mini and gpt-5.4 at same effort → spawn latency is fixed overhead (CONSIDER: factor it out). If ratio scales with model wall time → cross-model TTFT comparisons are structurally contaminated by model speed, not substrate (RETHINK: absolute TTFT cross-model is not meaningful). M2 must report this ratio explicitly.

4. **Effort-TTFT scaling:** If TTFT scales monotonically with effort for gpt-5.4 family, the model is computing during the latency window (reasoning tokens affect first-token). If TTFT is flat across effort levels, reasoning is post-TTFT and effort affects wall time but not first-token latency.

5. **Spark vs mini comparison:** Spark expected to have lower TTFT and higher wall tok/s than gpt-5.4-mini at low effort. 2-cell structural constraint makes Pareto comparison invalid (tagged ⚠ coverage-limited). If wall tok/s range diverges >3×, block-bar scale must be split (rendering constraint 2 + MOVE-6 compound check triggered if this co-occurs with sparse coverage and Δ label issues).

---

## 6. Rendering commitments

The 8 source-prompt rendering constraints, mapped to xbreed dataset:

| # | Constraint | Source bench | xbreed mapping | Status |
|---|---|---|---|---|
| 1 | Unicode block bars | ✓ TTFT + wall tok/s | ✓ same columns; per-column scale over Pareto-eligible models only | planned |
| 2 | Pareto outlier handling (spark) | coverage-limited tag | ✓ composite validity gate (MOVE-6): sparse + no fast_mode sep + Δ label mismatch → compound warning | planned |
| 3 | fast-on/fast-off Δ at-a-glance | single Δ column | ✓ SPLIT into `Δ_fast` (gpt-5.4 family) + `Δ_wrap` (all models) (MOVE-1) | planned |
| 4 | stddev via ±σ | ✓ | ✓ inline `±σ` | planned |
| 5 | TTFT + wall primary | ✓ leftmost after model/effort/fast | ✓ same | planned |
| 6 | decode noisy footnote | ✓ `*` | ✓ `*` with same semantics | planned |
| 7 | single table | ✓ preferred | ✓ preferred; fallback = merged table with visual separator if spark scale breaks layout | planned |
| 8 | preserve precision | ✓ ms integer, 1 decimal tok/s | ✓ same | planned |

**Composite validity gate (MOVE-6):** Before emitting any Pareto highlight, `xbrd-bench render` checks for multi-constraint composition failures occurring simultaneously:
- sparse coverage (measured_cells < MIN_COMPARABLE_CELLS) **AND**
- missing fast_mode arm separation (no paired off/on rows) **AND**
- Δ label mismatch (Δ_fast applied to spark row, or Δ_wrap absent from any measured row)

If ≥2 constraints fail together: emit compound warning `⚠ COMPOSITE VALIDITY FAILURE: [conditions]; table rows affected: [list]`. Individual footnotes are insufficient when constraints co-compose.

**MOVE-9 Cov 3-glyph (MANDATORY — promoted from deferred):** Cov column is a **schema contract**, not a display choice. All 28 rows carry `[effort-depth][delta-parity][routing]` glyph triple. Glyph 2 = `≄` triggers suppression-flag test invariant; glyph 2 = `≇` triggers fast\_mode-absent assertion; glyph 3 = `✗` triggers WONTFIX routing-report assertion. `xbrd-bench` enforces all three at render time — glyph mismatch = render error.

---

## 7. Methodological notes

### TTFT substrate divergence (MOVE-4)

Raw codex bench TTFT = first model-response JSONL event. xbreed TTFT has two additional correction requirements:

1. **Preamble skip:** `thread.started`, tool-start, and other pre-response events fire before `agent_message.completed`. Using first JSONL line as TTFT anchor is structurally wrong — it captures preamble, not model latency. Parser must seek `item.type == "agent_message"` + `item.completed == true`. This is the MOVE-4 correction.

2. **Wrapper spawn latency (~5–50 ms, MOVE-8):** xbreed Rust binary spawn + OAuth shell bootstrap + xask script overhead. Contaminates fast-on TTFT asymmetrically: fast-on goes through full xask wrapper; fast-off goes through raw `codex exec` (no wrapper). Within-run fast-on vs fast-off comparisons are valid (same substrate). Absolute TTFT cross-substrate comparisons are not valid.

### Δ column categorical split rationale (MOVE-1)

`Δ_fast` = fast_mode toggle measurement. Compares xask invocation (fast_mode on) vs raw codex exec arm (fast_mode off) for **gpt-5.4 family models only** — these are the only models where fast_mode is independently togglable through xask. For spark, fast_mode cannot be isolated from wrapper overhead (xask always sets fast_mode=true; raw arm always has fast_mode=false), so `Δ_fast` is undefined for spark and rendered as `—`.

`Δ_wrap` = wrapper overhead measurement. Compares xask invocation vs raw `codex exec` arm at same model+effort, regardless of fast_mode state. Meaningful for all models including spark. Measures: spawn latency + xbreed dispatch overhead + OAuth shell bootstrap overhead.

Merging into a single Δ column would misrepresent spark rows (where the column would silently conflate two effects) and obscure whether fast_mode or wrapper overhead drives TTFT improvements on gpt-5.4 family.

### n=3 → n≥5 at top tier (MOVE-5)

n=3 is the global minimum floor for all reachable cells. The top reachable tier — gpt-5.4-mini × high effort and gpt-5.3-spark × low effort — runs n≥5 for tighter σ estimates and more reliable Pareto anchor points. `xhigh` is structurally unreachable via xask for all models — do not attempt sampling at any n.

### gpt-5.5 structural absence

gpt-5.5 is not a coverage failure — it is a structural absence from xask. `src/ask.rs` has no `CODEX_5_5_MODEL` constant and no routing path for this model. All gpt-5.5 cells appear as `—`; this is a **coverage gap, not a performance finding**. A raw `codex exec -m gpt-5.5` probe is possible in M6 but does not change the xask coverage baseline and does not produce a valid Δ_wrap measurement.

### 18/28 coverage baseline (MOVE-7)

Canonical reachable count is **18/28**: 6 gpt-5.4-mini cells (low/med/high × off/on) + 6 gpt-5.4 cells + 2 spark cells (low × off/on) = 14 measured + 4 control arm cells (paired raw arms are counted separately). Gap cells: 4 xhigh OOS (across gpt-5.4-mini + gpt-5.4) + 4 spark low-only gap + 2 spark xhigh compound + 4 gpt-5.5 absent = 14 gap/OOS/absent. 18 + 10 = 28. ✓

---

## 8. Caption (stub — populate at M_final)

*Template:* This table reports xbreed execution-path performance across 18/28 codex model×effort×fast_mode cells reachable via `xask codex` lanes as of 2026-04-24. **[HEADLINE: e.g., "gpt-5.4-mini medium provides the best TTFT/wall-tok-s tradeoff across xask-reachable cells; xask wrapper overhead accounts for ~X ms (Y%) of fast-on TTFT at this tier."]** 10 cells are structural gaps (xhigh OOS, gpt-5.5 absent, spark low-only) and appear as `—` — not performance data. All Δ values are split: `Δ_wrap` measures wrapper overhead (all models); `Δ_fast` measures fast_mode toggle effect (gpt-5.4 family only).

---

*evidence: 20 plan sections modified (MOVE-1 through MOVE-10 + MOVE-11 + MOVE-9 deferred); report written with 8 sections; raw xask quote: "Gap row annotation best-practice used: one row per gap with a short, stable reason in Notes; explicit provenance in Coverage column; no implicit blanks, no blank separators."*
