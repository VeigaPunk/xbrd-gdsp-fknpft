# shim-handoff xbgst — Round 1 Findings Report
**Date:** 2026-04-17 | **Mission:** shim-handoff-0417 | **Session:** R1 | **Scribe:** ccs-scribe-r1

---

## 1. Mission charter

> "Make `effort:` frontmatter (or a mapped tier) operative per-teammate, closing the teammate-mode noop documented in `feedback_teammate_mode_effort_caveat.md` and R3 known-gap #3. Ship a Rust `claude` shim on PATH that reads the spawning teammate's name, maps name → effort tier, sets `CLAUDE_CODE_EFFORT_LEVEL` before `exec`ing the real `claude` binary. Do NOT start coding until M0 (the load-bearing probe) returns positive."
>
> — `docs/reports/handoff-per-teammate-effort-shim-2026-04-17.md` TL;DR

Round 1 dispatched 8 specialists (labrat, reviewer, critic, simplifier, planner, revenger, scout, connector) + 1 distiller + 1 scribe. Mission was to stress-test the PATH-shim design before M1 implementation begins.

**Session constraints:**
- Gemini rate-limited (session-wide; see `project_gemini_rate_limited_0416.md`). All gemini routes → codex fallback, except connector which is gemini-locked (`feedback_connector_gemini_high.md`) and may BLOCK.
- g-connector-cross-pattern: gemini-locked; brief included 60s hard-post cap (`feedback_connector_stall.md`).
- Teammate-mode effort frontmatter is a no-op in CC (`feedback_teammate_mode_effort_caveat.md`); session-wide `CLAUDE_CODE_EFFORT_LEVEL` workaround in effect for this round.

---

## 2. Axes table (8 axes)

| Axis | Code | Teammate | xask target | Finding summary |
|------|------|----------|-------------|-----------------|
| empirical probe | E-probe | `ccs-labrat-m0-probe` | codex spark | TIER-3-POSITIVE: argv/PPID surfaces --agent-name; env/tmux DEAD |
| deletion/YAGNI | D-yagni | `ccs-simplifier-altpath` | none (read-only) | 17-line bash wrapper viable in theory; all simple alternative paths dead except argv shim |
| planning/sequencing | P-plan | `cco-planner-wwkd` | none (CC-native wwkd) | M0 short-circuits on first positive; M2 gate insufficient; M3 lacks concurrency gate |
| adversarial-design | A-design | `cco-critic-hidden` | codex high | 3rd unknown = PATH-intercept surface; FALSIFIED by labrat M0.5 primary source |
| correctness | C-correct | `ccs-reviewer-cascade` | codex high | Cascade must collapse to argv-only; 2 new critical bugs: recursion guard + real-claude self-resolution; FM#7 now stronger (CLAUDE_CODE_EFFORT_LEVEL non-propagating finding) |
| reverse-engineering | R-rev | `cdx-revenger-teammate` | codex high | CC 2.1.112 spawns via absolute versioned path — PATH-shim DEAD |
| cross-axis | X-cross | `g-connector-cross-pattern` | gemini high | argv-only shim mirrors src/guard.rs:74 + src/launch.rs:42; 2nd-order risk: hook vs shim timing |
| research | S-scout | `ccs-scout-priorart` | codex medium | clap::multicall(true) shrinks argv[0] dispatch; stdlib CommandExt::exec() sufficient; no prior AI-CLI shim crate |

---

## 3. R1 roster (8 specialists + distiller + scribe)

| Teammate | Axis code | Role | Status |
|----------|-----------|------|--------|
| `ccs-labrat-m0-probe` | E-probe | labrat | DELIVERED + M0.5 addendum |
| `ccs-simplifier-altpath` | D-yagni | simplifier | DELIVERED (v3 final) |
| `cco-planner-wwkd` | P-plan | the-planner + wwkd Layer 0 | DELIVERED + post-probe consolidation |
| `cco-critic-hidden` | A-design | critic + heuer Layer 0 | DELIVERED (v2 post-M0.5) |
| `ccs-reviewer-cascade` | C-correct | reviewer | DELIVERED (3 blockers + addendum) |
| `cdx-revenger-teammate` | R-rev | the-revenger | DELIVERED |
| `g-connector-cross-pattern` | X-cross | connector | DELIVERED (60s cap enforced) |
| `ccs-scout-priorart` | S-scout | scout | DELIVERED |
| `ccs-distiller` | synthesis | distiller | SYNTHESIS_READY (concurrent) |
| `ccs-scribe-r1` | documentation | scribe | This report |

---

## 4. Per-finding detail

### Axis E-probe — M0 live probe (ccs-labrat-m0-probe)

**move_id:** `m0-probe-live`

**claim:** CC 2.1.112 teammate-mode exposes teammate name exclusively via PPID argv (`--agent-name <x>`); env and tmux pane_title are DEAD channels.

**linchpin:** The probe subject IS the probe instrument (labrat ran commands from inside its own spawned pane). PPID is the CC process; its cmdline is stable for the session lifetime.

**evidence (verbatim from `docs/reports/teammate-name-probe-2026-04-17.md`):**
```
# Probe 1 — env vars
CLAUDE_CODE_ENTRYPOINT=cli
CLAUDE_CODE_EXECPATH=/home/vhpnk/.local/share/claude/versions/2.1.112
CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1
TMUX=/tmp/tmux-1000/default,28819,9
TMUX_PANE=%267
# No CLAUDE_AGENT_NAME, CLAUDE_TEAMMATE_NAME — NEGATIVE

# Probe 2 — tmux pane_title
⠂ Review effort-shim design for correctness defects
# Not teammate name — NEGATIVE

# Probe 4 — ps on $PPID
PID    PPID COMMAND
1315614 1315446 /home/vhpnk/.local/share/claude/versions/2.1.112 \
  --agent-id ccs-labrat-m0-probe@shim-handoff-0417 \
  --agent-name ccs-labrat-m0-probe \
  --team-name shim-handoff-0417 \
  --agent-color blue \
  --parent-session-id 67b576a1-263e-4776-b10b-86d759d4ed75 \
  --agent-type labrat \
  --dangerously-skip-permissions \
  --plugin-dir /home/vhpnk/.claude/personal-plugin \
  --model sonnet
# TIER-3-POSITIVE (certain)
```

**M0.5 addendum (primary-source conflict):** Labrat's idle summary claimed "PATH-shim viable if inserted before symlink." Primary source CONTRADICTS: CC invokes via full absolute versioned path `/home/vhpnk/.local/share/claude/versions/2.1.112` — PATH is never consulted. Distiller flagged as `spoof_flagged` for arbitration.

**minimal shim read:**
```bash
agent_name=$(tr '\0' '\n' < /proc/$PPID/cmdline | grep -A1 '^--agent-name$' | tail -1)
```

**rejected alternative:** Using `$$` cmdline — that is the bash tool-execution wrapper, not the CC process.

**confidence:** HIGH (live primary source, self-probe, verbatim cmdline).

**peer DM cross-critiques:** revenger cross-validated independently (live ps on 9 teammates, all show --agent-name pattern). reviewer confirmed env/tmux dead. critic confirmed M0.5 falsifies PATH-intercept surface.

---

### Axis D-yagni — Simplest viable alternative (ccs-simplifier-altpath)

**move_id:** `simplifier-bash-wrapper` (v3 final)

**claim:** Replace 400 LoC Rust plan with 17-line bash wrapper. Parse `--agent-name` from own `$@` (argv). Case-match to tier. Export `CLAUDE_CODE_EFFORT_LEVEL`. Exec real claude via `type -a claude | awk 'NR==2{print $NF}'`. Delete manifest walker, CLI subcommand, effort-map YAML, and test suite.

**linchpin:** M0.5 — shim must actually be on CC's spawn path. If PATH-shim is DOA (absolute versioned spawn), simplification is moot.

**evidence:** Read `src/sync.rs` confirmed CC-owned spawn path (not user-space modifiable). Grep confirmed `SessionStart` hook fires in subshell — env changes don't propagate to parent CC. `settings.json env:` block is global (non-per-teammate). Direct `sync.rs` patch unavailable (CC is the launcher, xbreed can't inject per-teammate env at spawn via sync).

**rejected paths surveyed:**
- **Path 1 — SessionStart hook:** env set in subshell doesn't propagate to parent CC process. DEAD.
- **Path 2 — settings.json env: block:** global, not per-teammate. DEAD.
- **Path 3 — bash wrapper (this proposal):** viable IF PATH-shim fires, but substrate question is open at R1.
- **Path 4 — src/sync.rs patch:** CC owns the spawn; xbreed's sync.rs doesn't get to inject per-pane env. DEAD.
- **Path 5 — accept session-wide ceiling:** R3 workaround stays; no shim. Viable but closes the gap permanently.

**confidence:** MEDIUM (paths 1/2/4 confirmed dead via primary source; Path 3 viability contingent on M0.5 spoof-check resolution).

**peer DM:** critic challenged bash-wrapper as non-viable when substrate is wrong; simplifier held position pending spoof-check.

---

### Axis P-plan — Sequencing defects (cco-planner-wwkd)

**move_id:** `planner-wwkd-m0-short-circuit`

**claim (3 parts):**
1. M0 decision tree short-circuits on first positive — should commit ALL 3 tier outputs regardless, so a future CC convention drift from argv → env doesn't silently re-activate a dead cascade branch without evidence.
2. M2 gate `printenv | grep CLAUDE_CODE_EFFORT_LEVEL` only validates env propagation, not that CC re-reads the var at teammate spawn. Real gate: spawn 2 teammates at different tiers, compare behavioral output variance (time-to-first-token or output verbosity delta).
3. M3 generalizes via manifest with no concurrent-teammate gate. A 3-concurrent-teammate M2 load test belongs between M2 and M3.

**linchpin (wwkd):** "Overfit one case first, verify it holds under realistic load, then generalize."

**evidence:** none — planning artifact (non-executable).

**post-probe consolidation (advisory):** If PATH-shim substrate is confirmed dead, cascade design collapses to single-tier argv. Explicit logging `[shim] argv-resolve: <name> → <tier>` on hit, `[shim] argv-miss` on fall-through, is still the right observability pattern.

**rejected alternative:** Insert M0.5 concurrency probe between M0 and M1 — dropped; belongs between M2 and M3.

**confidence:** MEDIUM (planning axis; non-executable; wwkd framing solid).

---

### Axis A-design — Third load-bearing unknown (cco-critic-hidden)

**move_id:** `critic-third-unknown-v2`

**claim:** 3rd load-bearing unknown was PATH-intercept surface (does CC's spawn path reach the shim?). **FALSIFIED by labrat M0.5 primary source**: CC spawns via absolute versioned path, PATH-shim never fires. Design-level collapse on this axis.

**Pivot finding (post-falsification):** 3rd unknown now re-seats as: does CC propagate `settings.json env:{}` block to teammate subprocess AND re-read `CLAUDE_CODE_EFFORT_LEVEL` at teammate spawn? FM#7 (originally "does CC re-read env at teammate spawn?") now splits into:
- **FM#7a:** Does settings.json `env:` propagate to teammate subprocesses?
- **FM#7b:** If propagated, does CC re-read `CLAUDE_CODE_EFFORT_LEVEL` at per-teammate spawn vs once at outer session init?

**heuer_frame:** what-if + sensitivity analysis (Heuer Layer 0 per `feedback_cco_critic_heuer.md`).

**linchpin:** Splits the monolithic FM#7 into two independently testable sub-unknowns. FM#7a is the xbreed-side injection pivot; FM#7b is the CC-behavior question.

**evidence:** none — adversarial design; backed by labrat primary-source M0.5 (`docs/reports/teammate-name-probe-2026-04-17.md`).

**rejected alternative:** bash-wrapper simplifier path — simpler != viable when substrate is wrong (PATH not consulted).

**confidence:** MEDIUM (adversarial-design; primary-source-backed via labrat cross-validation; same-model-cap applies to heuer framing).

---

### Axis C-correct — Correctness defects in cascade design (ccs-reviewer-cascade)

**move_id:** `reviewer-cascade-collapse-argv-only` + `reviewer-recursion-guard-self-defeats` + `reviewer-resolve_real_claude_path-self-resolves`

**claim 1 (cascade-collapse):** 3-tier cascade must collapse to single-tier argv parse. Env/tmux branches are dead (M0 confirmed); keeping them in code is dead code that masks future regressions.

**evidence:** M0 broadcast — env/tmux dead, argv only positive. `src/sync.rs:26-29` injects only `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` (no identity env).

**claim 2 (recursion guard self-defeats):** `XBREED_SHIM_ACTIVE=1` is an inherited env var. Every spawned teammate **inherits** it from the parent shell → shim early-exits on every launch that matters. FM#2 (`XBREED_SHIM_RAN=1` guard in `cco-probe-opus-high.md`) is misclassified — it protects against shim re-entry but fires on EVERY teammate because the guard is already set in the parent.

**evidence:** verbatim from xask codex: "env vars exported in parent are inherited by all child processes; `XBREED_SHIM_ACTIVE=1` set at outer shell propagates to every teammate spawn."

**fix:** `std::env::remove_var("XBREED_SHIM_ACTIVE")` before exec + inode-compare realpath as a second independent guard.

**claim 3 (real-claude self-resolution loop):** `which claude` / `command -v claude` in the shim returns the shim itself (first match on PATH) → infinite exec loop before `XBREED_SHIM_ACTIVE` even fires.

**fix:** `type -a claude | sed -n '2p'` (second claude on PATH) or walk `$PATH` entries skipping shim dir, or resolve via `CLAUDE_CODE_EXECPATH` env var (already present in M0 probe env as an absolute path).

**claim 4 — ADDENDUM (new finding, HIGH):** `CLAUDE_CODE_EFFORT_LEVEL` does NOT propagate lead→teammate via CC's env chain. Only `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` propagates. The R3 "session-wide workaround" in `feedback_teammate_mode_effort_caveat.md` is **partially a no-op for teammates** — the env var may be set in the outer shell but not inherited by the spawned CC teammate processes. Flagged for memory correction.

**rejected alternative (for cascade):** Per-PPID argv scan — reviewer clarified own argv IS the canonical source (shim can read `std::env::args()` directly since it IS the process intercepting the spawn — no PPID needed).

**confidence:** HIGH for claims 2+3 (xask codex verbatim; standard UNIX env-inheritance semantics). MEDIUM for claim 4 (new finding from reviewer addendum; no primary empirical gate yet — contradicts R3 assumed behavior).

**CONFLICT flagged:** claim 4 directly contradicts the R3 assumption that session-wide `CLAUDE_CODE_EFFORT_LEVEL` works as workaround. Routes to distiller for arbitration. See §5.

---

### Axis R-rev — CC 2.1.112 spawn mechanism (cdx-revenger-teammate)

**move_id:** `revenger-argv-is-load-bearing`

**claim:** CC 2.1.112 teammate-mode propagates identity exclusively via argv (`--agent-name <name>`, `--agent-id <name>@<team>`, `--team-name <team>`). NOT via env. tmux pane_title holds task-summary UI string (rotates per task, not stable). config.json persists `name <-> tmuxPaneId` mapping. Cascade order in handoff (env→tmux→argv) is INVERTED — only Tier-3 (argv) fires. argv[0]-basename dispatch is USELESS — CC invokes via full versioned path, not `claude` symlink.

**evidence:**
- `src/sync.rs:26-29` injects only `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` (revenger grep, confirmed)
- Live `ps` on 9 teammates: all show `--agent-name` pattern in argv
- Live `tmux list-panes`: `%267 | ⠂ Review effort-shim design... | 2.1.112` — pane title is task description, not teammate name
- `config.json:19-31` stores `name` + `tmuxPaneId` mapping (xbreed-side record, not CC env injection)
- codex xask confirmed: CC uses config.json + argv for identity; does not set env at spawn

**rejected alternative:** Hypothesis that CC writes pane_title=teammate_name. **FALSIFIED**: pane titles are task-summary UI strings from labrat probe + revenger live-pane inspection.

**confidence:** HIGH (multi-source convergence: labrat empirical + revenger live ps on 9 panes + src/sync.rs primary source + xask codex).

---

### Axis X-cross — Cross-axis pattern match (g-connector-cross-pattern)

**move_id:** `connector-2phase-argv-guard-launch`

**claim:** 1-tier (argv-only) shim maps onto existing repo patterns in two phases: Phase 1 (identity resolution) mirrors `src/guard.rs:74` cmdline join+match; Phase 2 (env injection + exec) mirrors `src/ask.rs:308-331` `Command::new` + `.env`/`.arg` mutations and `src/launch.rs:42` passthrough pattern. No novel architecture needed.

**evidence:**
- `src/guard.rs:74` — cmdline join+match pattern
- `src/launch.rs:42` — passthrough pattern
- `src/ask.rs:308-331` — `Command::new` + `.env`/`.arg` mutations

**second-order risk:** positional arg scanning (`args().windows(2)`) not join — brittle on spaces or multi-token arg values. Use `args().zip(args().skip(1))` or parse full args with clap.

**rejected alternative:** multi-tier cascade — M0 falsifies the non-argv branches.

**CONFLICT flagged (hook vs shim timing):** Does `CLAUDE_CODE_EFFORT_LEVEL` take effect mid-session (if set by a SessionStart hook AFTER process init)? If CC caches effort at process start, hook-injected env is too late. PATH-shim (if viable) intercepts at spawn = before CC reads env. This is unresolved — see §5.

**confidence:** MEDIUM (non-executable cross-axis finding; pattern citations are primary source; substrate viability open pending spoof-check).

---

### Axis S-scout — Prior art (ccs-scout-priorart)

**move_id:** `scout-clap-multicall-shrinks-harness`

**claim:** `clap::Command::multicall(true)` — already a dep — eliminates the need for custom `argv[0]` basename dispatch code. Core shim drops to ~50 LoC (not ~100). The handoff's ~400 LoC total is dominated by install/doctor/manifest/tests, not the shim harness itself.

**evidence:** none — research axis (non-executable).

**additional observations:**
- rustup proxy is prior art for argv[0] dispatch (same pattern)
- No dedicated AI-CLI effort-shim crate exists (shimexe-core/vx_shim too heavy for this use case)
- `nix::unistd::execvpe` adds a dep; stdlib `std::os::unix::process::CommandExt::exec()` covers the need
- `which` crate (already in `cco-bench-opus-a.md` proposal) appropriate for real-claude resolution IF PATH-shim substrate is viable

**rejected alternative:** Dedicated shim-harness crate — none exist for this exact pattern; stdlib + existing clap dep suffices.

**confidence:** MEDIUM (research-only; no executable gate; clap multicall claim needs verification against current clap 4 docs).

---

## 5. Central CONFLICTS (for judge arbitration)

### CONFLICT A — PATH-shim viable vs DOA (CRITICAL)

**Claim A (DOA):** CC 2.1.112 spawns teammates via absolute versioned path `/home/vhpnk/.local/share/claude/versions/2.1.112`. PATH is never consulted. PATH-shim never fires. Design collapses.

**Sources for A:** critic-hidden v2, reviewer-cascade final, revenger-argv-is-load-bearing, simplifier-bash-wrapper (linchpin), distiller spoof-flagged.

**Claim B (viable):** Labrat M0.5 idle summary stated "PATH-shim viable if inserted before symlink."

**Primary-source verdict:** Labrat's OWN probe output (`/proc/$PPID/cmdline`) shows `argv[0] = /home/vhpnk/.local/share/claude/versions/2.1.112` — a full absolute versioned path, NOT a symlink. Labrat's summary CONTRADICTS labrat's own evidence. Distiller flagged as `spoof_flagged`.

**Status:** Claim A supported by 4 convergent sources + labrat's own primary source. Claim B is labrat's idle-summary only, directly falsified by labrat's own cmdline evidence. Routing to judge for formal verdict.

**Implication if A confirmed:** M1–M6 as written are DOA. Pivot required before any code is written. See §8 for pivot options.

---

### CONFLICT B — Hook vs PATH-shim timing (OPEN)

**Question:** Does `CLAUDE_CODE_EFFORT_LEVEL` modulate CC behavior when set at teammate-spawn time (shim), or is it cached once at outer session init (making any mid-session or per-spawn injection a no-op)?

**Source:** connector-cross-pattern (flagged as second-order risk); Grok external review (flagged as FM#7-class blocker: "CC might cache CLAUDE_CODE_EFFORT_LEVEL at outer session init").

**Status:** Unresolved. Requires Round-2 behavioral probe (spawn 2 teammates with different env values, measure output divergence — time-to-first-token, output verbosity, or explicit `printenv | grep EFFORT` from within the teammate).

**Note:** reviewer-cascade addendum suggests `CLAUDE_CODE_EFFORT_LEVEL` may NOT propagate lead→teammate at all via CC env chain. If true, this entire approach (env injection) is the wrong lever regardless of timing.

---

### CONFLICT C — R3 session-wide workaround is partially a no-op (HIGH)

**Claim:** `CLAUDE_CODE_EFFORT_LEVEL` set in outer shell does NOT propagate to spawned CC teammate processes. Only `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` propagates. The R3 "workaround" in `feedback_teammate_mode_effort_caveat.md` may be inoperative.

**Source:** reviewer-cascade addendum (new finding, R1).

**Status:** Contradicts the R3 assumption. No empirical gate closed. Needs behavioral divergence test. Routes to judge + memory update if confirmed.

---

## 6. Optimization routes surveyed

| Route | Verdict |
|-------|---------|
| **Rust shim binary (handoff plan)** — PATH-prepended `xbreed-claude` binary (~400 LoC) | DOA if PATH not consulted at CC spawn (substrate falsified by M0.5 primary source) |
| **Bash wrapper (17 lines)** — same PATH-intercept logic in shell | Same substrate problem — DOA if PATH not consulted |
| **clap multicall (`argv[0]` dispatch)** — existing dep, ~50 LoC core shim | Reduces LoC; irrelevant if shim never reaches PATH intercept point |
| **LD_PRELOAD shim** — intercept at shared-lib level | Invasive, fragile across CC upgrades (Grok: breaks on statically-linked binaries or explicit execve path). DEAD. |
| **CC binary wrap** — wrap the absolute versioned binary itself | Fragile: CC auto-updates version path; maintenance burden per-upgrade. HIGH RISK. |
| **settings.json `env:` block** — add CLAUDE_CODE_EFFORT_LEVEL to global env | Global only (non-per-teammate). DEAD for per-teammate use case. |
| **SessionStart hook** — CC hook fires before teammate work begins | Fires in subshell; env changes don't propagate to parent CC process. DEAD. |
| **`CLAUDE_CODE_EXECPATH` substitution** — probe whether CC reads `CLAUDE_CODE_EXECPATH` at spawn for indirect exec | NEW CANDIDATE (Round 2). Env var `CLAUDE_CODE_EXECPATH` was observed in M0 probe env pointing to the versioned binary. If CC re-reads this at teammate spawn, setting it to shim path is a clean interception surface. UNPROBED. |
| **xbreed-side sync.rs injection** — patch xbreed's spawn path to inject per-teammate env directly into the tmux `send-keys` invocation | Requires verifying xbreed owns the spawn command string (src/sync.rs does NOT — CC owns the spawn; xbreed only drives CC via send-keys). DEAD. |
| **R4 ceiling-honesty fold** — accept the current gap; document as CC feature request | Always valid; reviewer-cascade final verdict flagged as alternative if pivot probes fail. |

---

## 7. Load-bearing unknowns (post-R1 state)

| Unknown | Status | Next action |
|---------|--------|-------------|
| **M0** — Does CC expose teammate name via env/tmux/argv? | **RESOLVED: TIER-3 argv ($PPID)** | Archive. Shim can read name. |
| **M0.5** — Is CC's spawn path reachable via PATH shim? | **LIKELY DEAD** — absolute versioned path in spawn argv. Pending judge formal verdict on CONFLICT A. | Judge ack → pivot or abort. |
| **FM#7a** — Does settings.json `env:{}` propagate to teammate subprocesses? | **OPEN** — untested. New pivot candidate if PATH-shim dead. | R2 labrat: set `env: { CLAUDE_CODE_EFFORT_LEVEL: "max" }` in settings.json; spawn teammate; `printenv | grep EFFORT` inside teammate. |
| **FM#7b** — Does CC re-read `CLAUDE_CODE_EFFORT_LEVEL` at teammate spawn vs once at session init? | **OPEN** — Grok + reviewer both flag this as FM-class blocker. | R2 behavioral probe: 2 teammates with different env values; measure output divergence. |
| **4th candidate** — Does CC read `CLAUDE_CODE_EXECPATH` env var at teammate spawn for binary resolution? | **UNPROBED — NEW (R2 candidate)** | R2 labrat: check if overriding `CLAUDE_CODE_EXECPATH` at session init re-routes CC spawns through a custom path. |
| **CONFLICT C** — Does `CLAUDE_CODE_EFFORT_LEVEL` propagate at all from lead to teammate via env chain? | **OPEN** — reviewer addendum, no empirical gate. | R2 behavioral probe closes this; if confirmed non-propagating, R3 memory entry needs correction. |

---

## 8. Round 1 verdict

**The handoff plan as written is DOA.**

The PATH-shim design collapses at the substrate level: CC 2.1.112 spawns teammates via absolute versioned path (`/home/vhpnk/.local/share/claude/versions/2.1.112`), never consulting PATH. A shim on PATH is never reached. The M0 handoff condition (probe positive) was nominally met (TIER-3: argv surfaces name), but the M0.5 clarification (is the spawn path user-interceptable?) reveals the design is non-viable before M1 begins.

Three independent signals converge on DOA: labrat M0.5 primary source, revenger live ps on 9 teammates, critic what-if analysis. Distiller spoof-flagged labrat's own summary contradiction.

No code should be written against the current plan.

---

## 9. Round 2 dispatch direction

**Two pivot probes needed before M1 can be rescoped:**

**Probe A — `CLAUDE_CODE_EXECPATH` interception:** Does CC read `CLAUDE_CODE_EXECPATH` env var at teammate spawn for its own binary resolution? If yes, setting `CLAUDE_CODE_EXECPATH=/path/to/shim` at session init or per-teammate (via any channel) would be a clean interception point. One labrat, one `printenv | grep EXECPATH` inside teammate + xask codex re: CC source.

**Probe B — FM#7 behavioral divergence:** Spawn 2 teammates in same session: one with `CLAUDE_CODE_EFFORT_LEVEL=max` set in outer env, one without. Compare time-to-first-token or output verbosity. If they diverge: env injection path is viable (FM#7b positive). If same: env never read per-spawn → whole approach wrong, R4 ceiling-honesty fold.

**Probe C (optional) — settings.json `env:{}` propagation:** FM#7a. Set `{ "env": { "CLAUDE_CODE_EFFORT_LEVEL": "max" } }` in settings.json, spawn teammate, `printenv` inside. Cheapest path if EXECPATH probe fails.

**If all three fail:** Route to R4 ceiling-honesty fold. Document as CC feature request for native per-teammate `effort:` support in teammate-mode spawn args. Memory entry `feedback_teammate_mode_effort_caveat.md` marked UNRESOLVABLE at user-space tier.

**Priority order:** Probe A (EXECPATH) → Probe B (behavioral divergence) → Probe C (settings.json). Probe B is independent and should run concurrently with A.

---

## 10. Links

- Plan/handoff: `docs/reports/handoff-per-teammate-effort-shim-2026-04-17.md`
- Source proposals: `data/proposals/cco-probe-opus-high.md`, `data/proposals/cco-bench-opus-a.md`
- M0 probe results: `docs/reports/teammate-name-probe-2026-04-17.md`
- External reviews: `docs/reports/external-reviews-per-teammate-effort-shim-2026-04-17.md`
- Team config: `~/.claude/teams/shim-handoff-0417/config.json`
- Memory: `feedback_teammate_mode_effort_caveat.md`, `project_gemini_rate_limited_0416.md`, `feedback_connector_gemini_high.md`, `feedback_scribe_per_round.md`
- Next: Round 2 (pivot probes — EXECPATH + FM#7 behavioral divergence)
