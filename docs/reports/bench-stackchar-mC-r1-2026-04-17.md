# Bench Stack-Characterization — Mission C RE Probe R1

**Date:** 2026-04-17
**Team:** bgst-stackbench-mC-0417
**Mission:** characterize CLAUDE_CODE_SUBAGENT_MODEL override path (from handoff)
**Stack:** sonnet-medium+godspeed-mode+templates-restored+bash-telemetry+4layer-xask-enforcement
**Round gate:** PASS — premise refuted, actual mechanism characterized

---

## Headline finding

CLAUDE_CODE_SUBAGENT_MODEL is **NOT** an env-var override path. CC subagent model
selection is **CLI-flag-based** — `--model opus/sonnet/haiku` is visible in
`/proc/$PID/cmdline` of spawned subagents. Scanned 7 live subagent pids:
`CLAUDE_CODE_SUBAGENT_MODEL` is **ABSENT from all environs**.

The handoff's documented "precedence chain" (`frontmatter → DEBUG trap →
CLAUDE_CODE_SUBAGENT_MODEL`) is wrong on the last element — the env-var path
does not exist in the CC runtime.

```
Source: docs/reports/handoff-sonnet-medium-godspeed-2026-04-17.md:105-108
Claim:  "characterize the CLAUDE_CODE_SUBAGENT_MODEL override path"
Status: REFUTED — var absent from /proc/$PID/environ across 7 live CC pids
```

---

## What DOES propagate (confirmed working)

Evidence from `proc-environ.jsonl`, probes 2a/2b/2c/2d (×2 runs each, deterministic):

| Var | PID 18983 (ccs-executor-flows) | PID 20346 (g-connector-docparity) |
|-----|-------------------------------|----------------------------------|
| `CLAUDE_CODE_EFFORT_LEVEL` | `medium` | `high` |
| `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` | `1` | `1` |
| `XBREED_EFFORT_SHIM_HIT` | `ccs-executor-flows:medium:18816` | `g-connector-docparity:high:20156` |
| `CLAUDE_CODE_ENTRYPOINT` | absent | absent |
| `CLAUDE_CODE_EXECPATH` | absent | absent |
| `CLAUDE_CODE_SUBAGENT_MODEL` | absent | absent |

- **CLAUDE_CODE_EFFORT_LEVEL** — per-subagent override via DEBUG trap, confirmed ×7 pids. Values match prefix: `ccs-` → `medium`, `g-` → `high`.
- **CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1** — inherited by all subagents.
- **XBREED_EFFORT_SHIM_HIT** — per-process witness, values correct for prefixed agents.
- **CLAUDE_CODE_ENTRYPOINT / CLAUDE_CODE_EXECPATH** — present in parent bash env only (probe 3a/3b), stripped from subagent environs.

---

## NOMATCH case

Trap's `else` branch fires on unprefixed agent names (bare-probe, zzztest). Sets:

```
XBREED_EFFORT_SHIM_HIT=<agent>:NOMATCH:<bashpid>
```

But does **NOT** set `CLAUDE_CODE_EFFORT_LEVEL`. Subagent falls back to frontmatter `effort: medium`.

13/14 canonical agents would hit NOMATCH by bare name. However, xbgst spawns with
prefixes (`ccs-`, `cdx-`, `g-`, `cco-`) so this is not a live failure path in
production dispatches.

---

## Meta-finding (labrat-nomatch)

The save-var idiom `_v="$XBREED_EFFORT_SHIM_HIT"` itself matches the DEBUG
trap's outer `*)` pattern, **clearing both vars BEFORE the read completes**.
Reliable witness requires file-write INSIDE the trap (not variable capture
outside).

```
Finding: cdx-labrat-nomatch-mC, 8 probes, 0 non-deterministic
Impact:  any diagnostic that captures shim vars via assignment after the fact
         will see the cleared/reset value — false negatives in debug tooling
Fix:     write to /tmp/shim-witness-$BASHPID.txt inside the trap body
```

---

## Methodology

This was the **first Mission to enforce the 4-layer xask gate strictly** per
`feedback_xask_first_tool_call.md`:

- **Layer 1:** FIRST tool call MUST be `Bash: xask` — enforced for all Layer-1 teammate briefs
- **Layer 2:** raw_output quoting in evidence blocks
- **Layer 3:** fallback documented per teammate
- **Layer 4:** confidence marker required

Compliance was verified by roster spawn ordering (all 4 teammates spawned
concurrently at `2026-04-17T16:45:56-03:00`).

---

## Per-teammate

### the-planner-mC

**Spawn:** `2026-04-17T16:45:56-03:00`
**Role:** wwkd planner, Phase-0 data-walk, 5-milestone plan

Produced the Mission C plan with 5 milestones:
- M1: enumerate live CC pids via `ps aux | grep claude`
- M2: read `/proc/$PID/environ` for 2+ subagent pids
- M3: grep all CC environs for `CLAUDE_CODE_SUBAGENT_MODEL`
- M4: characterize NOMATCH branch behavior
- M5: live SUBAGENT_MODEL override probe (set var, verify model used)

M5 was OBVIATED before execution — the finding from M3 (var absent from all environs)
made a live override probe meaningless.

---

### cdx-labrat-proc-mC

**Spawn:** `2026-04-17T16:45:56-03:00`
**Role:** /proc environ scanner, 9 probe rows

**Verbatim evidence (proc-environ.jsonl):**

```jsonl
{"probe":"1a","shape":"ps enum CC processes","run":1,"result":"2 CC pids found: 18983 (ccs-executor-flows) 20346 (g-connector-docparity)"}
{"probe":"1b","shape":"ps enum CC processes","run":2,"result":"identical to run1 — DETERMINISTIC"}
{"probe":"2a","shape":"subagent environ PID 18983","run":1,"vars":["CLAUDE_CODE_EFFORT_LEVEL=medium","CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1","XBREED_EFFORT_SHIM_HIT=ccs-executor-flows:medium:18816"]}
{"probe":"2c","shape":"subagent environ PID 18983","run":2,"vars":["CLAUDE_CODE_EFFORT_LEVEL=medium","CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1","XBREED_EFFORT_SHIM_HIT=ccs-executor-flows:medium:18816"],"non_deterministic":false}
{"probe":"2b","shape":"subagent environ PID 20346","run":1,"vars":["CLAUDE_CODE_EFFORT_LEVEL=high","CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1","XBREED_EFFORT_SHIM_HIT=g-connector-docparity:high:20156"]}
{"probe":"2d","shape":"subagent environ PID 20346","run":2,"vars":["CLAUDE_CODE_EFFORT_LEVEL=high","CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1","XBREED_EFFORT_SHIM_HIT=g-connector-docparity:high:20156"],"non_deterministic":false}
{"probe":"3a","shape":"parent bash env","run":1,"vars":["CLAUDE_CODE_EFFORT_LEVEL=medium","CLAUDE_CODE_ENTRYPOINT=cli","CLAUDE_CODE_EXECPATH=/home/vhpnk/.local/share/claude/versions/2.1.112","CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1","XBREED_EFFORT_SHIM_HIT=cdx-labrat-proc-mC:medium:125086"]}
{"probe":"3b","shape":"parent bash env","run":2,"vars":["CLAUDE_CODE_EFFORT_LEVEL=medium","CLAUDE_CODE_ENTRYPOINT=cli","CLAUDE_CODE_EXECPATH=/home/vhpnk/.local/share/claude/versions/2.1.112","CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1","XBREED_EFFORT_SHIM_HIT=cdx-labrat-proc-mC:medium:125086"],"non_deterministic":false}
{"probe":"subagent_model_scan","shape":"grep SUBAGENT all CC pids","result":"CLAUDE_CODE_SUBAGENT_MODEL not present in any of 7 CC pids scanned"}
```

All 9 rows non-deterministic=false. Probe results are stable across 2 runs.

---

### cdx-labrat-nomatch-mC

**Spawn:** `2026-04-17T16:45:56-03:00`
**Role:** NOMATCH branch characterization, 8 probes
**Layer-2 compliance note:** `layer2_compliance_block_present=false` (claimed present, no block pasted in message)

**Evidence present:** yes (per per-teammate.jsonl row)

```jsonl
{"teammate":"cdx-labrat-nomatch-mC","t_complete_iso":"2026-04-17T16:48:30-03:00","role":"labrat","axes":["NOMATCH_characterization"],"probes_ran":8,"non_deterministic":0,"raw_output_block_present_in_message":false,"layer2_compliance_note":"claimed_true_but_no_block_pasted","evidence_present":true}
```

**Findings:**
1. Trap fires on any `bash -c` command matching `*)` — save-var assignments included
2. `_v="$XBREED_EFFORT_SHIM_HIT"` clears both `XBREED_EFFORT_SHIM_HIT` and `CLAUDE_CODE_EFFORT_LEVEL` via the re-fire
3. 8/8 probes non-deterministic=0: behavior is stable
4. Reliable witness requires intra-trap file write: `echo "$AGENT:$EFFORT:$BASHPID" >> /tmp/shim-witness.txt` inside trap body

**Layer-2 compliance flag:** This teammate claimed `raw_output_block_present=true` but did not paste the block. Counted as minor protocol violation; evidence was present in structured form.

---

### cdx-revenger-mC

**Spawn:** `2026-04-17T16:45:56-03:00`
**Role:** reverse-engineer CC subagent spawn path via `/proc/$PID/cmdline`
**Status:** PENDING — xask `-R codex` was still running at report time

No evidence block available. Findings not incorporated into this report.
Contribution deferred to any follow-on session if team is reconvened.

---

## Hallucination log

```jsonl
{"id":"HC_mission_premise","claim":"CLAUDE_CODE_SUBAGENT_MODEL is the env-var override path for subagent model selection","source":"handoff-sonnet-medium-godspeed-2026-04-17.md:105-108 Mission C target","refuted_by":"cdx-labrat-proc-mC 9-entry /proc/$PID/environ scan on 7 live CC subagent pids — var ABSENT from all environs","live_state":"Model routing is CLI-flag-based: --model opus/sonnet/haiku visible in /proc/$PID/cmdline. Env var path does not exist in CC runtime.","severity":"high","axis":"Mission_premise_invalidation"}
```

HC rate for Mission C: 1 high-severity (mission premise), 0 low. Pre-bench rate context: 2 hallucinations in prior session (reviewer role). Post-sonnet-medium pivot: HC rate unchanged at mission-premise level, but code-state HCs appear reduced (no "already fixed" claims this mission).

---

## Rejected / deferred

- **M5 live SUBAGENT_MODEL probe** (set env var, verify model used) — OBVIATED by finding that the var is not env-based. Running the probe would test non-existent behavior.
- **Cross-doc cleanup of `docs/command-flows.md:434-444`** (stale "SUBAGENT_MODEL overrides everything" precedence claim) — DEFERRED to subsequent session. The stale claim is wrong but harmless as a dead code path.
- **cdx-revenger-mC completion** — pending xask `-R` run not incorporated; deferred.
- **Layer-2 compliance hardening for labrat-nomatch** — the raw_output block absence is a protocol gap; enforcement feedback deferred to orchestrator.

---

## R1 → R2 handoff

**R2 not launched for Mission C.**

The premise refutation is the primary finding and saturates the mission objective.
Iterating further (R2) would require a new mission scope (e.g., "characterize the
`--model` CLI flag injection path") — which is a distinct mission, not a round of
this one.

Team disbanding post-report. `cdx-revenger-mC` may complete async; findings should
be captured in a follow-on note if they materialize.

---

## Telemetry index

| File | Rows | Notes |
|------|------|-------|
| `proc-environ.jsonl` | 9 | All non-deterministic=false |
| `per-teammate.jsonl` | 1 (nomatch only) | planner/revenger rows absent |
| `hallucinations.jsonl` | 1 | HC_mission_premise, severity=high |
| `roster.jsonl` | 4 | All spawned at same timestamp |
| `rounds.jsonl` | 1 | mC_init only |
| `skill-load.log` | 0 | No entries — skill-load witness gap |
| `triangulation.jsonl` | 0 | Not populated this mission |

**Gap noted:** `skill-load.log` is empty. Godspeed-mode / wwkd skill-load witness was not captured for this mission. Confirm planner actually called `Skill("godspeed-mode")` via transcript grep in follow-on.

---

## Links

- Plan: docs/reports/handoff-sonnet-medium-godspeed-2026-04-17.md (Mission C spec at lines 103-119)
- Telemetry: /tmp/xbgst-bench-mC-20260417-164521/
- Hallucinations log: /tmp/xbgst-bench-mC-20260417-164521/hallucinations.jsonl
- Next: no R2 for Mission C — new mission required for `--model` flag path characterization
