#!/usr/bin/env bash
# M2 — Build/CI tier: cargo check must fail when the SSoT path disappears.
# The `include_str!` in src/protocol.rs:4 forces the SSoT into the build tier;
# this test proves that a commit renaming/moving the file breaks `cargo check`
# BEFORE it can land, not after on-prem drift is discovered.
#
# Trap-restore guards against mid-test interrupts leaving the repo broken.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SSOT="$REPO_ROOT/commands/references/xbreed-shared.md"
BAK="$(mktemp -u /tmp/ssot.bak.XXXXXX.md)"

restore() {
  if [[ -f "$BAK" && ! -f "$SSOT" ]]; then
    mv "$BAK" "$SSOT"
    echo "RESTORED: $SSOT"
  fi
}
trap restore EXIT INT TERM HUP

[[ -f "$SSOT" ]] || { echo "FAIL: SSoT missing before test: $SSOT" >&2; exit 2; }

mv "$SSOT" "$BAK"

# cargo check MUST fail — capture stderr and confirm include_str! diagnostic
set +e
STDERR=$(cargo check 2>&1)
STATUS=$?
set -e

if [[ $STATUS -eq 0 ]]; then
  echo "FAIL: cargo check passed with SSoT removed (include_str! not gating build)" >&2
  exit 1
fi

if ! printf '%s' "$STDERR" | grep -qE "couldn't read.*xbreed-shared"; then
  echo "FAIL: cargo check failed, but not with include_str! diagnostic" >&2
  printf '%s\n' "$STDERR" | tail -20 >&2
  exit 1
fi

echo "PASS: SSoT removal fails cargo check with include_str! diagnostic"
