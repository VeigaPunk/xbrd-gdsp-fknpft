#!/usr/bin/env bash
set -euo pipefail

tmpdir="$(mktemp -d)"
stdout_file="$tmpdir/stdout.txt"
stderr_file="$tmpdir/stderr.txt"

set +e
XBREED_DISPATCH_DIR="/tmp/xask-nonexistent-$$" xask codex "probe" >"$stdout_file" 2>"$stderr_file"
status=$?
set -e

if (( status == 1 )) && grep -q "dispatch template not found" "$stderr_file" && [[ $(wc -l < "$stderr_file") -eq 3 ]]; then
  echo "PASS: xask exits with status 1 and stderr includes dispatch template error when dispatch dir is missing"
  exit 0
else
  echo "FAIL: xask should exit 1 and print 3 stderr lines containing 'dispatch template not found' when dispatch dir is missing"
  echo "--- stdout ---"
  cat "$stdout_file"
  echo "--- stderr ---"
  cat "$stderr_file"
  exit 1
fi
