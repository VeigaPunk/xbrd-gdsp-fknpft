#!/usr/bin/env bash
# M5 (codex #5 variant) — End-to-end transport for gemini effort routing.
#
# Assert:
#   (1) scripts/xask -e xhigh gemini "probe" invokes `xbreed ask` with
#       `ask gemini --with godspeed --effort xhigh` in argv (flag
#       transport);
#   (2) the last argv (the constructed PROMPT) carries
#       `# Effort: xhigh` and `# ThinkingBudget: 16384` on their own
#       lines (template substitution);
#   (3) no literal `{{EFFORT}}` or `{{THINKING_BUDGET}}` leaked through
#       (no un-substituted placeholder).
#
# Uses a fake `xbreed` stub on PATH to capture argv, so no network call
# and no real CLI state change happens.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
STAGE="$(mktemp -d /tmp/xask-m5.XXXXXX)"
cleanup() { rm -rf "$STAGE"; }
trap cleanup EXIT INT TERM HUP

FAKE_BIN="$STAGE/bin"
mkdir -p "$FAKE_BIN"
LOG="$STAGE/xbreed.argv"

# The stub writes each argv element NUL-separated (so spaces in args stay
# intact) and exits 0 immediately so xask's trailing invocation succeeds.
cat > "$FAKE_BIN/xbreed" <<EOF
#!/bin/sh
: > "$LOG"
for a in "\$@"; do
  printf '%s\0' "\$a" >> "$LOG"
done
exit 0
EOF
chmod 755 "$FAKE_BIN/xbreed"

# Run xask with the stub first on PATH so `xbreed` resolves to it.
PATH="$FAKE_BIN:$PATH" "$REPO_ROOT/scripts/xask" -e xhigh gemini "probe transport"

if [[ ! -s "$LOG" ]]; then
  echo "FAIL: xbreed stub was not invoked (or captured empty argv)" >&2
  exit 1
fi

# Read the NUL-separated argv into an array (portable bash 4+ approach).
mapfile -d '' ARGV < "$LOG"

# Trim any trailing empty element produced by the final separator.
if [[ ${#ARGV[@]} -gt 0 && -z "${ARGV[${#ARGV[@]}-1]}" ]]; then
  unset 'ARGV[${#ARGV[@]}-1]'
fi

# ------ (1) argv flag transport ----------------------------------------
# Expect argv to start with: ask gemini --with godspeed --effort xhigh <prompt>
[[ "${ARGV[0]:-}" == "ask"       ]] || { echo "FAIL: argv[0] != 'ask': '${ARGV[0]:-}'"       >&2; exit 1; }
[[ "${ARGV[1]:-}" == "gemini"    ]] || { echo "FAIL: argv[1] != 'gemini': '${ARGV[1]:-}'"    >&2; exit 1; }
[[ "${ARGV[2]:-}" == "--with"    ]] || { echo "FAIL: argv[2] != '--with': '${ARGV[2]:-}'"    >&2; exit 1; }
[[ "${ARGV[3]:-}" == "godspeed"  ]] || { echo "FAIL: argv[3] != 'godspeed': '${ARGV[3]:-}'"  >&2; exit 1; }
[[ "${ARGV[4]:-}" == "--effort"  ]] || { echo "FAIL: argv[4] != '--effort': '${ARGV[4]:-}'"  >&2; exit 1; }
[[ "${ARGV[5]:-}" == "xhigh"     ]] || { echo "FAIL: argv[5] != 'xhigh': '${ARGV[5]:-}'"     >&2; exit 1; }
echo "STEP 1 OK: argv flag transport [ask gemini --with godspeed --effort xhigh]"

# ------ (2) prompt template substitution -------------------------------
PROMPT="${ARGV[${#ARGV[@]}-1]}"
if ! printf '%s' "$PROMPT" | grep -qE '^# Effort: xhigh$'; then
  echo "FAIL: prompt missing '# Effort: xhigh' line" >&2
  printf '%s\n' "$PROMPT" | head -20 >&2
  exit 1
fi
if ! printf '%s' "$PROMPT" | grep -qE '^# ThinkingBudget: 16384$'; then
  echo "FAIL: prompt missing '# ThinkingBudget: 16384' line" >&2
  printf '%s\n' "$PROMPT" | head -20 >&2
  exit 1
fi
echo "STEP 2 OK: prompt carries # Effort: xhigh and # ThinkingBudget: 16384"

# ------ (3) no un-substituted placeholders left ------------------------
if printf '%s' "$PROMPT" | grep -q '{{EFFORT}}'; then
  echo "FAIL: literal {{EFFORT}} leaked into prompt (substitution broken)" >&2
  exit 1
fi
if printf '%s' "$PROMPT" | grep -q '{{THINKING_BUDGET}}'; then
  echo "FAIL: literal {{THINKING_BUDGET}} leaked into prompt (substitution broken)" >&2
  exit 1
fi
echo "STEP 3 OK: no literal {{EFFORT}} / {{THINKING_BUDGET}} placeholders leaked"

echo "PASS: gemini effort transport argv + prompt + substitution"
