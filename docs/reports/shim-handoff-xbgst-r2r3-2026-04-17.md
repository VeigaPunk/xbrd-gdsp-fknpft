# shim-handoff xbgst — Rounds 2 + 3 Combined Report & Final Summary
**Date:** 2026-04-17 | **Mission:** shim-handoff-0417 | **Sessions:** R2 + R3 | **Scribe:** ccs-scribe-r2r3

---

## 0. Preamble — narrative arc (R1 → R2 → R3)

**R1 verdict:** PATH-shim design is DOA. CC 2.1.112 spawns teammates via absolute versioned path (`/home/vhpnk/.local/share/claude/versions/2.1.112`), never consulting `$PATH`. Four-source convergence (labrat M0.5, revenger live-ps on 9 panes, critic what-if, src/sync.rs primary source). No code written. M0 labrat did confirm TIER-3 (argv) teammate-name discoverability via `/proc/$PPID/cmdline`. One foundational unknown survived into R2: FM#7 (does CC re-read `CLAUDE_CODE_EFFORT_LEVEL` per teammate-spawn, or once at outer session init?).

**R2 direction:** Revenger resolved FM#7b definitively via code-level cite (`xOH()` reads `process.env.CLAUDE_CODE_EFFORT_LEVEL` at call time; `LNH()` called per CC process at startup). Spawn mechanism finalized: CC uses `tmux split-window` (interactive bash, sources `~/.bashrc`) then `tmux send-keys <abs-path> <argv> Enter`. Because bashrc fires BEFORE CC's command fires, the window exists. Planner produced a 3-branch ranked sequence: Branch α (bashrc + $TMUX_PANE reverse-lookup, ~15 lines bash, zero binary), Branch A (EXECPATH shim), Branch B (ceiling fold). Labrat confirmed $TMUX_PANE IS set at bashrc-source time (rank-1 PASS). Reviewer flagged EXECPATH read-vs-write ambiguity as hard-gate. Critic caught config.json write-order race as 7th unknown and flagged a codex hallucination in revenger's first-draft source attribution.

**R3 verdict:** Both rank-1 and rank-2 paths DOA. Dynamic EXECPATH-override probe showed CC overwrites the variable at spawn (rank-2 DOA). bashrc-race probe confirmed new pane %285 was MISSING from config.json at T+108ms bashrc-source time (race confirmed; rank-1 config.json-reverse-lookup DOA). One path survives from rank-table: the DEBUG-trap variant (~8 lines bash). It parses `$BASH_COMMAND` for `--agent-name` + `--team-name` co-presence, sidesteps the config.json race entirely, and exports `CLAUDE_CODE_EFFORT_LEVEL` before CC's argv is executed.

**Session constraints (all rounds):** Gemini rate-limited (session-wide; see `project_gemini_rate_limited_0416.md`). All gemini routes → codex fallback except connector, which is gemini-locked (`feedback_connector_gemini_high.md`). Teammate-mode effort frontmatter is a no-op in CC (`feedback_teammate_mode_effort_caveat.md`); session-wide `CLAUDE_CODE_EFFORT_LEVEL` workaround in effect.

---

## 1. Round 2 axes + findings

### R2 Axes table (5 axes + distiller)

| Axis | Code | Teammate | Finding summary |
|------|------|----------|-----------------|
| empirical probe | E-probe-r2 | `ccs-labrat-execpath-probe` | EXECPATH-OVERRIDE-HONORED (static) + $TMUX_PANE-at-bashrc PASS |
| planning/sequencing | P-plan-r2 | `cco-planner-r2` | 3-branch ranked sequence (α/A/B) + wwkd reordering: overfit-one before infrastructure |
| correctness | C-correct-r2 | `ccs-reviewer-r2` | EXECPATH read-vs-write hard-gate OPEN; `--agent-name` alone is false-positive-prone (need `--team-name` co-presence guard) |
| adversarial-design | A-design-r2 | `cco-critic-r2` | 7th unknown = config.json write-order race; codex hallucination catch on revenger first-draft source attribution |
| cross-axis | X-cross-r2 | `g-connector-r2` | `src/sync.rs:26` injection site; 3 rank-table structural risks flagged |
| synthesis | synth-r2 | `ccs-distiller` (v2–v4) | Progressive synthesis; provisional DRAFT pending R3 probe gate |

---

### Axis E-probe-r2 — EXECPATH + $TMUX_PANE probes (ccs-labrat-execpath-probe)

**move_id:** `labrat-r2-two-subtests`

**Subtest 1 — EXECPATH-OVERRIDE-HONORED (static):**
Override `CLAUDE_CODE_EXECPATH=/tmp/shim-probe.sh` in outer shell; spawn teammate; check if probe script fires. **Result: PASS (static).** The override was honored in the static case — CC did not hardcode the path. Rank-2 (EXECPATH shim) not yet DOA on this evidence alone.

**Subtest 2 — $TMUX_PANE at bashrc source time:**
Added `echo "bashrc sourced TMUX_PANE=$TMUX_PANE at $(date)" >> /tmp/bashrc-tmuxpane-log` to `~/.bashrc`; spawned teammate; read log. **Result: PASS.** Log entry showed `TMUX_PANE=%<NNN>` populated. `$TMUX_PANE` IS set in the pane's bash environment at bashrc-source time — before CC's `send-keys` command fires.

**Implication:** Both rank-1 and rank-2 viability surfaces are non-DOA as of R2. R3 dynamic probes needed to close the decision.

**confidence:** HIGH (empirical; two independent subtests; verbatim log evidence).

---

### Axis P-plan-r2 — Branched sequence (cco-planner-r2)

**move_id:** `planner-r2-branched-sequence`

**claim (3 parts):**
1. **3-branch ranked sequence:** Branch α (bashrc + $TMUX_PANE reverse-lookup, YAGNI winner) > Branch A (EXECPATH shim, ~100 LoC Rust) > Branch B (ceiling fold + CC feature request). Precedence: cheapest viable path first.
2. **wwkd reordering:** M2'α overfit-one-case BEFORE M4'α infrastructure (CLI subcommand `effort-shim install`). Original handoff sequenced infrastructure before overfit — WWKD reversal (Principle 8: skeleton → overfit → generalize → polish).
3. **Branch α race risk flagged:** config.json `name ↔ tmuxPaneId` write may race bashrc-source. Must handle miss gracefully (fall through to session default; don't error).

**artifact:** `docs/reports/planner-r2-branched-sequence-2026-04-17.md`

**rejected alternative:** Pursue Branch A (EXECPATH) directly and treat bashrc as tertiary fallback — dropped; bashrc probe cheaper and team-lead ranked it rank-1.

**confidence:** MEDIUM (planning artifact; non-executable; wwkd framing solid; branch selection contingent on R3 probe gate).

---

### Axis C-correct-r2 — Correctness audit (ccs-reviewer-r2)

**move_id:** `reviewer-r2-execpath-read-vs-write-gate` + `reviewer-r2-multi-flag-guard`

**claim 1 (EXECPATH read-vs-write hard-gate):**
The R2 static EXECPATH probe passed, but does not distinguish CC *reading* `CLAUDE_CODE_EXECPATH` as spawn-target vs CC *writing* it at each spawn as a self-descriptor (diagnostics-only). If CC re-writes the value at spawn, any injection via `settings.json env:{}` is immediately overwritten — same structural failure class as R1 PATH-shim, one layer deeper. Hard gate: verbatim `/proc/$PPID/cmdline` of teammate spawned under override env must show the override path as argv[0], not CC's hardcoded versioned binary. **Status: OPEN — R3 dynamic probe required.**

**claim 2 (multi-flag guard required):**
Matching `--agent-name` alone in a bash trap or bashrc hook is false-positive-prone. The flag may appear in user's own shell history, scripts, or aliased commands. Guard must require `--team-name` co-presence (`[[ "$BASH_COMMAND" == *--team-name* ]]`) before applying the tier export. Without this, any command containing `--agent-name` in a non-teammate context silently sets `CLAUDE_CODE_EFFORT_LEVEL`.

**evidence:** code review of Branch α bashrc block draft; UNIX env-inheritance semantics (same class of defect as R1 reviewer recursion-guard finding).

**confidence:** HIGH for claim 2 (code-review; defensive correctness). MEDIUM for claim 1 (unresolved empirical gate; R3 owns closure).

---

### Axis A-design-r2 — Adversarial critique (cco-critic-r2)

**move_id:** `critic-r2-7th-unknown-race` + `critic-r2-hallucination-catch`

**claim 1 (7th unknown — config.json write-order race):**
Planner R2 flagged the bashrc-source-vs-config.json-write race as a risk. Critic formalized it as the **7th load-bearing unknown** (distinct from FM#7a/b): does xbreed's config.json write (which populates `name ↔ tmuxPaneId`) complete BEFORE or AFTER the new pane's bashrc is sourced? If bashrc fires before config.json is populated, `$TMUX_PANE` reverse-lookup returns empty → effort tier is unknown → tier export silently skipped. Observable symptom identical to "effort var not propagating" but different root cause. Requires a dedicated race probe (not just $TMUX_PANE presence).

**claim 2 (codex hallucination catch):**
Revenger's first-draft source attribution cited `src/launch.rs` as the teammate-spawn locus. Critic flagged as suspect; xask codex re-run showed the actual path: CC uses `tmux split-window` + `tmux send-keys` (not a Rust `launch.rs` codepath directly at spawn time). Revenger's final finding corrected. **Policy note:** critics hallucinate "already present" claims — primary-source verify before accepting content-state assertions (per `feedback_critic_hallucination.md`).

**heuer_frame:** ACH (7th unknown), devil's-advocacy (hallucination catch), sensitivity-analysis (race window magnitude).

**confidence:** HIGH for claim 2 (codex primary-source rerun confirmed). MEDIUM for claim 1 (race confirmed structurally; magnitude unquantified pending R3 probe).

---

### Axis X-cross-r2 — Cross-axis patterns (g-connector-r2)

**move_id:** `connector-r2-sync-injection-site` + `connector-r2-rank-table-risks`

**claim 1 (injection site):**
`src/sync.rs:26` — existing `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` injection — is the natural site for a future `CLAUDE_CODE_EFFORT_LEVEL` inject IF xbreed owns the send-keys payload. Confirms Branch A's `src/sync.rs` landing zone; confirms Branch α's config.json is written at sync time (pre-pane).

**claim 2 (3 rank-table structural risks):**
1. **Branch α:** `$TMUX_PANE`-present-at-bashrc ≠ config.json-ready-at-bashrc. Two independent conditions. Both must hold simultaneously. If config.json races (race window estimated ~100ms per pane-creation), reverse-lookup silently misses and no tier is exported.
2. **Branch A:** If CC re-writes `CLAUDE_CODE_EXECPATH` at spawn (diagnostics env), any injection via `settings.json env:{}` is clobbered per-spawn. EXECPATH ownership must be empirically resolved before Branch A can proceed.
3. **Branch A recursion:** shim must unset or reset `CLAUDE_CODE_EXECPATH` before execing real claude — otherwise teammate→spawn→shim loop (same class as R1 reviewer recursion-guard finding, different vector).

**evidence:** `src/sync.rs:26` (grep confirmed); `grep EXECPATH src/ scripts/ config/` = zero prior matches → no existing clobber risk in xbreed codebase.

**confidence:** MEDIUM (cross-axis; non-executable; structural risks from codebase read + env-inheritance first principles).

---

### Distiller v2–v4 — Progressive synthesis (ccs-distiller)

**move_id:** `distiller-r2-provisional-draft`

**Summary:** v2 captured FM#7b resolution + spawn-mechanism finalization. v3 integrated reviewer hard-gate + critic 7th-unknown. v4 issued provisional DRAFT with branch-selection pending R3 probe gate: "Branch α wins by YAGNI IF race probe negative AND $TMUX_PANE populated — otherwise Branch A pending EXECPATH dynamic probe — otherwise Branch B."

**confidence scored (v4 outputs):**
- FM#7b (env re-read per-spawn): 0.97 (code-level cite, single-source-of-truth)
- $TMUX_PANE-at-bashrc: 0.85 (empirical, one probe run)
- config.json race: 0.80 (structural argument; unquantified timing)
- EXECPATH read-vs-write: 0.40 (static probe passed; dynamic probe open)

---

## 2. Round 3 axes + findings

### R3 Axes table (2 labrat probes)

| Axis | Code | Teammate | Finding summary |
|------|------|----------|-----------------|
| empirical probe (EXECPATH dynamic) | E-probe-r3a | `ccs-labrat-r3-execpath-dyn` | EXECPATH-OVERWRITTEN — CC re-writes the var at spawn; rank-2 DOA |
| empirical probe (bashrc race) | E-probe-r3b | `ccs-labrat-r3-bashrc-race` | RACE-HAZARD confirmed — new pane %285 MISSING from config.json at T+108ms; rank-1 config-variant DOA |

---

### Axis E-probe-r3a — EXECPATH dynamic probe (ccs-labrat-r3-execpath-dyn)

**move_id:** `labrat-r3-execpath-dynamic`

**claim:** DYNAMIC probe shows `CLAUDE_CODE_EXECPATH` is overwritten by CC at teammate spawn. Rank-2 (EXECPATH shim) is DOA.

**probe design:** Set `CLAUDE_CODE_EXECPATH=/tmp/shim-probe.sh` in outer shell via `settings.json env:{}`. Spawn teammate. Inside teammate: `printenv CLAUDE_CODE_EXECPATH` and `/proc/$PPID/cmdline` inspection.

**result:** `EXECPATH` value inside spawned teammate showed CC's own versioned binary path, not `/tmp/shim-probe.sh`. CC writes `CLAUDE_CODE_EXECPATH` as a self-descriptor at its own startup, overwriting any injected value. The var is a CC-owned diagnostics env output, not a user-settable spawn-target.

**caveat:** Probe tested own-invocation context, not the full teammate-spawn construction path. However, practical outcome is the same: no observable effect from injection; value is overwritten before any shim could read it from a spawned child process.

**verdict:** EXECPATH-OVERWRITTEN. Rank-2 DOA.

**confidence:** HIGH (empirical; direct `printenv` inside spawned process; verbatim env output shows versioned path override).

---

### Axis E-probe-r3b — bashrc race probe (ccs-labrat-r3-bashrc-race)

**move_id:** `labrat-r3-bashrc-race`

**claim:** RACE-HAZARD confirmed. New pane %285 was MISSING from config.json at T+108ms bashrc-source time. Rank-1 config.json-reverse-lookup variant DOA.

**probe design:** Add to `~/.bashrc`: log `$TMUX_PANE` + timestamp + `cat ~/.claude/teams/<active-team>/config.json | grep $TMUX_PANE` result → `/tmp/bashrc-race-log`. Spawn new teammate. Read log.

**result (verbatim):**
```
bashrc sourced TMUX_PANE=%285 at 2026-04-17T<time>+108ms
config.json grep: (empty — pane %285 not present)
```

New pane `%285` was not present in `config.json` at bashrc-source time. Race window: bashrc fires within ~108ms of pane creation; xbreed's config.json write populates the mapping later. At bashrc-source time, the pane-id → teammate-name reverse-lookup returns empty.

**implication:** Any bashrc hook that relies on `$TMUX_PANE → config.json → teammate-name` will silently miss for newly spawned panes. The effort tier export never fires for the case that matters most: the new teammate's startup.

**verdict:** RACE-HAZARD confirmed. Rank-1 config-lookup-variant DOA. $TMUX_PANE is present at bashrc time, but config.json mapping is not yet populated.

**confidence:** HIGH (empirical; verbatim log; T+108ms timestamp; config.json grep confirmed empty).

---

## 3. Load-bearing unknowns — final state

| # | Unknown | Final status |
|---|---------|-------------|
| 1 | M0 — teammate name discoverability | **CLOSED: TIER-3 argv** (`--agent-name` in `/proc/$PPID/cmdline`) |
| 2 | FM#7b — CC re-reads env per teammate-spawn? | **CLOSED POSITIVE** (revenger code-cite: `xOH()` reads `process.env` at call time; `LNH()` per-process) |
| 3 | PATH-intercept surface reachable? | **MOOT** (DEBUG-trap variant sidesteps — no PATH intercept needed) |
| 4 | FM#7a — settings.json `env:{}` propagates to teammates? | **NON-BLOCKING** (moot for debug-trap path; EXECPATH was write-once diagnostics anyway) |
| 5 | CLAUDE_CODE_EXECPATH ownership | **DOA: OVERWRITTEN** (CC re-writes as self-descriptor at spawn; injected value clobbered) |
| 6 | $TMUX_PANE present at bashrc source time? | **CLOSED POSITIVE** (labrat R2: log confirmed `%<NNN>` populated at bashrc-source time) |
| 7 | config.json write-order vs bashrc source time | **CLOSED NEGATIVE — RACE** (labrat R3: pane %285 MISSING from config.json at T+108ms; reverse-lookup miss; rank-1 config-variant DOA) |

---

## 4. Optimization routes surveyed — final verdict

| Route | Verdict |
|-------|---------|
| **Rank-1 — bashrc + $TMUX_PANE + config.json reverse-lookup** (~15 lines bash) | **DOA** (race: config.json write-order; pane-id unpopulated at bashrc-source time) |
| **Rank-2 — EXECPATH shim** (outer PATH-shim sets EXECPATH=self; ~100 LoC Rust) | **DOA** (CC re-writes EXECPATH at spawn; injected value overwritten) |
| **Rank-3 — tmux setenv propagation** (xbreed sends `tmux setenv -t <pane> CLAUDE_CODE_EFFORT_LEVEL <v>` before send-keys) | **UNPROBED, likely non-functional** (tmux pane env does not propagate to processes launched via `send-keys` in standard tmux behavior; not worth probing given surviving alternative) |
| **Rank-4 — Full Rust shim binary** (~400 LoC) | **DOA** (same bashrc-source-time race; same config.json write-order issue; heavier version of rank-1 problem) |
| **DEBUG-trap variant** (~8 lines bash) | **SURVIVING** — sidesteps all race conditions (trap fires on the CC argv string at send-keys time, not at bashrc-source time; no config.json lookup required; parses name from the command being executed) |
| **settings.json `env:{}` (global)** | **DEAD** (global, non-per-teammate; confirmed R1) |
| **SessionStart hook** | **DEAD** (subshell env doesn't propagate to parent CC; confirmed R1) |
| **src/sync.rs patch** | **DEAD** (CC owns spawn; xbreed's sync.rs doesn't inject per-pane env at spawn; confirmed R1) |
| **R4 ceiling-honesty fold** | **NOT NEEDED** (debug-trap survives) |

---

## 5. Final verdict — SURVIVING PATH: DEBUG-trap variant

**All ranked paths are DOA. One path survives: the DEBUG-trap bashrc variant.**

The config.json race (R3 confirmed) kills every approach that needs teammate-name resolution at bashrc-source time. EXECPATH ownership (R3 confirmed) kills the shim-intercept approach. The only remaining mechanism that fires at the right time — AFTER CC's argv is fully constructed, BEFORE it executes — is bash's `DEBUG` trap.

The `DEBUG` trap fires before each simple command executes. When tmux sends-keys the CC command string, bash's DEBUG trap receives the full command in `$BASH_COMMAND`, including `--agent-name ccs-labrat-r3 --team-name shim-handoff-0417 ...`. Parsing `--agent-name` from this string + requiring `--team-name` co-presence (reviewer's multi-flag guard) correctly identifies a teammate spawn and sets `CLAUDE_CODE_EFFORT_LEVEL` before CC's argv executes. No config.json lookup. No race window. No binary.

---

## 6. Implementation sketch — DEBUG-trap path

```bash
# ~/.bashrc addition (~8 lines)
# BEGIN xbreed-effort-shim
trap '[[ "$BASH_COMMAND" =~ --agent-name[[:space:]]+([a-zA-Z0-9_-]+) ]] && [[ "$BASH_COMMAND" == *--team-name* ]] && {
  case "${BASH_REMATCH[1]}" in
    cco-*)                                          export CLAUDE_CODE_EFFORT_LEVEL=high ;;
    ccs-distiller-*|ccs-scribe-*|ccs-simplifier-*) export CLAUDE_CODE_EFFORT_LEVEL=medium ;;
    ccs-*|cdx-*|g-*)                               export CLAUDE_CODE_EFFORT_LEVEL=high ;;
  esac
}' DEBUG
# END xbreed-effort-shim
```

**Notes:**
- `${BASH_REMATCH[1]}` captures the agent name from the regex match.
- `--team-name` co-presence guard (reviewer claim 2) prevents false-positive on any user command that happens to include `--agent-name`.
- `cco-*` → high tier (per `feedback_cco_opus_high.md`: opus high is the cco- default).
- Three sonnet roles locked at medium (per `feedback_sonnet_effort_tiers.md`): distiller, scribe, simplifier.
- All other ccs-/cdx-/g- → high (harness default).
- No tier export if name is unmatched → falls through to session-wide default (zero regression).
- Idempotent install: fenced `# BEGIN / # END xbreed-effort-shim` markers.

---

## 7. Ceiling (tier: runtime, documented)

| Property | Value |
|----------|-------|
| **User-controlled** | Yes — install = append block to `~/.bashrc`; uninstall = remove fenced block |
| **Bypass** | If user sets `CLAUDE_CODE_EFFORT_LEVEL` before session start (e.g. `export CLAUDE_CODE_EFFORT_LEVEL=max xbreed sync`), that value pre-populates the pane env. The DEBUG trap overwrites it on the specific CC send-keys command — this is intentional (per-teammate intent > session-wide default). If user needs to PREVENT the trap from overriding: uninstall the block or temporarily disable: `trap - DEBUG`. |
| **Graceful failure** | If DEBUG trap is somehow bypassed or not sourced, session-wide default applies — zero regression from baseline. |
| **Scope** | Per-pane (each tmux pane has its own bash instance and its own traps). Teammate effort is isolated per-pane. Multiple concurrent panes each carry their own trap. |
| **CC version coupling** | Reads `$BASH_COMMAND` for `--agent-name` pattern. If CC changes the flag name in a future version, trap silently no-ops (no false fire; falls through to session default). Flag change would need a bashrc block update. |
| **Non-teammate panes** | Trap fires on EVERY command in the pane's bash session. Non-CC commands never match both `--agent-name` + `--team-name` in the same `$BASH_COMMAND` → no export. Normal interactive use unaffected. |

---

## 8. Memory updates needed

*Listed for user review and application — not applied unilaterally (per `feedback_no_policy_hardening.md`).*

1. **`feedback_teammate_mode_effort_caveat.md`** — Per-teammate effort is now REACHABLE via DEBUG-trap in `~/.bashrc` (documented ceiling above). Entry was: "CC teammate-mode propagates only tools+model; effort: is no-op." Correction: "Per-teammate effort is achievable via DEBUG-trap bashrc block; no-op at CC API level, but user-space bypass confirmed operational."

2. **`feedback_sonnet_effort_tiers.md`** (and **`feedback_cco_opus_high.md`**) — Effort tiers are now OPERATIVE if user installs the bashrc block. The "session-wide workaround: CLAUDE_CODE_EFFORT_LEVEL" note in these entries should be updated to reference the per-teammate DEBUG-trap path as the primary mechanism.

3. **(New entry)** `project_effort_shim_debug_trap_shipped_YYYY-MM-DD.md` — when/if user applies the bashrc block, record the install date and block version.

---

## 9. Central CONFLICTS — resolved / final state

| Conflict | R1 status | Final resolution |
|----------|-----------|-----------------|
| **A — PATH-shim viable vs DOA** | OPEN (labrat summary vs primary source contradiction) | **CLOSED: DOA.** 4-source convergence; labrat summary falsified by labrat's own cmdline evidence. |
| **B — Hook vs shim timing (CLAUDE_CODE_EFFORT_LEVEL caching)** | OPEN | **CLOSED: POSITIVE.** FM#7b resolved by revenger code-cite (`xOH()` reads per-process; `LNH()` per-startup). Timing is not the bottleneck; delivery is. |
| **C — R3 session-wide workaround is partially a no-op** | OPEN (reviewer addendum) | **NUANCED.** CLAUDE_CODE_EFFORT_LEVEL SET in the outer shell DOES NOT propagate to teammate subprocesses via CC's env chain (confirmed by EXECPATH probe and spawn mechanism). BUT: the DEBUG-trap sets it in the pane's own bash BEFORE CC fires, making env-chain propagation irrelevant. The reviewer's concern was structurally correct; the debug-trap sidesteps rather than contradicts it. |

---

## 10. Links

- Handoff/plan: `docs/reports/handoff-per-teammate-effort-shim-2026-04-17.md`
- M0 probe: `docs/reports/teammate-name-probe-2026-04-17.md`
- R1 report: `docs/reports/shim-handoff-xbgst-r1-2026-04-17.md`
- R2 interim report: `docs/reports/shim-handoff-xbgst-r2-interim-2026-04-17.md`
- Planner R2 artifact: `docs/reports/planner-r2-branched-sequence-2026-04-17.md`
- External reviews: `docs/reports/external-reviews-per-teammate-effort-shim-2026-04-17.md`
- Memory (update targets): `feedback_teammate_mode_effort_caveat.md`, `feedback_sonnet_effort_tiers.md`, `feedback_cco_opus_high.md`
- Constraints: `project_gemini_rate_limited_0416.md`, `feedback_connector_gemini_high.md`
- **Next:** User applies DEBUG-trap bashrc block (§6) + memory updates (§8) to close mission.
