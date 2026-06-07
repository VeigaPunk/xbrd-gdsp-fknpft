#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SSOT="$REPO_ROOT/commands/references/xbreed-shared.md"

# Canonical agent definitions live in ~/.claude/agents/
# (user directive 2026-04-17 — repo templates/ dir removed to kill ambiguity).
# The xbgst slash-command skill lives at commands/xbgst.md in-repo.
AGENTS_DIR="${XBREED_AGENTS_DIR:-$HOME/.claude/agents}"

COPY_FILES=(
  "$REPO_ROOT/AGENTS.md"
  "$AGENTS_DIR/the-judge.md"
  "$AGENTS_DIR/connector.md"
  "$REPO_ROOT/commands/xbgst.md"
)

# Extract canonical connector routing from SSoT routing table (connector row)
# Use -F (fixed string) to match the literal pipe character at line start.
# Model-agnostic regex (gemini→codex swap 68695ff proved anchoring on a model
# name silently breaks extraction); `|| true` keeps set -euo pipefail from
# killing the script before the empty-check diagnostic below can fire.
CANONICAL=$(grep -m1 -F '| Cross-axis patterns' "$SSOT" \
  | grep -oE 'xask --effort [a-z]+ [a-z]+' | head -1 || true)

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

  # Regex intentionally covers ANY `xask --effort <tier> <model>` — including
  # model drift (gemini → codex), not just tier drift. Anchoring the match on
  # "gemini" would silently miss the model-swap class of regression (M4).
  if [[ "$FILE" == *"/connector.md" ]]; then
    # connector.md IS the connector agent — check every delegation line.
    matches=$(grep -nE 'xask --effort [a-z]+ [a-z]+' "$FILE" || true)
  else
    # Other files: only lines mentioning connector in routing context.
    matches=$(grep -nE 'xask --effort [a-z]+ [a-z]+' "$FILE" | grep 'connector' || true)
  fi

  while IFS= read -r match; do
    [[ -z "$match" ]] && continue
    lineno="${match%%:*}"
    content="${match#*:}"
    actual=$(printf '%s' "$content" | grep -oE 'xask --effort [a-z]+ [a-z]+' | head -1)
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
