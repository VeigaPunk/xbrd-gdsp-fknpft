#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_PATH="$REPO_ROOT/docs/reports/mailbox-bench-baseline.json"
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
  python3 - "$file" <<'PY'
import sys

path = sys.argv[1]
values = [float(line.strip()) for line in open(path) if line.strip()]
values.sort()
if not values:
    print("0 0")
    raise SystemExit(0)
n = len(values)
p50 = values[int((n - 1) * 0.50)]
p95 = values[int((n - 1) * 0.95)]
print(f"{p50:.9f} {p95:.9f}")
PY
}

python3 - "$REPO_ROOT" <<'PY'
import json
from pathlib import Path
import sys
path = Path(sys.argv[1]) / "docs" / "reports" / "mailbox-bench-baseline.json"
path.parent.mkdir(parents=True, exist_ok=True)
if not path.exists():
    path.write_text('{}')
PY

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
  python3 - "$REPORT_PATH" "$count" "$p50" "$p95" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
count = sys.argv[2]
p50 = float(sys.argv[3])
p95 = float(sys.argv[4])
report = json.loads(path.read_text())
report.setdefault("process_wall_ms", {})
report["process_wall_ms"][f"n={count}"] = {
    "p50": p50,
    "p95": p95,
    "unit": "seconds",
    "inclusive": "process_start_inclusive",
}
path.write_text(json.dumps(report, sort_keys=True, indent=2) + "\n")
PY

  echo "n=${count}: p50=${p50}s p95=${p95}s"
done
