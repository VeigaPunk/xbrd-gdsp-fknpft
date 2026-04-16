#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SSOT="$REPO_ROOT/commands/references/xbreed-shared.md"

COPY_FILES=(
  "$REPO_ROOT/AGENTS.md"
  "$REPO_ROOT/templates/agents/the-judge.md"
  "$REPO_ROOT/templates/agents/connector.md"
  "$REPO_ROOT/commands/xbgst.md"
  "$REPO_ROOT/templates/skills/xbgst/SKILL.md"
)

# Extract canonical connector routing from SSoT routing table (connector row)
# Use -F (fixed string) to match the literal pipe character at line start
CANONICAL=$(grep -m1 -F '| Cross-axis patterns' "$SSOT" \
  | grep -oE 'xask --effort [a-z]+ gemini' | head -1)

if [[ -z "$CANONICAL" ]]; then
  echo "ERROR: cannot extract canonical connector routing from $SSOT" >&2
  exit 2
fi

echo "Canonical connector routing (SSoT): $CANONICAL"
echo ""

DRIFT=0

for FILE in "${COPY_FILES[@]}"; do
  if [[ ! -f "$FILE" ]]; then
    printf "MISSING: %s\n" "$FILE" >&2
    DRIFT=1
    continue
  fi

  if [[ "$FILE" == *"/connector.md" ]]; then
    # connector.md IS the connector agent — check all xask --effort * gemini delegation lines
    matches=$(grep -nE 'xask --effort [a-z]+ gemini' "$FILE" || true)
  else
    # Other files: only lines mentioning connector in routing context
    matches=$(grep -nE 'xask --effort [a-z]+ gemini' "$FILE" | grep 'connector' || true)
  fi

  while IFS= read -r match; do
    [[ -z "$match" ]] && continue
    lineno="${match%%:*}"
    content="${match#*:}"
    actual=$(printf '%s' "$content" | grep -oE 'xask --effort [a-z]+ gemini' | head -1)
    if [[ -n "$actual" && "$actual" != "$CANONICAL" ]]; then
      printf "DRIFT: %s:%s\n  expected: %s\n  actual:   %s\n" \
        "$FILE" "$lineno" "$CANONICAL" "$actual"
      DRIFT=1
    fi
  done <<< "$matches"
done

if [[ "$DRIFT" -eq 0 ]]; then
  echo "OK: all connector routing consistent with canonical ($CANONICAL)"
  exit 0
else
  exit 1
fi
