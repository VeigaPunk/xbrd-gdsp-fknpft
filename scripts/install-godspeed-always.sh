#!/usr/bin/env bash
# Install always-on Godspeed standing instructions — home harness roots only.
# Fully reversible via scripts/uninstall-godspeed-always.sh
#
# Does NOT mutate the xbreed repo AGENTS.md roster (SSoT — leave it alone).
# Strong no: never create or patch CLAUDE.md.
# Hook-free: no UserPromptSubmit, no ~/.claude/scripts triggers.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$REPO_ROOT/templates/rules/GODSPEED_ALWAYS.md"
MARKER_BEGIN="<!-- xbrd-godspeed-always:begin -->"
MARKER_END="<!-- xbrd-godspeed-always:end -->"
STATE_DIR="${XBREED_GODSPEED_STATE:-$HOME/.agents/godspeed-core}"
RECEIPT="$STATE_DIR/always-on.receipt"
BACKUP_DIR="$STATE_DIR/always-on-backups"
TS="$(date -u +%Y%m%dT%H%M%SZ)"

if [[ ! -f "$SRC" ]]; then
  echo "missing $SRC" >&2
  exit 1
fi

mkdir -p "$STATE_DIR" "$BACKUP_DIR"

body=$(cat "$SRC")
block="${MARKER_BEGIN}
${body}
${MARKER_END}"

: >"$RECEIPT.tmp"

record() {
  printf '%s|%s|%s\n' "$1" "$2" "${3:--}" >>"$RECEIPT.tmp"
}

# Write pre-managed content to backup (file without our marker block).
# Returns path to backup, or "-" if target did not exist.
backup_pre_managed() {
  local target="$1"
  if [[ ! -f "$target" ]]; then
    echo "-"
    return
  fi
  local dest="$BACKUP_DIR/$(echo "$target" | sed 's#/#_#g').$TS"
  local tmp
  tmp=$(mktemp)
  awk -v b="$MARKER_BEGIN" -v e="$MARKER_END" '
    $0 == b { skip=1; next }
    $0 == e { skip=0; next }
    !skip { print }
  ' "$target" >"$tmp"
  # If nothing remains, treat as "created by us previously" — backup empty marker file with special name
  if ! grep -q '[^[:space:]]' "$tmp"; then
    rm -f "$tmp"
    # sentinel: empty-before-block
    : >"${dest}.was-empty"
    echo "${dest}.was-empty"
    return
  fi
  mv "$tmp" "$dest"
  echo "$dest"
}

upsert() {
  local target="$1"
  case "$(basename "$target")" in
    CLAUDE.md|claude.md)
      echo "refused: CLAUDE.md is banned — skip $target" >&2
      record skipped "$target" -
      return 0
      ;;
  esac
  if [[ "$target" == "$REPO_ROOT/AGENTS.md" ]]; then
    echo "refused: will not mutate repo AGENTS.md roster — skip" >&2
    record skipped "$target" -
    return 0
  fi

  local bak existed=0
  if [[ -f "$target" ]]; then
    existed=1
  fi
  bak=$(backup_pre_managed "$target")

  mkdir -p "$(dirname "$target")"

  # Start from pre-managed content
  local base
  base=$(mktemp)
  if [[ "$bak" == "-" ]]; then
    : >"$base"
  elif [[ "$bak" == *.was-empty ]]; then
    : >"$base"
  else
    cp -a "$bak" "$base"
  fi

  if [[ -s "$base" ]] && [[ "$(tail -c1 "$base" | wc -l)" -eq 0 ]]; then
    printf '\n' >>"$base"
  fi
  printf '%s\n' "$block" >>"$base"
  mv "$base" "$target"

  if [[ "$existed" -eq 0 || "$bak" == *.was-empty || "$bak" == "-" ]]; then
    # Treat as created when no non-managed content pre-existed
    if [[ "$bak" == "-" || "$bak" == *.was-empty ]]; then
      echo "wrote $target (created)"
      record created "$target" "$bak"
    else
      echo "updated $target"
      record updated "$target" "$bak"
    fi
  else
    echo "updated $target"
    record updated "$target" "$bak"
  fi
}

upsert "${HOME}/.codex/AGENTS.md"
upsert "${HOME}/.agents/AGENTS.md"

if command -v grok >/dev/null 2>&1 || [[ -d "${HOME}/.grok" ]]; then
  upsert "${HOME}/.grok/AGENTS.md"
fi

ALWAYS_DST="${HOME}/.agents/godspeed-core/ALWAYS.md"
bak=$(backup_pre_managed "$ALWAYS_DST")
mkdir -p "${HOME}/.agents/godspeed-core"
cp -f "$SRC" "$ALWAYS_DST"
if [[ "$bak" == "-" || "$bak" == *.was-empty ]]; then
  record created "$ALWAYS_DST" "$bak"
else
  record updated "$ALWAYS_DST" "$bak"
fi
echo "wrote $ALWAYS_DST"

for f in "${HOME}/.claude/CLAUDE.md" "$REPO_ROOT/CLAUDE.md"; do
  if [[ -f "$f" ]]; then
    bak=$(backup_pre_managed "$f")
    # full file backup for claude (pre-managed strip may empty it)
    if [[ "$bak" == "-" ]]; then
      full="$BACKUP_DIR/$(echo "$f" | sed 's#/#_#g').full.$TS"
      cp -a "$f" "$full"
      bak="$full"
    fi
    rm -f "$f"
    echo "nuked $f (CLAUDE.md ban); backup=$bak"
    record removed_claude_md "$f" "$bak"
  fi
done

mv "$RECEIPT.tmp" "$RECEIPT"
echo "receipt: $RECEIPT"
echo "backups: $BACKUP_DIR"
echo "reverse: bash $REPO_ROOT/scripts/uninstall-godspeed-always.sh"
echo "GODSPEED-ALWAYS-OK (reversible; repo roster untouched)"
