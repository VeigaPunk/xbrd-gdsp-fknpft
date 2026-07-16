#!/usr/bin/env bash
set -euo pipefail

TMP_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMP_ROOT"' EXIT

ITERATIONS="${ITERATIONS:-20}"
PAYLOAD="${PAYLOAD:-bench}"
COUNTS=(10 100 1000)

if xbreed mailbox-send --help >/dev/null 2>&1; then
  MAILBOX_CMD=(xbreed mailbox-send)
  MAILBOX_ARGS=(--from=bench --kind=bench --payload)
else
  MAILBOX_CMD=(xbreed team mailbox write)
  MAILBOX_ARGS=(--from bench --kind bench --payload)
fi

run_once() {
  local count=$1
  local dir=$2
  local time_file=$3

  /usr/bin/time -f '%e' -o "$time_file" \
    bash -c "cd '$dir'; for _ in \$(seq 1 $count); do ${MAILBOX_CMD[*]} ${MAILBOX_ARGS[*]} '$PAYLOAD' >/dev/null; done"
}

read_times() {
  local file=$1
  local count
  local p50_idx
  local p95_idx
  local sorted

  count="$(awk 'NF { n++ } END { print n + 0 }' "$file")"
  if [ "$count" -eq 0 ]; then
    echo "0 0"
    return 0
  fi

  p50_idx=$(( (count - 1) * 50 / 100 + 1 ))
  p95_idx=$(( (count - 1) * 95 / 100 + 1 ))

  sorted="$(mktemp)"
  sort -n "$file" > "$sorted"
  p50="$(sed -n "${p50_idx}p" "$sorted")"
  p95="$(sed -n "${p95_idx}p" "$sorted")"
  rm -f "$sorted"

  if [ -z "$p50" ] || [ -z "$p95" ]; then
    echo "0 0"
    return 0
  fi

  printf "%.9f %.9f\n" "$p50" "$p95"
}

for count in "${COUNTS[@]}"; do
  times_file="$TMP_ROOT/times-$count.txt"
  : >"$times_file"
  for _ in $(seq 1 "$ITERATIONS"); do
    run_dir="$TMP_ROOT/run-$count-$RANDOM"
    mkdir -p "$run_dir"
    time_file="$TMP_ROOT/elapsed-$count-$RANDOM"
  run_once "$count" "$run_dir" "$time_file"
  cat "$time_file" >> "$times_file"
  rm -f "$time_file"
  done

  read p50 p95 < <(read_times "$times_file")
  echo "n=${count}: p50=${p50}s p95=${p95}s"
done
