#!/usr/bin/env bash
set -euo pipefail
# B8 (audit 2026-07-15): PATH self-heal + symlink-honest SCRIPT_DIR smoke test.
# Exercises xask under a deliberately stripped environment. `env -i` drops all
# inherited vars, leaving only a minimal PATH. If the self-heal (prepend
# _XASK_DIR + ~/.local/bin) and SCRIPT_DIR-relative template resolution both work,
# the script runs to completion in debug mode and emits the constructed prompt —
# without ever reaching the network dispatch. Full xbreed dispatch under a stripped
# PATH is covered by live use, not CI (it would require a real codex call).

XASK_BIN="$(command -v xask)"
tmpdir="$(mktemp -d)"
out="$tmpdir/out.txt"; err="$tmpdir/err.txt"

set +e
env -i HOME="$HOME" PATH="/usr/bin:/bin" "$XASK_BIN" -d codex "selfheal smoke" >"$out" 2>"$err"
status=$?
set -e

if (( status == 0 )) && grep -q "XASK DEBUG" "$out" && grep -q "CONSTRUCTED PROMPT" "$out"; then
  echo "PASS: xask resolves template + builds prompt under a stripped env (self-heal fires)"
  exit 0
else
  echo "FAIL: xask did not complete under stripped env (status=$status) — self-heal or SCRIPT_DIR resolution regressed"
  echo "--- stdout ---"; cat "$out"
  echo "--- stderr ---"; cat "$err"
  exit 1
fi
