#!/usr/bin/env bash
# M03 — verify-routing.sh mutation proof (drift-net-0607 R3).
# Codifies the judge acceptance battery: each drift class is injected and the
# checker must go red with the right exit code; baseline must re-pass after
# every restore. Trap + sha postcondition per mirror_drift_mutation.sh idiom.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"
CHECKER="scripts/verify-routing.sh"
JUDGE_MD="templates/agents/the-judge.md"
SSOT="commands/references/xbreed-shared.md"

BASE_SHA_JUDGE="$(sha256sum "$JUDGE_MD" | awk '{print $1}')"
BASE_SHA_AGENTS="$(sha256sum AGENTS.md | awk '{print $1}')"

BAK_JUDGE="$(mktemp)"; cp "$JUDGE_MD" "$BAK_JUDGE"
BAK_AGENTS="$(mktemp)"; cp AGENTS.md "$BAK_AGENTS"
# cp/mv backups (mirror_drift idiom), NOT git checkout: checkout restores HEAD
# and silently destroys uncommitted working-tree fixes (bit the judge twice).
restore() { cp "$BAK_JUDGE" "$JUDGE_MD" 2>/dev/null || true; cp "$BAK_AGENTS" AGENTS.md 2>/dev/null || true; }
cleanup() { restore; rm -f "$BAK_JUDGE" "$BAK_AGENTS"; }
trap cleanup EXIT INT TERM HUP

fail() { echo "FAIL: $1" >&2; exit 1; }

# Baseline must be green before any mutation.
bash "$CHECKER" >/dev/null 2>&1 || fail "baseline verify-routing.sh did not pass"
echo "STEP 0 OK: baseline green"

# P1 — judge counterexample: shared-row tier mutation (M-G class).
sed -i 's|xask --effort medium --gs codex "<q>"|xask --effort xhigh --gs codex "<q>"|' "$JUDGE_MD"
set +e; OUT=$(bash "$CHECKER" 2>&1); ST=$?; set -e
restore
[[ $ST -eq 1 ]] || fail "P1 shared-row mutation not caught (exit $ST)"
grep -q "row-scoped\|non-canonical" <<< "$OUT" || fail "P1 diagnostic missing"
echo "STEP 1 OK: shared-row tier mutation caught (M-G)"

# P2 — phantom lane appended to AGENTS.md (M-J class).
echo '| Phantom, fake | `phantom` | sonnet | `xask --effort low codex` | All |' >> AGENTS.md
set +e; OUT=$(bash "$CHECKER" 2>&1); ST=$?; set -e
restore
[[ $ST -eq 1 ]] || fail "P2 phantom lane not caught (exit $ST)"
grep -q "bidirectional" <<< "$OUT" || fail "P2 diagnostic missing"
echo "STEP 2 OK: phantom lane caught (M-J)"

# P5 — empty extraction must exit 2 (M-B floor).
set +e; SSOT=/dev/null bash "$CHECKER" >/dev/null 2>&1; ST=$?; set -e
[[ $ST -eq 2 ]] || fail "P5 empty extraction wrong exit ($ST, want 2)"
echo "STEP 3 OK: empty extraction exit 2 (M-B)"

# P6 — renamed section anchor must exit 2 (M-E).
TMP_SSOT="$(mktemp)"
sed 's/^## Axis → Profile Mapping/## ROUTING TABLE/' "$SSOT" > "$TMP_SSOT"
set +e; SSOT="$TMP_SSOT" bash "$CHECKER" >/dev/null 2>&1; ST=$?; set -e
rm -f "$TMP_SSOT"
[[ $ST -eq 2 ]] || fail "P6 anchor rename wrong exit ($ST, want 2)"
echo "STEP 4 OK: anchor rename exit 2 (M-E)"

# P7 — deprecated canonical-bearing row must exit 1 (Class C stub).
TMP_SSOT="$(mktemp)"
sed '0,/| Research, prior art |/s//| Research, prior art (deprecated) |/' "$SSOT" > "$TMP_SSOT"
set +e; OUT=$(SSOT="$TMP_SSOT" bash "$CHECKER" 2>&1); ST=$?; set -e
rm -f "$TMP_SSOT"
[[ $ST -eq 1 ]] || fail "P7 deprecated row not caught (exit $ST)"
grep -q "Class C" <<< "$OUT" || fail "P7 Class C diagnostic missing"
echo "STEP 5 OK: deprecated row caught (Class C)"

# Baseline re-pass + sha postconditions.
bash "$CHECKER" >/dev/null 2>&1 || fail "baseline did not re-pass after mutations"
trap - EXIT INT TERM HUP
rm -f "$BAK_JUDGE" "$BAK_AGENTS"
[[ "$(sha256sum "$JUDGE_MD" | awk '{print $1}')" == "$BASE_SHA_JUDGE" ]] || fail "$JUDGE_MD diverged from baseline"
[[ "$(sha256sum AGENTS.md | awk '{print $1}')" == "$BASE_SHA_AGENTS" ]] || fail "AGENTS.md diverged from baseline"
echo "PASS: verify-routing.sh catches all five mutation classes; baseline restored"
