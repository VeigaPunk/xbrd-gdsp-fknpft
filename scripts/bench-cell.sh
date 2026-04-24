#!/usr/bin/env bash
# bench-cell.sh — run one benchmark cell N times, emit TSV rows
# Usage: bench-cell.sh <model> <effort> <stream> -- <cmd...>
# Env:   BENCH_N (default 3), XBREED_BENCH_TEAMMATE (label for bench log)
# TSV:   model  effort  stream  run_n  wall_s  output_tokens  ttft_s
set -euo pipefail

MODEL="${1:?model required}"; EFFORT="${2:?effort required}"; STREAM="${3:?stream required}"
shift 3
[[ "${1:-}" == "--" ]] && shift

N="${BENCH_N:-3}"

for i in $(seq 1 "$N"); do
  out=$(mktemp)
  bench_log=$(mktemp)
  t_first_file=$(mktemp)
  t_spawn=$(date +%s.%N)

  # TTFT: subshell-pipe captures first-byte timestamp before re-emitting output
  XBREED_BENCH_LOG="$bench_log" \
    "$@" 2>/dev/null \
    | (IFS= read -r first_line \
        && date +%s.%N > "$t_first_file" \
        && printf '%s\n' "$first_line" \
        && cat) > "$out" || true

  t_end=$(date +%s.%N)

  # wall_s: from xask bench log (last JSON line); fallback to external timing (raw-arm)
  wall_s=$(tail -1 "$bench_log" 2>/dev/null \
    | jq -r '.wall_s // "NA"' 2>/dev/null || echo NA)
  if [[ "$wall_s" == "NA" || -z "$wall_s" ]]; then
    wall_s=$(awk -v s="$t_spawn" -v e="$t_end" 'BEGIN{printf "%.3f", e-s}')
  fi

  # output_tokens: from codex --json turn.completed line
  out_tok=$(grep -m1 'turn\.completed' "$out" 2>/dev/null \
    | jq -r '.usage.output_tokens // "NA"' 2>/dev/null || echo NA)

  # ttft_s: first-byte minus spawn time
  t_first=$(cat "$t_first_file" 2>/dev/null || echo NA)
  t_ttft=$(awk -v s="$t_spawn" -v f="$t_first" \
    'BEGIN{if(f=="NA") print "NA"; else printf "%.3f", f-s}')

  printf '%s\t%s\t%s\t%d\t%s\t%s\t%s\n' \
    "$MODEL" "$EFFORT" "$STREAM" "$i" "$wall_s" "$out_tok" "$t_ttft"

  rm -f "$out" "$bench_log" "$t_first_file"
done
