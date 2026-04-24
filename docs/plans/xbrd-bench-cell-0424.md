# Plan — bench-cell.sh (xbrd-bench-cell-0424)
**Session:** xbrd-bench-cell-0424 | **Dispatched by:** team-lead | **Date:** 2026-04-24

## Phase 0 — Data Walk

- **jq:** installed at `/usr/bin/jq` v1.7 ✓
- **`date +%s.%N`:** nanosecond precision confirmed ✓
- **`ts` (moreutils):** NOT installed — eliminates that TTFT option
- **`XBREED_BENCH_LOG` schema** (xask lines 220–226):
  `{"t_complete_iso":"...","cli":"codex","wall_s":X.XXX,"exit_code":N,"effort":"...","teammate":"..."}`
  Last line only — `jq -r '.wall_s'` extracts it cleanly.
- **`--json` flag:** codex-only in xask; gemini path ignores it → `out_tok=NA` in M1 smoke test (expected).
- **Flag order rule:** `-e`, `--json` MUST precede model positional in xask invocation.

### TTFT bug in draft

`t_ttft` uses `t_now` (captured *after* `"$@"` returns) as the comparison point in awk,
so it measures wall time not first-token latency — always ≈ `wall_s`. Bug confirmed.

**Fix:** first-line timestamp via subshell pipe + temp file (no moreutils needed):
```bash
t_first_file=$(mktemp)
"$@" 2>/dev/null \
  | (IFS= read -r first_line \
      && date +%s.%N > "$t_first_file" \
      && printf '%s\n' "$first_line" \
      && cat) > "$out"
t_first=$(cat "$t_first_file" 2>/dev/null || echo NA)
t_ttft=$(awk -v s="$t_spawn" -v f="$t_first" \
  'BEGIN{if(f=="NA") print "NA"; else printf "%.3f", f-s}')
```
The subshell reads the first byte of output and immediately stamps the wall clock before
re-emitting. Cost: one extra `date` call per run — negligible.

## WWKD

1. **What:** `scripts/bench-cell.sh` — one-cell TSV emitter (model, effort, fast_state, wall_s, output_tokens, ttft_s, iter_n), n=3, runnable end-to-end.
2. **Why:** Benchmark harness needs a per-cell primitive that other orchestrators can call; without it, multi-cell runs must re-implement timing inline.
3. **Assumptions/Risks:** `out_tok` extraction relies on `turn.completed` JSON line present in `--json` codex output; gemini cells always get NA there. xask `wall_s` is measured from inside xask (after template construction), not from script's `t_spawn` — ~100–200 ms delta exists but is consistent.
4. **How:** M1 skeleton (gemini smoke, one row produced) → M2 TTFT fix + real codex n=3 → M3 fast-off control arm.
5. **Escalation:** None — no ambiguities blocking dispatch.

## Milestones

| # | Title | Gate command | Expected output | Executor |
|---|---|---|---|---|
| M1 | Skeleton — one TSV row via gemini | `bash scripts/bench-cell.sh test-label medium on -- scripts/xask gemini "hi" 2>/dev/null` | One tab-separated line: `test-label\tmedium\ton\t<wall>\tNA\t<ttft>\t1` | executor |
| M2 | TTFT fix + real codex n=3 | `bash scripts/bench-cell.sh gpt-5.4-mini medium on -- scripts/xask --json -e medium codex "say one word" 2>/dev/null \| wc -l` | `3` (three TSV rows, all fields non-NA except possibly fast_state label) | executor |
| M3 | Fast-off control arm | `bash scripts/bench-cell.sh gpt-5.4-mini medium off -- codex --disable-fast-mode exec "say one word" 2>/dev/null \| wc -l` | `3` rows with `fast_state=off` | executor |

## Dependencies

M1 → M2 → M3 (strict sequence; each milestone's gate must pass before next begins)

## Notes

- Executor: `--json` flag before `codex` positional in xask (flag-order rule enforced).
- Executor: `wall_s` read via `jq -r '.wall_s // "NA"'` on bench_log last line (not `tail -1 | jq` on raw output).
- Script goes in `scripts/bench-cell.sh` (executable, `chmod +x`). No Rust changes, no binary mods.
