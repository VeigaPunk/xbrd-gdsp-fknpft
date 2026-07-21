#!/usr/bin/env bash
# Best-effort reverse of docs/SETUP.md install steps (hook-free, roster-safe).
# Does NOT uninstall system packages (rust, claude, codex).
# Does NOT delete the git clones under ~/repos or $REPO.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export REPO="${REPO:-$REPO_ROOT}"

echo "=== uninstall-setup (reversible teardown) ==="
echo "REPO=$REPO"

# 1) Always-on godspeed (receipt-aware)
if [[ -x "$REPO/scripts/uninstall-godspeed-always.sh" ]]; then
  bash "$REPO/scripts/uninstall-godspeed-always.sh"
else
  echo "skip godspeed uninstall (script missing)"
fi

# 2) Skills symlinks we created under ~/.claude/skills and ~/.grok/skills
for s in godspeed stahp xb xbgst xbreed xbreed-team xbt xgs; do
  for root in "$HOME/.claude/skills" "$HOME/.grok/skills"; do
    link="$root/$s"
    if [[ -L "$link" ]]; then
      target=$(readlink -f "$link" 2>/dev/null || readlink "$link")
      # only remove if it points at our canonical store
      case "$target" in
        "$HOME/.agents/skills/$s"|"$HOME/.agents/skills/$s/"*)
          rm -f "$link"
          echo "removed symlink $link"
          ;;
        *)
          echo "kept $link (points elsewhere: $target)"
          ;;
      esac
    fi
  done
done

# 3) Canonical skill copies under ~/.agents/skills (only if they match repo templates)
if [[ -d "$HOME/.agents/skills" && -d "$REPO/templates/skills" ]]; then
  for s in godspeed stahp xb xbgst xbreed xbreed-team xbt xgs; do
    d="$HOME/.agents/skills/$s"
    if [[ -d "$d" ]]; then
      # leave in place by default — user may have edited; optional --purge-skills
      if [[ "${PURGE_SKILLS:-0}" == "1" ]]; then
        rm -rf "$d"
        echo "purged $d"
      else
        echo "kept $d (set PURGE_SKILLS=1 to remove)"
      fi
    fi
  done
fi

# 4) Agent symlinks into this repo's templates/agents
if [[ -d "$HOME/.claude/agents" ]]; then
  for link in "$HOME/.claude/agents"/*.md; do
    [[ -L "$link" ]] || continue
    target=$(readlink -f "$link" 2>/dev/null || true)
    case "$target" in
      "$REPO/templates/agents/"*)
        rm -f "$link"
        echo "removed agent symlink $link"
        ;;
    esac
  done
fi
if [[ -d "$HOME/.grok/agents" ]]; then
  for link in "$HOME/.grok/agents"/*.md; do
    [[ -L "$link" ]] || continue
    target=$(readlink -f "$link" 2>/dev/null || true)
    case "$target" in
      "$REPO/templates/agents/"*)
        rm -f "$link"
        echo "removed grok agent symlink $link"
        ;;
    esac
  done
fi

# 5) Commands installed by install-commands.sh (symlinks into repo)
if [[ -d "$HOME/.claude/commands" ]]; then
  for link in "$HOME/.claude/commands"/*; do
    [[ -L "$link" ]] || continue
    target=$(readlink -f "$link" 2>/dev/null || true)
    case "$target" in
      "$REPO/commands/"*|"$REPO/"*)
        rm -f "$link"
        echo "removed command symlink $link"
        ;;
    esac
  done
fi

# 6) Binaries + dispatch templates from make install (only if they match this repo build)
for bin in xbreed xask; do
  p="$HOME/.local/bin/$bin"
  if [[ -f "$p" || -L "$p" ]]; then
    if [[ "${PURGE_BINS:-0}" == "1" ]]; then
      rm -f "$p"
      echo "removed $p"
    else
      echo "kept $p (set PURGE_BINS=1 to remove)"
    fi
  fi
done
if [[ -d "$HOME/.local/templates/dispatch" && "${PURGE_DISPATCH:-0}" == "1" ]]; then
  rm -rf "$HOME/.local/templates/dispatch"
  echo "removed ~/.local/templates/dispatch"
else
  echo "kept ~/.local/templates/dispatch (set PURGE_DISPATCH=1 to remove)"
fi

# 7) godspeed-core copy under ~/.agents (optional purge)
if [[ -d "$HOME/.agents/godspeed-core" ]]; then
  if [[ "${PURGE_GSCORE:-0}" == "1" ]]; then
    # keep backups dir if present? user can wipe whole dir
    rm -rf "$HOME/.agents/godspeed-core"
    echo "purged ~/.agents/godspeed-core"
  else
    echo "kept ~/.agents/godspeed-core (set PURGE_GSCORE=1 to remove trilogy+receipts)"
  fi
fi

# 8) Settings: we do NOT reverse CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS / teammateMode
#    (user may want them for other workflows). Document only.
echo "note: ~/.claude/settings.json team knobs left intact (manual revert if desired)"

echo "=== uninstall-setup done ==="
echo "Defaults keep bins/skills/gscore so a re-install is cheap."
echo "Full wipe: PURGE_SKILLS=1 PURGE_BINS=1 PURGE_DISPATCH=1 PURGE_GSCORE=1 $0"
