#!/usr/bin/env bash
# Tests that xask fails loudly on malformed profile settings.json.
# Regression guard for the profile-loop python block (line ~140 in scripts/xask).
set -euo pipefail

XASK="$(cd "$(dirname "$0")/.." && pwd)/scripts/xask"
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

fail() { echo "FAIL: $1" >&2; exit 1; }

# Fake xbreed so no real model call is needed
mkdir -p "$WORK/bin"
cat > "$WORK/bin/xbreed" <<'EOF'
#!/bin/bash
exit 0
EOF
chmod +x "$WORK/bin/xbreed"

# Fake HOME: valid main settings.json + malformed profile settings.json
FAKE_HOME="$WORK/home"
mkdir -p "$FAKE_HOME/.gemini"
echo '{}' > "$FAKE_HOME/.gemini/settings.json"
mkdir -p "$FAKE_HOME/.config/xbreed/gemini-profiles/foo/.gemini"
printf 'NOT VALID JSON{{{' > "$FAKE_HOME/.config/xbreed/gemini-profiles/foo/.gemini/settings.json"

set +e
STDERR_OUT=$(HOME="$FAKE_HOME" PATH="$WORK/bin:$PATH" "$XASK" --rich gemini 'probe' 2>&1 >/dev/null)
EXIT_CODE=$?
set -e

echo "exit_code=$EXIT_CODE"
echo "stderr=$STDERR_OUT"

[ "$EXIT_CODE" -ne 0 ] \
  || fail "expected non-zero exit on malformed profile JSON, got 0 (silent success — BUG)"
echo "$STDERR_OUT" | grep -qi "profile\|settings\|failed\|patch" \
  || fail "expected stderr to contain recognizable error fragment; got: $STDERR_OUT"

echo "PASS: malformed profile settings.json causes loud failure (exit=$EXIT_CODE)"
