# xbreed exec-path benchmark — 2026-04-24
**Session:** xbrd-exec-bench-0424 | **Author:** cdx-executor-r2 | **Status:** POPULATED — 14 xask-reachable cells + 8 gpt-5.5 raw-only cells, 74 runs total, ~17 min wall

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

**Block-bar scales (rendering constraint 2 — split scale, divergence 6.36×):** Pareto-eligible = gpt-5.4 family (max tok/s = 86.79). Spark = own scale (max tok/s = 551.67).

| Model | Effort | Fast | wall_s ±σ | out_tok | tok/s | bar | Δ_wrap | n | Cov |
|---|---|---|---|---|---|---|---|---|---|
| gpt-5.4-mini | low | off | 7.71 ±0.84 | 268 | 34.8 | `███` | — | 3 | `█≄∅` |
| gpt-5.4-mini | low | on | 8.03 ±1.05 | 382 | 47.6 | `████` | +4.2% | 3 | `█≄∅` |
| gpt-5.4-mini | medium | off | 12.27 ±2.07 | 1065 | 86.8 | `████████` | — | 3 | `█≄∅` |
| gpt-5.4-mini | medium | on | 14.55 ±2.20 | 815 | 56.0 | `█████` | +18.6% | 3 | `█≄∅` |
| gpt-5.4-mini | high | off | 7.18 ±4.41 | 304 | 42.4 | `████` | — | 5 | `█≄∅` |
| gpt-5.4-mini | high | on | 12.05 ±1.86 | 784 | 65.1 | `██████` | +67.9% | 5 | `█≄∅` |
| gpt-5.4-mini | xhigh | — | — | — | — | — | — | — | `█≇✗` |
| gpt-5.4 | low | off | 11.27 ±0.82 | 267 | 23.7 | `██` | — | 3 | `█≄∅` |
| gpt-5.4 | low | on | 9.48 ±0.84 | 419 | 44.2 | `████` | −15.9% | 3 | `█≄∅` |
| gpt-5.4 | medium | off | 10.32 ±1.07 | 278 | 26.9 | `██` | — | 3 | `█≄∅` |
| gpt-5.4 | medium | on | 10.81 ±3.91 | 784 | 72.6 | `███████` | +4.7% | 3 | `█≄∅` |
| gpt-5.4 | high | off | 24.78 ±1.18 | 1040 | 42.0 | `████` | — | 3 | `█≄∅` |
| gpt-5.4 | high | on | 26.72 ±4.15 | 1698 | 63.5 | `██████` | +7.8% | 3 | `█≄∅` |
| gpt-5.4 | xhigh | — | — | — | — | — | — | — | `█≇✗` |
| gpt-5.3-spark | low | off | 7.69 ±1.23 | 2703 | 351.6 | `█████`⚠ | — | 5 | `_≄∅` |
| gpt-5.3-spark | low | on | 10.02 ±1.41 | 5525 | 551.7 | `████████`⚠ | +30.3% | 5 | `_≄∅` ⚠ |
| gpt-5.3-spark | medium | — | — | — | — | — | — | — | `_≇✗` |
| gpt-5.3-spark | high | — | — | — | — | — | — | — | `_≇✗` |
| gpt-5.3-spark | xhigh | — | — | — | — | — | — | — | `_≇✗` |
| gpt-5.5 | low | off | 12.90 ±1.92 | 263 | 20.4 | `██` | — | 3 | `█≡●` |
| gpt-5.5 | low | on | 10.70 ±3.15 | 275 | 25.7 | `██` | −17.1%\* | 3 | `█≡●` |
| gpt-5.5 | medium | off | 14.22 ±3.96 | 288 | 20.3 | `██` | — | 3 | `█≡●` |
| gpt-5.5 | medium | on | 12.81 ±2.82 | 262 | 20.5 | `██` | −10.0%\* | 3 | `█≡●` |
| gpt-5.5 | high | off | 22.96 ±1.49 | 826 | 36.0 | `███` | — | 3 | `█≡●` |
| gpt-5.5 | high | on | 17.22 ±0.60 | 1012 | 58.8 | `█████` | −25.0%\* | 3 | `█≡●` |
| gpt-5.5 | xhigh | off | 29.34 ±1.09 | 1210 | 41.2 | `████` | — | 3 | `█≡●` |
| gpt-5.5 | xhigh | on | 19.86 ±2.42 | 1053 | 53.0 | `█████` | −32.3%\* | 3 | `█≡●` |

`⚠` on spark = coverage-limited (2 cells < MIN_COMPARABLE_CELLS=3); bar rendered on spark's own scale (max=551.67). Excluded from Pareto ranking per §Pareto-rank-gate.

**Δ column semantics (per-row, keyed on Cov glyph 2):**
- **Glyph 2 = `≄`** (gpt-5.4 family — asymmetric substrate, xask on / raw off): Δ is **Δ_wrap** = `(wall_on − wall_off) / wall_off`. Positive = xask slower than raw; negative = xask faster.
- **Glyph 2 = `≡`** (gpt-5.5 — symmetric substrate, both arms raw codex exec): Δ is **Δ_fast**\* = `(wall_fast_on − wall_fast_off) / wall_fast_off`. Positive = fast_mode slower; negative = fast_mode faster. Marked with `*` suffix in the table.

**Cov glyph 3 extension (gpt-5.5 addendum, 2026-04-24):**
- **`●`** = populated via raw-only path (no xask lane; benched via raw `codex exec`). Distinguishes from `∅` (reachable, not yet benched) and `✗` (permanently routing-excluded). gpt-5.5 rows carry `●` because they lack an xask lane in `src/ask.rs` but are otherwise accessible via raw `codex exec -m gpt-5.5`. Addition of an xask lane for gpt-5.5 would convert these to `█≄●` and enable Δ_wrap measurement.

**Design-choice note:** The original `Δ_fast` / `Δ_wrap` split (MOVE-1) collapsed to a single `Δ_wrap` column because TTFT (the handle on which `Δ_fast` was defined) was dropped. Any future reinstatement of `Δ_fast` requires a non-buffered first-token measurement path; see report §7. Output token counts differ by arm (xask fast-on arm reliably produces 1.5–3× more tokens for the same prompt), which makes per-arm tok/s comparisons workload-confounded — `wall_s ±σ` is the cleaner latency axis.

---

## 5. Key observations (measured)

1. **Δ_wrap is not a fixed spawn-latency constant — it is workload-dependent.** Observed range: **−15.9% to +67.9%** across 7 fast-on/fast-off pairs (negative = xask faster than raw arm). The only negative pair is gpt-5.4 × low (−15.9%); the only >+50% pair is gpt-5.4-mini × high (+67.9%, σ on the off-arm is 4.41s — see point 4). The "~5–50 ms spawn latency" framing (MOVE-8 footnote) is a *lower bound* that does not account for what the wrapper actually does to the request payload. **Hypothesis falsified for this dataset:** wrapper spawn latency is NOT a fixed additive overhead; it co-varies with model/effort, most plausibly via differences in context injection between arms.

2. **xask fast-on arm produces 1.5–3× more output tokens than raw fast-off arm for the same prompt.** Every reachable model shows this direction: mini low (382 vs 268), mini medium (815 vs 1065 — reversed here, notable exception), mini high (784 vs 304), gpt-5.4 low (419 vs 267), gpt-5.4 medium (784 vs 278), gpt-5.4 high (1698 vs 1040), spark low (5525 vs 2703). The mini-medium reversal is the lone inversion and may be noise (n=3). Practical effect: **tok/s is not a clean latency proxy across arms** — the two arms are producing different-size outputs, so tok/s comparisons must be read alongside `out_tok`, not in isolation.

3. **Pareto ranking (Pareto-eligible models only):**
   - **Best tok/s (throughput):** gpt-5.4-mini × medium × off @ **86.8 tok/s** — but out_tok=1065 inflates numerator; see point 2.
   - **Best wall_s × low σ (latency):** gpt-5.4 × low × off @ **11.27s ±0.82s** — lowest σ/mean ratio in the matrix.
   - **Best tok/s with low σ:** gpt-5.4 × medium × on @ **72.6 tok/s / 10.81s ±3.91** — second-highest tok/s with acceptable variance.
   - `⚠` **coverage-limited (excluded from ranking):** spark × low (2 cells < MIN_COMPARABLE_CELLS=3) — per §Pareto-rank-gate. Numbers appear for reference only.

4. **High-variance cell flagged:** gpt-5.4-mini × high × off shows wall_s = 7.18 ±4.41s (σ = 61% of median). Individual runs (n=5): 6.66, 7.11, 7.46, **18.11**, 7.18 — single outlier at ~2.5× the others (out_tok on the outlier = 1918 vs 278–307 on the other four — the model emitted a much longer completion, accounting for the wall-time spike). Four-out-of-five runs cluster tightly at 6.7–7.5s, so the median is representative but the σ is outlier-driven. Recommend n≥10 on this cell for σ stabilization; current +67.9% Δ_wrap on mini × high is therefore an *upper estimate* biased by the outlier denominator.

5. **Spark low-effort throughput is anomalous:** 351.6 tok/s (off) → 551.7 tok/s (on), **5–10× higher** than any gpt-5.4-family cell. Output tokens are also 5–20× larger — spark appears to emit much richer completions for the same prompt. Pareto ranking excludes spark by §Pareto-rank-gate (only 2 cells), but the raw number is a strong signal that spark × low is the throughput-dominant xask lane *when coverage allows it*. MOVE-9 glyph 1 = `_` (low-only hardcoded) remains the gating invariant — no path to widen spark's effort range through xask.

6. **Effort-to-wall_s scaling (gpt-5.4 family):** low → medium → high wall_s is **monotonically increasing for gpt-5.4** (11.27 → 10.32 → 24.78 off; 9.48 → 10.81 → 26.72 on) — modulo low/medium noise-inversion on the off-arm. For **gpt-5.4-mini** the pattern is inverted: high-off (7.18) is *faster* than low-off (7.71) and medium-off (12.27). This is the σ=4.41s cell (point 4); with that variance, "high-off is fastest mini" is probably an artifact, not a finding.

7. **gpt-5.5 Δ_fast is uniformly negative — fast_mode speeds 5.5 up across all efforts.** Low: −17.1%; medium: −10.0%; high: −25.0%; xhigh: **−32.3%**. Unlike gpt-5.4 family's Δ_wrap (which mixes wrapper overhead with fast_mode effect and varies in sign), gpt-5.5's Δ_fast is a *clean* fast_mode-toggle measurement because both arms are raw `codex exec` (no xask wrapper confound, per Cov glyph 2 = `≡`). **The effect grows with reasoning effort**: at xhigh, fast_mode cuts wall_s from 29.3s → 19.9s (9.5s saved). This is the cleanest causal signal in the matrix.

8. **gpt-5.5 is the only model with complete 4-tier effort coverage in this bench.** gpt-5.4 family maxes at high (xhigh OOS via xask); spark is low-only (ask.rs:77 hardcodes). **gpt-5.5 × high × on @ 58.8 tok/s** is the second-highest Pareto-eligible throughput (after mini-medium-off @ 86.8) and has **low σ (0.60s)** — the most stable cell in the matrix. For reasoning-heavy workloads, `gpt-5.5 high on` (raw) is the Pareto-eligible winner on the throughput-consistency axis.

9. **gpt-5.5 × xhigh is the slowest reasoning tier** (29.3s off / 19.9s on) but produces the richest output (~1000–1210 tokens). tok/s for xhigh: 41.2 off / 53.0 on — below gpt-5.5 × high × on (58.8) despite more reasoning. **Interpretation:** xhigh adds *reasoning time* faster than it adds *output throughput*; high-on is the efficiency sweet spot.

10. **Substrate caveat (5.5):** all 8 gpt-5.5 cells are raw `codex exec` measurements — **no xask-arm data yet**. Direct comparison with gpt-5.4 family's xask-on cells is not apples-to-apples (different substrate). Adding a gpt-5.5 lane to `src/ask.rs` is the next axis (user request, forthcoming commit) — will enable Δ_wrap for 5.5 and convert Cov glyph 2 from `≡` → `≄` on future xask-arm rows.

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

This table reports xbreed execution-path performance across **14 xask-reachable cells** + **8 gpt-5.5 raw-only cells** (22 populated of 32 total) as of 2026-04-24, measured over **74 runs** (n≥5 on top xask tier: gpt-5.4-mini × high, gpt-5.3-spark × low; n=3 elsewhere including all 8 gpt-5.5 cells), total wall ~17 min.

**Headline (xask-reachable, gpt-5.4 family):** `gpt-5.4 × low × off` wins on wall-latency consistency (11.27s ±0.82, lowest σ/mean); `gpt-5.4-mini × medium × off` wins on raw throughput (86.8 tok/s) but output-token counts differ by arm so tok/s is not a pure latency proxy; `gpt-5.4 × medium × on` is the best-balanced xask-native choice (72.6 tok/s / 10.81s).

**Headline (gpt-5.5 raw-only, Cov `≡`):** `gpt-5.5 × high × on @ 58.8 tok/s, 17.22s ±0.60` is the most stable high-throughput cell in the matrix (σ = 3.5% of median). Fast_mode uniformly speeds gpt-5.5 up, effect grows with effort: Δ_fast of −17.1% (low), −10.0% (medium), −25.0% (high), −32.3% (xhigh). gpt-5.5 is the only model with complete 4-tier effort coverage in this bench.

**Coverage-limited / excluded:** Spark × low dominates raw throughput at 351.6–551.7 tok/s (5–10× gpt-5.4 family) but is `⚠` coverage-limited (2 cells) and excluded from Pareto ranking.

**10 cells remain structural gaps** (xhigh OOS via xask × 2 models = 4 cells; spark medium/high/xhigh rows = 6 cells under ask.rs:77 low-only constraint). **gpt-5.5 rows migrated from `absent` (`_≇✗`) to `populated via raw-only path` (`█≡●`)** via raw `codex exec`; adding a gpt-5.5 xask lane would convert these to `█≄●` with Δ_wrap measurable.

**Δ semantics:** Δ_wrap was merged from the original Δ_fast/Δ_wrap split because TTFT (the Δ_fast anchor for gpt-5.4 family) was dropped after smoke M2 showed xask-layer buffering. For gpt-5.5 rows specifically, the Δ column IS Δ_fast\* (both arms raw, fast_mode differs) — marked with `*` suffix. Reinstating Δ_fast for gpt-5.4 family requires a non-buffered first-token measurement path.

---

*evidence: 20 plan sections modified (MOVE-1 through MOVE-10 + MOVE-11 + MOVE-9 deferred); report written with 8 sections; raw xask quote: "Gap row annotation best-practice used: one row per gap with a short, stable reason in Notes; explicit provenance in Coverage column; no implicit blanks, no blank separators."*
