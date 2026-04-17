# Plan — R2 revised milestone sequence (3-branch, bashrc-first)
**Session:** R2 | **Dispatched by:** team-lead | **Date:** 2026-04-17 | **Author:** cco-planner-r2 (wwkd Layer 0)
**Revision:** v2 — integrates team-lead R2 update (revenger FM#7 code-cite + bashrc+$TMUX_PANE as new rank-1 path) + g-connector-r2 sync timing critique.

## Phase 0 — State map (revised)

- **Exists:**
  - R1 scribe report `docs/reports/shim-handoff-xbgst-r1-2026-04-17.md` — PATH-shim falsified (4-source convergence on absolute versioned spawn).
  - M0 probe: teammate NAME discoverable via `--agent-name` in `/proc/$PPID/cmdline`; env + tmux pane_title DEAD.
  - `~/.claude/teams/<team>/config.json` persists `name ↔ tmuxPaneId` mapping (revenger R1; verified: e.g. `ccs-distiller` → `%275`, `ccs-scribe-r1` → `%276`). Reverse-lookup surface exists for Branch α.
  - R2 FM#7 DEFINITIVE (team-lead + revenger code-cite): `xOH()` reads `process.env.CLAUDE_CODE_EFFORT_LEVEL` at call time; `LNH()` called per CC process at startup. Opus-4-7 effort gating. **Each teammate's own process.env is what matters.** FM#7b (effort cached vs re-read) is RESOLVED — per-process env injection DOES modulate behavior.
  - Spawn mechanism finalized: CC uses `tmux split-window` (interactive bash sources ~/.bashrc) then `tmux send-keys <abs-path-argv> Enter`. ~/.bashrc IS sourced BEFORE CC fires.
  - `src/main.rs:25+33` — `write_claude_settings` fires on both `Sync` and `Claude` subcommands. Confirms g-connector-r2 critique: settings materialization is on the `claude` launch path, not gated to `sync`. Direct `launch_claude` calls (main.rs:40) bypass any settings re-materialization — good for Branch A's planning (shim EXECPATH is sourced from settings.json at write time, not read at launch).
- **Missing:**
  - Empirical verdict: is `$TMUX_PANE` set inside pane bash at ~/.bashrc source time? (Branch-α load-bearing probe; team-lead requested subtest for `ccs-labrat-execpath-probe`.)
  - Empirical verdict: does CC honor user-overridden `CLAUDE_CODE_EXECPATH` at teammate spawn? (Branch-A probe — still relevant as fallback.)
- **Risk:**
  - If `$TMUX_PANE` is NOT yet set at bashrc source time (e.g. set after bash exec by tmux post-init), Branch α collapses and we fall back to Branch A.
  - `~/.bashrc` hook runs for EVERY interactive shell (including non-teammate panes, terminal tabs, SSH sessions). Must be gated: only act if `$TMUX_PANE` matches a teammate entry in active team config.json AND the shell's task-context looks like a CC teammate launch (not a general-purpose interactive shell in the same pane).

## WWKD

1. **What:** Produce a 3-branch sequence. Branch α (bashrc+$TMUX_PANE) is rank 1 by YAGNI (zero Rust, ~15 lines bash). Branches A (EXECPATH shim) and B (ceiling fold) remain as fallbacks. Success boundary: plan handoff-ready, branches gated on tokenized probe outputs.
2. **Why:** Team-lead R2 update moves the cheapest viable path from "Rust EXECPATH shim" to "bashrc hook." Revenger's code-level cite (`xOH()`/`LNH()`) resolves FM#7b → per-teammate env injection IS the right lever behaviorally. Simpler path, if it works, wins on total surface area.
3. **Assumptions/Risks:**
   - Assumes `ccs-labrat-execpath-probe` executes BOTH probes (TMUX_PANE-at-bashrc + EXECPATH-override-honored). These are independent; could both pass, in which case Branch α wins by YAGNI.
   - Risk: bashrc hook collides with user's existing bashrc customizations. Must be append-only block with fence markers (like `# BEGIN xbreed-effort-shim ... # END xbreed-effort-shim`) and idempotent install.
   - Risk: config.json `name ↔ tmuxPaneId` mapping may race — bashrc fires at pane creation; does xbreed-side config.json write precede or follow? `tmuxPaneId: ""` exists for team-lead's own entry (no pane assigned yet), so the mapping is NOT guaranteed populated at every read. Branch α's reverse-lookup must handle miss (fall through to session default, don't error).
4. **How:** Three branches below, decision tree on probe outcome. Cleanest-path first (α); fallbacks ordered by cost. Each branch preserves wwkd posture: skeleton → overfit-one → generalize → polish.
5. **Escalation points:**
   - $TMUX_PANE probe positive BUT config.json mapping race observed → escalate to judge; consider a synchronization gate in sync.rs before teammate-mode activates.
   - $TMUX_PANE probe negative AND EXECPATH-override-honored → proceed to Branch A.
   - Both negative → Branch B.

## Key branching decision

**Original handoff's M0 conflated three independent unknowns into one gate:**
1. Teammate-name discoverability. (Resolved R1: argv.)
2. Shim-on-spawn-path reachability. (Resolved R1: PATH not consulted — DOA.)
3. Effort env re-read timing. (Resolved R2 via revenger code-cite: CC re-reads per-process.)

**R2 update surfaces a FOURTH axis:** WHICH interception surface is cheapest given the resolved unknowns. With name-via-argv + process-level effort re-read both confirmed, the choice reduces to: WHERE can we set `CLAUDE_CODE_EFFORT_LEVEL` in the teammate's environment before its CC process reads it? Three surfaces ranked by cost:

1. **Bashrc hook (rank 1 — Branch α):** set env in the pane's interactive bash at ~/.bashrc sourcing, BEFORE `tmux send-keys` fires CC. Zero binary, ~15 lines bash.
2. **EXECPATH shim (rank 2 — Branch A):** outer PATH-shim that sets `CLAUDE_CODE_EXECPATH=self`, CC propagates to teammate spawns via env inheritance, shim resets EFFORT per name lookup before execing real claude. ~100 LoC Rust.
3. **Ceiling fold (rank 3 — Branch B):** both above fail → R4 ceiling-honesty fold + CC feature request.

## Milestones — Branch α (if `$TMUX_PANE-AT-BASHRC-POSITIVE`)

| # | Title | Gate command | Expected output | Executor |
|---|---|---|---|---|
| M0'α | $TMUX_PANE-at-bashrc probe | append `echo "bashrc sourced TMUX_PANE=$TMUX_PANE" >> /tmp/bashrc-log` to ~/.bashrc; spawn teammate; `cat /tmp/bashrc-log` | log contains a line with `TMUX_PANE=%<NNN>` matching teammate's assigned pane in team config.json | `ccs-labrat-execpath-probe` (add as subtest) |
| M1'α | Skeleton bashrc block (log-only, no export) | install bashrc block that logs `[xbreed-shim] TMUX_PANE=$TMUX_PANE name=<resolved> tier=<tier>` to `~/.xbreed/shim.log`; spawn teammate; tail log | log line shows pane→name reverse-lookup succeeded OR graceful miss on unmatched pane; no effort-level export yet; no behavior change | `executor` |
| M2'α | Overfit ONE teammate to medium | hard-code `if name == "ccs-probe-medium": export CLAUDE_CODE_EFFORT_LEVEL=medium` in bashrc block; spawn `ccs-probe-medium`; teammate runs `printenv \| grep CLAUDE_CODE_EFFORT_LEVEL` | teammate env shows `CLAUDE_CODE_EFFORT_LEVEL=medium`. FM#7b resolved via code-cite — behavioral proxy now sanity-only, NOT gate. | `executor` + one labrat |
| M3'α | Generalize via config.json lookup | bashrc block walks `$TMUX_PANE` → lookup in active team config.json → extract name → case-match to tier (hard-coded map for now: `cco-critic-*` → xhigh, `cco-*` → high, `ccs-distiller`+`ccs-simplifier`+`ccs-scribe` → medium, rest → default). Spawn 3 mixed-tier teammates; each reports own env. | all three show correct `CLAUDE_CODE_EFFORT_LEVEL`; unmatched names fall through to session default | `executor` |
| M4'α | xbreed CLI subcommand `effort-shim install` | `xbreed effort-shim install` writes idempotent fenced block to `~/.bashrc`; `xbreed effort-shim uninstall` removes the fenced block cleanly; `xbreed effort-shim doctor` reports: bashrc block present, team config.json readable, last 5 log entries | install/uninstall are idempotent; fenced block has BEGIN/END markers; doctor exits 0 | `executor` |
| M5'α | Test suite | `cargo test effort_shim && cargo clippy && cargo fmt --check` | tests cover: install idempotency, uninstall cleanliness, name→tier map, config.json miss → session default fall-through, concurrent-teammate correctness | `ccs-reviewer-r2` + `executor` |
| M6'α | Ship — docs + memory | commit sequence; `feedback_teammate_mode_effort_caveat.md` → REACHABLE via bashrc-hook; new entry `project_bashrc_effort_shim_shipped_0417.md` | xbgst round with mixed-tier teammates shows each at intended tier in `~/.xbreed/shim.log` | `ccs-scribe-r2` |

### Branch α dependencies

- M0'α → M1'α → M2'α → M3'α → M4'α → M5'α → M6'α. Strictly linear.
- M2'α overfit precedes M4'α install CLI (wwkd discipline: one-case gate before infrastructure).
- M3'α must handle config.json race (empty `tmuxPaneId` field for yet-unassigned panes); miss MUST fall through to session default, not error.

## Milestones — Branch A (if $TMUX_PANE-AT-BASHRC-NEGATIVE AND EXECPATH-OVERRIDE-HONORED)

| # | Title | Gate command | Expected output | Executor |
|---|---|---|---|---|
| M0'A | EXECPATH probe (R2 labrat) | done in task #10 | probe report shows CC honored override `CLAUDE_CODE_EXECPATH` at teammate spawn — verbatim `/proc/$PPID/cmdline` evidence | `ccs-labrat-execpath-probe` |
| M1'A | Skeleton shim binary (log-only) | `cargo build --release && cp target/release/xbreed-effort-shim ~/.local/bin/ && CLAUDE_CODE_EXECPATH=~/.local/bin/xbreed-effort-shim claude --version` | shim logs PPID argv + resolved-real-claude path; execs real binary; `--version` returns normal output | `executor` |
| M2'A | Overfit ONE teammate to medium | outer `CLAUDE_CODE_EXECPATH=<shim>`; spawn `ccs-probe-medium`; teammate `printenv \| grep CLAUDE_CODE_EFFORT_LEVEL` | teammate env shows `CLAUDE_CODE_EFFORT_LEVEL=medium` (FM#7b resolved, behavioral proxy now sanity only) | `executor` + labrat |
| M3'A | Generalize via manifest walker | `cargo test effort_manifest` | longest-prefix-match; unknown name → None | `executor` |
| M4'A | CLI + sync.rs EXECPATH injection (connector gate) | `xbreed effort-shim install && xbreed effort-shim doctor`; **AUDIT per g-connector-r2:** verify `write_claude_settings` is called at every teammate-spawn launch path, not just `sync` subcommand. Confirmed via `src/main.rs:25+33` — both `Sync` and `Claude` subcommands materialize. Validate no other launch entry bypasses. | install idempotent; doctor reports real-claude, shim, manifest size, last 5 log entries; settings.json `env:{CLAUDE_CODE_EXECPATH: <shim>}` written on every `claude` invocation | `executor` |
| M5'A | Recursion + self-resolution guards | `cargo test effort_shim` | tests: XBREED_SHIM_ACTIVE removed before exec (R1 reviewer), real-claude via hardcoded versioned-binary glob (NOT via `CLAUDE_CODE_EXECPATH` — that's self-reference loop), EXECPATH unset or reset before exec to prevent teammate→teammate recursion | `ccs-reviewer-r2` |
| M6'A | Ship | commits + memory update (`feedback_teammate_mode_effort_caveat.md` → REACHABLE via EXECPATH-shim); `project_execpath_shim_shipped_0417.md` | xbgst round with mixed-tier teammates; each at intended tier | `ccs-scribe-r2` |

### Branch A dependencies

- M0'A → M1'A → M2'A → M3'A → M4'A → M5'A → M6'A. Strictly linear.
- M4'A carries connector's audit baked-in as explicit sub-gate.

## Milestones — Branch B (if both probes negative)

| # | Title | Gate command | Expected output | Executor |
|---|---|---|---|---|
| M0'B | Confirm outcome | `cat docs/reports/execpath-probe-2026-04-17.md` | both subtests negative | (advisory) |
| M1'B | R4 ceiling-honesty fold | update `commands/references/xbreed-shared.md` §Session Effort Configuration | paragraph acknowledges per-teammate effort unreachable at user-space tier as of CC 2.1.112 | `executor` |
| M2'B | CC feature request | file issue at github.com/anthropics/claude-code | issue URL committed; references R1+R2 reports as evidence | `executor` |
| M3'B | Memory update | `feedback_teammate_mode_effort_caveat.md` → UNRESOLVABLE | entry updated with cross-links | `executor` |
| M4'B | Mission closed | commit + final scribe report | done | `ccs-scribe-r2` |

## R2 late-round amendments (folded before despawn)

- **ccs-reviewer-r2 — EXECPATH read-vs-write hard-gate (Branch A):** M0'A probe MUST distinguish CC *reads* `CLAUDE_CODE_EXECPATH` as injection target vs CC *writes* it as self-descriptor at spawn. If CC re-writes EXECPATH at each spawn, settings.json `env:{}` injection is overwritten — same structural failure class as R1 PATH-shim DOA, one layer deeper. Gate contract: verbatim `/proc/$PPID/cmdline` of teammate spawned under override env must show the override path as argv[0], not CC's hardcoded versioned binary. Ambiguous → Branch A aborts to Branch B.
- **g-connector-r2 — Branch A file-scope + zero-clobber (confirmed):** `grep EXECPATH src/ scripts/ config/` = zero matches → no existing clobber. Injection site: `src/launch.rs:11` (`Command::new("claude")` via `.env("CLAUDE_CODE_EXECPATH", current_exe())`) + `src/sync.rs:26` (settings.json env block — global, valid for EXECPATH only since all CC processes share one shim). Shim MUST `unset CLAUDE_CODE_EXECPATH` before exec'ing real claude to prevent teammate→spawn→shim loop.
- **g-connector-r2 — Branch α shell-integration zero-conflict (confirmed):** no existing xbreed-written bashrc blocks in repo; `src/loadout.rs:63 # xbreed loadout:` comment is SKILL.md header, not shell block. Branch α is xbreed's first shell integration. Fence markers + check-before-set (don't clobber user-set CLAUDE_CODE_EFFORT_LEVEL) + idempotent install are reviewer-mandated.
- **cco-critic-r2 — M2'α disambiguation gate (folded into M2'α):** config.json `name↔tmuxPaneId` write-order vs `tmux split-window`-fires-bashrc is the real observable-disambiguation gate (not the now-closed FM#7b). Add to M2'α: teammate's first bashrc invocation must log whether config.json was readable AND contained teammate's pane-id at bashrc source time. If bashrc fires before config.json is populated → reverse-lookup miss has same observable as "effort-var not propagating" but different root cause. Gate artifact: `~/.xbreed/shim.log` line must include `config.json-ready: {yes|no}` field so M2'α failure is diagnosable.

## Decision gate (branch selection)

Probe report output literal determines branch. Precedence: Branch α > Branch A > Branch B (cheapest-first).

- `$TMUX_PANE-AT-BASHRC-POSITIVE` AND config.json reverse-lookup works → **Branch α** (wins by YAGNI even if EXECPATH also honored).
- `$TMUX_PANE-AT-BASHRC-NEGATIVE` AND `EXECPATH-OVERRIDE-HONORED` → **Branch A**.
- Both negative → **Branch B**.
- Ambiguous → escalate to team-lead.

## Dependencies across R2 roster

- `ccs-labrat-execpath-probe`: ADD $TMUX_PANE subtest per team-lead. Emit two tokenized verdicts (one per subtest).
- `ccs-reviewer-r2`: Branch α audit surface = bashrc idempotency + fenced-block safety + config.json race-miss handling. Branch A audit surface = EXECPATH self-reference loop + sync.rs injection timing (connector's critique pre-integrated).
- `cco-critic-r2`: FM#7b resolved via code-cite, drop that thread. NEW adversarial ask: model Branch α failure modes (user's existing bashrc override clobbers the fenced block; config.json empty-pane-id race; pane reassigned mid-session).
- `g-connector-r2`: already delivered sync.rs timing critique; integrated into M4'A gate. Further surface: Branch α touches `src/sync.rs` (to write team config.json) + new `src/effort_shim.rs` module (manifest emit + install/uninstall), NOT `src/bin/xbreed-claude.rs`.

## Epistemic notes

- **Non-obvious claim:** WWKD skeleton-before-capacity does NOT mean "build the Rust harness first." Under YAGNI, the skeleton for Branch α is the 15-line bashrc block, not the 100 LoC Rust binary. The Rust layer is polish (M4'α CLI installer), not capacity. Over-engineering the skeleton violates both wwkd Principle 8 ("fits in one file, one screen") and Karpathy's code-is-ephemeral frame.
- **Rejected sequencing alternative:** Pursue Branch A (EXECPATH) directly and keep bashrc as tertiary fallback. Dropped because: (i) EXECPATH probe is still load-bearing and not yet back, so Branch A is speculative; (ii) the bashrc probe is cheaper (one `echo` in bashrc, one teammate spawn), so it belongs earlier in the probe order; (iii) team-lead explicitly ranked bashrc rank-1, and its YAGNI profile (15 lines bash vs 100 LoC Rust) is decisive under wwkd "fits in one file, one screen."
