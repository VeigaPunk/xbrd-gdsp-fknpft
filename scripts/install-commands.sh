#!/usr/bin/env bash
# Install xbreed command files into ~/.claude/commands/ as symlinks.
# Run once after clone. Idempotent.
set -euo pipefail

REPO="$(cd "$(dirname "$0")/.." && pwd)"
CMDS="${CLAUDE_COMMANDS_DIR:-$HOME/.claude/commands}"

mkdir -p "$CMDS/references"

for f in wwkd xb xbgst xbreed-team xbreed xbt xgs; do
  ln -sfn "$REPO/commands/$f.md" "$CMDS/$f.md"
done
ln -sfn "$REPO/commands/references/xbreed-shared.md" "$CMDS/references/xbreed-shared.md"

echo "Installed xbreed commands into $CMDS (from $REPO)"
