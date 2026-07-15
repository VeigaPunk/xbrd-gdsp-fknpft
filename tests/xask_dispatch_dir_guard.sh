#!/usr/bin/env bash
set -euo pipefail
# B6 (audit 2026-07-15): XBREED_DISPATCH_DIR confused-deputy guard.
# An EXISTING dispatch dir outside the trusted tree ($HOME / install tree) must be
# refused before its templates can be fed to `codex exec --sandbox danger-full-access`.
# We build a fully-valid dispatch dir under /tmp (contains codex.md) so the ONLY
# reason to fail is the guard — not a missing template.

XASK_BIN="$(command -v xask)"
tmpdir="$(TMPDIR=/tmp mktemp -d)"   # under /tmp — outside $HOME and the install tree
out="$tmpdir/out.txt"; err="$tmpdir/err.txt"
mkdir -p "$tmpdir/dispatch"
printf 'malicious {{QUERY}}\n' > "$tmpdir/dispatch/codex.md"

set +e
XBREED_DISPATCH_DIR="$tmpdir/dispatch" "$XASK_BIN" -d codex "probe" >"$out" 2>"$err"
status=$?
set -e

if (( status == 1 )) && grep -q "refusing XBREED_DISPATCH_DIR outside trusted tree" "$err"; then
  echo "PASS: xask refuses an existing out-of-tree XBREED_DISPATCH_DIR"
  exit 0
else
  echo "FAIL: guard did not reject out-of-tree dispatch dir (status=$status)"
  echo "--- stdout ---"; cat "$out"
  echo "--- stderr ---"; cat "$err"
  exit 1
fi
