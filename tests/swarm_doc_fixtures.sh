#!/usr/bin/env bash
set -uo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DOC="$REPO_ROOT/docs/swarm-test-flow.md"
fails=0

# SELF-INTEGRITY: guard against silent-drop mutations (mut3 class)
# If a check() call is commented out, count drops below floor and script aborts immediately.
EXPECTED_CHECK_FLOOR=12
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

# 1. Judge tier must be xhigh (not max)
check "opus 4.7 xhigh present" \
  "grep -qE 'opus 4\\.7 · xhigh|opus 4\\.7 xhigh' \"$DOC\""

# 2. No stale 'opus 4.7 max' anywhere
check "no stale 'opus 4.7 max'" \
  "! grep -q 'opus 4\\.7 max' \"$DOC\""

# 1b. Negation-flip guard: positively confirm xhigh co-located with opus 4.7 (independent path)
# Catches: typo in check 1 pattern, negation-flip on check 2, or both simultaneously.
check "xhigh co-located with opus 4.7" \
  "grep -E 'opus 4\\.7' \"$DOC\" | grep -qiE 'xhigh'"

# 3. templates/ refs are now allowed (templates/ restored 2026-04-17 via f3882aa)

# 4. xask --direct absent OR immediately preceded by a historical/removed annotation
check "--direct absent or annotated" \
  "! grep -qE 'xask --direct' \"$DOC\" || grep -B2 'xask --direct' \"$DOC\" | grep -qiE 'historical|removed|deprecated'"

# 4b. --direct table row must NOT claim exit 0 (cdx-labrat confirmed: exits 1, flag doesn't exist)
check "--direct row does not claim exit 0" \
  "! grep -E 'xask --direct' \"$DOC\" | grep -qE '\\| 0 \\|'"

# 5. Gemini model id or cross-ref to command-flows.md present
check "gemini model id or command-flows.md ref present" \
  "grep -qE 'gemini-[0-9]|command-flows\\.md' \"$DOC\""

# 6. Required sections
check "section: ## Pipeline overview"   "grep -qF '## Pipeline overview'    \"$DOC\""
check "section: ## Round 1"             "grep -qF '## Round 1'              \"$DOC\""
check "section: ## Round 2"             "grep -qF '## Round 2'              \"$DOC\""
check "section: ## Judge Pareto filter" "grep -qF '## Judge Pareto filter'  \"$DOC\""
check "section: ## Dispatch chain"      "grep -qE '## Dispatch chain'       \"$DOC\""

# 7. Swarm table row count consistency
# Count numbered data rows (lines starting "| [digit]") in each swarm subsection,
# then verify the Pareto filter block claims the same leading count (e.g. "4/5" → 4).
check_swarm_rows() {
  local model="$1"      # e.g. "Gemini" — matched against "### Gemini swarm"
  local pareto="$2"     # e.g. "Gemini reliability" — matched in Pareto block

  local row_count
  row_count=$(awk "
    /^### ${model} swarm/{found=1; next}
    found && /^##/{exit}
    found && /^\| [0-9]/{count++}
    END{print count+0}
  " "$DOC")

  local claimed
  claimed=$(grep -m1 "${pareto}" "$DOC" | grep -oE '[0-9]+/[0-9]+' | head -1 | cut -d/ -f1)

  if [[ -z "${claimed:-}" ]]; then
    echo "FAIL: $model swarm — cannot parse claimed count from Pareto block"
    fails=$((fails + 1))
    return
  fi

  if [[ "$row_count" -ne "$claimed" ]]; then
    echo "FAIL: $model swarm row count — table=$row_count, Pareto claims=${claimed}"
    fails=$((fails + 1))
  else
    echo "OK:   $model swarm row count = $row_count matches Pareto claim (${claimed}/N)"
  fi
}

check_swarm_rows "Gemini" "Gemini reliability"
check_swarm_rows "Codex"  "Codex reliability"
check_swarm_rows "Claude" "Claude reliability"

# 8. Claude guard rows must be annotated as dead/removed (dispatch bails before guard — reviewer R1 finding)
# Once executor updates the doc, guard rows should either be removed or carry "dead path" / "removed" annotation.
check "claude guard rows annotated or absent" \
  "! grep -qiE 'guard deny|guard allow' \"$DOC\" || grep -B1 -A1 -iE 'guard deny|guard allow' \"$DOC\" | grep -qiE 'dead|removed|deprecated|non-functional'"

# TODO (out of R2 scope): once executor updates command-flows.md, add:
#   check "command-flows.md claude branch matches dispatch chain" "..."

echo ""
if [[ $fails -gt 0 ]]; then
  echo "FAIL: $fails check(s) failed"
  exit 1
fi
echo "PASS: all checks"
