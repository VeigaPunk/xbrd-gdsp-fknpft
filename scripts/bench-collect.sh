#!/usr/bin/env bash
# Collect metrics across multiple teammate jsonls.
# Usage: bench-collect.sh <jsonl-glob-or-paths...>
# Writes TSV to stdout with header row.
set -euo pipefail

HERE="$(dirname "$(readlink -f "$0")")"

if [ "$#" -eq 0 ]; then
  echo "usage: $0 <jsonl-path...>" >&2
  exit 1
fi

python3 "$HERE/bench-metrics.py" --header
for f in "$@"; do
  python3 "$HERE/bench-metrics.py" "$f"
done
