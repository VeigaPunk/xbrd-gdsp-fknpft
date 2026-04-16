#!/usr/bin/env bash
# M3 — REQUIRED_SECTIONS sentinels must catch heading drift.
# Baseline: `cargo test --lib protocol::tests` passes.
# Mutation: rename one of the contracted ## headings in the SSoT.
# Expected: cargo test must now fail, citing the renamed section.
# Restore and confirm baseline re-passes.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SSOT="$REPO_ROOT/commands/references/xbreed-shared.md"
BAK="$SSOT.m3-bak"
TARGET_HEADING="## xask Gate (4 layers)"
MUTATED_HEADING="## xask Gate (renamed by M3 mutation)"

BASELINE_SHA="$(sha256sum "$SSOT" | awk '{print $1}')"

restore() {
  if [[ -f "$BAK" ]]; then
    mv "$BAK" "$SSOT"
    echo "RESTORED: $SSOT"
  fi
}
trap 'restore; NOW_SHA=$(sha256sum "$SSOT" 2>/dev/null | awk "{print \$1}" || echo MISSING); [[ "$NOW_SHA" == "$BASELINE_SHA" ]] || { echo "FAIL: $SSOT content diverged from baseline (restore trap failed or incomplete; baseline=${BASELINE_SHA:0:16}... now=${NOW_SHA:0:16}...)" >&2; exit 1; }' EXIT INT TERM HUP

cd "$REPO_ROOT"

# Step 1 — baseline must pass
if ! cargo test --lib protocol::tests >/dev/null 2>&1; then
  echo "FAIL: baseline cargo test --lib protocol::tests did not pass" >&2
  exit 1
fi
echo "STEP 1 OK: baseline protocol::tests passed"

# Step 2 — mutate the SSoT (copy aside + in-place sed)
cp "$SSOT" "$BAK"
sed -i "s|^${TARGET_HEADING}\$|${MUTATED_HEADING}|" "$SSOT"

# Confirm mutation actually landed
if grep -qF "${TARGET_HEADING}" "$SSOT"; then
  echo "FAIL: mutation did not rename '${TARGET_HEADING}' in $SSOT" >&2
  exit 1
fi

# Step 3 — cargo test MUST now fail, and failure output must cite the
# renamed heading so the sentinel is self-describing (not generic "assert").
set +e
OUT=$(cargo test --lib protocol::tests 2>&1)
STATUS=$?
set -e

if [[ $STATUS -eq 0 ]]; then
  echo "FAIL: heading rename did not break protocol::tests — sentinel is dead" >&2
  exit 1
fi

if ! printf '%s' "$OUT" | grep -qF "xask Gate (4 layers)"; then
  echo "FAIL: cargo test failed but did not cite the renamed heading" >&2
  printf '%s\n' "$OUT" | tail -20 >&2
  exit 1
fi
echo "STEP 3 OK: mutation caught — test failure cites '${TARGET_HEADING}'"

# Step 4 — restore and reconfirm baseline. `mv` preserves the backup's mtime,
# which is older than the compiled-with-mutation artifact; cargo would skip
# the rebuild and the restored-content test would still fail. `touch` forces
# the mtime forward so cargo sees the source as newer than its cached dep.
mv "$BAK" "$SSOT"
touch "$SSOT"
trap - EXIT INT TERM HUP  # disarm the restore trap now that we've restored

if ! cargo test --lib protocol::tests >/dev/null 2>&1; then
  echo "FAIL: baseline did not re-pass after restore — repo state diverged" >&2
  exit 1
fi
echo "STEP 4 OK: baseline re-passes after restore"

# Post-condition (class-wide M2 defense-in-depth per R2 cco-critic-r2-r1):
# sha256 of SSoT must match pre-mutation baseline. Catches any normal-exit
# path where restore was silenced or incomplete.
FINAL_SHA="$(sha256sum "$SSOT" | awk '{print $1}')"
[[ "$FINAL_SHA" == "$BASELINE_SHA" ]] || { echo "FAIL: $SSOT content diverged from baseline (post-condition sha mismatch)" >&2; exit 1; }

echo "PASS: REQUIRED_SECTIONS sentinels catch heading drift"
