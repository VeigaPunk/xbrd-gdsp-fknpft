#!/usr/bin/env bash
# Scaffold a throwaway Harbor task under /tmp for mutation-tester.
# Copies a snapshot of REPO into the task environment workspace.
# Does not touch git worktrees. Does not mutate REPO.
set -euo pipefail

REPO="${REPO:-$PWD}"
TEST_CMD="${TEST_CMD:-}"  # e.g. "cargo test" / "npm test" — auto-detect if empty
TS="$(date -u +%Y%m%dT%H%M%SZ)"
ROOT="${MUT_HARBOR_ROOT:-/tmp/mut-harbor-$TS}"
TASK_NAME="${MUT_TASK_NAME:-mutant-1}"
TASK="$ROOT/tasks/$TASK_NAME"

if [[ ! -d "$REPO" ]]; then
  echo "REPO not a directory: $REPO" >&2
  exit 1
fi

# Auto-detect a reasonable test command
if [[ -z "$TEST_CMD" ]]; then
  if [[ -f "$REPO/Cargo.toml" ]]; then
    TEST_CMD="cargo test"
  elif [[ -f "$REPO/package.json" ]]; then
    TEST_CMD="npm test --silent"
  elif [[ -f "$REPO/pyproject.toml" ]] || [[ -f "$REPO/pytest.ini" ]]; then
    TEST_CMD="pytest -q"
  elif [[ -f "$REPO/Makefile" ]] && grep -qE '^test:' "$REPO/Makefile"; then
    TEST_CMD="make test"
  else
    TEST_CMD="echo 'set TEST_CMD=...'; exit 1"
  fi
fi

mkdir -p "$TASK/environment/workspace" "$TASK/tests" "$ROOT/jobs"

# Snapshot (read from REPO, write only under /tmp)
rsync -a --delete \
  --exclude '.git/' \
  --exclude 'target/' \
  --exclude 'node_modules/' \
  --exclude '.xbreed/' \
  --exclude '.venv/' \
  --exclude '**/__pycache__/' \
  --exclude '.DS_Store' \
  "$REPO"/ "$TASK/environment/workspace/"

cat >"$TASK/task.toml" <<EOF
schema_version = "1.3"

[task]
name = "mutation/$TASK_NAME"
description = "Isolated mutation experiment under /tmp via Harbor."

[metadata]
difficulty = "easy"
category = "mutation"
tags = ["mutation-tester", "xbreed"]

[agent]
timeout_sec = 600.0

[verifier]
timeout_sec = 600.0

[environment]
network_mode = "no-network"
build_timeout_sec = 600.0
cpus = 2
memory_mb = 4096
storage_mb = 20480
gpus = 0
EOF

cat >"$TASK/instruction.md" <<EOF
# Mutation experiment ($TASK_NAME)

You are inside an isolated Harbor environment. Project snapshot is at
\`/workspace\` (or the task environment workspace root).

Apply exactly ONE behavioral mutation, run the test suite, and exit.
Do not attempt to modify the host machine or use git worktrees.
EOF

# Generic image with common build tools; project tools install as needed
cat >"$TASK/environment/Dockerfile" <<'EOF'
FROM ubuntu:24.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    bash ca-certificates curl git build-essential \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /workspace
COPY workspace/ /workspace/
EOF

# Verifier: re-run project tests; reward 1 if tests fail (mutant KILLED), 0 if pass (SURVIVED)
# Convention for mutation-tester: KILLED = tests failed after mutation = good for suite
cat >"$TASK/tests/test.sh" <<EOF
#!/usr/bin/env bash
set -euo pipefail
cd /workspace 2>/dev/null || cd "\$(dirname "\$0")/../environment/workspace"
set +e
$TEST_CMD
status=\$?
set -e
# Harbor reward: write 1 if mutant killed (tests failed), 0 if survived (tests passed)
mkdir -p /logs/verifier 2>/dev/null || true
if [[ \$status -ne 0 ]]; then
  echo 1 | tee reward.txt 2>/dev/null || echo 1
  echo "KILLED (tests failed, status=\$status)"
  exit 0
else
  echo 0 | tee reward.txt 2>/dev/null || echo 0
  echo "SURVIVED (tests passed)"
  # exit 0 so harbor still collects reward; interpretation is in reward.txt
  exit 0
fi
EOF
chmod +x "$TASK/tests/test.sh"

# Host-side quick runner without full harbor (optional)
cat >"$ROOT/run-local.sh" <<EOF
#!/usr/bin/env bash
# Run tests against the snapshot without Docker (still isolated under /tmp).
set -euo pipefail
cd "$TASK/environment/workspace"
set +e
$TEST_CMD
status=\$?
set -e
if [[ \$status -ne 0 ]]; then echo "RESULT=KILLED"; exit 0; else echo "RESULT=SURVIVED"; exit 0; fi
EOF
chmod +x "$ROOT/run-local.sh"

cat >"$ROOT/README.md" <<EOF
# mut-harbor scaffold

- MUT_HARBOR_ROOT=$ROOT
- Task: $TASK
- Snapshot of: $REPO
- Test command: $TEST_CMD

## Apply a mutation

Edit files only under:
  $TASK/environment/workspace/

## Run

Harbor (Docker):
  harbor run -p $ROOT/tasks -o $ROOT/jobs -n 1 -y

Local fallback (still /tmp, no worktree):
  $ROOT/run-local.sh

## Cleanup

  rm -rf $ROOT
EOF

echo "MUT_HARBOR_ROOT=$ROOT"
echo "MUT_TASK=$TASK"
echo "WORKSPACE=$TASK/environment/workspace"
echo "TEST_CMD=$TEST_CMD"
echo "Harbor: harbor run -p $ROOT/tasks -o $ROOT/jobs -n 1 -y"
echo "Local:  $ROOT/run-local.sh"
