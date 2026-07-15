# Shared Orchestration Protocol

Referenced by `/xbreed`, `/xbt`, `/xgs`, `/xbgst`. Do not duplicate — load this file.

## Godspeed Mode — Purest Form (2026-04-17)

**Purest form directive.** Every godspeed teammate dispatch appends the literal string ` | godspeed` to the Agent() prompt. No preamble, no explanation block. The executor lane uses ` | godspeed-impl` instead (red-before-green evidence discipline).

The single-token suffix IS the whole directive. Sonnet-medium teammates read it as: iterate cheap in parallel, no clarifying questions, no verbose plans, act via tool calls, drop philosophical reasoning. Any teammate needing more than that marker is mis-cast for the lane.

**Skill split (2026-04-17):** godspeed is backed by TWO user-scoped CC skills, by scope of use:

- **`godspeed` skill** (`~/.claude/skills/godspeed/SKILL.md`) — the **universal 4-line posture** for any teammate. Content is the four rules (name the axes / iterate cheap in parallel / keep axis-improving moves only / don't aim — let the frontier walk itself) plus "stop asking clarifying questions; act via tool calls". Triggered by "godspeed", `--with godspeed`, or the `| godspeed` suffix on any subagent prompt. Idempotent with the marker — the suffix encodes the posture even when the skill isn't loaded.

- **`godspeed-mode` skill** (`~/.claude/skills/godspeed-mode/SKILL.md`) — the **full behavioral directive** for `the-judge` (orchestrator) and `the-planner` (Phase 0 skeleton). Loads the velocity half (cheap stubs / catalog bootstrap / parallel batches / auto-recording / auto-frontier dominance), the filter half (antimetabolic Pareto constraint — improves ≥1 axis, harms none), operational constraints (no pre-specified goal, no "definition of done" mid-walk, suppress clarifying-question reflexes), and stop conditions (saturation or external boundary). Symlinks to `directive.md` / `filter.md` / `velocity.md` / `codex-AGENTS.md` live in the skill dir, sourced from the upstream repo cloned read-only at `/home/vhpnk/godspeed-mode/`. Canonical upstream: [VeigaPunk/godspeed-mode](https://github.com/VeigaPunk/godspeed-mode) — **READ-ONLY from our side, never edit / commit / push**; `git pull` into the clone is allowed (one-way consumer) and propagates through the symlinks automatically, but any proposed adaptation of the spec goes in OUR repo / memory, not upstream. Triggered when judge or planner operates in a godspeed-framed session; the judge's and planner's persona files load it via `Skill(skill="godspeed-mode")` on first turn when framing is detected.

Teammates spawned via Agent() with the ` | godspeed` suffix still discover the universal `godspeed` skill at turn-start if listed in the session's skill manifest; the suffix alone still suffices when the skill isn't loaded. Only the-judge and the-planner load `godspeed-mode` — the other roles don't need the orchestrator-depth spec.

Rationale: user directive 2026-04-17 — "opus is terrible for being the intermediator; make sure that all teammate (agent) dispatches inherit godspeed in their prompt in the purest form". The prior long "GODSPEED MODE (inherited from judge): You are a Godspeed-enabled subagent..." preamble burned reasoning cycles before the task even started. The marker replaces it; the skill makes it auto-surfacing at session scope.

## Escalation: advisor() (Layer 0)

All sonnet teammates can call `advisor()` (CC-native, zero parameters) for in-session fable 5 max escalation. The teammate's full conversation context is forwarded automatically.

**When to use advisor():** Before committing to non-obvious architectural decisions, when stuck, when a finding contradicts a peer, or before declaring work complete.

**advisor() vs xask:** advisor() is Layer 0 — it runs before and independently of the 4-layer xask gate. It is NOT cross-model delegation; it's in-session reasoning review. `xask claude` is deprecated (advisor() with Fable 5 Max supersedes it); use `xask codex` for contamination-controlled cross-model dispatch and `advisor()` for full-context reasoning escalation.

Include in teammate briefs: `"You have access to advisor() — call it before substantive decisions for fable 5 max review of your full context. Zero parameters, blocks until response."`

## xask Gate (4 layers)

Include as FIRST instruction in every teammate brief that requires cross-model delegation.

**Layer 1 — Gate (structural):**
- **scout**: `"Your FIRST tool call MUST be Bash: xask --effort medium --gs codex '<research question>'. No other tool before xask returns."` (scout applies built-in curation taste; escalate to `--effort high` for high-ambiguity research.)
- **reviewer**: `"Your FIRST tool call MUST be Bash: xask --gpt55 --gs -e low codex '<review question>'. No other tool before xask returns."` (`--gpt55 -e low` = gpt-5.6-sol + `features.fast_mode=true` + reasoning=low; uniform codex lane for review-class roles per 2026-04-24 pivot — supersedes the prior `-R codex` → gpt-5.6-sol routing. For diffs spanning >10 files, caller MUST pass `-scp <behavioral-change-files>` to scope the review (e.g. `git diff --name-only | grep -v generated | grep -v lock`). Closes the churn-padding attack vector where reviewer misses real bugs behind noisy renames/lockfiles.)
- **labrat**: `"Your FIRST tool call MUST be Bash: xask --spark --gs codex '<probe hypothesis>'. No other tool before xask returns."`
- **connector**: `"Your FIRST tool call MUST be Bash: xask --effort medium codex '<pattern question>'. No other tool before xask returns."` *(codex-medium primary; fallback on failure is **sonnet in-session** — compose from Grep/Read within the reasoning cap. Connector deliberately omits `--gs` to avoid stacking a second godspeed frame on top of the `| godspeed` suffix on a lane already prone to pontification (`feedback_connector_stall.md`).)*
- **the-revenger**: `"Your FIRST tool call MUST be Bash: xask --gpt55 --gs -e high codex '<RECON / surface enumeration question>'. No other tool before xask returns."` (`--gpt55 -e high` = gpt-5.6-sol + fast_mode + reasoning=high; uniform with other codex lanes per 2026-04-24. Supersedes the prior `-R -F codex` → full gpt-5.6-sol / 1.05M context route — RECON now works within gpt-5.6-sol's default window. For deep single-file reverse engineering, skip the xask gate and use advisor() instead.)
- **sentinel**: `"Your FIRST tool call MUST be Bash: xask --gpt55 --gs -e low codex '<exploit/vulnerability analysis question>'. No other tool before xask returns."` (gpt-5.6-sol-low, uniform codex lane)
- **critic**: `"Your FIRST tool call MUST be Skill(skill='heuer-planning') — this is Layer 0. After the skill loads, your SECOND tool call MUST be Bash: xask --gpt55 --gs -e low codex '<design review question>'. No other tool before xask returns."` (critic runs sonnet · medium per the unified scheme 2026-04-17 — the Axis → Profile Mapping below is authoritative; Layer-0 heuer-planning load applies to all critic teammates via on_spawn_skill frontmatter. If the skill is unavailable in the environment, the critic notes it and proceeds to Layer 1.)
- **mutation-tester**: `"Your FIRST tool call MUST be Bash, EITHER: (a) `xask --spark --gs codex '<generate mutation for this function>'` for a single targeted mutation (fast spot-check), OR (b) `xask --effort high --gs codex '<generate N mutations of <fn>; vary angle per mutation (boundary, operator-flip, return-swap, error-path, off-by-one); return HYPOTHESIS/METHOD/RESULT per mutation>'` for systematic breadth coverage. No other tool before xask returns. Pick (a) for ≤4 mutation targets, (b) for ≥5 or for breadth discovery."`
- **executor**: `"Your FIRST tool call MUST be Bash: xask --spark --gs codex '<task>'. No other tool before xask returns."`
- **the-planner**: `"Your FIRST tool call MUST be Skill(skill='wwkd') — this is Layer 0 (loads the What Would Karpathy Do planning posture: data-walk-first, end-to-end skeleton before capacity, overfit-one-case before generalizing, structural verification at every step). After the skill loads, proceed to Phase 0 data-walk + WWKD skeleton per the-planner.md template. NO Layer-1 xask gate — CC-native planning."` See `feedback_the_planner_wwkd.md`.
- **simplifier/distiller/scribe**: No xask gate, no Layer 0 skill load.

**Layer 2 — Raw-quote gate:** `"After xask, paste verbatim passage in <raw_output> tags. Must be literal substring of xask stdout. Empty = invalid. CLI output only."`

**Layer 3 — Fallback:**
- scout/reviewer: xask failure → DM judge with `BLOCKED: xask [reason]`, then continue in-session with `[xask dry — in-session fallback]` marker. Do not deadlock.
- labrat: xask failure → emit `obs: xask BLOCKED [reason]` as the finding, despawn. Failure IS the result.

**Layer 4 — Confidence:** `[xask dry]` marks source provenance, not quality. Judge assesses confidence case-by-case.

## Epistemic Constraints

Include in every teammate brief:

- **Epistemic role:** `"AT MOST one non-obvious claim + AT MOST one rejected alternative. Do not fabricate — return nothing if no well-grounded finding exists."`
- **Divergence mandate:** `"If your finding contradicts a peer's, flag: CONFLICT: [claim] — my position: [X] — peer: [Y]"`
- **Judge weighting:** Weight xask quotes contradicting agent's conclusion more heavily than confirming quotes.

## Axis → Profile Mapping

**This table is the single source of truth for agent routing.** AGENTS.md and the-judge.md carry read-only copies for discoverability. On any edit here, update those two.

Allowed `axis_family` values (must match frontmatter in `~/.claude/agents/*.md`): `research`, `correctness`, `empirical`, `execution`, `cross-axis`, `synthesis`, `complexity`, `reverse-engineering`, `security`, `orchestration`, `adversarial-design`, `test-validation`, `deletion`, `documentation`, `planning`.

**Sonnet-medium unified scheme (2026-04-17 pivot — supersedes opus-medium; judge downgraded xhigh→high 2026-04-19):**
All teammate dispatches run **sonnet medium** uniformly. Only `the-judge`
itself stays fable-**high** (orchestrator depth required; downgraded from
xhigh 2026-04-19 — user directive, reasoning-cycle savings without
sacrificing arbitration depth). User directive 2026-04-17: "opus is
terrible for being the intermediator" — sonnet at medium effort is fast
enough for the Pareto loop and avoids the reasoning-cycle overhead opus
imposes in teammate-mode. Effort tiers collapse to a single `medium` value
across the board for teammates; frontmatter `effort:` on individual agent
files still reads `medium` (per earlier unified scheme work). The
~/.bashrc DEBUG trap maps every teammate prefix (`cco-`, `ccs-`, `cdx-`)
to `CLAUDE_CODE_EFFORT_LEVEL=medium`; the judge keyword maps to `high`.

Codex dispatches unified on gpt-5.6-sol + `features.fast_mode=true` per 2026-04-24
pivot — one model, effort dial: review-class roles (reviewer/sentinel/critic)
route via `xask --gpt55 --gs -e low codex`; the-revenger RECON via
`xask --gpt55 --gs -e high codex`; labrat/executor/mutation-tester-single
via `xask --spark --gs codex` (gpt-5.3-codex-spark, reasoning=low). Supersedes
the prior `--review`/`-R` and `-R -F` split now routes on the single
codex family (`gpt-5.6-sol`) via `src/ask.rs` constants and `features.fast_mode`
(handled by `src/ask.rs` `CODEX_55_MODEL` / `CODEX_SPARK_MODEL`).

**Mandatory connector on every round:** the-judge MUST spawn a `connector`
teammate on each Pareto round (including Round 1), no exceptions. Cross-axis
pattern matching catches second-order effects, contradictions between axis
proposals, and whole-table regressions that the focused specialists miss by
construction. Skipping connector is a structural gap, not a speed optimization.

| Axis family | Role | Model | xask target | Tools |
|---|---|---|---|---|
| Research, prior art | `scout` | sonnet · medium | `xask --effort medium --gs codex` (scout applies built-in curation taste; escalate to `--effort high` for high-ambiguity research) | All |
| Correctness, bugs | `reviewer` | sonnet · medium | `xask --gpt55 --gs -e low codex` (gpt-5.6-sol + fast_mode + reasoning=low, uniform codex lane per 2026-04-24) | All |
| Empirical probes | `labrat` | sonnet · medium | `xask --spark --gs codex` | All |
| Code execution | `executor` | sonnet · medium | `xask --spark --gs codex` | All |
| Cross-axis patterns | `connector` | sonnet · medium | via Bash tool — xask is a shell CLI on PATH, not a native tool: `xask --effort medium codex` (primary; no `--gs` — avoids double-godspeed frame on pontification-prone lane) → **sonnet in-session** (fallback — composes from Grep/Read within the reasoning cap; emit `obs: xask BLOCKED [exact stderr]` only after that Bash invocation actually runs and errors) | All |
| Synthesis, dedup | `distiller` | sonnet · medium | in-session | All |
| Deletion, YAGNI | `simplifier` | sonnet · medium | CC native | All |
| Reverse engineering | `the-revenger` | sonnet · medium | `xask --gpt55 --gs -e high codex` for RECON (gpt-5.6-sol high, uniform lane per 2026-04-24); skip xask for in-repo single-file RE and use advisor() instead | All |
| Security auditing | `sentinel` | sonnet · medium | `xask --gpt55 --gs -e low codex` + `xask --effort medium --gs codex` for CVE prior art | All |
| Planning, Phase 0, WWKD sequencing | `the-planner` | sonnet · medium · Layer-0 wwkd skill load | CC native — spawned FIRST at Phase 0 by the-judge to map skeleton before specialist dispatch | All |
| Adversarial design | `critic` | sonnet · medium · Layer-0 heuer-planning skill load | `xask --gpt55 --gs -e low codex` | All |
| Test validation | `mutation-tester` | sonnet · medium | `xask --spark --gs codex` (single mutation, ≤4 targets) or `xask --effort high --gs codex` for ≥5-target breadth | All |
| Documentation, audit trail | `scribe` | sonnet · medium | CC native | All |
| Orchestration, arbitration | `the-judge` | **fable 5 · xhigh** (user directive 2026-06-07; model opus→fable 5 per 2026-07-04) | top-of-stack; dispatches specialists | All |

**Gemini auth (single-path, 2026-04-19 collapse):** `src/ask.rs` reads only `~/.gemini/oauth_creds.json`. No named profiles, no API-key fallback, no cascade retry, no health canary — the user's OAuth subscription is effectively unlimited, so dispatch either succeeds on the first try or bails with a `gemini login` hint. There is no secondary OAuth lane to probe; if a gemini call auth-errors, refresh creds and retry. **Lane retired 2026-07-04 (user directive): no role routes to gemini anymore — the runtime keeps the capability, definitions must not use it. This paragraph is runtime documentation only.**

## Enforcement Tiers

When proposing or evaluating any "enforcement" claim in xbreed (xask gate, deny-list, timeout, drift-detection, etc.), categorize against three tiers — **all three reachable from this repo**:

| Tier | Reach | Guarantee | Bypass surface | xbreed examples |
|------|-------|-----------|----------------|-----------------|
| **Build/CI** | rustc + cargo test + shell lints | Fails the build / test suite if violated; cannot ship a binary that breaks the invariant | Skip `cargo build` / skip `make verify` (CI-side mitigation) | `src/protocol.rs` `include_str!` SSoT binding (R3 A2'); `cargo test` content-sentinel asserts; `scripts/verify-docs.sh` connector-routing drift check + `make verify-docs` (R2 A2) |
| **Runtime** | Rust binary + bash wrappers | Wraps subprocesses xbreed launches; per-call timeout / kill / cleanup | Bypassable by skipping the wrapper (`unset XASK_TIMEOUT_SECS`, raw shell call to gemini/codex, alternative tool) | `src/guard.rs` deny-list at PreToolUse hook; `src/ask.rs` `execute_with_timeout` + `Child::kill` (R2 A1 + R3 A1'); `scripts/xask` 4-layer transport gate; `scripts/xask` HOME-scoped tmpdir + sweeper |
| **Protocol** | Brief strings + agent templates + docs | Convention only — agents read and follow | Non-compliant agent can skip by ignoring brief text | xask Layer 1 gate per-role briefs (`xbreed-shared.md:25-43`); cco-critic Layer 0 heuer-skill load directive; skill briefs; the-judge.md sub-role table |

**Out of scope (NOT a tier we claim):** A "harness-broker" enforcement surface (Anthropic-side CC tool broker that could intercept tool calls before Bash/Read/Grep/Agent dispatch) does not exist for cross-CLI enforcement from user-space. Also out-of-scope by the same ceiling-honesty rule: **native CC tool registration** (no user-space surface in CC 2.1.112 to register a non-MCP first-class tool alongside Bash/Read/Grep — MCP is the only documented path, and charter-rejected per user directive 2026-04-17); **Anthropic-side batch-spawn queue inside the split-pane allocator** (over-cap teammate spawns fail with "no space for new pane" rather than queueing — the reachable user-space mitigation is a preflight cap check, not queue semantics). Listing any of these as a tier would canonize aspirational external surfaces and worsen ceiling honesty rather than improve it. If Anthropic exposes such interfaces in the future, this section gets a 4th tier; until then, the honest ceiling is Build/CI.

**Standing axis label:** "**Runtime-tier hardening with documented ceiling**" — used in DRAFT/synthesis to honestly frame what a given runtime move achieves. Build/CI moves use "**Build/CI-tier enforcement**" as the parallel label. Avoid claims that conflate tiers (e.g. "we closed the compile-time gate" when the move is runtime-tier; or "we enforce X" when the move is protocol-tier). The ceiling MUST be documented when proposing a runtime-tier or protocol-tier move.

**Why three tiers, not two or four:** Build/CI and runtime are commonly conflated ("we have a test for it" vs "the binary refuses to do it"); runtime and protocol are commonly conflated ("xask wraps the call" vs "the brief says to use xask"). Each pair has different bypass surface and different guarantee strength — the granularity matters. The aspirational broker tier was tried in an earlier draft and rejected per R3 A4' heuer-ACH analysis (overclaim of unreachable surface).

**Origin:** R2 ask-resilience-r2-0416 cco-critic-compile-gate (audit_hash cfe3e176...) coined "runtime-tier hardening with documented ceiling." R3 ask-resilience-r3-0416 cco-critic-r3-overclaim (heuer Layer 0 loaded) refined the tier set: dropped the harness-broker overclaim, added Build/CI as the third real tier where include_str! / cargo test / verify-docs.sh actually enforce.

## Session Effort Configuration

**Per-teammate `effort:` frontmatter is a no-op in teammate-mode** (confirmed harness-r2-0417 R2: `ccs-labrat-effort-mechobs-r2` + `xask --spark codex` docs anchor). CC propagates only `tools` + `model` into teammate-mode spawns; `effort:` is honored on the subagent-delegation path, not the teammate path. `src/sync.rs:20` forces `teammateMode: "tmux"`, so every teammate inherits the outer session's effort — which is `settings.json effortLevel`, which defaults to `xhigh` on this user's profile. Aspirational per-role effort memories (`feedback_sonnet_effort_tiers.md`, `feedback_cco_opus_high.md`, `feedback_the_planner_wwkd.md`) are **non-operative** until/unless CC exposes per-teammate effort in teammate-mode spawn args.

**Reachable workaround (session-wide, not per-teammate):** set `CLAUDE_CODE_EFFORT_LEVEL=<tier>` in the shell env BEFORE invoking `claude`. The env var is documented to override `settings.json effortLevel` at session init and applies to every teammate in that session (env inheritance verified via `/proc/$PPID/environ` showing `CLAUDECODE=1` propagating into teammate processes — harness-r2-0417 R3 `ccs-labrat-effort-env-r3`). Example: `CLAUDE_CODE_EFFORT_LEVEL=medium claude` caps all teammates at medium; unset to return to `settings.json` default. There is no per-teammate override in this path — if you need fable-xhigh for `cco-critic-*` and sonnet-medium for `ccs-distiller` in the same session, that is currently **not reachable from user-space**.

## Naming Convention

`{prefix}-{role}-{suffix}` where prefix = `ccs-` (Claude Sonnet), `cco-` (Claude Fable 5, effort: **xhigh** — LOCKED, user directive 2026-06-07; model opus→fable 5 per 2026-07-04), `cdx-` (Codex). <!-- g- (gemini) prefix retired 2026-07-04 — gemini delegation killed; cco- synced to the-judge.md which carries the newer LOCKED directive -->

Prefix signals where reasoning lives (the target model for xask delegation), not which CC runtime spawned the teammate. `cco-` is reserved for `the-judge` under the sonnet-medium pivot; the other three prefixes route their primary reasoning to the named model.


## Labrat Invocation (Universal)

Any agent can spawn a labrat probe. Two paths:

1. **Subagent spawn:** `Agent(subagent_type="labrat", name="cdx-labrat-<hypothesis>", model="sonnet", prompt="<probe>")`
2. **Bash call:** `xask --spark codex "<probe hypothesis>"` — codex-5.3-spark, fire-and-forget

**Codex-spark is the sole labrat channel (user directive 2026-04-18).** No gemini labrat delegation. The codex-5.3-spark lane is fast, cheap, and expendable enough to be the complete labrat surface — both for single probes and in-model fanout.

**Codex labrat swarm (universal):** Any agent can fire a codex-spark swarm via `xask codex "Orchestrate 10 parallel labrat probes on: <hypothesis>. Vary angle per probe. Report HYPOTHESIS/METHOD/RESULT."` — 1 call runs 10 probes inside codex-spark's context. Up to 3 refire rounds (30 probes total) — independent of judge rounds.

## Distiller Spawn Template

```
Agent(
  subagent_type="distiller",
  team_name="<team>",
  name="ccs-distiller",
  model="sonnet",
  prompt="You are the distiller. Sonnet effort: medium (per feedback_sonnet_effort_tiers.md — synthesis is structural pattern-matching over peer outputs; sonnet medium is sufficient for spoof-checking, contradiction surfacing, consensus capping, and brief-error catching). Synthesize these N teammate proposals and peer critiques into one deduplicated, confidence-scored brief. <paste all proposals + DM critiques>. Deduplicate overlapping moves, flag contradictions (cross-model if xask used, cross-teammate if all-Claude), assign confidence. Preserve each surviving move's `evidence:` field verbatim (see Pareto Filter Evidence Schema) — do not absorb into prose; the filter reads it post-synthesis. Apply opus-harness rigor: spoof-check cited file:line excerpts via literal-substring grep; cap single-prefix consensus at MED; upweight cross-model divergence. Use SYNTHESIS_READY mapping for judge consumption. SendMessage your synthesis to the judge (team lead) when done."
)
```

## Pareto Filter Evidence Schema

> **Scope:** Enforced in `/xgs` and `/xbgst` (Pareto-walk modes). Informational in `/xbt` (deliberative) and `/xbreed` / `/xb` (solo pipeline) — judge mediates directly, no drop gate runs. Distiller passthrough still fires, so the field travels intact if present.

The Pareto filter reads a structured `evidence:` field on every proposed move. Moves without required evidence are **dropped, not scored** — the verification discipline is enforced by the filter, not by the agent's willingness to comply.

**Schema (task-aware by role):**

| Role axis_family | Required evidence form |
|---|---|
| `execution` (executor) | failing-test output + passing-test output (red-before-green); OR diff + rationale if no harness |
| `correctness` (reviewer), `test-validation` (mutation-tester), `security` (sentinel) | verbatim xask output OR test/lint stdout + exit code |
| `empirical` (labrat) | probe HYPOTHESIS/METHOD/RESULT triple |
| `deletion` (simplifier) | diff of removed symbols + test pass/fail output (pre- and post-removal) |
| `research` (scout), `cross-axis` (connector), `synthesis` (distiller), `orchestration`, `adversarial-design` (critic), `complexity`, `reverse-engineering`, `documentation` (scribe) | `evidence: none — <axis reason>` (non-executable) |
| `planning` (the-planner) | `evidence: none — planning artifact` |

**Exempt-role allowlist is a closed enum keyed on `axis_family`**, not free-text self-classification. Any new role must land with a schema update to this table or ship with executable evidence. Distiller passes the field through verbatim.

### Evidence Audit Line (MANDATORY in judge round summaries)

After distiller synthesis and before the Pareto walk, the judge MUST emit exactly one line:

```
EVIDENCE AUDIT: <N> moves with evidence, <M> moves without, <M> dropped, <K> spoof_flagged
```

If `M > 0` and the distiller did not log drops, flag as protocol violation. If `K > 0`, route the spoof-flagged `move_id`s to reviewer before scoring. Rationale: mutation-tester Round-1 finding — dropping the `evidence:` field requirement is otherwise a *silent* regression; no counter exists in legacy round summaries. Converts a silent-drop path into a visible counter.

### Evidence Authenticity Spot-Check (counter-spoofing)

xbgst Round-1 live incident: `ccs-simplifier-bloat` proposed a 5-file deletion diff whose "before" text was fabricated — `Grep "Gemini labrat swarm" ~/.claude/agents/` returned 1 file, not 5. Filter must assume evidence spoofing is a possible failure mode (malfunctioning agent, not external attacker).

- **Distiller spot-check (mandatory):** On any proposal whose `evidence:` cites a file state: (1) proposer must supply a **specific line span and exact excerpt** — bare path citations are rejected; (2) distiller matches the excerpt as a **literal substring** (`rg -F` / fixed-string, not regex) within that span; (3) if the excerpt appears only in comments/tests/docs but the claim is about implementation state, flag as non-supporting. If observed state ≠ claimed state, emit `evidence_unverified: <reason>` and do not pass the move through verbatim; flag to judge under `spoof_flagged`. Known gap: teammate-created fresh files (path created after proposal) require git/mtime provenance — route to reviewer if file is untracked or mtime postdates the proposal timestamp.
- **Reviewer triage:** Judge routes spoof-flagged `move_id`s to reviewer (blocking) before Pareto walk can accept them.

## Judge Blinding Protocol

**Source of truth for `the-judge.md` line 82 ("see xbreed-shared.md §Judge Blinding Protocol").** Closes the dangling-reference defect (R1 reviewer finding).

### Hold rule

The judge MUST NOT form or record per-axis scores based on raw teammate DMs. Wait for distiller `SYNTHESIS_READY` (schema defined in `distiller.md`), which carries only `move_id / axis / claim / confidence / linchpin / evidence` — **no source or model labels**.

### Score against `move_id`

Record provisional Pareto verdicts against `move_id`s. Emit `EVIDENCE AUDIT` line before scoring. Commit provisional survivors to a draft block.

### Source reveal (late-binding)

After provisional scores are posted, the judge requests `SOURCE_MAP` from distiller via SendMessage. The map returns `move_id → source` (role + model prefix).

**Use the map ONLY for:**
1. Contradiction routing (CONFLICTS block): which model said what.
2. Follow-up dispatch decisions (which role owns the next-round ask).
3. Cross-model vs same-model confidence adjustment (inputs to R2+, not retroactive to R1 scores).

**Never use the map to:**
- Retroactively adjust R1 scores (halo-leak).
- Privilege or penalize model prefix as a scoring axis.
- Break tie in Pareto filter.

### Cross-model vs same-model confidence

When SOURCE_MAP reveals a claim's supporting sources share a model prefix (all `ccs-`, all `cdx-`, etc.), the distiller has already capped confidence at `medium` (per distiller.md rule). The judge MUST NOT upgrade same-model consensus to `high` post-reveal. Cross-model confirmation is the only path to `high`.

### Audit-commit handshake (distiller ↔ judge)

Closes the structural-gap labrat R2 finding (SOURCE_MAP late-binding was prose-only).

**Step 1 — SYNTHESIS_READY commit:** Distiller posts per-move confidence plus `audit_hash`. Compute as: sort `[{move_id, source_prefix}]` by `move_id`, serialize as the literal sorted string, SHA-256 it. Post hash alongside the synthesis payload. The hashed list IS the exact same-model-audit evidence used under the cap.

**Step 2 — Provisional scoring:** Judge posts provisional Pareto scores citing `audit_hash`. Judge MUST NOT inspect the source mapping before this post.

**Step 3 — SOURCE_MAP reveal + verification:** Judge sends `SOURCE_MAP` request. Distiller returns the `{move_id, source_prefix}` map. Judge recomputes the hash from the returned map using the same serialization. Hashes match → provisional scores stand. Hashes diverge → round invalid, rerun from SYNTHESIS_READY.

**Step 4 — Spot-check (closes false-attestation vector):** After SOURCE_MAP reveal, judge picks one random `move_id` and sends a direct `confirm_model` DM to the original proposer. If the proposer's self-reported model prefix contradicts the distiller's map, flag round as `SPOOF_SUSPECT` and route to reviewer BEFORE Pareto walk continues.

Hash-commit alone closes early-reveal; spot-check closes distiller fabrication. Both together bound the attack surface to colluding-team — outside the threat model under SendMessage-only infra.

## Parallel Dispatch Reference

Phase 2 concurrent dispatch follows the crafted-brief + isolated-context + parallel-Agent pattern documented in the Superpowers `dispatching-parallel-agents` skill — cited as reference only; this file remains the SSoT for xbreed dispatch.

## DESPAWN Protocol

Any agent (labrat, reviewer, or other) may send DESPAWN signal after completing all assigned work. Judge acknowledges and releases the session slot. Format:

```
DESPAWN: <agent-name> — signal delivered. Send me shutdown_request.
```

## Team Cleanup

**Graceful path:** `SendMessage shutdown_request` to each teammate → wait for `shutdown_approved` → `TeamDelete`.

**Force path (when TeamDelete fails with "active members"):** Run `xbreed-cleanup <team-name>` via Bash. This kills stale processes and removes team + task dirs. Use when:
- A teammate process hung or was killed externally
- `TeamDelete` refuses due to stale config.json member entries
- Orphan task dirs accumulate from prior sessions

**Periodic maintenance:** `xbreed-cleanup --stale` cleans all teams with no live processes + orphan UUID task dirs.

## Per-Round Scribe & Commit

**Scribe per round (mandatory in xgs / xbgst / xbt).** After each round's distiller `SYNTHESIS_READY`, the judge dispatches a fresh scribe teammate `ccs-scribe-r{N}` (sonnet · medium, filter-exempt per `axis_family: documentation`), **concurrent with Pareto scoring** — do NOT serialize. Report path: `docs/reports/<mission-slug>-r{N}-<YYYY-MM-DD>.md`. Mission slug = team name minus timestamp. Report sections: round overview (axes + teammates + xask targets + wall-time), per-teammate MOVE/AXIS/CLAIM/EVIDENCE/REJECTED-ALTERNATIVE/confidence, cross-model CONFLICTS + judge resolution, Pareto verdict per move, optimization routes surveyed, spoof-flags, audit_hash, commit delta (if round landed a commit).

**Commit per round (mandatory).** After each round's execution lands (scribe report + any code changes), judge commits with pattern `<type>(<mission>-r{N}): <description>` — auditable per-round trail so rounds can be reviewed/reverted/cherry-picked individually. The scribe report is committed AS the round's evidence body.

## Codex-Topic Dispatch

For xbgst/xgs/xbt runs whose topic IS codex itself (defaults, flags, latency, routing, effort tiers, invocation shape), the judge MUST include at least one `cdx-*` prefix teammate in the Phase-1 roster whose reasoning layer is `xask codex`. Codex is the primary source on its own CLI surface. Not required for non-codex topics — topic-gated.

## Round Limits

- **Godspeed Pareto** (xgs, xbgst): 4 rounds max
- **Deliberative** (xbt): 4 rounds max (sequential depth)
- **Solo pipeline** (xbreed, xb): 12 sub-role dispatches max
- **Labrat Codex swarm**: 3 refire rounds (30 probes) — independent of judge rounds

## Exit Condition (strict, applies to xgs/xbgst/xbt)

The frontier has stopped moving **iff Round N produced zero axis improvements vs Round N-1** (all survivors duplicate prior-round survivors, or filter accepted nothing new). "Distiller reports no open questions" is NOT the exit condition — clean synthesis still typically moves axes off the pre-walk baseline.

**Materiality rule.** A surviving move counts as an improvement only if at least one axis observable (the triplet defined in Phase 0: name + direction + observable) has changed state vs Round N-1. Proposal-prose difference alone does not qualify — paraphrased findings against unchanged observables are not improvements.

**Anti-premature-halt rule.** After each round, before declaring frontier-stable, judge MUST:
1. Compare Round N survivors to Round N-1 survivors (or pre-walk baseline for Round 1).
2. If any axis improved → dispatch Round N+1 immediately. Do not emit final DRAFT. Do not ask the user.
3. Only on true zero-improvement OR round cap → emit final DRAFT + auto-cleanup.

Round 1 by construction improves axes off baseline, so **Round 2 always runs** unless the user halts or a cap triggers. Jumping to cleanup after Round 1 is a protocol violation.

## Coherence Check

After parallel execution rounds (multiple executors editing files concurrently), spawn a reviewer for cross-file consistency before committing. Checklist:

1. **Cross-file reference consistency** — dispatch tables, xask gate strings, and tool lists agree across xbreed-shared.md, AGENTS.md, the-judge.md, and skill templates
2. **Stale agent name/model references** — no haiku/sonnet mismatches, no removed agent names, delegation targets current
3. **Canonical agent state** — `~/.claude/agents/*.md` is the single source of truth (formerly mirrored in `templates/agents/*.md`; repo mirror removed 2026-04-17 to kill source-of-truth ambiguity)

This is not a blocking gate — the judge decides when the scope of changes warrants it. Multi-file parallel edits always warrant it.
