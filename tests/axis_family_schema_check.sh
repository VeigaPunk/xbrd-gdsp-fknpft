#!/usr/bin/env bash
# M9 — axis_family frontmatter in templates/agents/*.md must be members of the
# closed enum declared in commands/references/xbreed-shared.md line 58.
# Baseline must pass; a mutation injecting a bogus axis_family must be caught;
# after restore, baseline must re-pass.
#
# Trap covers EXIT/INT/TERM/HUP so the repo is never left broken.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SSOT="$REPO_ROOT/commands/references/xbreed-shared.md"
AGENTS_DIR="$REPO_ROOT/templates/agents"
TARGET="$AGENTS_DIR/executor.md"
BAK="$TARGET.m9-bak"

restore() {
  if [[ -f "$BAK" ]]; then
    mv "$BAK" "$TARGET"
    echo "RESTORED: $TARGET"
  fi
}
trap restore EXIT INT TERM HUP

# --- helper: run the schema check, return 0=pass 1=fail, emit violations on stdout ---
run_check() {
  # Extract allowed set from SSoT prose line:
  #   Allowed `axis_family` values ...: `research`, `correctness`, ...
  local allowed_line
  allowed_line=$(grep 'Allowed `axis_family` values' "$SSOT")

  # Pull every backtick-delimited token after the colon
  local allowed=()
  while IFS= read -r tok; do
    allowed+=("$tok")
  done < <(printf '%s\n' "$allowed_line" | grep -oP '`[a-z][a-z-]*`' | tr -d '`')

  if [[ ${#allowed[@]} -lt 10 ]]; then
    echo "ERROR: enum extraction yielded only ${#allowed[@]} values — regex may be broken" >&2
    return 1
  fi

  local fail=0
  for f in "$AGENTS_DIR"/*.md; do
    local val
    val=$(awk 'BEGIN{in_fm=0} NR==1&&/^---/{in_fm=1;next} in_fm&&/^---/{exit} /^axis_family:/{sub(/^axis_family:[[:space:]]*/,""); gsub(/[[:space:]]/,""); print; exit}' "$f")
    [[ -z "${val:-}" ]] && continue
    local found=0
    for a in "${allowed[@]}"; do
      [[ "$val" == "$a" ]] && { found=1; break; }
    done
    if [[ $found -eq 0 ]]; then
      echo "SCHEMA DRIFT: file=$(basename "$f") axis_family='$val' not in enum [${allowed[*]}]"
      fail=1
    fi
  done
  return "$fail"
}

cd "$REPO_ROOT"

# Step 1 — baseline: all templates must pass schema check
set +e
OUT=$(run_check 2>&1)
STATUS=$?
set -e

if [[ $STATUS -ne 0 ]]; then
  echo "FAIL: baseline schema check did not pass" >&2
  printf '%s\n' "$OUT" >&2
  exit 1
fi
echo "STEP 1 OK: baseline schema check passed (all axis_family values in enum)"

# Step 2 — mutate executor.md: inject an invalid axis_family
cp "$TARGET" "$BAK"
sed -i 's/^axis_family: execution$/axis_family: bogus_value/' "$TARGET"

if ! grep -q 'axis_family: bogus_value' "$TARGET"; then
  echo "FAIL: mutation did not land in $TARGET" >&2
  exit 1
fi

# Step 3 — schema check must fail and emit SCHEMA DRIFT with the bogus value
set +e
OUT=$(run_check 2>&1)
STATUS=$?
set -e

if [[ $STATUS -eq 0 ]]; then
  echo "FAIL: schema check passed despite bogus axis_family — drift not caught" >&2
  exit 1
fi

if ! printf '%s' "$OUT" | grep -q 'SCHEMA DRIFT'; then
  echo "FAIL: schema check exited non-zero but did not emit SCHEMA DRIFT" >&2
  printf '%s\n' "$OUT" >&2
  exit 1
fi

if ! printf '%s' "$OUT" | grep -q 'bogus_value'; then
  echo "FAIL: SCHEMA DRIFT did not name the injected value 'bogus_value'" >&2
  printf '%s\n' "$OUT" >&2
  exit 1
fi
echo "--- mutation-step captured output ---"
printf '%s\n' "$OUT"
echo "--- end mutation-step output ---"
echo "STEP 3 OK: SCHEMA DRIFT caught — axis_family='bogus_value' rejected"

# Step 4 — restore and reconfirm baseline
mv "$BAK" "$TARGET"
trap - EXIT INT TERM HUP

set +e
OUT=$(run_check 2>&1)
STATUS=$?
set -e

if [[ $STATUS -ne 0 ]]; then
  echo "FAIL: baseline did not re-pass after restore — repo state diverged" >&2
  printf '%s\n' "$OUT" >&2
  exit 1
fi
echo "STEP 4 OK: baseline re-passes after restore"
echo "PASS: axis_family_schema_check catches enum drift in templates/agents/*.md"
