#!/usr/bin/env bash
# Install always-on Godspeed standing instructions into harness roots.
# Hook-free: no UserPromptSubmit, no ~/.claude/scripts triggers.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$REPO_ROOT/templates/rules/GODSPEED_ALWAYS.md"
MARKER_BEGIN="<!-- xbrd-godspeed-always:begin -->"
MARKER_END="<!-- xbrd-godspeed-always:end -->"

if [[ ! -f "$SRC" ]]; then
  echo "missing $SRC" >&2
  exit 1
fi

body=$(cat "$SRC")
block="${MARKER_BEGIN}
${body}
${MARKER_END}"

upsert() {
  local target="$1"
  mkdir -p "$(dirname "$target")"
  if [[ ! -f "$target" ]]; then
    printf '%s\n' "$block" >"$target"
    echo "wrote $target"
    return
  fi
  # Strip previous managed block, then append fresh.
  local tmp
  tmp=$(mktemp)
  awk -v b="$MARKER_BEGIN" -v e="$MARKER_END" '
    $0 == b { skip=1; next }
    $0 == e { skip=0; next }
    !skip { print }
  ' "$target" >"$tmp"
  # Ensure trailing newline before block
  if [[ -s "$tmp" ]] && [[ "$(tail -c1 "$tmp" | wc -l)" -eq 0 ]]; then
    printf '\n' >>"$tmp"
  fi
  printf '%s\n' "$block" >>"$tmp"
  mv "$tmp" "$target"
  echo "updated $target"
}

# Claude Code — user-level project instructions (loaded for every session from home)
upsert "${HOME}/.claude/CLAUDE.md"

# Codex — global agents instructions
upsert "${HOME}/.codex/AGENTS.md"

# Canonical agents mirror (some tools read ~/.agents/AGENTS.md)
upsert "${HOME}/.agents/AGENTS.md"

# Grok Build — project rules path when present / for discovery
if command -v grok >/dev/null 2>&1 || [[ -d "${HOME}/.grok" ]]; then
  upsert "${HOME}/.grok/AGENTS.md"
fi

# Repo-local copy for in-tree sessions
upsert "$REPO_ROOT/CLAUDE.md"

echo "GODSPEED-ALWAYS-OK"
