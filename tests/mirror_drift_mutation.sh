#!/usr/bin/env bash
# M4 — verify-docs.sh must flag a mirror that disagrees with SSoT on the
# canonical connector routing. Baseline should pass; a single-line mutation
# in AGENTS.md should flip verify-docs to exit non-zero with a DRIFT message;
# after restore, baseline must re-pass.
#
# Trap covers EXIT/INT/TERM/HUP so the repo is never left broken.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
AGENTS_MD="$REPO_ROOT/AGENTS.md"
BAK="$AGENTS_MD.m4-bak"

restore() {
  if [[ -f "$BAK" ]]; then
    mv "$BAK" "$AGENTS_MD"
    echo "RESTORED: $AGENTS_MD"
  fi
}
trap restore EXIT INT TERM HUP

cd "$REPO_ROOT"

# Step 1 — baseline verify-docs must pass
if ! ./scripts/verify-docs.sh >/dev/null 2>&1; then
  echo "FAIL: baseline scripts/verify-docs.sh did not pass" >&2
  ./scripts/verify-docs.sh >&2 || true
  exit 1
fi
echo "STEP 1 OK: baseline verify-docs passed"

# Step 2 — mutate the connector-row routing in AGENTS.md: gemini -> codex
cp "$AGENTS_MD" "$BAK"
# Restrict the sed to lines mentioning connector to avoid clobbering scout/others.
sed -i '/connector/ s|xask --effort high gemini|xask --effort high codex|' "$AGENTS_MD"

if ! grep -q 'xask --effort high codex' "$AGENTS_MD"; then
  echo "FAIL: mutation did not land in AGENTS.md" >&2
  exit 1
fi

# Step 3 — verify-docs must fail, and stderr/stdout must carry DRIFT diagnostic
set +e
OUT=$(./scripts/verify-docs.sh 2>&1)
STATUS=$?
set -e

if [[ $STATUS -eq 0 ]]; then
  echo "FAIL: verify-docs.sh passed despite AGENTS.md connector drift" >&2
  exit 1
fi

if ! printf '%s' "$OUT" | grep -q "DRIFT"; then
  echo "FAIL: verify-docs.sh exited non-zero but didn't emit DRIFT diagnostic" >&2
  printf '%s\n' "$OUT" >&2
  exit 1
fi

# Assert the diagnostic names the expected vs actual routing
if ! printf '%s' "$OUT" | grep -q 'expected: xask --effort high gemini'; then
  echo "FAIL: DRIFT did not name the canonical SSoT routing" >&2
  printf '%s\n' "$OUT" >&2
  exit 1
fi
if ! printf '%s' "$OUT" | grep -q 'actual:   xask --effort high codex'; then
  echo "FAIL: DRIFT did not name the actual mutated routing" >&2
  printf '%s\n' "$OUT" >&2
  exit 1
fi
echo "STEP 3 OK: DRIFT caught — expected gemini, actual codex"

# Step 4 — restore and reconfirm baseline
mv "$BAK" "$AGENTS_MD"
trap - EXIT INT TERM HUP

if ! ./scripts/verify-docs.sh >/dev/null 2>&1; then
  echo "FAIL: baseline did not re-pass after restore — repo state diverged" >&2
  exit 1
fi
echo "STEP 4 OK: baseline re-passes after restore"
echo "PASS: verify-docs.sh catches AGENTS.md connector routing drift"
