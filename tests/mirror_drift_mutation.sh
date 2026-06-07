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

BASELINE_SHA="$(sha256sum "$AGENTS_MD" | awk '{print $1}')"

restore() {
  if [[ -f "$BAK" ]]; then
    mv "$BAK" "$AGENTS_MD"
    echo "RESTORED: $AGENTS_MD"
  fi
}
trap 'restore; NOW_SHA=$(sha256sum "$AGENTS_MD" 2>/dev/null | awk "{print \$1}" || echo MISSING); [[ "$NOW_SHA" == "$BASELINE_SHA" ]] || { echo "FAIL: $AGENTS_MD content diverged from baseline (restore trap failed or incomplete; baseline=${BASELINE_SHA:0:16}... now=${NOW_SHA:0:16}...)" >&2; exit 1; }' EXIT INT TERM HUP

cd "$REPO_ROOT"

# Step 1 — baseline verify-docs must pass
if ! ./scripts/verify-docs.sh >/dev/null 2>&1; then
  echo "FAIL: baseline scripts/verify-docs.sh did not pass" >&2
  ./scripts/verify-docs.sh >&2 || true
  exit 1
fi
echo "STEP 1 OK: baseline verify-docs passed"

# Step 2 — mutate the connector-row routing in AGENTS.md by flipping the model
# token of the canonical lane (codex<->gemini). Derive CANONICAL from the SSoT
# the same way verify-docs.sh does, so this test self-synchronizes with future
# lane changes instead of hardcoding a routing string that drifts stale (the
# pre-68695ff version hardcoded 'xask --effort high gemini' and silently
# no-op'd after the gemini->codex swap).
SSOT="$REPO_ROOT/commands/references/xbreed-shared.md"
CANONICAL=$(grep -m1 -F '| Cross-axis patterns' "$SSOT" \
  | grep -oE 'xask --effort [a-z]+ [a-z]+' | head -1 || true)
if [[ -z "$CANONICAL" ]]; then
  echo "FAIL: cannot extract canonical connector routing from $SSOT" >&2
  exit 1
fi
MODEL="${CANONICAL##* }"
if [[ "$MODEL" == "codex" ]]; then MUT_MODEL="gemini"; else MUT_MODEL="codex"; fi
MUTATED="${CANONICAL% *} $MUT_MODEL"

cp "$AGENTS_MD" "$BAK"
# Restrict the sed to lines mentioning connector to avoid clobbering scout/others.
sed -i "/connector/ s|$CANONICAL|$MUTATED|" "$AGENTS_MD"

# Landing check is also connector-scoped: MUTATED may legitimately appear in
# other agents' rows (e.g. a different lane using the same effort tier).
if ! grep 'connector' "$AGENTS_MD" | grep -qF "$MUTATED"; then
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
if ! printf '%s' "$OUT" | grep -qF "expected: $CANONICAL"; then
  echo "FAIL: DRIFT did not name the canonical SSoT routing" >&2
  printf '%s\n' "$OUT" >&2
  exit 1
fi
if ! printf '%s' "$OUT" | grep -qF "actual:   $MUTATED"; then
  echo "FAIL: DRIFT did not name the actual mutated routing" >&2
  printf '%s\n' "$OUT" >&2
  exit 1
fi
echo "STEP 3 OK: DRIFT caught — expected $MODEL, actual $MUT_MODEL"

# Step 4 — restore and reconfirm baseline
mv "$BAK" "$AGENTS_MD"
trap - EXIT INT TERM HUP

if ! ./scripts/verify-docs.sh >/dev/null 2>&1; then
  echo "FAIL: baseline did not re-pass after restore — repo state diverged" >&2
  exit 1
fi
echo "STEP 4 OK: baseline re-passes after restore"

# Post-condition (class-wide M2 defense-in-depth per R2 cco-critic-r2-r1):
# sha256 of AGENTS.md must match pre-mutation baseline. Catches any
# normal-exit path where restore was silenced or incomplete.
FINAL_SHA="$(sha256sum "$AGENTS_MD" | awk '{print $1}')"
[[ "$FINAL_SHA" == "$BASELINE_SHA" ]] || { echo "FAIL: $AGENTS_MD content diverged from baseline (post-condition sha mismatch)" >&2; exit 1; }

echo "PASS: verify-docs.sh catches AGENTS.md connector routing drift"
