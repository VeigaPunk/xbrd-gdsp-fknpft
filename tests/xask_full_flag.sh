#!/usr/bin/env bash
# Verifies scripts/xask line 215 correctly forwards -R -F to xbreed as --review --full.
# Case A: -R -F → --review + --full in xbreed argv, both before prompt positional.
# Case B: -F alone → --review absent (no-op at Rust level; ask.rs:621 covers model selection).
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
STAGE=$(mktemp -d /tmp/xask-mfull.XXXXXX)
cleanup() { rm -rf "$STAGE"; }
trap cleanup EXIT INT TERM HUP

FAKE_BIN="$STAGE/bin"
mkdir -p "$FAKE_BIN"
LOG="$STAGE/xbreed.argv"

# Fake xbreed captures raw argv as NUL-separated tokens.
cat > "$FAKE_BIN/xbreed" <<'EOF'
#!/bin/sh
: > "$XASK_FAKE_LOG"
for a in "$@"; do
  printf '%s\0' "$a" >> "$XASK_FAKE_LOG"
done
exit 0
EOF
chmod 755 "$FAKE_BIN/xbreed"

fail() { echo "FAIL: $*" >&2; exit 1; }

# ── Case A: -R -F forwards both --review and --full before the prompt arg ──
: > "$LOG"
XASK_FAKE_LOG="$LOG" PATH="$FAKE_BIN:$PATH" "$REPO_ROOT/scripts/xask" -R -F codex "probe"

mapfile -d '' ARGV < "$LOG"
LAST_INDEX=$((${#ARGV[@]} - 1))

REVIEW_IDX=-1
FULL_IDX=-1
for i in "${!ARGV[@]}"; do
  case "${ARGV[$i]}" in
    --review) REVIEW_IDX=$i ;;
    --full)   FULL_IDX=$i ;;
  esac
done

[[ $REVIEW_IDX -ge 0 ]] || fail "-R -F: --review missing from argv"
[[ $FULL_IDX -ge 0 ]]   || fail "-R -F: --full missing from argv (scripts/xask:215 regression)"
(( REVIEW_IDX < LAST_INDEX )) || fail "--review at/after prompt positional"
(( FULL_IDX   < LAST_INDEX )) || fail "--full at/after prompt positional"

echo "PASS: -R -F forwarded as --review --full before prompt (HINGE: fails if scripts/xask:215 drops --full)"

# ── Case B: -F alone does NOT forward --review ─────────────────────────────
# --full may still appear in argv; that is a no-op per src/ask.rs:621.
# What must be absent is --review — -F without -R must not enable review mode.
: > "$LOG"
XASK_FAKE_LOG="$LOG" PATH="$FAKE_BIN:$PATH" "$REPO_ROOT/scripts/xask" -F codex "probe-mini"

mapfile -d '' ARGV < "$LOG"

HAS_REVIEW=false
for arg in "${ARGV[@]}"; do
  [[ "$arg" == "--review" ]] && HAS_REVIEW=true
done

[[ "$HAS_REVIEW" == false ]] || fail "-F-only unexpectedly injected --review (must remain no-op without -R)"

echo "PASS: -F-only leaves --review absent, preserving no-op semantics"
