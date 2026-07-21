#!/usr/bin/env bash
# Reverse scripts/install-godspeed-always.sh
# Safe: only undoes receipt actions / managed marker blocks.
# Never invents CLAUDE.md (ban). Never touches repo AGENTS.md roster content
# except stripping a managed block if one was wrongly inserted.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MARKER_BEGIN="<!-- xbrd-godspeed-always:begin -->"
MARKER_END="<!-- xbrd-godspeed-always:end -->"
STATE_DIR="${XBREED_GODSPEED_STATE:-$HOME/.agents/godspeed-core}"
RECEIPT="$STATE_DIR/always-on.receipt"
BACKUP_DIR="$STATE_DIR/always-on-backups"

strip_block() {
  local target="$1"
  [[ -f "$target" ]] || return 0
  if ! grep -qF "$MARKER_BEGIN" "$target" 2>/dev/null; then
    return 0
  fi
  local tmp
  tmp=$(mktemp)
  awk -v b="$MARKER_BEGIN" -v e="$MARKER_END" '
    $0 == b { skip=1; next }
    $0 == e { skip=0; next }
    !skip { print }
  ' "$target" >"$tmp"
  if ! grep -q '[^[:space:]]' "$tmp"; then
    rm -f "$target" "$tmp"
    echo "removed $target (only managed block)"
  else
    mv "$tmp" "$target"
    echo "stripped managed block from $target"
  fi
}

echo "uninstall-godspeed-always: reversing always-on install"

if [[ -f "$RECEIPT" ]]; then
  while IFS='|' read -r action path bak; do
    [[ -n "${action:-}" ]] || continue
    case "$action" in
      created)
        # Did not exist (or was empty of non-managed content) before install
        rm -f "$path"
        echo "removed created $path"
        ;;
      updated)
        if [[ "${bak:--}" != "-" && -f "$bak" && "$bak" != *.was-empty ]]; then
          mkdir -p "$(dirname "$path")"
          cp -a "$bak" "$path"
          echo "restored $path <- $bak"
          # belt: strip block if backup somehow still had one
          strip_block "$path"
        else
          strip_block "$path"
          # if only managed content remains empty file, remove
          if [[ -f "$path" ]] && ! grep -q '[^[:space:]]' "$path"; then
            rm -f "$path"
            echo "removed empty $path"
          fi
        fi
        ;;
      removed_claude_md)
        echo "note: CLAUDE.md not restored (ban). backup=${bak:--}"
        ;;
      skipped) ;;
      *)
        echo "unknown receipt action: $action ($path)" >&2
        ;;
    esac
  done <"$RECEIPT"
  mv "$RECEIPT" "$RECEIPT.uninstalled.$(date -u +%Y%m%dT%H%M%SZ)"
else
  echo "no receipt — best-effort strip of known paths"
  for path in \
    "${HOME}/.codex/AGENTS.md" \
    "${HOME}/.agents/AGENTS.md" \
    "${HOME}/.grok/AGENTS.md"
  do
    strip_block "$path"
  done
fi

# ALWAYS.md is install-owned
rm -f "${HOME}/.agents/godspeed-core/ALWAYS.md"

# Final pass: known paths must not retain managed blocks
for path in \
  "${HOME}/.codex/AGENTS.md" \
  "${HOME}/.agents/AGENTS.md" \
  "${HOME}/.grok/AGENTS.md"
do
  strip_block "$path"
done

if [[ -f "$REPO_ROOT/AGENTS.md" ]] && grep -qF "$MARKER_BEGIN" "$REPO_ROOT/AGENTS.md" 2>/dev/null; then
  echo "WARN: managed block in repo AGENTS.md — stripping" >&2
  strip_block "$REPO_ROOT/AGENTS.md"
fi

echo "backups kept under: $BACKUP_DIR (delete manually when sure)"
echo "GODSPEED-ALWAYS-UNINSTALLED"
