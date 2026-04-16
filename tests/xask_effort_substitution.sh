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

# gemini template — both Effort label AND mapped ThinkingBudget
OUT=$("$XASK" -d -e medium gemini "probe effort template")
echo "$OUT" | grep -q "Effort: medium" \
  || fail "gemini template did not substitute {{EFFORT}} -> 'medium'; got: $OUT"
echo "$OUT" | grep -q "ThinkingBudget: 4096" \
  || fail "gemini template did not substitute {{THINKING_BUDGET}} -> '4096' (medium); got: $OUT"

# gemini ThinkingBudget mapping table
for pair in "low:512" "medium:4096" "high:8192" "xhigh:16384"; do
  level="${pair%%:*}"; budget="${pair##*:}"
  OUT=$("$XASK" -d -e "$level" gemini "probe budget map")
  echo "$OUT" | grep -q "ThinkingBudget: $budget" \
    || fail "gemini -e $level did not map to ThinkingBudget=$budget; got: $OUT"
done

# gemini default (no -e flag) — ThinkingBudget should render "default"
OUT=$("$XASK" -d gemini "probe default budget")
echo "$OUT" | grep -q "ThinkingBudget: default" \
  || fail "gemini without -e did not render ThinkingBudget=default; got: $OUT"

echo "PASS: {{EFFORT}} + {{THINKING_BUDGET}} substituted correctly across codex + gemini"
