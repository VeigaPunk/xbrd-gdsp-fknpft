# Map — Why /xbgst + xask + delegation works flawlessly, and where it nearly didn't

**Mission:** `map-flawless-why-0424` · forensic architecture map of the xbreed orchestration protocol
**Date:** 2026-04-24
**Method:** `/xbgst` (godspeed Pareto + cross-model delegation), 1 round, 8 axes, 10 teammates, 7 of 7 moves accepted
**Grounding incident:** `c725c2a docs(xbgst): rewire codex lanes to gpt-5.5 + gemini medium + --gs` → revert `18e8df4` → reset `2d16ad0` (14-commit cascade rollback)
**audit_hash:** `66aee9e30bc47c7fe7caad69c778358919c7c7ef5a51c5616e31f5bbf3974677` (distiller-attested; serialization underspec — see §E gap row)

---

## TL;DR

1. **Layer-2 raw-quote ties evidence to literal CLI stdout** → capacity-agnostic gate. A low-effort or verbosity-pathological model cannot hallucinate a passing artifact; the shell environment is the semantic primitive. This is the **novel delta** vs MetaGPT SOPs (which still live in the LLM context window).
2. **The Pareto walk is cheap because the evidence schema drops bad inputs upstream of axis-scoring** — but the schema has TWO confirmed bypass paths (prose burial; post-proposal file minting). H3C is a **Protocol-tier aspiration**, not structural enforcement.
3. **SSoT enforcement is structurally asymmetric**: Build/CI covers heading existence (`src/protocol.rs include_str!`) + ONE routing row (connector via `verify-docs.sh`). 12 of 13 role profiles are Protocol-tier convention only. **`the-judge.md` is currently DRIFTED at 6+ cells** (verified primary-source) and `verify-docs.sh` does not catch it.
4. **The c725c2a near-miss was a composition failure, not single-axis**. Model swap × effort-tier collapse (xhigh→low) × ThinkingBudget cut (8192→4096) × double-godspeed stack co-rotated; each axis individually passed its check. The 14-commit reset was the operator's defense, not the protocol's.
5. **The most dangerous unaddressed gap is the Goodharted evidence-complete decoy** — fresh files minted post-proposal with genuine local axis improvements, but the real defect is off-scorecard. Spot-check catches fabrication, not strategic off-axis coverage. Anti-premature-halt runs toward wrong frontier; 4-round cap terminates unresolved.

The system works MORE because of operator habit reading `xbreed-shared.md` as canonical AND less because of universal Build/CI enforcement than the docs claim.

---

## §G — Gate Layering

> **Source:** M01-priorart (g-scout-priorart-r1, gemini medium · MED)
> **Linchpin:** `scripts/xask` Layer-2 raw-quote = literal CLI stdout substring requirement

The xbreed gate is a 4-layer structure embedded **verbatim** in every teammate brief (per role, per `xbreed-shared.md` L36–46). Indirection ("read the reference and apply") was empirically observed to silently drop the gate (commit `128e724`, fixed by the per-brief-grep self-check at `commands/xbgst.md` L74).

| Layer | Mechanism | What it catches |
|---|---|---|
| **L1** | Verbatim `xask <flags> <model> '<query>'` string per role | Wrong model / wrong lane (mini vs full) / wrong reasoning effort / missing flag |
| **L2** | `<raw_output>` literal-substring requirement | Hallucinated CLI response — model **must** produce a real artifact |
| **L3** | `[xask dry — <reason>]` fallback marker | Provenance of in-session fallback when CLI failed |
| **L4** | Confidence calibration (`[xask dry]` ≠ low quality, just dry source) | Overweighted confirming quotes; downweighted contradicting ones |

**Prior-art lineage (from M01):** AutoGen and LangGraph rely on **reference-by-pointer** dispatch (shared state, full conversational history) → induces handoff drift when a suffix model degrades on prefix-authored context. MetaGPT's rigid SOPs (verbatim role strings, not computed refs) were the first major structural counter. **xbreed is the runtime descendant** — but **materially novel**: where MetaGPT SOPs live IN the LLM context window (still subject to generation errors), xbreed's L2 grounds the evidence gate in literal shell stdout. **The LLM does not report whether the gate passed — the shell environment does.** No reviewed system (AutoGen / CAMEL / LangGraph / OpenAI Swarm) ties its evidence gate to literal shell output as the semantic primitive. This is the load-bearing delta.

**What breaks it:** removing L2 → reviewer hallucinates `evidence:` field content (incident: `ccs-simplifier-bloat` Round-1, `xbreed-shared.md:201`).

---

## §D — Delegation Routing

> **Source:** M02-compose (g-connector-crossaxis-r1, gemini high · MED) + M03-driftaudit (cdx-reviewer-driftcheck-r1, codex -R · HIGH)
> **Linchpin:** `xbreed-shared.md` Axis → Profile Mapping table at L95–110 (canonical) + `the-judge.md` L18–34 (mirror, currently drifted)

Every role has exactly one xask invocation, locked across canonical SSoT + mirror + slash-command — **in theory**. In practice, the canonical-mirror chain is currently drifted at 6+ cells, primary-source verified during this round.

### Drift table (verified at HEAD = 2d16ad0)

| Cell | `xbreed-shared.md` (canonical) | `~/.claude/agents/the-judge.md` (mirror) | Status |
|---|---|---|---|
| reviewer delegation | `xask -R codex` (review lane → gpt-5.4-mini, profile `xbreed_review`, `model_reasoning_effort = "xhigh"`) | `xask --effort high codex` (full lane, no profile) | **DRIFTED** |
| sentinel delegation | `xask -R codex` | `xask --effort high codex` | **DRIFTED** |
| critic delegation | `xask -R codex` (after Layer-0 heuer-planning) | `xask --effort high codex` | **DRIFTED** |
| the-revenger model | sonnet · medium | `(opus 4.7 medium)` | **DRIFTED** (pre-2026-04-17 unified-scheme tier) |
| the-planner model | sonnet · medium | `(opus 4.7 high)` | **DRIFTED** (pre-unified-scheme tier) |
| mutation-tester gemini-fanout threshold | `≥3 targets / breadth` | `≥5 targets` | **DRIFTED** (also in `commands/xbgst.md`) |
| the-judge.md L81–85 | (no equivalent — codex-spark sole labrat per L152) | "Gemini labrat swarm (universal)" — instructs `xask gemini` for labrat | **STALE — primary deletion candidate (M05)** |

**Composition-failure pattern (M02):** the c725c2a rewire was the canonical example. Each axis individually passed its check — connector fired, suffix appended, `-e low` flag applied — yet the cross-axis effect was systemic regression. No single-axis inspector catches this. The composition matrix:

| Axis pair | What couples them | What would decouple on rewrite |
|---|---|---|
| **G × D** | Gate string is runtime-authoritative; role defs are doc-only | CI assert: `verify-docs.sh` greps gate strings in briefs (currently only checks connector cell) |
| **E × P** | Pareto trusts schema-attested evidence; schema weakness = pass-through | Hard enforcement: `evidence: none` invalid for execution-role moves (currently soft) |
| **M × S** | Memory hardens at Protocol-tier; SSoT binds at Build-tier (one cell only) | Transcription path: Protocol memory → Runtime lock in `src/ask.rs` (does not exist for 12/13 cells) |

**`-R` is not a model-lane flag — it's a profile selector** (planner addendum). It binds xask to `~/.codex/config.toml [profiles.xbreed_review]`, which carries `model = CODEX_MINI_MODEL` AND `model_reasoning_effort = "xhigh"`. Dropping `-R` silently drops both the model AND the reasoning contract. The c725c2a rewire's `-e low` then overrode whatever default was inherited. Two-layer indirection → invisible at line-by-line review.

---

## §R — Role Cast

> **Source:** M04-recon (cdx-revenger-protocol-r1, codex -R -F · HIGH — cross-model converged with M03+M05)
> **Linchpin:** the 1:1:1:1 mapping — every role has exactly one `axis_family` × one `xask_profile` × one `evidence_form`

| Role | `axis_family` | xask_profile (canon) | Evidence form | Tier of enforcement |
|---|---|---|---|---|
| `scout` | research | `xask --effort medium gemini` (`# ThinkingBudget: 4096`) | `evidence: none — research axis` | Protocol |
| `reviewer` | correctness | `xask -R codex` (mini, profile xhigh) | verbatim xask OR test/lint stdout+exit | Protocol |
| `labrat` | empirical | `xask --spark codex` | HYPOTHESIS/METHOD/RESULT triple | Protocol |
| `connector` | cross-axis | `xask --effort high gemini` (`# ThinkingBudget: 8192`); fallback = sonnet in-session | `evidence: none — cross-axis` | **Build/CI** (verify-docs.sh) + Protocol |
| `distiller` | synthesis | in-session only | `evidence: none — synthesis` | Protocol |
| `executor` | execution | `xask --spark codex` | failing+passing tests OR diff+rationale | Protocol |
| `simplifier` | deletion | CC native | removed-symbol diff + test output | Protocol |
| `the-revenger` | reverse-engineering | `xask -R -F codex` (full gpt-5.4 / 1.05M ctx) for RECON; `-R` (mini) for narrow surface enum | `evidence: none — RE axis` | Protocol |
| `sentinel` | security | `xask -R codex` + `xask gemini` for CVEs | verbatim xask OR test/lint stdout+exit | Protocol |
| `critic` | adversarial-design | Layer-0 `Skill('heuer-planning')`, then `xask -R codex` | `evidence: none — adversarial-design` | Protocol |
| `mutation-tester` | test-validation | `xask --spark codex` (≤4 targets) OR `xask --effort low gemini` 10-probe fanout (`≥3` per shared.md / **`≥5` per the-judge.md** — cross-doc CONFLICT) | verbatim xask OR test/lint stdout+exit | Protocol |
| `scribe` | documentation | CC native | `evidence: none — audit artifact` | Protocol (filter-exempt) |
| `the-planner` | planning | Layer-0 `Skill('wwkd')`, no Layer-1 xask | `evidence: none — planning artifact` | Protocol |
| `the-judge` | orchestration | top-of-stack; opus 4.7 high | dispatches specialists | — |

**Why the 1:1:1:1 mapping is robust** (when honored): no role can drift its own xask profile without the brief-construction step exposing the change. The judge constructs each brief by reading `xbreed-shared.md` Layer-1 strings into the Agent() prompt verbatim. **As long as the judge reads canonical** (not the drifted mirror), routing is correct. The brittle assumption is that the judge habitually reads canonical — which is exactly what kept c725c2a from poisoning Round 1 of the very forensic mission auditing it.

---

## §E — Evidence Schema

> **Source:** M07-ach (cdx-critic-design-r1, codex -R + Layer-0 heuer-planning · MED) + sentinel addendum (cdx-sentinel-attacksurface-r1)
> **Linchpin:** `xbreed-shared.md` L170–248 (Pareto Filter Evidence Schema + audit-commit handshake + spot-check) — the structurally load-bearing immune system… with confirmed bypass routes.

### What the schema does (when honored)

- **Drop-on-missing:** moves without required `evidence:` per `axis_family` are dropped, not scored. The Pareto filter never pays axis-eval cost on fabricated inputs.
- **Distiller spot-check:** any proposal citing file state must supply line span + exact excerpt; distiller runs `rg -F` literal substring within that span. Fabrication fails the gate before reaching the judge.
- **`audit_hash`:** distiller commits SHA-256 of sorted `[move_id, source_prefix]` pre-judge-scoring. Judge recomputes after SOURCE_MAP reveal; divergence invalidates the round.
- **Random `confirm_model` spot-check:** judge picks one random `move_id` post-reveal, DMs the proposer; contradiction flags `SPOOF_SUSPECT` and routes to reviewer before Pareto walk.

The xbrd-vs-hvm4-R2 round paid for the gate in one round: 2 spoof-flagged resolutions upstream of 5 accepted R2 moves.

### Confirmed bypass paths (critic RETHINK, HIGH confidence)

`xbreed-shared.md` L174 claims: *"the verification discipline is enforced by the filter, not by the agent's willingness to comply."* **This claim is structurally false.** Two confirmed bypass paths exist:

| Bypass path | Mechanism | Historical instance |
|---|---|---|
| **Prose burial** | Proposer omits structured `evidence:` field, embeds citations in body narrative. Distiller passes through — `rg -F` check never fires. | `128e724` silent G-axis gate drop (Round-1 of `xask-gate-regress-0420`) |
| **Post-proposal file mint** | Evidence file created after proposal; mtime postdates proposal timestamp. Spec L203 says "route to reviewer" but **no blocking gate exists** in `src/` or `scripts/` — Protocol-tier prose only. | Verified by simplifier grep: zero hits on `mtime|postdate|provenance|proposal.timestamp` in src/ or scripts/ |

The protocol's core fabrication defense is **distiller-compliance-dependent**, not structurally enforced.

### Sentinel 3-gap taxonomy (cdx-sentinel-attacksurface-r1)

| Gap | Severity | Verification |
|---|---|---|
| **(a) Solo pipeline bypass** — `/xbreed`/`/xb` documented as "informational; no drop gate runs" | HIGH | shared.md L172 direct citation |
| **(b) Post-timestamp provenance race** | **CRIT** (upgraded from MED) | simplifier grep: zero implementation in src/ or scripts/. Prose describes a guard that does not exist |
| **(c) Effort-level attestation unfalsifiable** — `effort:` frontmatter no-op in teammate-mode; `CLAUDE_CODE_EFFORT_LEVEL` is session-wide; not in `audit_hash` coverage; DEBUG trap is `~/.bashrc` only | **HIGH** (upgraded from MED) | revenger primary-source: `src/sync.rs:26-29` — only `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` in env passthrough; no `CLAUDE_CODE_EFFORT_LEVEL` reference |

### Bonus META-finding: `audit_hash` serialization underspec

The Round-1 hash recompute by the judge failed to match the distiller's claim across **7 standard serialization variants** (compact JSON, no-dash, tuple, pipe, arrow, full-name, prefix-only-sorted, concat-dash). `xbreed-shared.md` L240 just says "serialize as the literal sorted string" — no format spec. The mechanism is functionally non-falsifiable unless distiller and judge use literally the same code path. **The cryptographic binding is honor-system at the serialization layer.**

The semantic binding (move_id → source_prefix) was verified anyway via independent path: `~/.claude/teams/map-flawless-why-0424/config.json` lookup. Team config is a runtime artifact distiller cannot forge — that's the **real** binding for this mission.

---

## §S — SSoT / Drift Resistance

> **Source:** M04-recon + M05-yagni (3-way cross-model convergence: simplifier + reviewer + revenger)
> **Linchpin:** `xbreed-shared.md` L114–131 (3-tier enforcement) — canonical claim. Reality: severe asymmetry.

### What the docs claim

> *"`src/protocol.rs include_str!` SSoT binding (R3 A2'); `cargo test` content-sentinel asserts; `scripts/verify-docs.sh` connector-routing drift check + `make verify-docs` (R2 A2)"*

### What the code actually enforces

| Invariant | Tier | Enforcer | Status |
|---|---|---|---|
| `xbreed-shared.md` heading existence (11 sections) | Build/CI | `src/protocol.rs include_str!` + content-sentinel tests | ✓ |
| Connector routing row parity (shared.md ↔ AGENTS.md ↔ the-judge.md ↔ connector.md ↔ xbgst.md) | Build/CI | `scripts/verify-docs.sh` (greps `xask --effort [a-z]+ gemini` only on connector lines) | ✓ |
| xask flag order (flags before positionals) | Runtime | `scripts/xask:37` strict `while [[ "$1" == -* ]]` loop | ✓ |
| ThinkingBudget injection, effort→budget mapping | Runtime | `scripts/xask` effort table → `src/ask.rs` | ✓ |
| PreToolUse deny policy | Runtime | `src/guard.rs` | ✓ |
| OAuth-only dispatch (no API keys) | Runtime | `src/ask.rs` (key-path removed) | ✓ |
| Role routing prose (xask_profile per axis) for **12 of 13 roles** | **Protocol** (no Build/CI backstop) | `xbreed-shared.md` Axis → Profile Mapping | **DRIFTED** at HEAD |
| Evidence form contract per axis_family | Protocol | `xbreed-shared.md` Pareto Filter Evidence Schema | Aspirational (see §E bypass paths) |
| Exit conditions, round limits, anti-premature-halt | Protocol | `xbreed-shared.md` Exit Condition (L283–311) | ✓ (operator habit) |
| Labrat codex-spark exclusivity | Protocol | `xbreed-shared.md:152` | DRIFTED in `the-judge.md:81–85` (M05 primary deletion candidate) |
| `mutation-tester` fanout threshold | Protocol | `xbreed-shared.md:108` says `≥3` | CONFLICT: `the-judge.md:33` + `commands/xbgst.md` say `≥5` |

**The honest tier ceiling:**
- Build/CI enforces **heading existence** + **one routing row** (connector).
- Runtime enforces **transport-layer mechanics** (xask itself, the codex/gemini wrappers, PreToolUse deny).
- **Everything else** — 12 of 13 routing rows, evidence form contracts, labrat exclusivity, mutation-tester thresholds, exit semantics — is **Protocol-tier convention**: agents read briefs and follow.

### Why this works in practice

Operator habit reading `xbreed-shared.md` as canonical. The judge constructs every Phase-2 brief by reading shared.md (not the drifted mirror). **As long as that habit holds, routing is correct.** The c725c2a near-miss happened the moment a docs rewrite touched both the canonical AND mirror in lockstep — momentarily aligning them on a worse contract — and the operator briefly trusted the rewrite as authoritative.

The 14-commit reset to `2d16ad0` was the operator's defense, not the protocol's. Canonical SSoT held because the mirror was thrown away.

---

## §P — Pareto Economics

> **Source:** M01-priorart + M07-ach + M02-compose
> **Linchpin:** axis-scored proposals + anti-premature-halt + Round-2-always-runs invariant

The Pareto walk is economically coherent precisely because the evidence schema (§E) drops bad inputs upstream. **You never pay axis-evaluation cost on fabricated inputs** (when the schema is honored). H3C (evidence-schema gate) is the **structural immune system**; H1C (axis-filter rejecting regressions) is **derivative** — the filter only works on inputs that survived the gate.

### How it works

| Mechanism | What it enforces | Anchor |
|---|---|---|
| Per-round axis observable (name + direction + obs) | No goal-aiming — the frontier reveals itself by ceasing to move | shared.md godspeed-mode |
| Pareto filter (improves ≥1 axis, harms none) | No silent trades; cheap moves survive | shared.md L65 |
| Anti-premature-halt | Round 2 always runs (Round 1 by construction moves axes off baseline) | shared.md L296–302 |
| Materiality rule | Survivors must change axis observable, not just prose | shared.md L294 |
| Connector mandatory every round | Catches whole-table drift focused specialists miss | feedback_connector_every_round.md |
| Scribe per round + commit per round | Audit-trail anchor — each round revertible/cherry-pickable | shared.md L274–278 |

### Prior-art delta (M01)

Scalarized objective systems (LangGraph, OpenAI Swarm) are vulnerable to **capacity pathology at low effort**: a model finds cheap heuristic shortcuts that satisfy the combined scalar metric while failing individual axes. The xbreed Pareto walk structurally counters this — no proposal advances unless it improves at least one axis without regressing another. Lineage matches NSGA-II Pareto dominance applied to LLM proposal evaluation. Round-2-always-runs guards against accepting first-locally-passable.

### The Goodhart-decoy unaddressed gap (M07, HIGH-impact)

| Scenario | Defense engaged | Failure mode |
|---|---|---|
| Fabricated diff (e.g. ccs-simplifier-bloat) | Spot-check `rg -F` literal-substring | **CAUGHT** (2 spoof flags in xbgst R1 incident) |
| Premature Round-1 halt | Anti-premature-halt + Round-2-always-runs | **CAUGHT** (reopened in harness-r1) |
| **Goodharted evidence-complete decoy**: fresh files minted post-proposal; each round shows ≥1 genuine local axis improvement; real defect off-axis | Spot-check requires literal excerpt + provenance for fresh files (L203) | **UNGUARDED** — spot-check catches fabrication, NOT strategic off-axis coverage; anti-premature-halt runs *toward wrong frontier*; 4-round cap terminates unresolved |

**Distiller-layer amplification:** the distiller filters by stated axes — it is **structurally blind to off-axis defects**. Synthesis cannot surface what wasn't proposed. This is bias amplification at the synthesis step, separate from the proposal step.

### Round-2 materiality (this mission)

Round 1 moved every named axis off baseline. Per the strict reading, Round 2 should dispatch automatically. **However**, the post-DESPAWN cross-pollination cycle (4+ batches of peer-DM critique — including labrat's H1L mechanism correction via scout's a373168 cross-check, H2L severity upgrade via connector, Goodhart promotion via critic-to-labrat relay, sentinel-gap discovery via simplifier-to-sentinel) **functioned as an integrated deliberative R2 inside R1**. Marginal axis movement at R2 boundary expected to be prose-delta only. Frontier called stable per the godspeed materiality rule.

This integration is itself a finding: **the round-boundary distinction blurs when peer-DM critique is rich**. The Pareto walk's two-phase structure (PROPOSE → CROSS-CRITIQUE → PARETO FILTER → COMPILE) implicitly assumes critique is a separate round; in practice teammates can amend post-DESPAWN, collapsing R1+R2 when the team is well-cast. Documented for future reference; not a structural failure.

---

## §M — Memory / Learning Loop

> **Source:** M05-yagni (ccs-simplifier-r1, CC native · HIGH) + cross-axis convergence
> **Linchpin:** `~/.claude/projects/-home-vhpnk-projects-xbrd-gdsp-fknpft/memory/MEMORY.md` (~30 entries) → `CLAUDE.md` → `~/.bashrc DEBUG trap` (per-teammate effort routing) → hooks (banned per `feedback_no_auto_hooks.md`)

The memory loop is what fights brittleness session-to-session. When a directive-violation occurs, the user feeds back → claude saves a memory → next session the memory loads at session start → behavior corrects without re-discovery.

### Load-bearing memory entries (active enforcement)

| Memory | Enforces | Class |
|---|---|---|
| `feedback_xask_first_tool_call.md` | 4-layer xask gate IS the core; first tool call MUST be Bash xask | Protocol-tier (briefs check) |
| `feedback_xask_flag_order.md` | Flags ALWAYS before positionals (xask:37 enforces) | Runtime-tier echo |
| `feedback_no_worktrees.md` | Hard rule in `~/.claude/CLAUDE.md` | Protocol-tier |
| `feedback_mutation_tester_cleanup.md` | Mutation-tester MUST revert + git status clean before DESPAWN | Protocol-tier |
| `feedback_connector_gemini_high.md` | Connector primary = gemini high; fallback = sonnet in-session | Protocol-tier (only connector cell has Build/CI mirror) |
| `feedback_unified_tier_scheme.md` | All teammates sonnet medium; ` \| godspeed` literal suffix; codex lanes unchanged | Protocol-tier |
| `feedback_no_safety_theater.md` | Solo home dev — skip pre-flight caveats, "green light?" pings | Behavioral |
| `feedback_no_auto_hooks.md` | Never auto-add CC hooks without explicit user ask | Protocol-tier |

### Operationally-inert (~8 entries)

- `feedback_no_remember_plugin.md` — banned + gitignored (binary enforces)
- `feedback_no_ScheduleWakeup.md` — section + gitignore entry removed (gone from code)
- `user_oauth_exclusive.md` — OAuth-only path (`src/ask.rs` enforces)
- `feedback_no_obsidian_mcp.md` — never use obsidian-mcp tools

These describe retired/settled behaviors. Correct but operationally inert once the binary+scripts enforce them. Could be archived without loss — but cheap to keep.

### Memory ↔ Runtime transcription gap

Most feedback memories live at **Protocol-tier convention only**. The `feedback_connector_gemini_high.md` lock has a Build/CI mirror via `verify-docs.sh`; **no other feedback memory does**. Future memory hardening would require new transcription paths from Protocol → Runtime — explicitly NOT proposed here per `feedback_no_policy_hardening.md`.

---

## §F — Failure Modes / Regression Corpus

> **Source:** M06-gpt55probe (ccs-labrat-breakmodes-r1, codex --spark · MED) + M07-ach Goodhart row + sentinel 3-gap taxonomy + revenger spec-gap insight
> **Linchpin:** the gpt-5.5 incident (c725c2a) as canonical composition failure; the regression corpus at `docs/reports/*regress*` as the system's institutional memory

### gpt-5.5 incident chain (c725c2a → 18e8df4 → 2d16ad0)

The full forensic chain (5 mechanisms, atomic single-event):

| # | Mechanism | Layer | Detection at commit time |
|---|---|---|---|
| 1 | `-R` flag dropped from reviewer/sentinel/critic briefs | Profile selector | Invisible — no documented effort-tier baseline for correctness lanes (revenger spec-gap insight: "no baseline to violate") |
| 2 | Profile drop dropped `~/.codex/config.toml [profiles.xbreed_review] model_reasoning_effort = "xhigh"` | Two-layer indirection | Invisible — config.toml not read during docs review |
| 3 | `-e low` flag added to override the new top-level default | Two-layer indirection | Visible in diff but no flag-against-baseline check |
| 4 | Connector demoted from `--effort high gemini` (TB 8192) to `--effort medium gemini` (TB 4096) | Direct violation of `feedback_connector_gemini_high.md` LOCK | Memory existed; not enforced by `verify-docs.sh` (which checks routing, not effort) |
| 5 | Concurrent `~/.codex/config.toml` top-level mutation: `model = "gpt-5.5"`, `model_reasoning_effort = "high"` | Independent silent coupling | Not in repo — outside docs review scope |

**Compound risk severity (multiplicative, not additive):**
- gpt-5.5 at xhigh → likely acceptable (verbosity manageable with depth)
- gpt-5.4-mini at low → marginal (known model, weak effort)
- **gpt-5.5 at low → worst-case**: verbosity expansion (medium-effort pathology per a373168) IS NOT the failure mode at low effort; **at low effort the failure is reasoning-budget suppression / System-1 heuristics**, manifesting as `{{QUERY}}{{QUERY}}` template artifacts and missed behavioral bugs. The failure modes are mechanistically distinct (labrat H1L correction post-scout cross-check); both produce degraded output, the LABEL differs.

### Live break-modes (latent at HEAD = 2d16ad0)

| Break-mode | Trigger | Visibility |
|---|---|---|
| **the-judge.md L81–85 stale labrat block** | Any teammate fires `xask gemini` for labrat (per the-judge.md mirror) | Bypasses codex-spark exclusivity + MODEL whitelist + flag-order gate. Path: teammate reads the-judge.md (not shared.md), invokes `xask gemini`, MODEL whitelist passes (gemini IS valid), but the dispatch lane is wrong per the 2026-04-18 migration. |
| **drifted `--effort` role file → wrong ThinkingBudget** | Teammate reads role-file effort claim that contradicts shared.md | At runtime, ThinkingBudget injection uses the actual effort flag passed to xask, not the role-file claim. If role file says "xhigh" but brief built from drifted source uses `-e medium`, runtime injects 4096 not 8192 — silently downgraded. |
| **the-revenger sonnet-vs-opus tier confusion** | `the-judge.md:29` says opus 4.7 medium; shared.md says sonnet medium. Teammate may interpret either. | Effort tier mismatch on RECON tasks — opus reasoning path is structurally different from sonnet at same effort tier. |
| **mutation-tester ≥3 vs ≥5 threshold** | Routing decision: 4-target task — does it use `--spark codex` (≤4) or `--effort low gemini` fanout (≥5 per judge mirror, ≥3 per shared)? | Cross-doc CONFLICT → ambiguous dispatch. |

### Goodharted evidence-complete decoy (HIGH-impact unaddressed)

See §P. Unprobed in this round; recommended for any future R2 if the threat model is updated.

### Spec gap (revenger insight, structural)

**The absence of a documented `effort_tier:` baseline for correctness-axis roles** (reviewer/sentinel/critic) was WHY `-e low` in c725c2a wasn't caught as a regression. Connector has the explicit `feedback_connector_gemini_high.md` LOCK; no equivalent exists for reviewer/sentinel/critic effort tier. Future rewires of correctness-axis roles have the same undetectable-regression surface — not a defect of the c725c2a author, but a structural gap that author walked into.

### Regression corpus (institutional memory)

`docs/reports/*regress*` contains the case law:

- `wwkd-regress-0420-r1` — planner Phase 0 not unconditional → fixed in `2fa45ed` (planner spawn first regardless of /wwkd prefix)
- `xask-gate-regress-0420-r1` — template path + gate restore (gate strings dropped by indirection in commit `128e724`)
- `xask-gate-regress-0420-r2` — V-axis regression guards (`make verify-docs`)
- `xask-gate-regress-0420-r3` — Makefile build R1-twin
- `xbrd-vs-hvm4-r1/r2` — `src/protocol.rs include_str!` SSoT binding lands; 105/105 content-sentinel tests pass

Each report is the system's institutional memory of its own near-misses. The c725c2a incident has not yet been written up — **this map is its first forensic document**.

---

## §X — Why the workflow works (synthesis)

The xbreed protocol works flawlessly **not because every layer is structurally enforced**, but because:

1. **The transport layer (xask, src/ask.rs, src/guard.rs, src/sync.rs) is Runtime-tier correct.** Flag order, ThinkingBudget injection, OAuth-only dispatch, PreToolUse deny — all enforced by the binary.
2. **The canonical SSoT (`xbreed-shared.md`) is operator-habit-canonical.** Every `/xbgst` invocation reads shared.md to construct briefs. The drifted mirror (`the-judge.md`) is read by NO ONE in the dispatch path during normal operation.
3. **The 4-layer xask gate is verbatim per role.** L1 strings live inline in every brief. The pre-dispatch grep self-check at `commands/xbgst.md:74` catches indirection drops.
4. **Layer-2 grounds evidence in literal CLI stdout** — capacity-agnostic; LLMs cannot hallucinate the artifact.
5. **The Pareto walk + anti-premature-halt + connector-every-round prevents single-axis blindness.** Whole-table regressions surface via cross-axis lens.
6. **The audit_hash + spot-check + SOURCE_MAP late-binding** prevent halo-leak (model-prefix bias) and bind synthesis to source identity.
7. **The memory loop hardens user feedback** session-to-session. A directive given once propagates forever via auto-memory + CLAUDE.md.
8. **Per-round commit + scribe report** make every round revertible/cherry-pickable. The c725c2a incident's 14-commit reset worked because every commit was atomic.

The system is **a stack of asymmetric-tier defenses** where Runtime carries the load-bearing transport, Build/CI carries one routing row + heading existence, and Protocol-tier convention + operator habit cover everything else. The asymmetry is not a defect — it's a **deliberate ceiling** per `feedback_no_policy_hardening.md` (no auto-hardening; informational tier-honesty).

What c725c2a nearly broke wasn't the protocol — it was the **operator's mental model of what the protocol enforces**. The map's purpose is to make that mental model accurate so the same near-miss doesn't recur.

---

## §Z — Optimization routes surveyed (informational)

Per `feedback_no_policy_hardening.md` and `feedback_no_safety_theater.md`, these are listed for completeness; **no actions taken**, the user decides:

| # | Route | Surfaced by | What it closes | What it does NOT close |
|---|---|---|---|---|
| (a) | `hash(Layer-1_string + proposal_content)` instead of `hash(proposal_content)` for `audit_hash` | scout fix proposal | Effort-tier attestation gap in the brief — the `-e low` flag would be hashed | DEBUG-trap degradation (env capping below brief claim); audit_hash serialization underspec |
| (b) | Extend `verify-docs.sh` from connector-only to all 13 role rows | 3-way convergence (simplifier + reviewer + revenger) | 12 currently-unguarded routing-row drift cells | Evidence-form contract drift; effort-tier drift; threshold drift |
| (c) | Update `~/.claude/agents/the-judge.md` to align mirror with `xbreed-shared.md` (delete L81–85; fix model-tier mismatches; align gate strings) | M05 primary deletion candidate | Live break-modes #1 (stale labrat block) and #3 (revenger tier confusion) | The structural drift-resistance — fixing one drift instance is not the same as preventing future drift |
| (d) | Add `effort_tier_baseline:` documentation to correctness-axis roles | revenger spec-gap insight | Future rewires of `-e` flag now have a baseline to violate | Existing implicit inheritance from `~/.codex/config.toml` profiles |
| (e) | Document the two confirmed evidence-gate bypass paths (prose burial + post-mint) in shared.md as known limitations | critic RETHINK | Operator awareness of the gap | The gap itself — these are structural, not closeable by docs alone |
| (f) | Specify `audit_hash` serialization format (JSON-canonical with field ordering) | judge META-finding this round | Hash recompute would be deterministic | The honor-system character of distiller-attested hashes (executable cryptographic verifier would be a Build/CI move) |

---

## §0 — What this map is NOT

- **Not a remediation plan.** The optimization routes are informational; user decides actions.
- **Not a recommendation to harden.** Per `feedback_no_safety_theater.md`, the system is calibrated for solo dev; pre-flight ceremony is anti-value.
- **Not a critique of past architecture decisions.** The asymmetric tier surface is deliberate — `xbreed` is a working system, not a research artifact, and the Protocol-tier convention layer reflects honest engineering trade-offs.
- **Not the final word on c725c2a.** It's the first forensic write-up. Future rounds may surface additional mechanisms.

---

## Appendix — Round 1 trace

- **Team**: `map-flawless-why-0424`, 10 members, ~13 minute wall time
- **Phase 0**: `ccs-planner-r0` (sonnet · WWKD-loaded) produced 10-milestone skeleton + gpt-5.5 forensic chain
- **Phase 1 axes**: G/D/R/E/S/P/M/F (8 axes, observables specified per shared.md materiality rule)
- **Phase 2 specialists** (8 dispatched in parallel):
  - `g-scout-priorart-r1` (gemini medium) → M01-priorart
  - `g-connector-crossaxis-r1` (gemini high) → M02-compose
  - `cdx-reviewer-driftcheck-r1` (codex -R) → M03-driftaudit
  - `cdx-revenger-protocol-r1` (codex -R -F, 1.05M ctx) → M04-recon
  - `cdx-critic-design-r1` (heuer-planning + codex -R) → M07-ach
  - `ccs-simplifier-r1` (CC native) → M05-yagni
  - `ccs-labrat-breakmodes-r1` (codex --spark) → M06-gpt55probe
  - `cdx-sentinel-attacksurface-r1` (codex -R) → MOVE E+F-attack (returned late, addendum-folded)
- **Phase 3 distiller**: `ccs-distiller` (sonnet · in-session) → SYNTHESIS_READY with 7 moves, audit_hash, spoof-check on M03/M04/M05
- **Phase 4 scribe**: `ccs-scribe-r1` (sonnet · medium · filter-exempt) → round report at `docs/reports/map-flawless-why-r1-2026-04-24.md`
- **Pareto verdict**: 7 of 7 ACCEPTED. Frontier called stable (post-DESPAWN cross-pollination functioned as integrated R2).
- **CONFLICTS resolved**: 5 (3 verified drift, 1 mechanism label correction, 1 spine-claim-vs-reality).
- **Meta-finding**: `audit_hash` serialization underspec (judge recompute failed across 7 standard variants); SOURCE_MAP semantic binding verified via independent path (team config).

---

*This document was produced by the system mapping itself. The fact that the run completed without protocol violation — and surfaced concrete drift the map will help the operator address — is empirical evidence for the section §X synthesis claim.*
