# Plan — Teammate Benchmark (Phase C → A; Phase B deferred handoff)

**Spec:** user-provided, 2026-04-17 — "testing on teammates using telemetry/debug/monitor for latency, quality, write speed (token/s), tool use, total tokens, wall time; same task, hard complexity"
**Author:** wwkd posture, 2026-04-17
**Dispatcher:** `/xbgst` at godspeed, after this plan is approved.

---

## Data Walk (evidence before plan)

### What I inspected on disk

| Target | Finding |
|---|---|
| `~/.claude/projects/-home-vhpnk-projects-xbrd-gdsp-fknpft/*.jsonl` | Top-level session jsonls. Current planning session: `2c792a48-10ee-425c-93be-7c1242677cdb`. |
| Teammate transcript location | **Top-level jsonl per teammate**, NOT `subagents/agent-*.jsonl`. Teammates run as full CC sessions via `teammateMode: tmux`, not via the `Agent` tool. |
| Teammate identity in jsonl | First user msg is `<teammate-message teammate_id="team-lead">\nYou are \`<teammate-name>\` on team \`<team-name>\`, ...` — deterministic handle. |
| jsonl schema (assistant msg) | Top-level: `parentUuid, isSidechain, message, timestamp, sessionId, gitBranch`. `message.model` (e.g. `claude-sonnet-4-6`, `claude-opus-4-7`). `message.usage.{input_tokens, output_tokens, cache_read_input_tokens, cache_creation_input_tokens}`. `message.content[].{type: tool_use, name}` for tool counts. |
| Live extraction proof | Ran extraction on `022fb3b3-380a-47bf-bdff-e82124948007.jsonl` (R1 teammate `ccs-reviewer-xbreed-shared`) → model=sonnet-4-6, 34 assistant msgs, 42,718 out tokens, wall 2026-04-17T10:11:06 → 10:15:54 (288s), tok/s = 148, tools = {SendMessage:6, Bash:2, Read:2, ToolSearch:2, Glob:1, TaskUpdate:1, Grep:1, Edit:1}. **All six requested metrics fall out of one script.** |
| `~/.claude/teams/` + `~/.claude/tasks/` | **Empty.** No stale teams. TeamCreate is unblocked. |
| `xbreed --version` → `xbreed precheck pane-cap --team-size 9` | **v0.4.0 installed; `precheck` subcommand not recognized.** Source has it (R2 shipped), installed binary does not. Classic stale-shadow per `feedback_recompile_on_change`. **Blocker for M0.** |
| OTEL envs | `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE=delta` set; `CLAUDE_CODE_ENABLE_TELEMETRY` **not** set. OTEL pipeline inert. Don't plan around it. |
| Current session effort | `effortLevel: xhigh` in `~/.claude/settings.json`; `CLAUDE_CODE_EFFORT_LEVEL` env unset. |

### Spec/reality reconciliations

1. **"7 effort levels in one team"** → **cannot run as described** per `feedback_teammate_mode_effort_caveat.md`. Phase C reframed: spawn 7 teammates with distinct `effort:` frontmatter values precisely so the benchmark **proves the noop empirically** (negative-result artifact). Phase A drops the effort axis.
2. **"telemetry/debug/monitor"** → CC `Monitor` tool watches processes, not tokens. OTEL pipeline inert. Real measurement surface = jsonl transcripts. Planning from transcripts only.
3. **Quality metric** → no mechanical instrument. Scored out-of-band by advisor-as-judge with a 4-axis rubric, dispatched post-hoc per phase.

---

## Milestones

### M0 — Rebuild + install xbreed (precondition)
**Does:** rebuild the xbreed binary from current HEAD so `precheck` subcommand is available; install to `~/.local/bin/`.
**Gate:**
```bash
cargo build --release && \
  cp target/release/xbreed ~/.local/bin/xbreed && \
  xbreed precheck pane-cap --team-size 9
```
→ exit 0, prints a Pass/Fail verdict (not "unrecognized subcommand").
**Touches:** (build artifact only; no src changes)
**Out-of-scope:** any src change to precheck; if the formula needs adjustment, that's a separate mission.
**Risk if skipped:** TeamCreate(team_size=7) can hit "no space for new pane" mid-dispatch, wasting a round.

### M1 — Skeleton: metrics extraction on ONE known transcript
**Does:** write `scripts/bench-metrics.py` — one file, <80 lines — that takes a jsonl path and emits a single TSV row:
```
team	teammate	model	wall_s	out_tokens	tok_per_s	tool_count	tool_breakdown	input_tokens	cache_read_tokens	assistant_msgs
```
Also write `scripts/bench-collect.sh` — walks a list of jsonl paths, concatenates TSV rows.
**Overfit target:** run against the proven jsonl `022fb3b3-380a-47bf-bdff-e82124948007.jsonl` from the data walk.
**Gate:**
```bash
python3 scripts/bench-metrics.py ~/.claude/projects/-home-vhpnk-projects-xbrd-gdsp-fknpft/022fb3b3-380a-47bf-bdff-e82124948007.jsonl
```
→ emits a row where `model=claude-sonnet-4-6 AND out_tokens=42718 AND 280 < wall_s < 300 AND tok_per_s > 140` (values verified in data walk). No exceptions. No empty fields.
**Touches:** `scripts/bench-metrics.py` (new), `scripts/bench-collect.sh` (new).
**Out-of-scope:** quality scoring (separate milestone), pretty output (M6 polish), histograms or plots.

### M2 — Overfit: one freshly-spawned teammate, metrics match
**Does:** spawn ONE throwaway teammate (`ccs-probe-smoke`, sonnet, session-default effort) with a stub task ("Reply SYNTHESIS_READY with one line of analysis"). After it completes, locate its jsonl via `<teammate-message>` header match, run M1 script, confirm TSV row is sensible.
**Gate:**
1. Teammate sends SYNTHESIS_READY within 120s.
2. Jsonl discoverable by `grep -l 'teammate_id=.*ccs-probe-smoke' ~/.claude/projects/-home-vhpnk-projects-xbrd-gdsp-fknpft/*.jsonl`.
3. M1 script on that jsonl emits row with `model=claude-sonnet-4-6 AND out_tokens > 0 AND tool_count >= 1` (at minimum the SendMessage for SYNTHESIS_READY).

**Why this is its own milestone:** if the locator can't find the teammate's jsonl, or if the parse breaks on a fresh transcript structure (new field, different shape), no amount of scaling to 7 teammates will save it. This IS the capacity smoke test.
**Touches:** team `bench-smoke-0417` (created + TeamDelete'd within this milestone), no src.
**Out-of-scope:** quality scoring, multiple teammates, effort variation.

### M3 — Phase C: No-op probe (7 teammates, distinct `effort:` frontmatter)
**Does:** spawn 7 teammates in parallel on team `bench-noop-0417` with the shared task. Wait for all SYNTHESIS_READY. Collect 7 jsonls. Run M1 → TSV with 7 rows. Compare within-model variance across effort values.

**Dispatch table:**
| # | Name | Template | Model | `effort:` frontmatter (noop, for forensics) | subagent_type |
|---|------|----------|-------|---|---|
| 1 | `ccs-probe-sonnet-low` | `labrat` | claude-sonnet-4-6 | low | labrat |
| 2 | `ccs-probe-sonnet-med` | `labrat` | claude-sonnet-4-6 | medium | labrat |
| 3 | `ccs-probe-sonnet-high` | `labrat` | claude-sonnet-4-6 | high | labrat |
| 4 | `cco-probe-opus-low` | `labrat` | claude-opus-4-7 | low | labrat |
| 5 | `cco-probe-opus-med` | `labrat` | claude-opus-4-7 | medium | labrat |
| 6 | `cco-probe-opus-high` | `labrat` | claude-opus-4-7 | high | labrat |
| 7 | `cco-probe-opus-xhigh` | `labrat` | claude-opus-4-7 | xhigh | labrat |

**Shared task (identical brief to all 7):**
> Design a per-teammate effort propagation mechanism for xbreed that circumvents the teammate-mode noop documented in `feedback_teammate_mode_effort_caveat.md` and R3 known-gap #3 (`docs/reports/xbreed-harness-r3-2026-04-17.md`). Output MUST include: (a) design sketch (code or pseudocode), (b) explicit failure modes, (c) implementation-cost estimate (files touched / LoC / new tests). Reply via one SendMessage to `team-lead` with subject `PROPOSAL` carrying all three. Under 800 words. Reply again with subject `SYNTHESIS_READY` once done.

**Gate:**
- `xbreed precheck pane-cap --team-size 7` → Pass (from M0).
- All 7 teammates send SYNTHESIS_READY within 8 minutes (godspeed timeout).
- `scripts/bench-collect.sh` on the 7 jsonls → 7 clean TSV rows.
- Within-model effort variance statement in report: if sonnet{low,med,high} wall-times / tok/s are within ±20% of each other, memory claim confirmed. If not, memory claim challenged — flag to user before Phase A.

**Touches:** team `bench-noop-0417` (created, used, TeamDelete'd at milestone end); `data/bench-phase-c.tsv` (output artifact).
**Out-of-scope:** advisor-as-judge (M5), report writing (M6).

### M4 — Phase A: Model benchmark (godspeed parallel sonnet + opus at session xhigh)
**Does:** spawn 6 teammates (3× sonnet + 3× opus) on team `bench-model-0417` for statistical power on the model axis. Session default xhigh applies uniformly. Same task as M3.

**Dispatch table:**
| # | Name | Template | Model |
|---|------|----------|-------|
| 1 | `ccs-bench-sonnet-a` | `labrat` | claude-sonnet-4-6 |
| 2 | `ccs-bench-sonnet-b` | `labrat` | claude-sonnet-4-6 |
| 3 | `ccs-bench-sonnet-c` | `labrat` | claude-sonnet-4-6 |
| 4 | `cco-bench-opus-a` | `labrat` | claude-opus-4-7 |
| 5 | `cco-bench-opus-b` | `labrat` | claude-opus-4-7 |
| 6 | `cco-bench-opus-c` | `labrat` | claude-opus-4-7 |

**Gate:**
- `xbreed precheck pane-cap --team-size 6` → Pass.
- All 6 SYNTHESIS_READY within 8 minutes.
- `data/bench-phase-a.tsv` has 6 rows.

**Touches:** team `bench-model-0417`; `data/bench-phase-a.tsv`.
**Out-of-scope:** advisor scoring (M5), report (M6).

### M5 — Quality scoring (advisor-as-judge, one pass per phase)
**Does:** dispatch `cco-judge-bench-quality` (opus, xhigh) as a subagent (NOT a teammate — avoids polluting benchmark data). Gives it all 13 design-doc outputs extracted from the 13 jsonls' SendMessage PROPOSAL payloads. Scores each on 4 axes using the rubric below, returns JSON.

**Rubric (4 axes, 1-5 scale, with anchors):**
- **Soundness:** does the proposed mechanism actually work given xbreed's tmux+subprocess architecture and the CC CLI's surface? (1 = fundamentally broken; 5 = works as designed on primary-source evidence)
- **Completeness:** edge cases, failure modes, rollback path? (1 = missing all three; 5 = all three explicit)
- **Implementability:** churn cost, dependencies, test plan? (1 = no estimate; 5 = LoC estimate + test plan + file list)
- **Reasoning depth:** surfaces assumptions, alternatives considered, constraints named? (1 = one path, no alternatives; 5 = ≥2 alternatives with tradeoffs)

**Gate:** `data/bench-quality.json` exists with 13 objects: `{teammate, soundness, completeness, implementability, reasoning_depth, total, notes}`.
**Touches:** `data/bench-quality.json` (output).
**Out-of-scope:** integrating quality into a single report (M6).

### M6 — Reports + commits (per-phase scribe, per `feedback_scribe_per_round`)
**Does:** produce four markdown artifacts, commit in three commits (one per phase, one final summary).

| Artifact | Contents | Commit |
|---|---|---|
| `docs/reports/teammate-benchmark-phase-c-2026-04-17.md` | Phase C: 7-row TSV, within-model variance analysis, noop-confirmed/refuted verdict | `feat(bench): Phase C no-op probe — 7 teammates × effort frontmatter` |
| `docs/reports/teammate-benchmark-phase-a-2026-04-17.md` | Phase A: 6-row TSV, model-axis comparison (sonnet vs opus at xhigh across all 6 metrics) | `feat(bench): Phase A godspeed model benchmark — sonnet vs opus` |
| `docs/reports/teammate-benchmark-phase-b-handoff-2026-04-17.md` | **DEFERRED** Phase B spec: serial restart protocol, `CLAUDE_CODE_EFFORT_LEVEL={low,med,high,xhigh}`, commands to run per session, side-benefit of closing R3 known-gap #3 | (bundled with summary) |
| `docs/reports/teammate-benchmark-summary-2026-04-17.md` | Cross-phase summary: per-axis winner, tradeoff notes, cost summary (tokens spent on benchmark), link to deferred-B | `docs(bench): cross-phase summary + deferred Phase B handoff` |

**Gate:** `ls docs/reports/teammate-benchmark-*.md` → 4 files. `git log --oneline -3 | head -3` → 3 benchmark commits. `ls ~/.claude/teams/ ~/.claude/tasks/` → both empty (no dangling state).
**Touches:** `docs/reports/teammate-benchmark-*.md` (4 new), 3 commits on `main`.
**Out-of-scope:** merging to a branch; running Phase B (explicitly deferred).

---

## Structural verification — per-phase go/no-go gates

| Between | Required evidence before proceeding |
|---|---|
| M0 → M1 | `xbreed precheck pane-cap --team-size 9` exits 0 with Pass/Fail verdict. |
| M1 → M2 | TSV row for `022fb3b3` matches data-walk values (model, out_tokens, wall_s range, tok/s). |
| M2 → M3 | Fresh-teammate jsonl locator works by teammate name; M1 script parses without error on fresh transcript. |
| M3 → M4 | 7 TSV rows present; **within-model effort variance printed to stdout**. If variance contradicts noop-memory claim, **halt**, surface to user, do not proceed to M4 blind. |
| M4 → M5 | 6 TSV rows present; teams TeamDelete'd. |
| M5 → M6 | `data/bench-quality.json` has 13 objects, all 4 axes populated, totals computed. |
| M6 → done | 4 reports + 3 commits + empty teams/tasks dirs. |

---

## Teammate dispatch mechanics (operational)

**Pane-cap precheck (M0 must have installed this):**
```bash
xbreed precheck pane-cap --team-size <N>  # run BEFORE every TeamCreate
```

**TeamCreate + teammate spawn:** use the xbreed-shared teammate-mode protocol. Each teammate brief embeds:
```
<teammate-message teammate_id="team-lead">
You are `<name>` on team `<team>`, axis bench/solo.
Your task: <the shared hard task from M3>
Reply by SendMessage to team-lead with subject PROPOSAL carrying the design.
Reply again with subject SYNTHESIS_READY when done.
DO NOT coordinate with peers. DO NOT read other teammates' inboxes. Work solo.
```

**Jsonl locator (M2+):**
```bash
grep -l "teammate_id=\"team-lead\"" ~/.claude/projects/-home-vhpnk-projects-xbrd-gdsp-fknpft/*.jsonl \
  | xargs -I{} sh -c 'grep -q "ccs-bench-sonnet-a" {} && echo {}' \
  | head -1
```
(substitute teammate name; first match wins).

**Proposal extraction (for M5 quality scoring):**
```bash
python3 -c "
import json,sys
path=sys.argv[1]
with open(path) as f:
    for l in f:
        d=json.loads(l)
        m=d.get('message',{})
        for b in m.get('content',[]) or []:
            if isinstance(b,dict) and b.get('type')=='tool_use' and b.get('name')=='SendMessage':
                inp=b.get('input',{})
                if 'PROPOSAL' in str(inp.get('message','')):
                    print(inp.get('message'))
                    sys.exit(0)
" <jsonl-path>
```

---

## Metrics formulas (one-file Python in M1)

```python
# scripts/bench-metrics.py
# Inputs: path to teammate jsonl. Output: one TSV row.
# Metrics:
#   wall_s = (last_assistant_ts - first_assistant_ts) in seconds
#   out_tokens = sum(message.usage.output_tokens) over assistant msgs
#   tok_per_s = out_tokens / wall_s
#   tool_count = count of content blocks with type=tool_use
#   tool_breakdown = dict of tool_name -> count
#   input_tokens = sum(message.usage.input_tokens)
#   cache_read_tokens = sum(message.usage.cache_read_input_tokens)
#   assistant_msgs = count of type=assistant entries
#   team, teammate = parsed from first '<teammate-message>' user msg
#   model = message.model (verify single value across all assistant msgs; fail loud if mixed)
```

---

## Commit plan

| Commit | Scope | Trigger |
|---|---|---|
| `feat(bench): rebuild + metrics scripts` | M0+M1 (M0 touches no src but unblocks everything; bundle with the new scripts/) | After M1 gate passes |
| `feat(bench): Phase C no-op probe — 7 teammates × effort frontmatter` | M3 TSV + Phase C report | After M3+M5-partial for Phase C |
| `feat(bench): Phase A godspeed model benchmark — sonnet vs opus` | M4 TSV + Phase A report | After M4+M5-partial for Phase A |
| `docs(bench): cross-phase summary + deferred Phase B handoff` | summary + handoff doc | After M6 |

Per `feedback_commit_per_round`, commit cadence matches phase boundaries.

---

## Deferred — Phase B handoff doc outline (M6 writes this, we DON'T execute it)

`docs/reports/teammate-benchmark-phase-b-handoff-2026-04-17.md`:
1. **Scope:** 4 serial CC sessions, each started fresh with `CLAUDE_CODE_EFFORT_LEVEL={low|medium|high|xhigh} claude`. Each session spawns the Phase A team (3 sonnet + 3 opus).
2. **Pre-session checklist:** confirm env var present (`env | grep EFFORT`), confirm session.jsonl created cleanly, confirm no stale teams.
3. **Per-session recipe:** TeamCreate → spawn 6 → collect 6 jsonls → run `scripts/bench-collect.sh`. Session jsonl naming captures env at start.
4. **Cross-session join:** 4 sessions × 6 teammates = 24 TSV rows. Join by `(model, effort_env)` to produce 2×4 = 8 cells.
5. **R3 known-gap #3 closure criterion:** if per-effort variance within a model is >30%, env is honored → R3 gap closed. If variance remains ±20%, env is ignored in teammate-mode too → escalate to CC docs/reply bug report.
6. **Expected runtime:** 4 × (startup + 8min bench + cleanup) ≈ 60-90 minutes total wall time. Not godspeed.
7. **Commit plan:** 1 commit per session + 1 summary.

---

## Out-of-scope (explicit)

- Building a benchmark framework / CLI. Stick to bash + one python script.
- Plots, charts, html dashboards. TSV + markdown tables only.
- Running Phase B this session. Deferred explicitly.
- Measuring advisor/judge latency. Only teammates are under test.
- Cross-model delegation (xask gemini/codex). Claude-only mission per spec.
- Policy hardening based on findings (`feedback_no_policy_hardening`).

---

## Known risks + mitigations

| Risk | Mitigation |
|---|---|
| Teammate hangs past 8-min godspeed window | M3/M4 gate has explicit timeout; non-reporting teammates counted as "hung" in TSV with wall_s=NULL; report surfaces. |
| Jsonl locator finds wrong session (name collision with past R1 teammate) | Locator also filters by file mtime > benchmark start time. |
| One teammate pane fails to spawn (precheck false-negative) | M0 gate uses team-size 9 to leave slack; if precheck says Pass for 9, the 7-team dispatch has headroom. |
| Advisor quality scores drift between Phase C and Phase A passes | Single judge (`cco-judge-bench-quality`) scores all 13 in one batch — consistent rubric application. |
| Teammate task output format drifts — SendMessage PROPOSAL subject absent | Task brief explicitly pins the subject; M5 extraction falls back to "last SendMessage to team-lead" if subject filter returns empty. |
| Stale `~/.local/bin/xbreed` reappears mid-mission (if src changes and we forget to reinstall) | We do not modify src during this mission. No rebuild needed between M0 and M6. |

---

## Invention risk flag

Per wwkd principle 7: novel architecture is where debugging time hides. This plan has minimal invention:
- **Reference implementations:** R2 `xbreed precheck pane-cap` (already shipped); R1/R2/R3 xbgst dispatch protocol; existing teammate brief format (recovered from disk in R0).
- **New code:** `scripts/bench-metrics.py` (~80 lines) + `scripts/bench-collect.sh` (~20 lines). Single-file, single-screen each.
- **Invention step:** the `<teammate-message>`-as-locator trick. Verified on one live jsonl (`022fb3b3`). Flagged as M2 gate: if fresh transcripts don't carry the header, M2 fails loud — detected before scaling.

No novel xbreed infrastructure. No new subagent templates. No harness changes.
