#!/usr/bin/env bash
# Scaffold throwaway Harbor tasks under /tmp for mutation-tester.
# Default: N parallel independent task dirs (never serialize trials).
# Does not touch git worktrees. Does not mutate REPO.
#
# Harbor = laude-institute/harbor (PyPI: harbor, harborframework.com).
# NOT npm @hapic/harbor, NOT medical “harboring mutations” repos.
set -euo pipefail

REPO="${REPO:-$PWD}"
TEST_CMD="${TEST_CMD:-}"
TS="$(date -u +%Y%m%dT%H%M%SZ)"
ROOT="${MUT_HARBOR_ROOT:-/tmp/mut-harbor-$TS}"
N="${N_MUTANTS:-1}"

# Args: --parallel N | -n N | --task-name name (single)
while [[ $# -gt 0 ]]; do
  case "$1" in
    --parallel|-n)
      N="${2:?}"
      shift 2
      ;;
    --task-name)
      MUT_TASK_NAME="${2:?}"
      N=1
      shift 2
      ;;
    -h|--help)
      echo "Usage: $0 [--parallel N] [--task-name name]"
      echo "  REPO=... TEST_CMD=... MUT_HARBOR_ROOT=..."
      exit 0
      ;;
    *)
      echo "unknown arg: $1" >&2
      exit 1
      ;;
  esac
done

if ! [[ "$N" =~ ^[1-9][0-9]*$ ]]; then
  echo "N must be positive integer, got: $N" >&2
  exit 1
fi
if [[ "$N" -gt 32 ]]; then
  echo "capping N=32 (got $N)" >&2
  N=32
fi

if [[ ! -d "$REPO" ]]; then
  echo "REPO not a directory: $REPO" >&2
  exit 1
fi

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

mkdir -p "$ROOT/tasks" "$ROOT/jobs" "$ROOT/.base-snapshot"

# One base snapshot, then clone to each mutant task (cheap parallel prep)
rsync -a --delete \
  --exclude '.git/' \
  --exclude 'target/' \
  --exclude 'node_modules/' \
  --exclude '.xbreed/' \
  --exclude '.venv/' \
  --exclude '**/__pycache__/' \
  --exclude '.DS_Store' \
  "$REPO"/ "$ROOT/.base-snapshot/"

write_task() {
  local TASK_NAME="$1"
  local TASK="$ROOT/tasks/$TASK_NAME"
  mkdir -p "$TASK/environment/workspace" "$TASK/tests"

  rsync -a --delete "$ROOT/.base-snapshot"/ "$TASK/environment/workspace/"

  cat >"$TASK/task.toml" <<EOF
schema_version = "1.3"

[task]
name = "mutation/$TASK_NAME"
description = "Isolated mutation experiment under /tmp via Harbor (parallel wave)."

[metadata]
difficulty = "easy"
category = "mutation"
tags = ["mutation-tester", "xbreed", "parallel"]

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

Isolated Harbor environment. Project snapshot at workspace root.
Apply exactly ONE behavioral mutation, run tests, exit.
No host writes. No git worktrees.
EOF

  cat >"$TASK/environment/Dockerfile" <<'EOF'
FROM ubuntu:24.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    bash ca-certificates curl git build-essential \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /workspace
COPY workspace/ /workspace/
EOF

  cat >"$TASK/tests/test.sh" <<EOF
#!/usr/bin/env bash
set -euo pipefail
cd /workspace 2>/dev/null || cd "\$(dirname "\$0")/../environment/workspace"
set +e
$TEST_CMD
status=\$?
set -e
mkdir -p /logs/verifier 2>/dev/null || true
if [[ \$status -ne 0 ]]; then
  echo 1 | tee reward.txt 2>/dev/null || echo 1
  echo "KILLED (tests failed, status=\$status)"
  exit 0
else
  echo 0 | tee reward.txt 2>/dev/null || echo 0
  echo "SURVIVED (tests passed)"
  exit 0
fi
EOF
  chmod +x "$TASK/tests/test.sh"
}

# Fan-out task dirs (parallel-safe: each has own workspace)
if [[ -n "${MUT_TASK_NAME:-}" && "$N" -eq 1 ]]; then
  write_task "$MUT_TASK_NAME"
else
  for i in $(seq 1 "$N"); do
    write_task "mutant-$i"
  done
fi

# Single-task local runner (first mutant) — kept for smoke
FIRST="$(ls -1 "$ROOT/tasks" | head -1)"
cat >"$ROOT/run-local.sh" <<EOF
#!/usr/bin/env bash
set -euo pipefail
cd "$ROOT/tasks/$FIRST/environment/workspace"
set +e
$TEST_CMD
status=\$?
set -e
if [[ \$status -ne 0 ]]; then echo "RESULT=KILLED task=$FIRST"; exit 0
else echo "RESULT=SURVIVED task=$FIRST"; exit 0; fi
EOF
chmod +x "$ROOT/run-local.sh"

# Parallel local runners — NEVER serialize
cat >"$ROOT/run-all-parallel.sh" <<'EOF'
#!/usr/bin/env bash
# Run every mutant workspace test concurrently. Never serial for-loop wait-per-task.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
TEST_CMD_FILE="$ROOT/test-cmd.txt"
TEST_CMD="$(cat "$TEST_CMD_FILE")"
OUT="$ROOT/jobs/local-parallel"
mkdir -p "$OUT"
pids=()
for task_dir in "$ROOT/tasks"/*; do
  [[ -d "$task_dir" ]] || continue
  name="$(basename "$task_dir")"
  ws="$task_dir/environment/workspace"
  (
    set +e
    cd "$ws" || exit 90
    eval "$TEST_CMD" >"$OUT/$name.stdout" 2>"$OUT/$name.stderr"
    status=$?
    set -e
    if [[ $status -ne 0 ]]; then
      echo "KILLED" >"$OUT/$name.result"
      echo "RESULT=KILLED task=$name status=$status"
    else
      echo "SURVIVED" >"$OUT/$name.result"
      echo "RESULT=SURVIVED task=$name"
    fi
  ) &
  pids+=($!)
done
ec=0
for pid in "${pids[@]}"; do
  wait "$pid" || ec=1
done
echo "PARALLEL_DONE n=${#pids[@]} out=$OUT"
# summary
killed=0; survived=0
for r in "$OUT"/*.result; do
  [[ -f "$r" ]] || continue
  if grep -q KILLED "$r"; then killed=$((killed+1)); else survived=$((survived+1)); fi
done
echo "SUMMARY killed=$killed survived=$survived total=$((killed+survived))"
exit 0
EOF
chmod +x "$ROOT/run-all-parallel.sh"
printf '%s\n' "$TEST_CMD" >"$ROOT/test-cmd.txt"

cat >"$ROOT/README.md" <<EOF
# mut-harbor scaffold (parallel)

- MUT_HARBOR_ROOT=$ROOT
- N_MUTANTS=$N
- Snapshot of: $REPO
- Test command: $TEST_CMD

## Apply mutations (all mutants, same turn / parallel edits)

Edit only under each:
  $ROOT/tasks/mutant-k/environment/workspace/

## Run — ALWAYS parallel

Harbor:
  harbor run -p $ROOT/tasks -o $ROOT/jobs -n $N -y

Local (concurrent background jobs):
  $ROOT/run-all-parallel.sh

NEVER: for t in tasks/*; do harbor run ...; done

## Cleanup

  rm -rf $ROOT
EOF

echo "MUT_HARBOR_ROOT=$ROOT"
echo "N_MUTANTS=$N"
echo "TEST_CMD=$TEST_CMD"
echo "TASKS=$ROOT/tasks"
echo "Harbor: harbor run -p $ROOT/tasks -o $ROOT/jobs -n $N -y"
echo "Local:  $ROOT/run-all-parallel.sh"
