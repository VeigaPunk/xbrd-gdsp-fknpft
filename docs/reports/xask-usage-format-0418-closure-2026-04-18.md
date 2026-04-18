# xask Usage-Format — Closure

**Mission:** xask-usage-format-0418
**Date:** 2026-04-18
**Status:** CLOSED Round 1. Strict halt (7 axes converged, zero axis improvement remaining). Variant B rejected. α/β paths named; α recommended for immediate land, β deferred.

---

## 1. Verdict

**User's proposed multi-line Variant B is REJECTED** on three independent grounds:

1. **YAGNI** (Axis D / simplifier): `[-d|--debug]` inline expansion adds zero discoverability value for boolean flags; only `--spk`/`-o` have non-obvious short forms and those are already covered by `scripts/xask:47`'s `valid:` error string.
2. **Ecosystem mismatch on pipe delimiter** (Axes G + C): both gemini-cli AND codex-cli use comma-space `-s, --long <VALUE>` in their own `--help` output. Zero pipe delimiters in either. Variant B's `[-s|--scope]` pipe form **diverges from both CLIs xask wraps** — the opposite of the stated "isonomy" goal.
3. **Drift risk vs line 86** (Axis X / connector): `scripts/xask:86` already has the canonical expanded runtime form. Adding a divergent multi-line variant at line 3 would create a three-way drift surface (L3 comment, L86 runtime echo, L47 `valid:` string) with no sync mechanism.

The mission's originally-stated goal (sleek, isonomous, respects-our-dicts) is **achievable** — just not through the POSIX-expanded inline form. It's achievable through the industry-canonical `[options]` synopsis + tabular Options block (Mutation 7 pattern, refined).

---

## 2. Axis findings

| Axis | Role | Verdict | Primary evidence |
|---|---|---|---|
| **D** (Readability) | `ccs-simplifier-readability` (CC native) | Variant A compact wins on YAGNI. Long-form aliases on boolean flags add noise, not value. Mutation 7's `[OPTIONS]` compression is weaker than it looks because L86 already serves that purpose. | L47 `valid:` error string already carries short\|long forms |
| **G** (Gemini alignment) | `g-scout-gemini-conventions` (xask gemini medium + librarian) | gemini-cli uses `Usage: gemini [options] [command]` compact synopsis + `-s, --long` tabular Options block. NOT inline `[-s|--long]` expansion. POSIX expanded usage is anti-pattern for 8+ flags per GNU coding standards. | `gemini --help` stdout format |
| **C** (Codex alignment) | `cdx-delegate-recommender` (xask -R codex) | codex-cli uses comma-space (`-s, --sandbox <SANDBOX_MODE>`), zero pipe delimiters. Both CLIs xask wraps converge on comma-space convention. | `codex --help` stdout format |
| **M** (Mutation variants) | `g-mutation-tester-format` (xask gemini low fanout, 10 probes) | Winning variant: lowercase `[options]` + positionals explicit + 4-flag Options block (`-s`, `-e`, `-o`, `-d`) with tabular `[type]` annotations + pointer to `--help`. Matches gemini AND codex convention. | 10-probe fanout synthesis |
| **T** (Telemetry) | `cdx-labrat-telemetry` (xask --spark codex) | Empirical: probe 1 emits single-line model error; probe 2 (`--help`) parses as unknown flag (no `--help` handler exists); probe 3 (`-d` debug) emits dispatch trace, not usage. **No usage BLOCK exists at any runtime path today except L86's no-args trigger.** Multi-line format has no surface to improve unless a `--help` handler lands first. | 3 empirical probes |
| **X** (Cross-axis) | `g-connector-couplings` (xask gemini high; sonnet fallback held) | **Invariant**: L3 comment + L86 runtime echo + L47 `valid:` string form a single logical unit. Diverged earlier this session (2026-04-18 self-referential precedent). Any format change = atomic 3-location update. Live dispatch directives at `shared.md:36-46` are highest drift risk. | Session history + shared.md L36-46 |
| **Planner** (WWKD) | `ccs-planner-xask-format` (CC native + wwkd skill) | Re-scoped M01 to content-patch only after D+G+T+C+X convergence. 5-milestone plan (M01 content atomic → M02 shared.md verify → M03 protocol doc synopsis → M04 optional --help → M05 judge β escalation). | Planning artifact |

All 7 axes converge. No CONFLICT block needed — cross-model agreement (gemini scout + codex delegate) on comma-space convention is independent primary-source confirmation.

---

## 3. Two paths — α (ship now), β (ship after --help lands)

### Path α — Content patch, minimal, zero regression

Fix the stale compact form at 3 locations. Does NOT change format direction — just closes the drift:

| File | Line | Current | Fixed |
|---|---|---|---|
| `scripts/xask` | 3 | `# Usage: xask [-d] [-s <scope>] [-r] [--spk] [-R] [-e <level>] <model> "<query>" ["<context>"] ["<skill>"]` | Add `[-o <file>]` and `[--json]` between `[-e <level>]` and `<model>` |
| `scripts/xask` | 8 | `# Flags: -d (debug) \| -s (scope) \| -r (rich) \| --spk (spark) \| -R (review) \| -e (effort)` | Add `\| -o (output-last-message) \| --json` |
| `scripts/xask` | 86 | expanded runtime echo with `<boundary>` placeholder | Change `<boundary>` → `<scope>` (matches script's `SCOPE` variable) |
| `docs/xask-protocol.md` | 11 | compact synopsis missing `-R`, `-o`, `--json` | Sync to match scripts/xask:3 |

**Verification gate:** `grep -n 'Usage\|-o\|--output-last\|--json' scripts/xask docs/xask-protocol.md` shows consistent flag inventory across all 4 locations.

**Pareto:** strict improver on current state (closes real drift), zero axis regression.

### Path β — Mutation 7 refined (deferred)

Once `--help` handler lands (optional M04), adopt the full canonical form:

```
xask [options] <model> "<query>" ["<context>"] ["<skill>"]

Options:
  -s, --scope <scope>              Define operational boundary    [string]
  -e, --effort <level>             Set reasoning depth            [string]
  -o, --output-last-message <FILE> Save last response to file     [string]
  -d, --debug                      Enable verbose output          [boolean]

Run 'xask --help' for full options.
```

**Contingent on:** `--help` handler addition (currently parses as unknown flag per Labrat T). Without it, Mutation 7's "Run 'xask --help'" line is a broken promise — regression on T axis unless bundled.

**Pareto:** strict improver on α's result once `--help` handler exists. **Sequencing matters: α → β → --help, not either/or.**

### Key atomicity invariant (connector)

Whichever path ships, the 3-location co-update rule applies: `scripts/xask:3` header + `scripts/xask:86` runtime echo + `scripts/xask:47` valid-flag error string MUST stay consistent. Independent update of any one creates the drift class we're closing.

---

## 4. Re-open triggers

| Trigger | Observable | Action |
|---|---|---|
| **X1** `--help` handler lands | `xask --help` emits usage block (not "unknown flag") | Evaluate Path β; decide if `[options]` compression + Options block ships |
| **X2** New codex or gemini flag added to xask | `scripts/xask` diff introduces flag not present in L3 / L86 / L47 | Apply α's 3-location atomic-update invariant |
| **X3** Positional reorder proposed | Any move touching `$1=MODEL`, `$2=QUERY`, `$3=CONTEXT`, `$4=SKILL` in `scripts/xask:63-66` | Verify `docs/swarm-test-flow.md:41-51` test invocations still pass |
| **X4** User revisits "multi-line expanded" preference | Explicit ask to re-evaluate Variant B | Re-dispatch `/xbgst` with this closure as Phase 0 entry point; primary-source kills (ecosystem mismatch on pipe) are durable |

---

## 5. Durability statement

This closure stands on:
1. **Primary-source cross-model agreement** — gemini-cli AND codex-cli both use compact + comma-space + tabular Options. The convention xask diverges from is the convention of both CLIs xask wraps.
2. **Empirical T-axis probe** — no usage block surface at runtime paths today except L86's no-args trigger; format proposals that assume one are analyzing non-existent output.
3. **Session-referential X-axis precedent** — scripts/xask:3 vs scripts/xask:86 already drifted earlier in this same session; the atomicity invariant is observed-not-theoretical.

Variant B doesn't come back without upstream change in both codex-cli AND gemini-cli help conventions simultaneously — neither has signaled such a change.

---

## 6. References

- **Mission team:** `ccs-planner-xask-format`, `ccs-simplifier-readability`, `g-scout-gemini-conventions`, `cdx-delegate-recommender`, `g-mutation-tester-format`, `cdx-labrat-telemetry`, `g-connector-couplings`, `ccs-distiller` (auto-cleaned on TeamDelete)
- **Primary-source anchors:**
  - `scripts/xask:3` (current compact header, stale — missing `-o`, `--json`)
  - `scripts/xask:47` (expanded `valid:` error string — short|long inline)
  - `scripts/xask:86` (runtime error echo — expanded form with `<boundary>` placeholder)
  - `docs/xask-protocol.md:11` (synopsis, stalest — missing `-R`, `-o`, `--json`)
  - `commands/references/xbreed-shared.md:36-46` (live-injected Layer-1 gate strings — NOT touched by format change per connector)
- **Ecosystem primary sources:** `gemini --help` stdout + `codex --help` stdout (both converge on comma-space tabular)
- **Memory cross-linked:**
  - `feedback_xask_flag_order.md` (installed this session — flags-before-positionals)
  - `reference_gemini_fanout_skill.md` (mutation-tester-only — M axis used this correctly)
  - `feedback_reviewer_brief_propose_only.md` (applied to cdx-delegate-recommender brief)

---

## 7. Forward pointer

Ship **Path α** now. Clean 3-location atomic patch. Reviewer/executor can land in a single commit.

If Path β is desired later, add the `--help` handler first (optional M04 milestone in planner's table), then atomic 3-location update to the Mutation 7 refined form. No rush — Path α is a strict improver on its own.

---

*Authoritative closure — judge-direct DRAFT (claude-opus-4-7, xhigh). Based on Round 1 convergence across 7 axes with zero dissent on Variant B rejection. Distiller did not emit a formal SYNTHESIS_READY block — synthesis was distributed across planner's progressive deltas + mutation-tester's integrated-peer-findings final + connector's whole-table map. Judge-direct draft is warranted per the-judge.md:16 ("A stalled judge is worse than a wrong judge") given the clarity of the 7-axis convergence.*
