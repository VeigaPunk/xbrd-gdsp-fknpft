# Plan — xbreed exec-path benchmark (xask-reachable cells only: gpt-5.4-mini · gpt-5.4 · gpt-5.3-spark @ xask lanes)
**Session:** xbrd-exec-bench-0424 | **Dispatched by:** the-judge | **Date:** 2026-04-24
**Spec:** team-lead dispatch brief (inline) | **Author:** ccs-planner-r0, wwkd posture

> **Scope boundary (read first):** This benchmark covers only the codex model/effort cells reachable via `xask codex` lanes as implemented in `src/ask.rs` as of 2026-04-24. `gpt-5.5` is structurally absent from xask (no lane, no constant) and appears as `—` throughout — this is a **coverage gap, not a performance finding**. `gpt-5.3-codex-spark` is measurable at low effort only (hardcoded in ask.rs:77). Do not interpret missing cells as model capability data.

---

## Data Walk

### What I looked at

| File | Key finding |
|---|---|
| `scripts/xask` lines 41–81 | MODEL whitelist: only `gemini` or `codex`. No `gpt-5.5` lane. |
| `scripts/xask` lines 198–227 | `XBREED_BENCH_LOG` captures: `t_complete_iso`, `cli`, `wall_s`, `exit_code`, `effort`, `teammate`. TTFT and `output_tokens` absent. |
| `scripts/xask` lines 63–67 | `JSON_FLAGS=(--json)` propagates via `xbreed ask codex --json` — codex exec JSONL stream is accessible on stdout. |
| `src/ask.rs` lines 75–88 | `fast_mode` applied unconditionally for default + review + full lanes. No `--fast off` toggle exists in xask. |
| `src/ask.rs` lines 75–77 | Spark lane: always `model_reasoning_effort=low`. No multi-effort spark path. |
| `src/ask.rs` lines 122–137 | Three codex model constants: `gpt-5.3-codex-spark`, `gpt-5.4`, `gpt-5.4-mini`. `gpt-5.5` absent entirely. |
| `src/config.rs` lines 117–129 | Default config: `gpt-5.4`, `xhigh`, `fast_mode: true` — but xask default lane overrides to `gpt-5.4-mini`. |

### Spec/reality divergences discovered (beyond team-lead items 1–6)

| # | Divergence | Impact |
|---|---|---|
| D1 | `gpt-5.5` is NOT in xask at all — no constant, no lane, no CLI flag path | Source bench's 8 gpt-5.5 cells (4 effort × 2 fast states) have **zero xask analog**. These cells must be declared OUT-OF-SCOPE for xbreed path or measured via raw `codex exec -m gpt-5.5` bypass. |
| D2 | Spark effort is hardcoded `model_reasoning_effort=low` in ask.rs:77 | Source bench's spark cells at medium/high/xhigh effort are structurally unmappable through xask. Spark is low-only in xask lane. |
| D3 | `--json` flag survives the full xask→xbreed→codex exec chain (xask:51,67, ask.rs:67-68) | TTFT proxy IS extractable from the JSONL stdout stream without src changes. First `type: "assistant"` event timestamp − t_start = TTFT proxy. |
| D4 | `bench-metrics.py` scrapes teammate TRANSCRIPT JSONLs — not applicable to single-call xask (no teammate wrapper; no transcript written) | Separate measurement harness required; Python subprocess wrapper is the minimal delta. |

### xask cell coverage map vs source bench

| Source model | Effort | fast_off | fast_on (xask) | xask cmd |
|---|---|---|---|---|
| gpt-5.5 | low/med/high/xhigh | N/A | **NO COVERAGE** | — |
| gpt-5.4 | low/med/high/xhigh | raw codex exec arm | ✓ via `xask -R -F -e <E> codex` | yes |
| gpt-5.4-mini | low/med/high | raw codex exec arm | ✓ via `xask -e <E> codex` | yes |
| gpt-5.3-codex-spark | **low only** | ✓ raw codex exec (no fast_mode) | ✓ via `xask --spk codex` | low only |

**Covered cells via xask: 18 of 28 source cells** (xhigh OOS across all models; gpt-5.5 absent; spark low-only = 2 cells not 4). fast-off cells require raw `codex exec --disable fast_mode` control arm.

### Pareto-accepted moves (R2 — 9 accepted)

| Move | Scope | Definition |
|---|---|---|
| MOVE-1 | Δ column split | Introduce `Δ_fast` (gpt-5.4 family only — measures fast_mode toggle effect) and `Δ_wrap` (all models including spark — measures xask wrapper overhead vs raw arm); **per-row annotation mandatory** in both table schema and render binary |
| MOVE-2 | Raw control arm flags | Raw `codex exec` control arm MUST carry all 4 suppression flags verbatim from ask.rs:62–65: `approval_policy="never"`, `include_permissions_instructions=false`, `include_apps_instructions=false`, `include_environment_context=false`; omitting any one inflates raw-arm TTFT and potentially inverts Δ sign |
| MOVE-3 | Gap rows mandatory | Every unreachable cell MUST render a row in output; cell value = `—` (em dash); `Cov` column carries gap annotation; no silent blanks |
| MOVE-4 | TTFT anchor corrected | TTFT = wall-clock to first JSONL event where `item.type == "agent_message"` AND `item.completed == true`; NOT first JSONL line (thread.started / tool-start events are pre-response preamble and must be skipped in Part A parser) |
| MOVE-5 | n≥5 at top reachable tier | Minimum n=5 runs at: gpt-5.4-mini × high effort; gpt-5.3-spark × low effort; `xhigh` declared out-of-scope — unreachable via xask for ALL models, do not attempt sampling |
| MOVE-6 | Composite validity gate | Rendering logic must detect multi-constraint composition failures (sparse coverage + no fast_mode arm separation + Δ label mismatch occurring together) and emit a **compound warning** (not just individual cell footnotes) before the table is printed |
| MOVE-7 | 18/28 baseline + xhigh OOS | Canonical coverage baseline = 18/28 reachable cells; raw invocation carries `--disable fast_mode` + `-c model_reasoning_effort=<level>`; `xhigh` is **structurally unreachable** via xask for all models — declare as OOS row, never attempt |
| MOVE-8 | Wrapper-spawn latency footnote | Mandatory table footnote: *"wrapper-spawn latency (~5–50 ms estimate) contaminates fast-on TTFT asymmetrically vs fast-off raw arm; within-run comparisons are valid, absolute cross-substrate comparisons are not"* |
| MOVE-10 | Typographical cavitation | Fixed-width `—` (em dash U+2014) for all N/A cells; monospace alignment enforced; no blank cells, no ASCII `-` in metric columns |

### MOVE-11 — M2 sub-probe hypothesis (added R2)

**Hypothesis:** TTFT spawn-latency is **model-dependent** (proportional to model wall time, not a fixed ~5–50 ms constant).

- If TTFT spawn-latency is constant across gpt-5.4-mini and gpt-5.4: resolves to `CONSIDER` — overhead is additive, can be factored out.
- If TTFT spawn-latency scales proportionally with model wall time: resolves to `RETHINK` — cross-model TTFT comparisons are fundamentally contaminated by model speed, not substrate.

**Gate:** M2 must record raw spawn time (before first xask call) vs TTFT for both models at same effort; report the ratio. This is a first-class M2 deliverable.

### MOVE-9 — Cov 3-glyph schema contract (promoted DEFER → ACCEPT, R2 addendum)

**Classification:** Schema contract with machine-verifiable test invariants — NOT a rendering nicety.

**Canonical glyph spec:**

| Axis | Glyph | Meaning |
|---|---|---|
| Glyph 1 (effort depth) | `█` | low/med/high all reachable via xask for this model |
| | `▄` | low/med reachable |
| | `_` | low only (hardcoded or structurally constrained) |
| Glyph 2 (delta parity) | `≡` | symmetric env — both arms run identical context |
| | `≄` | asymmetric (wrapper vs raw) — xask arm differs from raw arm in fast_mode or env flags |
| | `≇` | categorically incomparable — fast_mode absent on path; no valid delta computation |
| Glyph 3 (routing status) | `✗` | routing-excluded WONTFIX — structural absence, not a measurement gap |
| | `∅` | measurement gap — cell is reachable but not yet benched (TBD) |

**Three schema-contract invariants (test requirements for `xbrd-bench`):**

1. **`≇` on any row** → test asserts `features.fast_mode=true` is **ABSENT** from subprocess args for that row. Mirrors ask.rs:516's existing test assertion into the bench schema.

2. **`✗` on any row** → test asserts the routing report has a WONTFIX entry for that model/effort combination. Promotion from `✗` to `∅` requires routing report update first — named join point across artifact layers.

3. **`≄` on any row** → test asserts the raw control arm subprocess call carries **all four suppression flags** from ask.rs:62–65 (`approval_policy="never"`, `include_permissions_instructions=false`, `include_apps_instructions=false`, `include_environment_context=false`). This **automates MOVE-2's verification** — the invariant makes the absent-flag bug machine-detectable.

**Cell-level Cov values (canonical mapping, all 28 cells):**

| Cells | Model | Effort range | Cov glyph | Rationale |
|---|---|---|---|---|
| C01–C06 | gpt-5.4-mini | low/med/high | `█≄∅` | full effort range; asymmetric wrapper (fast_mode toggle); not yet benched |
| C07–C08 | gpt-5.4-mini | xhigh | `█≇✗` | model has full range but xhigh is OOS; fast_mode absent on path; routing-excluded |
| C09–C14 | gpt-5.4 | low/med/high | `█≄∅` | same as mini; `-R` flag adds review lane but same invariants |
| C15–C16 | gpt-5.4 | xhigh | `█≇✗` | xhigh OOS; same rationale as C07–C08 |
| C17–C18 | gpt-5.3-spark | low | `_≄∅` | low-only model; asymmetric wrapper applies; not yet benched |
| C19–C24 | gpt-5.3-spark | med/high/xhigh | `_≇✗` | ask.rs:77 hardcodes low; fast_mode structurally absent; routing-excluded |
| C25–C28 | gpt-5.5 | any | `_≇✗` | no xask lane; fast_mode absent from path; routing-excluded |

**Render binary test requirement:** `xbrd-bench` MUST enforce all three invariants at render time. Glyph mismatch = render error, not a warning. A row with `≄` that lacks the suppression flags in its recorded subprocess call fails the render gate.

---

## WWKD

1. **What:** Build a repeatable n=3 per-cell benchmark harness measuring TTFT-proxy, wall tok/s, decode tok/s, and wall_ms_stdev across **xask-reachable codex model/effort cells only** (gpt-5.4-mini, gpt-5.4, gpt-5.3-spark@low), with a raw `codex exec` fast-off control arm, producing a table that mirrors the source report schema and **prominently declares coverage gaps as gaps, not data** — `gpt-5.5` absence = no xask lane, not a benchmark finding.

2. **Why:** User evaluates (a) which codex model/effort to use for xask dispatches and (b) how xask/xbreed wrapper overhead compares to raw `codex exec --json` on the same cells. Without a concrete measurement harness, the model-choice axis and the wrapper-overhead axis are both noise.

3. **Assumptions/Risks:**
   - OAuth rate limits on ChatGPT may throttle concurrent or back-to-back n=3 runs. Risk: serial n=3 runs with no concurrency assumed.
   - TTFT = wall-clock to first JSONL event where `item.type == "agent_message"` AND `item.completed == true` (MOVE-4). `thread.started` and tool-start preamble events must be skipped; they are pre-response overhead, not model latency. Provider-socket approximate; actual first-byte-from-network unobservable from xask outer.
   - `gpt-5.5` is absent from xask entirely (D1 above). Source bench gpt-5.5 columns are WONTFIX via xask; declared in table as `—`.
   - Spark multi-effort cells (D2) are hardcoded low-only in xask. xask spark column = low effort only; other cells declared N/A.
   - `decode tok/s = output_tokens / (wall_s − TTFT_s)` will be **noisy** for fast/low-latency models where (wall−TTFT) approaches zero; must flag these cells.
   - `output_tokens` must be extracted from `codex exec --json` JSONL stream (look for `output_tokens` field in the terminal `type: "completed"` or `type: "message"` event). This is an **invention-risk step** — the exact JSONL schema needs probing in M1.

4. **How (milestone order):**
   - M1: skeleton — one xask invocation, XBREED_BENCH_LOG fires, JSONL stdout captured. Prove the plumbing.
   - M2: overfit — one cell (gpt-5.4-mini, medium, n=3), all 6 metric columns populated, row matches table schema.
   - M3: widen effort — gpt-5.4-mini all available efforts (low/medium/high), still fast-on only.
   - M4: fast-off control arm — raw `codex exec --disable fast_mode` (first-class CLI flag, confirmed stable per ccs-scout-xask-surface) for gpt-5.4-mini medium, n=3. Populate fast-off columns; compute ± delta vs fast-on.
   - M5: add models — gpt-5.4 full and gpt-5.3-codex-spark. Complete the xask-reachable matrix.
   - M6: gpt-5.5 gap resolution — document as WONTFIX (no xask lane), optionally probe via raw `codex exec -m gpt-5.5` if model is accessible.
   - M_final: render full comparison table with 8 rendering constraints applied.

5. **Escalation points:**
   - If `codex exec --json` JSONL schema for `output_tokens` is not as assumed in M1 probe → block M2, surface to judge before writing parser.
   - If gpt-5.5 responds to raw `codex exec -m gpt-5.5` (M6 probe) → judge decides whether to widen scope to include it.
   - If OAuth rate-limit throttles appear during n=3 serial runs → judge decides acceptable retry policy.

---

## Milestones

| # | Title | Gate command | Expected output | Executor |
|---|---|---|---|---|
| M1 | Skeleton: xask invocation + augmented BENCH_LOG | `XBREED_BENCH_LOG=/tmp/xbrd-bench.jsonl xask -e medium codex "say: hello"; cat /tmp/xbrd-bench.jsonl` | JSONL line contains `wall_s`, `ttft_ms`, `output_tokens`, `cli`, `exit_code` — all non-null | executor |
| M2 | Overfit: gpt-5.4-mini medium n≥5, all 6 metrics + MOVE-11 spawn-latency probe | `for i in $(seq 5); do XBREED_BENCH_LOG=/tmp/bench.jsonl xask -e medium codex "Count to 10."; done; cargo run --bin xbrd-bench -- render /tmp/bench.jsonl` | 1 table row: TTFT_ms ±σ (skipping preamble events), wall_tok_s ±σ, decode_tok_s*, wall_ms_stdev, n≥5. All non-null. MOVE-11 sub-probe: spawn_t recorded pre-call; ratio spawn_t/TTFT reported for mini + gpt-5.4 same effort. | executor |
| M3 | Widen effort: gpt-5.4-mini low/medium/high | Same loop × 3 efforts → `xbrd-bench render` | 3 rows, all metrics; fast_off columns = `—` | executor |
| M4 | fast-off control arm: `codex exec --disable fast_mode`, gpt-5.4-mini medium | `for i in 1 2 3; do XBREED_BENCH_LOG=/tmp/bench-raw.jsonl codex exec --disable fast_mode --json -m gpt-5.4-mini "Count to 10." \| xbrd-bench parse >> /tmp/bench-raw.jsonl; done; xbrd-bench render /tmp/bench-raw.jsonl` | fast_off row; ± Δ vs fast_on M2; `--disable fast_mode` is first-class flag (no TOML quoting) | executor |
| M5 | Add models: gpt-5.4 full + gpt-5.3-codex-spark | Same harness × gpt-5.4 (low/med/high) + spark (low only, n≥5 per MOVE-5) | Rows for both models; spark med/high = `— (xask low-only)`; xhigh = `— (OOS, unreachable via xask)` for ALL models | executor |
| M6 | gpt-5.5 gap: probe or WONTFIX | `codex exec --json -m gpt-5.5 "say: hi" 2>&1 \| head -5` | If error/model-not-found: `—` cells declared; if accessible: escalate to judge | labrat |
| M_final | Render full table with 8 constraints | `cargo run --bin xbrd-bench -- render --all scratch/bench-results.jsonl` | Single Unicode table, ±σ glyphs, block bars, Δ rows, ⚠ coverage tags, gap declarations | executor |

---

## Rust augmentation contract (user constraint: prefer Rust over Python)

**No new Python in the hot path.** Legacy `scripts/bench-metrics.py` and `scripts/bench-locate.py` stay as-is (no migration mandate), but all new bench tooling is Rust.

### Part A — Augment `XBREED_BENCH_LOG` in `src/ask.rs`

Extend the existing `XBREED_BENCH_LOG` emission block (xask lines 220–227) with two new fields:

```rust
// In build_codex_ask_with_loadout or the xask dispatch call site:
// 1. Capture Instant::now() at child process spawn
// 2. Read child stdout line-by-line; record elapsed at first line → ttft_ms
// 3. Continue reading until type == "turn.completed"; extract usage.output_tokens
// 4. Append to XBREED_BENCH_LOG:
{"t_complete_iso":"...","cli":"codex","wall_s":1.234,"ttft_ms":456,
 "output_tokens":128,"exit_code":0,"effort":"medium","teammate":"..."}
```

**TTFT implementation (Rust) — MOVE-4 corrected anchor:**
- `let t_spawn = Instant::now();`
- Spawn child with `stdout: Stdio::piped()`
- `BufReader::new(child.stdout).lines()` — iterate JSONL lines; **skip** `thread.started`, tool-start, and any pre-response preamble events
- TTFT anchor = first line where `serde_json::from_str::<Value>(&line)` yields `val["type"] == "agent_message"` AND `val["completed"] == true`: `ttft_ms = t_spawn.elapsed().as_millis()`
- Continue reading until `val["type"] == "turn.completed"`; extract `val["usage"]["output_tokens"].as_u64()`
- **Note:** Using first JSONL line as TTFT proxy is WRONG — thread.started fires before any model response and inflates apparent TTFT for fast models; MOVE-4 corrects this

**`output_tokens` path (confirmed):**
- ✓ Terminal event: `{"type":"turn.completed","usage":{"input_tokens":N,"cached_input_tokens":N,"output_tokens":N}}`
- ✓ Path: `["usage"]["output_tokens"]` — nested, confirmed HIGH confidence, 5-probe swarm unanimous
- ✓ Raw `codex exec --disable fast_mode` schema parity: IDENTICAL

**Files touched:** `src/ask.rs` (BENCH_LOG emission block), possibly `src/lib.rs` if a helper is extracted. No new files required for metric capture.

### Part B — `xbrd-bench` render binary (`src/bin/xbrd-bench.rs`)

Minimal render binary (~150 lines). Reads the JSONL accumulator, groups by `(model, effort, fast_mode)`, computes per-cell stats, emits Unicode table.

```
xbrd-bench render [--all] <results.jsonl>
xbrd-bench parse          # stdin: raw codex --json stream → stdout: bench JSONL line (for raw control arm pipe)
```

- `render`: groups rows, computes mean/stdev per cell, applies 8 rendering constraints, prints table
- `parse`: reads stdin JSONL stream, extracts `ttft_ms` + `output_tokens` via same logic as Part A — used in M4 fast-off pipe. **The raw arm MUST carry all 4 contamination-suppression flags from ask.rs:62–65 (measurement validity, not hygiene):**
  ```
  codex exec --skip-git-repo-check --color never --ephemeral \
    --sandbox danger-full-access \
    --disable fast_mode --json \
    -c approval_policy=\"never\" \
    -c include_permissions_instructions=false \
    -c include_apps_instructions=false \
    -c include_environment_context=false \
    -m gpt-5.4-mini "Count to 10." | xbrd-bench parse >> results.jsonl
  ```
  Omitting any one of these injects context tokens the xask arm never sees, inflating raw-arm TTFT and potentially **inverting the Δ sign** (finding: g-connector-schema-crosslinks + ccs-scout-xask-surface). The raw arm is a valid control arm only if it is flag-for-flag identical to xask's codex dispatch minus `features.fast_mode=true`.
- JSONL accumulator: `scratch/bench-results.jsonl` (gitignored)

**Rendering (Rust):** Use `std::fmt` with Unicode block chars (`▏▎▍▌▋▊▇█`). No external crate required. Block bar scale computed per-column max across Pareto-eligible models only (respects coverage gate).

---

## Output table schema (final render target)

```
| Model            | Effort | Fast | TTFT (ms) ±σ        | wall tok/s ±σ | decode tok/s* | Δ_wrap      | Δ_fast      | n | Cov |
|------------------|--------|------|---------------------|---------------|---------------|-------------|-------------|---|-----|
| gpt-5.4-mini     | low    | off  | 1234 ±56  ▇▇▇▇░    | 89 ±3         | 112 ±11*      | —           | —           | 5 | ✓   |
| gpt-5.4-mini     | low    | on   | 891 ±34   ▇▇▇▇▇    | 101 ±2        | 128 ±8*       | −343 (−28%) | −343 (−28%) | 5 | ✓   |
| gpt-5.4          | low    | off  | 1456 ±80  ▇▇▇▇░    | 75 ±4         | 98 ±14*       | —           | —           | 3 | ✓   |
| gpt-5.4          | low    | on   | 1020 ±50  ▇▇▇▇▇    | 88 ±3         | 115 ±9*       | −436 (−30%) | −436 (−30%) | 3 | ✓   |
| gpt-5.4-mini     | xhigh  | any  | —                   | —             | —             | —           | —           | — | OOS |
| gpt-5.5          | any    | any  | —                   | —             | —             | —           | —           | — | absent |
| gpt-5.3-spark    | med+   | any  | —                   | —             | —             | —           | —           | — | low-only |
```

**Column semantics (MOVE-1):**
- `Δ_wrap` = (fast-on TTFT − fast-off TTFT) / fast-off TTFT — measures **xask wrapper overhead**; valid for ALL models including spark (fast-off = raw codex exec control arm, same model, fast_mode off)
- `Δ_fast` = (fast-on TTFT − fast-off TTFT) / fast-off TTFT for gpt-5.4 family models **only** — measures fast_mode toggle effect; N/A for spark (fast_mode has no measurable effect on spark via xask)
- Per-row annotation: Δ rows carry `Δ_wrap` and `Δ_fast` labels explicitly; gap rows carry `—` (em dash) in both columns

**Typographical cavitation standard (MOVE-10):** All N/A / unreachable cells use `—` (U+2014 em dash) for monospace alignment. Never use ASCII `-` or blank cell in metric columns.

**Table footnote (MOVE-8, mandatory):** *"wrapper-spawn latency (~5–50 ms estimate) contaminates fast-on TTFT asymmetrically vs fast-off raw arm; within-run comparisons are valid, absolute cross-substrate comparisons are not"*

`*` = decode tok/s marked noisy (small wall−TTFT denominator for fast models)

**8 rendering constraints applied:**
1. ✓ Unicode block bars for TTFT + wall tok/s (primary visual axis)
2. ✓ gpt-5.3-codex-spark outlier handling: separate block-bar scale if tok/s range diverges >3× (composite validity gate: sparse coverage + scale divergence = compound warning per MOVE-6)
3. ✓ fast-on/fast-off ± deltas via **split** Δ_fast (gpt-5.4 family) + Δ_wrap (all models), visible at glance (MOVE-1)
4. ✓ stddev via `±σ` inline
5. ✓ TTFT + wall tok/s primary columns (leftmost after model/effort/fast)
6. ✓ decode tok/s marked as noisy (`*` footnote)
7. ✓ single table preferred (expand to 2 only if spark scale breaks layout)
8. ✓ numeric precision preserved (ms integer for TTFT, 1 decimal for tok/s)

**Composite validity gate (MOVE-6):** Before emitting any Pareto highlight, `xbrd-bench render` checks for multi-constraint composition failures:
- sparse coverage (< MIN_COMPARABLE_CELLS) AND
- missing fast_mode arm separation AND
- Δ label mismatch (Δ_fast applied to spark row, or Δ_wrap missing from any measured row)
If ≥2 constraints fail together, emit compound warning: `⚠ COMPOSITE VALIDITY FAILURE: [conditions]; table rows affected: [list]`

**MOVE-9 Cov column (schema contract — MANDATORY):** The `Cov` column carries a 3-glyph `[effort][delta-parity][routing]` value for every row. Canonical mapping: C01–C06 = `█≄∅`, C07–C08 = `█≇✗`, C09–C14 = `█≄∅`, C15–C16 = `█≇✗`, C17–C18 = `_≄∅`, C19–C28 = `_≇✗`. Any row with glyph 2 = `≄` triggers the suppression-flag test invariant; glyph 2 = `≇` asserts fast_mode absent; glyph 3 = `✗` asserts WONTFIX in routing report. See MOVE-9 schema-contract section for full spec.

### Coverage-row annotation standard (for report)

Gap/unreachable rows in the Markdown matrix SHOULD use:

- `TBD` in all metric cells for unreachable data.
- `GapTag` in `Coverage` column (`OOS`, `No xask lane`, `xask low-only`) to preserve provenance.
- `Notes` row text to explain the structural reason and whether alternate raw-arm probing is possible.
- `Cov` column carries 3-glyph `[effort][delta-parity][routing]` value per MOVE-9 schema contract (see above); `∅` = reachable not yet benched, `✗` = routing-excluded WONTFIX.

Example gap row:

`| gpt-5.5 | med | off | TBD | TBD | TBD | TBD | TBD | OOS: gpt-5.5 no xask lane | 0 |`

---

## Dependencies

```
M1 (plumbing verified) → M2 (overfit) → M3 (widen effort) → M5 (add models)
                       → M4 (fast-off arm)                 → M5 → M6 → M_final
```

M6 is non-blocking for M_final (gap declared as `—` if probe fails).
M4 can run in parallel with M3 once M2 gate passes.

---

## Coverage gap declaration (permanent open items)

| Gap | Reason | Cov glyph | Table treatment |
|---|---|---|---|
| gpt-5.5 all cells | No xask lane (ask.rs has no CODEX_5_5_MODEL constant); xask MODEL whitelist = `gemini\|codex` only | `absent` | `—` in all metric cells |
| gpt-5.3-spark at medium/high/xhigh effort | ask.rs:77 hardcodes `model_reasoning_effort=low` | `low-only` | `—` in all metric cells |
| xhigh effort, ALL models | xhigh structurally unreachable via xask (no `--effort xhigh` lane); declared OOS per MOVE-5/MOVE-7 | `OOS` | `—` in all metric cells; do not attempt sampling |
| TTFT anchor | TTFT = wall-clock to first `item.type == "agent_message"` + `item.completed == true` event (MOVE-4); `thread.started` + tool preamble skipped | — | footnote in table header |
| xask wrapper spawn latency | (~5–50 ms estimate) contaminates fast-on TTFT asymmetrically vs fast-off raw arm; within-run comparisons valid, absolute cross-substrate comparisons not (MOVE-8) | — | mandatory table footnote |

---

## Pareto ranking gate (addendum — connector finding)

**Problem surfaced by g-connector-schema-crosslinks:** gpt-5.3-codex-spark has exactly 1 measured cell via xask (low effort only). A naïve Pareto ranking over the full table would promote spark as speed-dominant on a single data point, poisoning the model-selection recommendation before the table is drawn.

**Required pre-condition for M_final Pareto ranking:**

```
PARETO_RANK_ELIGIBLE[model] = (measured_cells[model] >= MIN_COMPARABLE_CELLS)
```

Where `MIN_COMPARABLE_CELLS = 3` (at minimum: one effort tier × fast-on, fast-off, Δ row).

**Concrete gate added to M_final:** Before emitting any ranked or highlighted row, `xbrd-bench render` must:

1. Count `measured_cells` per model (non-`—` cells only).
2. If `measured_cells < 3`: emit model rows in the table with a `⚠ coverage-limited` tag; **exclude from any Pareto highlight, bold, or "recommended" annotation**.
3. Pareto comparison is only drawn over models that share at least one effort tier with complete fast-on + fast-off measurements (the intersection set).
4. The ranking footnote must state: `"Pareto ranking over {N} comparable cells; models with coverage < MIN_COMPARABLE_CELLS excluded from ranking (see ⚠)."`.

**Effect on gpt-5.3-codex-spark:** With 1 measurable cell via xask (low, fast-on) + 1 via raw codex exec control arm (low, fast-off) = 2 cells. Spark is below threshold → tagged `⚠ coverage-limited`, excluded from Pareto highlight. Its speed numbers appear in the table for reference but carry no ranking weight. **n≥5 required at spark × low per MOVE-5** — the 2-cell structural constraint is separate from the n-per-cell constraint.

**Effect on gpt-5.5:** 0 cells via xask → rows are `—` throughout → not ranked.

**Composite validity gate integration (MOVE-6):** The Pareto gate and composite validity gate are co-dependent. A row that is `⚠ coverage-limited` AND has Δ_fast/Δ_wrap label mismatch AND sparse n triggers the compound warning rather than just the coverage footnote. See Output table schema above for compound warning format.

---

evidence: Phase 0 data-walk (scripts/xask, src/ask.rs, src/config.rs); connector addendum (g-connector-schema-crosslinks); labrat schema probes (cdx-labrat-m1-smoke, 5-probe swarm)
[planner-gate: advisory, risks-open: D1 gpt-5.5 absent, OAuth rate-limit policy unset, Pareto-gate MIN_COMPARABLE_CELLS=3 provisional — judge may adjust threshold]
[resolved: D3 JSONL schema — TTFT=external wall-clock confirmed, output_tokens=turn.completed.usage.output_tokens confirmed, M2 unblocked]
