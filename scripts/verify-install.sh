#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
H="${XBREED_HOME:-$HOME}"

DRIFT=0
AGENTS_DIR="$H/.claude/agents"
COMMANDS_DIR="$H/.claude/commands"
REF_COMMANDS_DIR="$COMMANDS_DIR/references"
DISPATCH_DIR="$H/.local/templates/dispatch"
BIN_DIR="$H/.local/bin"

fail() {
  local path="$1"
  local issue="$2"
  local remedy="$3"

  echo "DRIFT: $path" >&2
  echo "  issue: $issue" >&2
  echo "  remedy: $remedy" >&2
  DRIFT=1
}

ensure_symlink_resolves_under_dir() {
  local link_path="$1"
  local base_path="$2"
  local expected_dir="$3"
  local label="$4"

  if [[ ! -L "$link_path" ]]; then
    fail "$link_path" "$label symlink missing or not a symlink" "ln -sfn \"$base_path\" \"$link_path\""
    return
  fi

  local resolved
  resolved="$(readlink -f "$link_path" || true)"
  if [[ -z "$resolved" ]]; then
    fail "$link_path" "symlink did not resolve" "Recreate as: ln -sfn \"$base_path\" \"$link_path\""
    return
  fi

  if [[ "$resolved" != "$expected_dir"/* ]]; then
    fail "$link_path" "resolved outside expected directory: $resolved" "Recreate as: ln -sfn \"$base_path\" \"$link_path\""
    return
  fi

  if [[ -n "$base_path" && "$resolved" != "$base_path" ]]; then
    fail "$link_path" "resolved to unexpected target: $resolved" "Recreate as: ln -sfn \"$base_path\" \"$link_path\""
    return
  fi
}

check_agents() {
  local expected local_agent_files
  shopt -s nullglob
  expected=("$REPO_ROOT"/templates/agents/*.md)
  local_agent_files=("$AGENTS_DIR"/*.md)
  shopt -u nullglob
  declare -A local_index=()

  if [[ ${#expected[@]} -eq 0 ]]; then
    fail "$AGENTS_DIR" "no repo agent templates found at $REPO_ROOT/templates/agents" "Check checkout integrity for templates/agents"
    return
  fi

  if [[ ${#local_agent_files[@]} -eq 0 ]]; then
    fail "$AGENTS_DIR" "no local ~/.claude/agents/*.md files found" "Run make install to sync agent links"
    return
  fi

  for local_file in "${local_agent_files[@]}"; do
    local repo_file agent
    agent="$(basename "$local_file")"
    repo_file="$REPO_ROOT/templates/agents/$agent"
    local_index["$agent"]=1

    ensure_symlink_resolves_under_dir \
      "$local_file" \
      "$repo_file" \
      "$REPO_ROOT/templates/agents" \
      "agent file $agent"
  done

  for expected_file in "${expected[@]}"; do
    local repo_agent
    repo_agent="$(basename "$expected_file")"
    if [[ -z "${local_index[$repo_agent]:-}" ]]; then
      fail "$expected_file" "missing local symlink for agent template $repo_agent" "Run make install to sync agent links"
    fi
  done
}

check_commands() {
  local cmds
  cmds=(wwkd xb xbgst xbreed-team xbreed xbt xgs)

  for cmd in "${cmds[@]}"; do
    ensure_symlink_resolves_under_dir \
      "$COMMANDS_DIR/$cmd.md" \
      "$REPO_ROOT/commands/$cmd.md" \
      "$REPO_ROOT/commands" \
      "command $cmd.md"
  done

  ensure_symlink_resolves_under_dir \
    "$REF_COMMANDS_DIR/xbreed-shared.md" \
    "$REPO_ROOT/commands/references/xbreed-shared.md" \
    "$REPO_ROOT/commands/references" \
    "references/xbreed-shared.md"
}

check_dispatch() {
  local repo_files local_files
  shopt -s nullglob
  mapfile -t repo_files < <(printf '%s\n' "$REPO_ROOT"/templates/dispatch/*.md | sort)
  mapfile -t local_files < <(printf '%s\n' "$DISPATCH_DIR"/*.md | sort)
  shopt -u nullglob

  if [[ ${#repo_files[@]} -eq 0 ]]; then
    fail "$REPO_ROOT/templates/dispatch" "no repo dispatch templates found" "Check checkout integrity for templates/dispatch"
    return
  fi

  if [[ ${#local_files[@]} -eq 0 ]]; then
    fail "$DISPATCH_DIR" "no local dispatch templates found" "Run make install to populate $DISPATCH_DIR"
    return
  fi

  if [[ ${#repo_files[@]} -ne ${#local_files[@]} ]]; then
    fail "$DISPATCH_DIR" "template count mismatch (repo ${#repo_files[@]}, local ${#local_files[@]})" "Copy dispatch templates from repo: make install"
  fi

  declare -A local_index=()
  for local_file in "${local_files[@]}"; do
    local_index["$(basename "$local_file")"]=1
  done

  for repo_file in "${repo_files[@]}"; do
    local basename_repo file_local
    basename_repo="$(basename "$repo_file")"
    file_local="$DISPATCH_DIR/$basename_repo"

    if [[ -z "${local_index[$basename_repo]:-}" ]]; then
      fail "$file_local" "missing local dispatch template" "Copy dispatch templates from repo: make install"
      continue
    fi

    if [[ ! -f "$file_local" ]]; then
      fail "$file_local" "local dispatch path is missing" "Copy dispatch templates from repo: make install"
      continue
    fi

    if ! cmp -s "$repo_file" "$file_local"; then
      fail "$file_local" "dispatch template differs from repo source" "Re-sync by running make install"
      continue
    fi
  done
}

check_binaries() {
  local tool tool_path command_path resolved_tool resolved_command
  for tool in xbreed xask; do
    tool_path="$BIN_DIR/$tool"

    if [[ ! -x "$tool_path" ]]; then
      fail "$tool_path" "binary missing or non-executable" "Re-run make install"
      continue
    fi

    if [[ "$H" == "$HOME" ]]; then
      command_path="$(command -v "$tool" || true)"
      if [[ -z "$command_path" ]]; then
        fail "$tool_path" "command -v $tool returned nothing" "Ensure $BIN_DIR is in PATH before installation checks"
        continue
      fi

      if [[ "$command_path" != /* ]]; then
        fail "$tool_path" "command -v $tool did not resolve to a path: $command_path" "Run `command -v $tool` from a non-interactive shell context"
        continue
      fi

      resolved_tool="$(realpath "$tool_path")"
      resolved_command="$(realpath "$command_path")"
      if [[ "$resolved_tool" != "$resolved_command" ]]; then
        fail "$tool_path" "command -v resolves to $resolved_command, expected $resolved_tool" "Adjust PATH to prefer $resolved_tool"
        continue
      fi
    else
      echo "verify-install: NOTE: PATH-shadow check skipped for H=$H (not $HOME) in fixture mode" >&2
      continue
    fi
  done
}

check_agents
check_commands
check_dispatch
check_binaries

if [[ "$DRIFT" -ne 0 ]]; then
  echo "verify-install: DRIFT detected" >&2
  exit 1
fi

echo "verify-install: OK"
exit 0
