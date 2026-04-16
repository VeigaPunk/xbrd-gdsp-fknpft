#!/usr/bin/env bash
# Tests that scripts/xask substitutes {{EFFORT}} in dispatch templates via -d (debug) mode.
# Debug mode renders the prompt and exits 0 — no xbreed call needed.
set -euo pipefail

XASK="$(cd "$(dirname "$0")/.." && pwd)/scripts/xask"

fail() { echo "FAIL: $1"; exit 1; }

# codex template
OUT=$("$XASK" -d -e high codex "probe effort template")
echo "$OUT" | grep -q "Effort: high" \
  || fail "codex template did not substitute {{EFFORT}} -> 'high'; got: $OUT"

# gemini template
OUT=$("$XASK" -d -e medium gemini "probe effort template")
echo "$OUT" | grep -q "Effort: medium" \
  || fail "gemini template did not substitute {{EFFORT}} -> 'medium'; got: $OUT"

echo "PASS: {{EFFORT}} substituted in codex + gemini dispatch templates"
