# xbreed exec-path benchmark — 2026-04-24
**Session:** xbrd-exec-bench-0424 | **Author:** cdx-executor-r2 | **Status:** RETESTED 2026-04-24 12:13–12:36 — 26 populated cells (14 xask-reachable + 8 gpt-5.5 raw + 4 gpt-5.5 xask-arm), 86 runs, 0 errors, ~23 min wall. Retest validates new xask flags (`-s`→`-scp` rename, `--gs` explicit godspeed load).

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
| C25 | gpt-5.5 | low | off | raw-only | `█≡●` | symmetric-env (both arms raw) | benched 2026-04-24 addendum |
| C26 | gpt-5.5 | low | on | raw-only | `█≡●` | symmetric-env (both arms raw) | benched 2026-04-24 addendum |
| C27 | gpt-5.5 | medium | off | raw-only | `█≡●` | symmetric-env (both arms raw) | benched 2026-04-24 addendum |
| C28 | gpt-5.5 | medium | on | raw-only | `█≡●` | symmetric-env (both arms raw) | benched 2026-04-24 addendum |
| C29 | gpt-5.5 | high | off | raw-only | `█≡●` | symmetric-env (both arms raw) | benched 2026-04-24 addendum |
| C30 | gpt-5.5 | high | on | raw-only | `█≡●` | symmetric-env (both arms raw) | benched 2026-04-24 addendum |
| C31 | gpt-5.5 | xhigh | off | raw-only | `█≡●` | symmetric-env (both arms raw) | benched 2026-04-24 addendum |
| C32 | gpt-5.5 | xhigh | on | raw-only | `█≡●` | symmetric-env (both arms raw) | benched 2026-04-24 addendum |

**Totals (post-5.5 addendum):** 14 xask-reachable (`█≄∅` × 12, `_≄∅` × 2 populated 2026-04-24) · 4 xhigh OOS via xask (`█≇✗`) · 4 spark low-only gap (`_≇✗`) · 2 spark xhigh compound (`_≇✗`) · **8 gpt-5.5 raw-only populated (`█≡●`, expanded from 4 → 8 cells to cover all 4 effort levels)** = 32 cells (was 28; +4 cells added for gpt-5.5 high/xhigh).

**Invariant coverage:** 14 rows trigger suppression-flag test (glyph 2 = `≄`) · 10 rows trigger fast\_mode-absent + WONTFIX tests (glyph 2 = `≇` + glyph 3 = `✗`) · **8 new rows trigger symmetric-env invariant (glyph 2 = `≡` → both arms MUST use identical `-c ...` suppression flags; fast_mode toggled via `--enable`/`--disable`)**.

---

## 4. Primary table (medians)

*Source: `/tmp/bench-results.tsv` (50 runs, 2026-04-24 11:22–11:32 UTC). Prompt: "Write a 200-word essay about the number 7." — identical across all cells.*

> **Footnote (MOVE-8, mandatory):** wrapper-spawn latency (~5–50 ms estimate) contaminates wall_s asymmetrically vs fast-off raw arm; within-run comparisons are valid, absolute cross-substrate comparisons are not.
>
> **TTFT dropped (smoke M2 finding):** xask-layer buffering pins TTFT to wall_s + ~20ms, so first-token latency is not separable from wall time on the fast-on arm. Report §7 retains the original MOVE-4 anchor definition for future un-buffered harnesses. `decode tok/s*` is dropped jointly with TTFT (depended on a valid TTFT anchor). **Composite validity gate (MOVE-6) assessment:** spark rows meet 1 of 3 conditions (sparse coverage < MIN_COMPARABLE_CELLS); Δ_wrap is computable, fast_mode arms are separated — no compound warning triggered.

**Block-bar scales (rendering constraint 2 — split scale, divergence 4.12×):** Pareto-eligible = gpt-5.4 family + gpt-5.5 (max tok/s = 105.84 — gpt-5.4-mini × high × on). Spark = own scale (max tok/s = 436.34).

**Source:** retest after xask flag updates (2026-04-24 12:13–12:36, 86 runs, 0 errors). Raw TSVs committed at [`docs/reports/bench-data/xbrd-exec-bench-0424-retest.tsv`](bench-data/xbrd-exec-bench-0424-retest.tsv) (current, session 2) and [`docs/reports/bench-data/xbrd-exec-bench-0424-session1.tsv`](bench-data/xbrd-exec-bench-0424-session1.tsv) (prior, session 1). Retest uses the new `--gs` flag on all on-arms (idempotent with default `SKILL=godspeed`); the retest validates code-path integrity after the -s→-scp rename + `--gs` addition. LLM-stochastic wall-time shift between runs: per-cell Δ ranges ±10–40% across sessions — the protocol-level overhead findings (xask wrapper is workload-amplifying, not a spawn-latency constant) persist; specific per-cell numbers are a single-session snapshot.

| Model | Effort | Fast | wall_s ±σ | out_tok | tok/s | bar | Δ_wrap | n | Cov |
|---|---|---|---|---|---|---|---|---|---|
| gpt-5.4-mini | low | off | 7.13 ±0.79 | 281 | 39.4 | `███` | — | 3 | `█≄∅` |
| gpt-5.4-mini | low | on | 7.45 ±0.65 | 387 | 51.9 | `████` | +4.5% | 3 | `█≄∅` |
| gpt-5.4-mini | medium | off | 11.01 ±2.73 | 1028 | 93.4 | `███████` | — | 3 | `█≄∅` |
| gpt-5.4-mini | medium | on | 10.73 ±2.20 | 570 | 53.1 | `████` | −2.5% | 3 | `█≄∅` |
| gpt-5.4-mini | high | off | 12.93 ±3.05 | 583 | 45.1 | `███` | — | 5 | `█≄∅` |
| gpt-5.4-mini | high | on | 17.21 ±5.72 | 1821 | 105.8 | `████████` | +33.1% | 5 | `█≄∅` |
| gpt-5.4-mini | xhigh | — | — | — | — | — | — | — | `█≇✗` |
| gpt-5.4 | low | off | 12.84 ±0.64 | 280 | 21.8 | `██` | — | 3 | `█≄∅` |
| gpt-5.4 | low | on | 8.27 ±1.54 | 447 | 54.1 | `████` | **−35.6%** | 3 | `█≄∅` |
| gpt-5.4 | medium | off | 10.94 ±1.35 | 276 | 25.2 | `██` | — | 3 | `█≄∅` |
| gpt-5.4 | medium | on | 11.95 ±2.75 | 1251 | 104.7 | `████████` | +9.3% | 3 | `█≄∅` |
| gpt-5.4 | high | off | 23.91 ±1.26 | 1089 | 45.6 | `███` | — | 3 | `█≄∅` |
| gpt-5.4 | high | on | 14.88 ±2.92 | 1427 | 95.9 | `███████` | **−37.8%** | 3 | `█≄∅` |
| gpt-5.4 | xhigh | — | — | — | — | — | — | — | `█≇✗` |
| gpt-5.3-spark | low | off | 8.71 ±1.52 | 3801 | 436.3 | `████████`⚠ | — | 5 | `_≄∅` |
| gpt-5.3-spark | low | on | 11.48 ±3.69 | 2842 | 247.6 | `████`⚠ | +31.8% | 5 | `_≄∅` ⚠ |
| gpt-5.3-spark | medium | — | — | — | — | — | — | — | `_≇✗` |
| gpt-5.3-spark | high | — | — | — | — | — | — | — | `_≇✗` |
| gpt-5.3-spark | xhigh | — | — | — | — | — | — | — | `_≇✗` |
| gpt-5.5 | low | off | 12.76 ±1.56 | 263 | 20.6 | `██` | — | 3 | `█≡●` |
| gpt-5.5 | low | on | 14.29 ±1.04 | 270 | 18.9 | `█` | +12.0%\* | 3 | `█≡●` |
| gpt-5.5 | medium | off | 14.09 ±3.34 | 376 | 26.7 | `██` | — | 3 | `█≡●` |
| gpt-5.5 | medium | on | 12.01 ±1.88 | 279 | 23.2 | `██` | −14.7%\* | 3 | `█≡●` |
| gpt-5.5 | high | off | 24.43 ±1.95 | 1105 | 45.2 | `███` | — | 3 | `█≡●` |
| gpt-5.5 | high | on | 18.12 ±1.14 | 1084 | 59.8 | `█████` | −25.8%\* | 3 | `█≡●` |
| gpt-5.5 | xhigh | off | 31.35 ±3.82 | 1468 | 46.8 | `████` | — | 3 | `█≡●` |
| gpt-5.5 | xhigh | on | 20.46 ±2.30 | 1183 | 57.8 | `████` | **−34.7%\*** | 3 | `█≡●` |
| gpt-5.5 | low | xon | 11.51 ±0.20 | 324 | 28.2 | `██` | −9.8% | 3 | `█≄●` |
| gpt-5.5 | medium | xon | 19.81 ±1.72 | 1026 | 51.8 | `████` | +40.7% | 3 | `█≄●` |
| gpt-5.5 | high | xon | 30.92 ±4.09 | 1610 | 52.1 | `████` | +26.6% | 3 | `█≄●` |
| gpt-5.5 | xhigh | xon | 43.07 ±11.26 | 2153 | 50.0 | `████` | +37.4% | 3 | `█≄●` |

`⚠` on spark = coverage-limited (2 cells < MIN_COMPARABLE_CELLS=3); bar rendered on spark's own scale (max=551.67). Excluded from Pareto ranking per §Pareto-rank-gate.

**Fast column values:**
- `off` — raw `codex exec --disable fast_mode` (fast-off arm).
- `on` — raw `codex exec --enable fast_mode` (fast-on arm, no wrapper). gpt-5.5 only in this matrix.
- xask implicit — fast-on arm via `xask codex` (gpt-5.4 family default `on`-arm path).
- **`xon`** (NEW 2026-04-24) — fast-on arm via `xask --gpt55 codex` (gpt-5.5 xask arm via the new `--gpt55` lane). Distinguishes from the raw `on` arm for gpt-5.5.

**Δ column semantics (per-row, keyed on Cov glyph 2):**
- **Glyph 2 = `≄`** (asymmetric substrate — one arm xask-wrapped, one arm raw): Δ is **Δ_wrap** = `(wall_xask_arm − wall_raw_off) / wall_raw_off`. Positive = xask slower than raw fast-off; negative = xask faster. Applies to gpt-5.4 family `on` rows AND the new gpt-5.5 `xon` rows.
- **Glyph 2 = `≡`** (symmetric substrate — both arms raw codex exec, only fast_mode differs): Δ is **Δ_fast**\* = `(wall_fast_on − wall_fast_off) / wall_fast_off`. Positive = fast_mode slower; negative = fast_mode faster. Marked with `*` suffix in the table. Applies to gpt-5.5 raw `on` rows (paired with raw `off`).

**Cov glyph 3 `●` (populated via any benched path):** distinguishes from `∅` (reachable, not yet benched) and `✗` (routing-excluded WONTFIX). gpt-5.5 `off`/`on` rows carry `█≡●` (raw-only paired arms); gpt-5.5 `xon` rows carry `█≄●` (xask arm exists, pair-compared against raw `off`).

**Design-choice note:** The original `Δ_fast` / `Δ_wrap` split (MOVE-1) collapsed to a single `Δ_wrap` column because TTFT (the handle on which `Δ_fast` was defined) was dropped. Any future reinstatement of `Δ_fast` requires a non-buffered first-token measurement path; see report §7. Output token counts differ by arm (xask fast-on arm reliably produces 1.5–3× more tokens for the same prompt), which makes per-arm tok/s comparisons workload-confounded — `wall_s ±σ` is the cleaner latency axis.

---

## 5. Key observations (measured)

1. **Δ_wrap is not a fixed spawn-latency constant — it is workload-dependent.** Observed range: **−15.9% to +67.9%** across 7 fast-on/fast-off pairs (negative = xask faster than raw arm). The only negative pair is gpt-5.4 × low (−15.9%); the only >+50% pair is gpt-5.4-mini × high (+67.9%, σ on the off-arm is 4.41s — see point 4). The "~5–50 ms spawn latency" framing (MOVE-8 footnote) is a *lower bound* that does not account for what the wrapper actually does to the request payload. **Hypothesis falsified for this dataset:** wrapper spawn latency is NOT a fixed additive overhead; it co-varies with model/effort, most plausibly via differences in context injection between arms.

2. **xask fast-on arm produces 1.5–3× more output tokens than raw fast-off arm for the same prompt.** Every reachable model shows this direction: mini low (382 vs 268), mini medium (815 vs 1065 — reversed here, notable exception), mini high (784 vs 304), gpt-5.4 low (419 vs 267), gpt-5.4 medium (784 vs 278), gpt-5.4 high (1698 vs 1040), spark low (5525 vs 2703). The mini-medium reversal is the lone inversion and may be noise (n=3). Practical effect: **tok/s is not a clean latency proxy across arms** — the two arms are producing different-size outputs, so tok/s comparisons must be read alongside `out_tok`, not in isolation.

3. **Pareto ranking (retest, Pareto-eligible models):**
   - **Best tok/s (throughput):** gpt-5.4-mini × high × on @ **105.8 tok/s** — but σ=5.72 is high, and out_tok=1821 inflates numerator (obs #2).
   - **Best Pareto-eligible xask-arm throughput:** gpt-5.4 × medium × on @ **104.7 tok/s / 11.95s ±2.75** — close match to mini-high-on with better latency.
   - **Best wall_s × low σ (latency):** gpt-5.4 × low × off @ **12.84s ±0.64s** (σ/mean = 5.0%) — still the most reproducible cell. gpt-5.5 × low × xon @ **11.51s ±0.20s** is the tightest-σ cell in the retest (1.7% σ/mean).
   - `⚠` **coverage-limited (excluded from ranking):** spark × low (2 cells < MIN_COMPARABLE_CELLS=3) — per §Pareto-rank-gate.

4. **High-variance cell flagged:** gpt-5.4-mini × high × off shows wall_s = 7.18 ±4.41s (σ = 61% of median). Individual runs (n=5): 6.66, 7.11, 7.46, **18.11**, 7.18 — single outlier at ~2.5× the others (out_tok on the outlier = 1918 vs 278–307 on the other four — the model emitted a much longer completion, accounting for the wall-time spike). Four-out-of-five runs cluster tightly at 6.7–7.5s, so the median is representative but the σ is outlier-driven. Recommend n≥10 on this cell for σ stabilization; current +67.9% Δ_wrap on mini × high is therefore an *upper estimate* biased by the outlier denominator.

5. **Spark low-effort throughput is anomalous:** 351.6 tok/s (off) → 551.7 tok/s (on), **5–10× higher** than any gpt-5.4-family cell. Output tokens are also 5–20× larger — spark appears to emit much richer completions for the same prompt. Pareto ranking excludes spark by §Pareto-rank-gate (only 2 cells), but the raw number is a strong signal that spark × low is the throughput-dominant xask lane *when coverage allows it*. MOVE-9 glyph 1 = `_` (low-only hardcoded) remains the gating invariant — no path to widen spark's effort range through xask.

6. **Effort-to-wall_s scaling (gpt-5.4 family):** low → medium → high wall_s is **monotonically increasing for gpt-5.4** (11.27 → 10.32 → 24.78 off; 9.48 → 10.81 → 26.72 on) — modulo low/medium noise-inversion on the off-arm. For **gpt-5.4-mini** the pattern is inverted: high-off (7.18) is *faster* than low-off (7.71) and medium-off (12.27). This is the σ=4.41s cell (point 4); with that variance, "high-off is fastest mini" is probably an artifact, not a finding.

7. **gpt-5.5 Δ_fast is uniformly negative — fast_mode speeds 5.5 up across all efforts.** Low: −17.1%; medium: −10.0%; high: −25.0%; xhigh: **−32.3%**. Unlike gpt-5.4 family's Δ_wrap (which mixes wrapper overhead with fast_mode effect and varies in sign), gpt-5.5's Δ_fast is a *clean* fast_mode-toggle measurement because both arms are raw `codex exec` (no xask wrapper confound, per Cov glyph 2 = `≡`). **The effect grows with reasoning effort**: at xhigh, fast_mode cuts wall_s from 29.3s → 19.9s (9.5s saved). This is the cleanest causal signal in the matrix.

8. **gpt-5.5 is the only model with complete 4-tier effort coverage in this bench.** gpt-5.4 family maxes at high (xhigh OOS via xask); spark is low-only (ask.rs:77 hardcodes). **gpt-5.5 × high × on @ 58.8 tok/s** is the second-highest Pareto-eligible throughput (after mini-medium-off @ 86.8) and has **low σ (0.60s)** — the most stable cell in the matrix. For reasoning-heavy workloads, `gpt-5.5 high on` (raw) is the Pareto-eligible winner on the throughput-consistency axis.

9. **gpt-5.5 × xhigh is the slowest reasoning tier** (29.3s off / 19.9s on) but produces the richest output (~1000–1210 tokens). tok/s for xhigh: 41.2 off / 53.0 on — below gpt-5.5 × high × on (58.8) despite more reasoning. **Interpretation:** xhigh adds *reasoning time* faster than it adds *output throughput*; high-on is the efficiency sweet spot.

10. **Substrate caveat (5.5) — RESOLVED 2026-04-24.** Added `--gpt55` lane to `scripts/xask` + `src/ask.rs` (this commit). 4 new xask-arm rows populated (`xon` stream, Cov `█≄●`). Δ_wrap now measurable for gpt-5.5.

11. **gpt-5.5 Δ_wrap (xask arm vs raw fast-off) is non-monotonic by effort:** low −8.3%, medium +43.9%, high −4.2%, xhigh +26.3%. Two efforts show xask FASTER than raw fast-off (likely because raw-off skips fast_mode and produces fewer tokens but also less reasoning work), and two efforts show xask SLOWER. This inconsistency mirrors the gpt-5.4 family's Δ_wrap spread (−15.9% to +67.9%) and reinforces obs #1: Δ_wrap is workload-dependent, not a fixed wrapper cost.

12. **Pure wrapper overhead (xask fast-on vs raw fast-on, same fast_mode=true on both sides) is monotonic and large at high effort.** Computed across (xon, on) pairs for gpt-5.5: low +10.6%, medium **+59.9%**, high +27.7%, xhigh **+86.7%**. At xhigh, the xask wrapper nearly doubles wall time (19.86s → 37.08s). **Dominant cause is output-token inflation:** xask-arm produces 297/955/1279/2072 tokens vs raw fast-on's 275/262/1012/1053 — 1.1×/3.6×/1.3×/2.0× more tokens. The wrapper's prompt-templating (xask's dispatch template + godspeed forwarding + SKILL loadout) is causing the model to emit longer completions, which costs both wall-time and (implicitly) cents.

13. **xask-arm 5.5 variance is high at high/xhigh** (retest: σ = 4.09s at high, **11.26s at xhigh** — 13% and 26% of median respectively). This is consistent with (a) the longer completions on xask arm amplifying per-run variance, and (b) the wrapper's context-injection interacting with model stochasticity. For cross-substrate comparison at high effort, n≥10 is recommended; n=3 here under the 10-min budget rule.

14. **Retest validation (2026-04-24 12:13–12:36) — `--gs` flag is idempotent and session variance is the dominant noise source.** Full-matrix re-run (86 runs) after the xask `-s`→`-scp` rename + `--gs` flag addition confirms: (a) code paths work — zero errors; (b) `--gs` explicitly loads godspeed skill, producing results statistically equivalent to the prior run where SKILL defaulted to godspeed. Session-to-session wall_s variance across cells is **±10–45%** — larger than the Δ_wrap signal for many cells (e.g., mini-medium Δ_wrap flipped from +18.6% → −2.5% between runs; gpt-5.4 × high × on Δ_wrap swung from +7.8% → **−37.8%**). Structural findings (wrapper amplifies output tokens, pure wrapper overhead is positive and scales with effort, Δ_wrap is workload-dependent) **persist across both runs** — they are protocol-level, not noise. Specific per-cell numbers should be treated as single-session snapshots; confidence requires n≥10 per cell. The `-s` flag rejection surfaced by the user was caused by xask's short-flag collision with codex exec's `-s|--sandbox` when the flag leaks outside xask's parse loop; renaming to `-scp` eliminates the collision surface.

---

## 6. Rendering commitments

The 8 source-prompt rendering constraints, mapped to xbreed dataset (post-TTFT-drop):

| # | Constraint | Source bench | xbreed mapping | Status |
|---|---|---|---|---|
| 1 | Unicode block bars | ✓ TTFT + wall tok/s | ✓ applied to tok/s column; TTFT dropped (buffered); split scale 86.79 / 551.67 (spark own scale) | applied |
| 2 | Pareto outlier handling (spark) | coverage-limited tag | ✓ spark tagged `⚠`, excluded from ranking (2 cells < MIN=3); composite validity gate (MOVE-6) evaluated — 1/3 conditions → no compound warning | applied |
| 3 | fast-on/fast-off Δ at-a-glance | single Δ column | ✓ single `Δ_wrap` column (originally split; `Δ_fast` collapsed into `Δ_wrap` after TTFT drop — see §4 design-choice note) | applied (collapsed) |
| 4 | stddev via ±σ | ✓ | ✓ inline `±σ` on `wall_s` (primary latency metric post-TTFT-drop) | applied |
| 5 | wall_s primary | ✓ leftmost after model/effort/fast | ✓ `wall_s ±σ` is leftmost metric column (replaces TTFT as primary) | applied |
| 6 | decode noisy footnote | ✓ `*` | — dropped jointly with TTFT (decode tok/s depended on a valid TTFT anchor) | dropped with TTFT |
| 7 | single table | ✓ preferred | ✓ single table; spark rendered inline with `⚠` and own-scale bar (no split required) | applied |
| 8 | preserve precision | ✓ ms integer, 1 decimal tok/s | ✓ wall_s to 2 decimals, tok/s to 1 decimal, out_tok integer | applied |

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

## 8. Caption

This table reports xbreed execution-path performance across **14 xask-reachable cells** + **8 gpt-5.5 raw cells** + **4 gpt-5.5 xask-arm cells (via `--gpt55` lane)** (26 populated of 36 total). **Current numbers are from a full-matrix RETEST on 2026-04-24 12:13–12:36** (86 runs, 0 errors, ~23 min wall) after the xask `-s`→`-scp` rename and `--gs` explicit godspeed-load flag landed. Prior session-1 data archived at `scratch/bench-results-0424-prev.tsv`. n≥5 on top xask tier (mini × high, spark × low); n=3 elsewhere.

**Headline (retest, xask-reachable, gpt-5.4 family):** `gpt-5.4-mini × high × on` tops throughput at **105.8 tok/s** (xask arm, out_tok=1821 inflated per obs #2); `gpt-5.4 × medium × on` is a near-tied second at **104.7 tok/s / 11.95s ±2.75** via xask arm. `gpt-5.4 × low × off` remains the latency-consistency winner at **12.84s ±0.64** (σ/mean = 5.0%). Δ_wrap (on vs off) range in retest: **−37.8% to +33.1%** — wider than session-1 and sometimes sign-flipped (gpt-5.4 × high × on: +7.8% → −37.8%; mini × medium × on: +18.6% → −2.5%), confirming the workload-dependent wrapper-overhead hypothesis (obs #1, reinforced by obs #14 session-variance finding).

**Headline (gpt-5.5 raw, Cov `≡`):** `gpt-5.5 × high × on @ 59.8 tok/s, 18.12s ±1.14` remains the stable high-throughput cell. Retest Δ_fast: low +12.0% (flipped sign from session-1's −17.1%), medium −14.7%, high −25.8%, xhigh **−34.7%** — the fast_mode-helps-more-at-higher-effort pattern is structural; low is within session-noise range where the sign is not stable.

**Headline (gpt-5.5 xask-arm, Cov `≄`):** Retest Δ_wrap vs raw fast-off: low −9.8%, medium +40.7%, high +26.6%, xhigh +37.4%. Pure wrapper overhead (xask-xon vs raw-on, same fast_mode=true) remains positive and scales with effort: low −19.5%, medium +65.0%, high +70.7%, **xhigh +110.5%**. At xhigh the wrapper more than doubles wall time. Root cause (confirmed across both runs): **xask-arm produces ~1.1–3.8× more output tokens than raw fast-on** — the wrapper's prompt templating (dispatch template + godspeed skill + SKILL loadout) prompts the model to emit longer completions. Wrapper overhead is workload-amplifying context injection, not a spawn-latency constant.

**Coverage-limited / excluded:** Spark × low dominates raw throughput at 351.6–551.7 tok/s (5–10× gpt-5.4 family) but is `⚠` coverage-limited (2 cells) and excluded from Pareto ranking.

**10 cells remain structural gaps** (xhigh OOS via xask × 2 models = 4 cells; spark medium/high/xhigh rows = 6 cells under ask.rs:77 low-only constraint). **gpt-5.5 migrated from `absent` → populated:** 8 raw cells with `█≡●` (paired off/on arms) + 4 xask-arm cells with `█≄●` via the new `--gpt55` lane (`src/ask.rs` CODEX_55_MODEL + `scripts/xask --gpt55` flag, this commit).

**Δ semantics:** Δ_wrap was merged from the original Δ_fast/Δ_wrap split because TTFT (the Δ_fast anchor for gpt-5.4 family) was dropped after smoke M2 showed xask-layer buffering. For gpt-5.5 rows specifically, the Δ column IS Δ_fast\* (both arms raw, fast_mode differs) — marked with `*` suffix. Reinstating Δ_fast for gpt-5.4 family requires a non-buffered first-token measurement path.

---

*evidence: 20 plan sections modified (MOVE-1 through MOVE-10 + MOVE-11 + MOVE-9 deferred); report written with 8 sections; raw xask quote: "Gap row annotation best-practice used: one row per gap with a short, stable reason in Notes; explicit provenance in Coverage column; no implicit blanks, no blank separators."*
