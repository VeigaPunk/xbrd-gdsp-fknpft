#!/usr/bin/env bash
# verify-routing.sh — SSoT routing drift checker (all xask lanes, 4 surface classes).
#
# Judge-drafted (drift-net-0607 R3) after two executor-lane false-negative
# rejections. Carries the full R1–R3 finding set:
#   M-B floor check · M-C backtick-cell parse · M-E section anchor
#   M-F append-drift guard (ere-escaped) · M-G row-scoped shared surfaces
#   M-J bidirectional membership sweep · Class-C deprecated stub
#   critic(a): NO static expectation matrix — all checks derived from extraction
#   critic(b): keyed on lane strings; role names read from SSoT rows at runtime
#   critic(c): -scp args stripped for membership only; scope shape asserted
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SSOT="${SSOT:-$REPO_ROOT/commands/references/xbreed-shared.md}"

SHARED_SURFACES=("AGENTS.md" "templates/agents/the-judge.md" "commands/xbgst.md")

DRIFT=0
drift() { echo "DRIFT: $1" >&2; [[ -n "${2:-}" ]] && echo "  $2" >&2; DRIFT=1; }

# Escape a string for safe interpolation into ERE (M-F hardening).
ere_escape() { printf '%s' "$1" | sed 's/[.[\*^$(){}+?|\\]/\\&/g'; }

# Normalize an extracted xask invocation. Order matters: strip `-scp/--scope
# <arg>` FIRST (the arg may be quoted and pipe-bearing, e.g. sentinel's
# "<auth|input|secrets>"), THEN truncate at the first remaining quote (gate
# strings carry '<question>' suffixes), collapse whitespace, rtrim.
normalize() {
  printf '%s' "$1" \
    | sed -E 's/(--scope|-scp)[[:space:]]+("[^"]*"|<[^>]*>|[^[:space:]]+)[[:space:]]*/ /' \
    | sed -E "s/['\"].*$//" \
    | tr -s ' ' | sed 's/[[:space:]]*$//'
}

# Extract every xask-invocation-shaped string from a file (bidirectional sweep
# input). Primary channel: backtick-delimited code spans starting "xask "
# (markdown convention everywhere in these surfaces; preserves pipes/quotes
# inside scoped args). Secondary: raw-regex residue outside backticks, so a
# phantom lane written without code formatting still gets swept. Prose
# mentions ("no xask gate", "skip xask for...") don't match either channel.
extract_xask_strings() {
  { grep -oP '(?<=`)xask [^`]+(?=`)' "$1" || true
    sed 's/`[^`]*`//g' "$1" | grep -oE 'xask (-scp|--?[A-Za-z][^'"'"'"|]*)' || true
  } | sed 's/[[:space:]]*$//'
}

# ---------- Step 1: Extract (role, pattern) pairs from SSoT ----------
# "## Axis → Profile Mapping" section (M-E anchor); backtick cells starting
# "xask " (M-C — no model-name filters). Dual-lane rows yield 2 patterns.

declare -A ROLE_PATTERNS=()   # role → newline-joined patterns. =() required:
declare -A CANONICAL=()       # pattern → 1. bash 5.3 set -u treats empty assoc
                              # arrays as unbound without explicit init.
in_section=0
while IFS= read -r line; do
  if [[ "$line" == "## Axis"*"Profile Mapping"* ]]; then in_section=1; continue; fi
  [[ "$in_section" -eq 1 && "$line" == "## "* ]] && break
  [[ "$in_section" -eq 1 && "$line" == \|* ]] || continue

  role=$(awk -F'|' '{print $3}' <<< "$line" | tr -d '`*' | xargs || true)
  [[ -z "$role" || "$role" == "Role" || "$role" == :* || "$role" == -* ]] && continue
  # Strip parenthetical qualifiers: "labrat (sonnet)" → "labrat"
  role="${role%% (*}"

  # Class-C stub (critic(e)): a canonical-bearing row marked deprecated is DRIFT.
  if grep -q '`xask ' <<< "$line" && grep -qi 'deprecated' <<< "$line"; then
    drift "Class C — canonical-bearing SSoT row marked deprecated [$role]" "row: $(xargs <<< "$line" | cut -c1-120)"
  fi

  while IFS= read -r cell; do
    pattern=$(sed 's/[[:space:]]*$//' <<< "$cell")
    [[ "$pattern" == xask\ * ]] || continue
    ROLE_PATTERNS["$role"]+="${pattern}"$'\n'
    CANONICAL["$pattern"]=1
  done < <(grep -oP '(?<=`)([^`]+)(?=`)' <<< "$line" || true)
done < "$SSOT"

# ---------- Step 2: Floor check (M-B) ----------
PATTERN_COUNT=${#CANONICAL[@]}
ROLE_COUNT=${#ROLE_PATTERNS[@]}
# CANONICAL dedupes shared strings (spark×3, gpt55-low×3 → 6 unique); the pair
# total must still reach 11 across roles.
PAIR_COUNT=0
for r in "${!ROLE_PATTERNS[@]}"; do
  n=$(grep -c . <<< "${ROLE_PATTERNS[$r]}")
  PAIR_COUNT=$((PAIR_COUNT + n))
done

if (( PAIR_COUNT < 11 || ROLE_COUNT < 9 )); then
  echo "DRIFT: floor check failed [anchor: '## Axis → Profile Mapping' in $SSOT]" >&2
  echo "  pairs=${PAIR_COUNT} (need >=11)  roles=${ROLE_COUNT} (need >=9)" >&2
  exit 2
fi
echo "SSoT floor check OK: ${PAIR_COUNT} (role,pattern) pairs, ${PATTERN_COUNT} unique patterns, ${ROLE_COUNT} roles"

# ---------- Step 3: Per-role template files (forward, scp-normalized) ----------
# Each role's canonical pattern(s) must appear in templates/agents/<role>.md,
# matched against the file's scp-normalized xask strings (covers sentinel's
# scoped primary — previously an acknowledged hole). Extra strings in role
# files (documented escalation lanes) are allowed: no membership test here.

for role in "${!ROLE_PATTERNS[@]}"; do
  file="$REPO_ROOT/templates/agents/${role}.md"
  if [[ ! -f "$file" ]]; then
    drift "missing role template: templates/agents/${role}.md"
    continue
  fi
  normalized_set=$'\n'
  while IFS= read -r s; do
    [[ -n "$s" ]] && normalized_set+="$(normalize "$s")"$'\n'
  done < <(extract_xask_strings "$file")

  while IFS= read -r pattern; do
    [[ -n "$pattern" ]] || continue
    if [[ "$normalized_set" != *$'\n'"$pattern"$'\n'* ]]; then
      drift "missing pattern in templates/agents/${role}.md" "expected: $pattern"
    fi
  done <<< "${ROLE_PATTERNS[$role]}"
done

# ---------- Step 4: Shared surfaces — row-scoped (M-G) + membership (M-J) ----------
# Role rows are anchored as `role` or **role** (plain prose mentions like
# "Distinct from reviewer (code bugs)" do not count — prevents cross-row
# false-pass, the judge counterexample class).

for rel in "${SHARED_SURFACES[@]}"; do
  file="$REPO_ROOT/$rel"
  if [[ ! -f "$file" ]]; then drift "missing shared surface: $rel"; continue; fi

  for role in "${!ROLE_PATTERNS[@]}"; do
    role_lines=$(grep -E "(\*\*${role}\*\*|\`${role}\`)" "$file" | grep -F 'xask ' || true)
    [[ -n "$role_lines" ]] || continue   # surface doesn't document this role's lanes

    # (a) at least one of the role's own canonical patterns in its rows
    own_found=0
    while IFS= read -r pattern; do
      [[ -n "$pattern" ]] || continue
      grep -Fq "$pattern" <<< "$role_lines" && own_found=1
    done <<< "${ROLE_PATTERNS[$role]}"
    if [[ "$own_found" -eq 0 ]]; then
      drift "row-scoped: no canonical lane for '$role' in $rel" "expected one of: $(tr '\n' ';' <<< "${ROLE_PATTERNS[$role]}")"
    fi

    # (b) every xask string in the role's rows must be canonical (M-J, row level)
    while IFS= read -r s; do
      [[ -n "$s" ]] || continue
      norm=$(normalize "$s")
      if [[ -z "${CANONICAL[$norm]:-}" ]]; then
        drift "non-canonical lane in '$role' row of $rel" "found: $norm"
      fi
    done < <(extract_xask_strings /dev/stdin <<< "$role_lines")
  done

  # (c) file-wide bidirectional sweep (M-J): catches phantom rows whose role
  # is unknown to the SSoT (their role anchor never enters the loop above).
  while IFS= read -r s; do
    [[ -n "$s" ]] || continue
    norm=$(normalize "$s")
    [[ "$norm" == "xask" || "$norm" == "xask "[a-z]* && "$norm" != *" -"* ]] && continue  # bare flag-usage examples (e.g. `xask --scope "<boundary>"`), not lanes
    if [[ -z "${CANONICAL[$norm]:-}" ]]; then
      drift "non-canonical xask lane in $rel (bidirectional sweep)" "found: $norm"
    fi
  done < <(extract_xask_strings "$file")

  # (d) append-drift guard (M-F), file-wide, ere-escaped
  for pattern in "${!CANONICAL[@]}"; do
    escaped=$(ere_escape "$pattern")
    if grep -qE "${escaped} --[A-Za-z]" "$file"; then
      drift "append-drift in $rel" "pattern: $pattern"
    fi
  done
done

# ---------- Verdict ----------
if [[ "$DRIFT" -eq 0 ]]; then
  echo "OK: all routing patterns consistent with canonical SSoT"
  exit 0
fi
exit 1
