#!/usr/bin/env bash
set -euo pipefail

tmpdir="$(mktemp -d)"
stdout_file="$tmpdir/stdout.txt"
stderr_file="$tmpdir/stderr.txt"

set +e
xask -d -e high gemini "probe" >"$stdout_file" 2>"$stderr_file"
status=$?
set -e

if (( status == 0 )) && grep -q "# ThinkingBudget: 8192" "$stdout_file"; then
  echo "PASS: xask includes # ThinkingBudget: 8192 in stdout with -e high gemini"
  exit 0
else
  echo "FAIL: xask should include # ThinkingBudget: 8192 in stdout when -e high is passed to gemini"
  echo "--- stdout ---"
  cat "$stdout_file"
  echo "--- stderr ---"
  cat "$stderr_file"
  exit 1
fi
