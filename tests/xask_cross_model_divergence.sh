#!/usr/bin/env bash
# M8 — Cross-model divergence sentinel.
#
# Assert, for the SAME query, that gemini and codex produce DIFFERENT
# rendered prompts at the dispatch layer — proving template isolation
# is intact and no raw-quote / template-collapse leak has occurred.
#
# Discriminating assertions (dispatch-level, no real API quota):
#   (1) ARGV[1] differs: "gemini" vs "codex" (routing intact)
#   (2) Final argv element (rendered PROMPT) differs byte-for-byte
#       (templates not collapsed into one blob)
#   (3) Gemini PROMPT contains "# ThinkingBudget:" line; codex PROMPT
#       does NOT (gemini.md template marker absent from codex.md)
#
# If these ever converge, the dispatch templates have merged or the
# raw-quote gate is leaking uniform output across models.
#
# Uses a fake `xbreed` stub on PATH — no network, no real CLI state.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
STAGE="$(mktemp -d /tmp/xask-m8.XXXXXX)"
cleanup() { rm -rf "$STAGE"; }
trap cleanup EXIT INT TERM HUP

FAKE_BIN="$STAGE/bin"
mkdir -p "$FAKE_BIN"
LOG_GEMINI="$STAGE/xbreed-gemini.argv"
LOG_CODEX="$STAGE/xbreed-codex.argv"

# Stub writes each argv element NUL-separated so spaces in args stay intact.
# The XASK_M8_LOGFILE env var is set by each xask invocation via a wrapper.
cat > "$FAKE_BIN/xbreed" <<'STUBEOF'
#!/bin/sh
: > "$XASK_M8_LOGFILE"
for a in "$@"; do
  printf '%s\0' "$a" >> "$XASK_M8_LOGFILE"
done
exit 0
STUBEOF
chmod 755 "$FAKE_BIN/xbreed"

FIXED_QUERY="divergence probe: cross-model sentinel"

# --- gemini run ---
XASK_M8_LOGFILE="$LOG_GEMINI" PATH="$FAKE_BIN:$PATH" \
  "$REPO_ROOT/scripts/xask" -e high gemini "$FIXED_QUERY"

# --- codex run ---
XASK_M8_LOGFILE="$LOG_CODEX" PATH="$FAKE_BIN:$PATH" \
  "$REPO_ROOT/scripts/xask" -e high codex "$FIXED_QUERY"

if [[ ! -s "$LOG_GEMINI" ]]; then
  echo "FAIL: xbreed stub not invoked for gemini (empty log)" >&2; exit 1
fi
if [[ ! -s "$LOG_CODEX" ]]; then
  echo "FAIL: xbreed stub not invoked for codex (empty log)" >&2; exit 1
fi

# Read NUL-separated argv into arrays (bash 4+)
mapfile -d '' ARGV_G < "$LOG_GEMINI"
mapfile -d '' ARGV_C < "$LOG_CODEX"

# Trim trailing empty element from final separator
if [[ ${#ARGV_G[@]} -gt 0 && -z "${ARGV_G[${#ARGV_G[@]}-1]}" ]]; then
  unset 'ARGV_G[${#ARGV_G[@]}-1]'
fi
if [[ ${#ARGV_C[@]} -gt 0 && -z "${ARGV_C[${#ARGV_C[@]}-1]}" ]]; then
  unset 'ARGV_C[${#ARGV_C[@]}-1]'
fi

# ------ (1) routing: ARGV[1] must differ --------------------------------
MODEL_G="${ARGV_G[1]:-}"
MODEL_C="${ARGV_C[1]:-}"
[[ "$MODEL_G" == "gemini" ]] || { echo "FAIL: gemini run argv[1] != 'gemini': '$MODEL_G'" >&2; exit 1; }
[[ "$MODEL_C" == "codex"  ]] || { echo "FAIL: codex run argv[1] != 'codex': '$MODEL_C'"  >&2; exit 1; }
[[ "$MODEL_G" != "$MODEL_C" ]] || { echo "FAIL: both runs dispatched to the same model (routing collapsed)" >&2; exit 1; }
echo "STEP 1 OK: routing differs [gemini vs codex]"

# ------ (2) rendered prompts differ byte-for-byte -----------------------
PROMPT_G="${ARGV_G[${#ARGV_G[@]}-1]}"
PROMPT_C="${ARGV_C[${#ARGV_C[@]}-1]}"
if [[ "$PROMPT_G" == "$PROMPT_C" ]]; then
  echo "FAIL: rendered prompts are identical across gemini and codex (template collapse / raw-quote leak)" >&2
  exit 1
fi
echo "STEP 2 OK: rendered prompts differ (no template collapse)"

# ------ (3) ThinkingBudget present in gemini prompt, absent in codex ----
if ! printf '%s' "$PROMPT_G" | grep -q '# ThinkingBudget:'; then
  echo "FAIL: gemini prompt missing '# ThinkingBudget:' line (gemini template not loaded)" >&2
  printf '%s\n' "$PROMPT_G" | head -15 >&2
  exit 1
fi
if printf '%s' "$PROMPT_C" | grep -q '# ThinkingBudget:'; then
  echo "FAIL: codex prompt contains '# ThinkingBudget:' (templates collapsed or wrong template used)" >&2
  printf '%s\n' "$PROMPT_C" | head -15 >&2
  exit 1
fi
echo "STEP 3 OK: ThinkingBudget in gemini prompt, absent in codex prompt"

echo "PASS: cross-model divergence sentinel — gemini and codex templates are isolated"
