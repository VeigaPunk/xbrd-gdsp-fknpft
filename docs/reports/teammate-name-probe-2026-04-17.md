# Teammate Name Probe — M0 Live Results
date: 2026-04-17
probe-subject: ccs-labrat-m0-probe on team shim-handoff-0417
cc-version: 2.1.112

## Hypothesis
CC teammate-mode exposes teammate name via one of: {env var, tmux pane_title, parent process argv}.

## Method
Four commands run live inside the spawned teammate pane:
1. `env | grep -iE 'agent|teammate|claude_|tmux' | sort`
2. `tmux display-message -p '#{pane_title}'; tmux display-message -p '#{session_name}:#{window_index}.#{pane_index}'`
3. `tr '\0' ' ' < /proc/$$/cmdline`
4. `ps -o pid,ppid,args -p $$ -p $PPID`

## Results

### Probe 1 — env vars
```
CLAUDE_CODE_ENTRYPOINT=cli
CLAUDE_CODE_EXECPATH=/home/vhpnk/.local/share/claude/versions/2.1.112
CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1
PATH=...
SSH_AGENT_PID=912
SSH_AUTH_SOCK=/tmp/ssh-VwEzp87tb4c1/agent.911
TERM_PROGRAM=tmux
TMUX=/tmp/tmux-1000/default,28819,9
TMUX_PANE=%267
```

**Verdict: NEGATIVE.** No `CLAUDE_AGENT_NAME`, `CLAUDE_TEAMMATE_NAME`, or any identity env var present. `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` confirms teams mode is active but carries no identity.

### Probe 2 — tmux pane_title
```
⠂ Review effort-shim design for correctness defects
---
9:1.3
```

**Verdict: NEGATIVE.** Pane title is the current task description (spinner + task subject), NOT the teammate name. Session `9`, window `1`, pane `3`. Silent-degrade characteristic: pane title changes with each task, making it unreliable even if it momentarily contained the name.

### Probe 3 — cmdline of $$ (bash tool wrapper)
```
/bin/bash -c source /home/vhpnk/.claude/shell-snapshots/snapshot-bash-1776435615994-bd4zqz.sh 2>/dev/null || true && shopt -u extglob 2>/dev/null || true && eval 'tr '\''\\0'\'' '\'' '\'' < /proc/$$/cmdline; echo' && pwd -P >| /tmp/claude-7d82-cwd
```

**Verdict: NEGATIVE.** The bash wrapper process (`$$`) is the tool-execution shell. It contains only the eval'd command; no agent name or identity present in this process's cmdline.

### Probe 4 — ps on $$ and $PPID (parent CC process)
```
PID    PPID COMMAND
1315614 1315446 /home/vhpnk/.local/share/claude/versions/2.1.112 --agent-id ccs-labrat-m0-probe@shim-handoff-0417 --agent-name ccs-labrat-m0-probe --team-name shim-handoff-0417 --agent-color blue --parent-session-id 67b576a1-263e-4776-b10b-86d759d4ed75 --agent-type labrat --dangerously-skip-permissions --plugin-dir /home/vhpnk/.claude/personal-plugin --model sonnet
1318445 1315614 /bin/bash -c source ...
```

**Verdict: TIER-3-POSITIVE (certain).** Parent CC process (PPID) argv exposes full identity:
- `--agent-name ccs-labrat-m0-probe`
- `--agent-id ccs-labrat-m0-probe@shim-handoff-0417`
- `--team-name shim-handoff-0417`
- `--agent-type labrat`
- `--model sonnet`
- `--agent-color blue`
- `--parent-session-id 67b576a1-263e-4776-b10b-86d759d4ed75`

Read via: `tr '\0' ' ' < /proc/$PPID/cmdline` or `ps -o args= -p $PPID`

## Summary Table

| Channel | Result | Failure mode |
|---|---|---|
| `CLAUDE_AGENT_NAME` env | **NEGATIVE** | Silent: var simply absent |
| `CLAUDE_TEAMMATE_NAME` env | **NEGATIVE** | Silent: var simply absent |
| tmux pane_title | **NEGATIVE** | Silent-degrade: shows current task text, rotates per task |
| bash `$$` cmdline | **NEGATIVE** | Silent: wrapper shell has no identity |
| parent `$PPID` argv | **TIER-3-POSITIVE** | Observable: always present, stable for session lifetime |

## Shim Design Implication

Env-based identity injection (Tier 1/2 designs) is DOA — CC 2.1.112 does not inject any identity env vars at teammate spawn. The only reliable channel is `$PPID` argv.

Minimal shim read:
```bash
agent_name=$(tr '\0' '\n' < /proc/$PPID/cmdline | grep -A1 '^--agent-name$' | tail -1)
```

Or via `ps`:
```bash
agent_name=$(ps -o args= -p $PPID | grep -oP '(?<=--agent-name )\S+')
```

Both confirmed working in this probe. `$PPID` is the CC process; its cmdline is stable for the full session lifetime (does not rotate like pane title).
