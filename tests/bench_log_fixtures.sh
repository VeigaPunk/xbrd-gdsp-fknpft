#!/usr/bin/env bash
set -uo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOG=$(mktemp /tmp/xbgst-bench-fix-XXXXXX.jsonl)
trap 'rm -f "$LOG"' EXIT
fails=0

# Self-integrity floor: guards against silent check-drop mutations (mut3 class).
# Update this constant whenever check() calls are added or removed.
EXPECTED_CHECK_FLOOR=9
ACTUAL_CHECK_COUNT=$(grep -cE '^\s*check "' "${BASH_SOURCE[0]}")
if [[ "$ACTUAL_CHECK_COUNT" -lt "$EXPECTED_CHECK_FLOOR" ]]; then
  echo "FAIL: fixture integrity — $ACTUAL_CHECK_COUNT check() calls, floor is $EXPECTED_CHECK_FLOOR"
  exit 1
fi

check() {
  local label="$1" expr="$2"
  if eval "$expr" 2>/dev/null; then
    echo "OK:   $label"
  else
    echo "FAIL: $label"
    fails=$((fails + 1))
  fi
}

# ── Positive path: XBREED_BENCH_LOG set, real dispatch (~6-8s) ───────────────
rm -f "$LOG"
XBREED_BENCH_LOG="$LOG" XBREED_BENCH_TEAMMATE="test-fix" \
  "$REPO_ROOT/scripts/xask" --spark codex "say TEST"

check "log file created" \
  "[ -f '$LOG' ]"

check "t_complete_iso is ISO date" \
  "jq -r '.t_complete_iso' '$LOG' | grep -qE '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}'"

check "cli == codex" \
  "jq -e '.cli == \"codex\"' '$LOG'"

check "wall_s > 0" \
  "jq -e '.wall_s > 0' '$LOG'"

check "exit_code == 0" \
  "jq -e '.exit_code == 0' '$LOG'"

check "effort == default" \
  "jq -e '.effort == \"default\"' '$LOG'"

check "teammate == test-fix" \
  "jq -e '.teammate == \"test-fix\"' '$LOG'"

check "exactly 1 line + 6-field keyset (M7 prompt-leak guard)" \
  "jq -se 'length == 1 and (.[0] | keys | sort == [\"cli\",\"effort\",\"exit_code\",\"t_complete_iso\",\"teammate\",\"wall_s\"])' '$LOG'"

# ── Negative path: mock dispatch, XBREED_BENCH_LOG unset (M4 guard) ──────────
# Uses a stub xbreed (exit 0 instantly) so no real dispatch is needed here.
# The guard `if [ -n "${XBREED_BENCH_LOG:-}" ]` prevents emit; removing it
# causes `>> ""` (ambiguous redirect) which bash prints to stderr.
_MOCK=$(mktemp -d)
printf '#!/bin/bash\nexit 0\n' > "$_MOCK/xbreed"
chmod +x "$_MOCK/xbreed"
_STDERR_TMP=$(mktemp)

(
  unset XBREED_BENCH_LOG
  export PATH="$_MOCK:$PATH"
  "$REPO_ROOT/scripts/xask" --spark codex "say X"
) 2>"$_STDERR_TMP" >/dev/null
rm -rf "$_MOCK"

check "no redirect error when XBREED_BENCH_LOG unset (M4 guard)" \
  "[ ! -s '$_STDERR_TMP' ]"
rm -f "$_STDERR_TMP"

echo ""
if [[ $fails -gt 0 ]]; then
  echo "FAIL: $fails check(s) failed"
  exit 1
fi
echo "PASS: all checks"
