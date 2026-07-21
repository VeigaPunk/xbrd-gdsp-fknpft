#!/usr/bin/env bash
# Install always-on Godspeed standing instructions — home harness roots only.
# Does NOT mutate the xbreed repo AGENTS.md roster (SSoT — leave it alone).
# Strong no: never create or patch CLAUDE.md.
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
  case "$(basename "$target")" in
    CLAUDE.md|claude.md)
      echo "refused: CLAUDE.md is banned — skip $target" >&2
      return 0
      ;;
  esac
  # Never write into the git checkout roster
  if [[ "$target" == "$REPO_ROOT/AGENTS.md" ]]; then
    echo "refused: will not mutate repo AGENTS.md roster — skip" >&2
    return 0
  fi
  mkdir -p "$(dirname "$target")"
  if [[ ! -f "$target" ]]; then
    printf '%s\n' "$block" >"$target"
    echo "wrote $target"
    return
  fi
  local tmp
  tmp=$(mktemp)
  awk -v b="$MARKER_BEGIN" -v e="$MARKER_END" '
    $0 == b { skip=1; next }
    $0 == e { skip=0; next }
    !skip { print }
  ' "$target" >"$tmp"
  if [[ -s "$tmp" ]] && [[ "$(tail -c1 "$tmp" | wc -l)" -eq 0 ]]; then
    printf '\n' >>"$tmp"
  fi
  printf '%s\n' "$block" >>"$tmp"
  mv "$tmp" "$target"
  echo "updated $target"
}

# Home-only surfaces — do not touch the repo tree
upsert "${HOME}/.codex/AGENTS.md"
upsert "${HOME}/.agents/AGENTS.md"

if command -v grok >/dev/null 2>&1 || [[ -d "${HOME}/.grok" ]]; then
  upsert "${HOME}/.grok/AGENTS.md"
fi

# Optional dedicated always-on note next to trilogy (does not replace directive)
mkdir -p "${HOME}/.agents/godspeed-core"
cp -f "$SRC" "${HOME}/.agents/godspeed-core/ALWAYS.md"
echo "wrote ${HOME}/.agents/godspeed-core/ALWAYS.md"

# Nuke CLAUDE.md leftovers from earlier experiments only
for f in "${HOME}/.claude/CLAUDE.md" "$REPO_ROOT/CLAUDE.md"; do
  if [[ -f "$f" ]]; then
    rm -f "$f"
    echo "nuked $f (CLAUDE.md ban)"
  fi
done

echo "GODSPEED-ALWAYS-OK (home AGENTS.md only; repo roster untouched)"
