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

echo "bench-collect is deprecated: benchmark Python helper scripts were removed" >&2
echo "Use xbrd-bench (if available) or reintroduce a JSONL parser before running." >&2
exit 1
