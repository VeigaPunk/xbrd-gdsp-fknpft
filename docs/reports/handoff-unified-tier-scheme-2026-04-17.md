# Handoff — Unified opus-medium tier + codex-mini default + OAuth-only auth

**Date:** 2026-04-17
**Commit:** `0ac5571` (`feat(xbreed): unified opus-medium tier + codex-mini default + OAuth-only auth`)
**Branch:** `main` (ahead of `origin/main` by 3: this refactor + 2 prior scribe reports)
**Session type:** `/xbgst /wwkd | godspeed` — model-selection adaptations per user directive 2026-04-17

---

## Mission for the new session

1. **Read this handout first** to get the state of the world as of 2026-04-17.
2. **Then update `docs/command-flows.md`** (same structure, refreshed content) so the public-facing flow diagrams match what the code + configs now actually do. The current `docs/command-flows.md` predates the refactor below and misstates the gemini auth cascade, the codex model routing, the agent count, and several mermaid diagrams. The "same structure" directive: keep the section order, keep the mermaid diagrams, keep the Quick Reference table at the end, but retarget every mention of concrete models / auth steps / agent counts to the current state.

---

## Context summary (what changed 2026-04-17)

The session collapsed a multi-tier model-selection scheme into a single unified one, added a codex review lane, and ripped out the API-key code path. Three concurrent user directives:

1. **All Claude teammates run Opus 4.7**, effort tiered by role:
   - `the-judge` → **xhigh**
   - `critic` / `connector` / `the-planner` → **high**
   - everything else (scout, reviewer, labrat, executor, simplifier, sentinel, mutation-tester, scribe, the-revenger) → **medium**
   - `distiller` → **sonnet medium** (lone holdout — synthesis is pattern-match throughput, not adversarial reasoning; user rejected an earlier attempt at opus-4.6-low-fast because per-subagent fast-mode is not a supported Claude Code frontmatter field).

2. **Codex dispatch has three lanes** in `src/ask.rs`:
   - **spark** (`xask --spark codex`): `gpt-5.3-codex-spark` + `model_reasoning_effort=low`, no fast_mode.
   - **review** (`xask -R codex` / `--review`): full `gpt-5.4` + `features.fast_mode=true`; reasoning inherits codex's own `xhigh` default from `~/.codex/config.toml`. Used by reviewer / critic / sentinel / the-revenger.
   - **default** (`xask codex` with no `--spark` / `--review`): `gpt-5.4-mini` + `features.fast_mode=true` + `model_reasoning_effort=high`. New standing default 2026-04-17 — covers executor, labrat when not spark, scout-fallback, etc.

3. **xbreed is OAuth-exclusive in code.** The `GeminiAuth::ApiKey` variant, `GeminiKeys` struct, `load_gemini_keys`, `parse_env_line`, `clean_env_value`, and the `.env.local` GEMINI_API_KEY/FALLBACK parsing were deleted from `src/ask.rs`. The gemini auth cascade is now **3 levels**: `OAuthProfile("primary")` → `OAuthProfile("fallback")` → `OAuthDefault`. `env_remove("GEMINI_API_KEY")` stays on both OAuth variants to strip any inherited shell env.

4. **the-judge MUST spawn a connector teammate on every Pareto round.** Landed as a new bullet in `~/.claude/agents/the-judge.md` and a standalone paragraph in `~/.claude/commands/references/xbreed-shared.md`. Cross-axis pattern matching is structural, not optional.

5. **All `gemini-rate-limited 2026-04-15/16` annotations removed** from live SSoT files (quota restored 2026-04-17). Scout default restored to `xask --effort medium gemini … librarian`.

---

## Files touched (authoritative sources)

### In-repo (committed in `0ac5571`)

| File | Change |
|---|---|
| `src/ask.rs` | New `CODEX_MINI_MODEL` const; `build_codex_ask_with_loadout` now takes `review: bool`; `dispatch` takes `review: bool`; `#[allow(clippy::too_many_arguments)]`; gemini auth reduced to 3-level OAuth; `GeminiKeys` / `load_gemini_keys*` / `parse_env_line` / `clean_env_value` / `GeminiAuth::ApiKey` all deleted; related tests removed; default-lane test renamed + new review-lane test. |
| `src/cli.rs` | New `--review` / `-R` flag on `Commands::Ask`. |
| `src/main.rs` | Thread `review` from `Commands::Ask` through to `ask::dispatch`. |
| `scripts/xask` | New `-R` / `--review` flag; threads `--review` to `xbreed ask codex`; debug output + usage line updated. |
| `commands/references/xbreed-shared.md` (symlinked from `~/.claude/commands/references/`) | Axis → Profile table retargeted; "Unified scheme 2026-04-17" explainer; "Mandatory connector on every round" paragraph; Layer-1 xask gate for reviewer / sentinel / critic / the-revenger now says `xask -R codex`; gemini-rate-limited annotations purged; canary rewritten as `OK` / `DEGRADED` health check. |
| `templates/agents/the-judge.md` | the-revenger row updated to `xask -R codex` + opus 4.7 medium; rate-limit annotation removed. |
| `tests/ask_with_loadout.rs` | `fs::write(home.join(".env.local"), "GEMINI_API_KEY=…")` replaced with `fs::create_dir_all(home.join(".gemini"))` + `oauth_creds.json` stub; default-lane assert now pins `gpt-5.4-mini` (was `gpt-5.4`). |

### Out-of-repo (user-level dotfiles)

| File | Change |
|---|---|
| `~/.bashrc` (DEBUG trap lines ~228-266) | `__xbreed_effort_trap` rewritten: role-keyword pattern (`*the-judge*` → xhigh, `*critic*\|*connector*\|*planner*` → high, `cco-*\|ccs-*\|cdx-*\|g-*` → medium, else NOMATCH). Closes open R2 I2 finding (inner `*)` missing). |
| `~/.claude/agents/*.md` (13 files) | Frontmatter retargeted (`model: opus` uniformly; distiller remains `model: sonnet`; `effort:` now present on every file). `the-judge.md` additionally gained the connector-every-round bullet. `scout.md` default delegation restored from codex-fallback to gemini-primary. |
| `~/.codex/config.toml` | Top-level `model = "gpt-5.4-mini"`, `model_reasoning_effort = "high"`. Profiles `xbreed`, `xbreed_spark`, `xbreed_review` unchanged. |
| `~/.claude/projects/.../memory/` | 5 files deleted (`project_gemini_rate_limited_0416.md`, `feedback_cco_opus_high.md`, `feedback_sonnet_effort_tiers.md`, `feedback_cco_critic_heuer.md`, `feedback_the_planner_wwkd.md`). 3 files created (`feedback_unified_tier_scheme.md`, `feedback_connector_every_round.md`, `feedback_oauth_exclusive_code.md`). `MEMORY.md` index updated. |

---

## Verification (all green as of commit `0ac5571`)

```bash
cargo clippy --all-targets -- -D warnings   # clean
cargo test                                  # 91 tests — all pass
cargo fmt --check                           # clean
scripts/verify-docs.sh                      # connector SSoT consistent
xask -d -R codex "review"                   # argv audit: REVIEW: true
```

Release binary installed at `~/.local/bin/xbreed` (`cargo build --release && cp …`) per the always-recompile-on-change rule.

---

## Directive — update `docs/command-flows.md` to match current state

Keep the file's existing structure (binary commands → skill commands → "How it all connects" → Quick reference), keep every mermaid block, keep the comparison tables. Retarget the concrete content in each block to reflect what the code now actually does.

### Per-section update checklist

**`## Overview` (lines ~5-16):** still correct. No change needed.

**`### xbreed guard` (lines ~21-36):** still correct. No change needed.

**`### xbreed sync` (lines ~39-48):** still correct. No change needed.

**`### xbreed claude` (lines ~51-68):**
- The `models.yaml`-driven launch is still live.
- Worth adding a pointer to `~/.bashrc`'s DEBUG trap as the authoritative per-teammate effort override (since it beats frontmatter `effort:` per Anthropic's precedence rules). Can be a one-sentence note below the mermaid.

**`### xbreed ask` (lines ~71-126):** heavily stale. Rewrite:
- Add the new `--review` / `-R` flag to the command signature.
- Replace the codex branch of the mermaid with the three-lane split:
  ```
  codex
    ├─ spark → gpt-5.3-codex-spark + reasoning=low (no fast_mode)
    ├─ review → gpt-5.4 (full) + features.fast_mode=true
    └─ default → gpt-5.4-mini + features.fast_mode=true + reasoning=high
  ```
  Authoritative: `src/ask.rs` `build_codex_ask_with_loadout` — inspect it before drawing.
- Replace the 5-step gemini auth cascade (lines ~87-111) with 3 OAuth-only levels:
  ```
  1. OAuth profile: primary    (HOME override to ~/.config/xbreed/gemini-profiles/primary/)
  2. OAuth profile: fallback   (same mechanism, fallback profile)
  3. OAuth default             (user's real ~/.gemini/oauth_creds.json; env_remove GEMINI_API_KEY)
  ```
  Drop the "API key from .env.local" and "Fallback API key" steps entirely. Authoritative: `src/ask.rs` `gemini_auth_chain` + `GeminiAuth` enum.
- The "Gemini auth cascade (v0.3.5)" paragraph (lines ~121-125) needs to become "(v0.4+, OAuth-exclusive)". Update the "tries up to 5 auth methods" to "tries up to 3 OAuth levels". The 14s OAuth / 5-7s API key timing note: keep only the OAuth number; drop the API-key one. Mention that the cascade only advances on 429 / 401 / 403 / PERMISSION_DENIED / UNAUTHENTICATED / API_KEY_INVALID.

**`### xbreed team init` / `mailbox` (lines ~129-156):** still correct (mailbox WIP on `src/mailbox.rs` is pre-existing, not part of this refactor).

**`## Skill commands` intro (lines ~159-163):** still correct.

**`### /xbreed` / `/xb` (lines ~165-195):**
- "up to 3 Agent() calls (scout→xask gemini, reviewer→xask codex, labrat→xask --spark codex)" — reviewer should now read `xask -R codex` (review lane). Update the mermaid label + the prose.
- No change to the one-shot-no-team substrate.

**`### /xbreed-team` / `/xbt` (lines ~198-224):**
- Same reviewer retarget: `xask codex` → `xask -R codex`.
- Keep the deliberative cap note.

**`/xbgst` / `/xgs` comparison table (lines ~226-234):** structurally fine; no change needed.

**`### /xgs` (lines ~238-262):**
- Mention that `connector` is mandatory on every round (not optional). This is a new rule landed 2026-04-17 in the judge persona and shared.md.

**`### /xbgst` (lines ~265-325):** biggest rewrite.
- The sequence diagram currently shows "scout brief (axis + xask gemini gate)" / "reviewer brief (axis + xask codex gate)". Update reviewer to `xask -R codex`. Add connector as a MANDATORY lane in Phase 2 (not optional in the `par`).
- "Auth cascade: OAuth first → API key fallback" note (right of `G` in the sequence) — change to "Auth cascade: OAuth-only (3 levels)".
- Timing annotations (lines ~327-337) date to 2026-04-12. If you have fresh timings, update. Otherwise, flag the date and keep the table; note that default-lane codex calls now use `gpt-5.4-mini` so the ~6s codex number is indicative of the mini path (full `gpt-5.4` via `-R` may be slower).
- CONFLICTS block schema is unchanged.

**`## How it all connects` (lines ~350-393):**
- The "`~/.claude/agents/*.md (8 agent definitions)`" node is stale — there are now **13 xbreed agent definitions** (scout, reviewer, labrat, executor, simplifier, sentinel, mutation-tester, scribe, the-revenger, the-planner, the-judge, critic, connector, distiller) plus two symlinked external personas (`the-musketeer`, `the-puppeteer`) that are not xbreed-managed. Update the label.
- Dispatch arrows (`xb`, `xbt`, `xbgst` → `ask`, labeled "xask (cross-model)") remain correct; xask now carries a `-R` flag but the arrow doesn't need to encode it.

**`## Quick reference` (lines ~397-411):** unchanged commands. Still 10 entries. No structural change.

### Additional content worth adding (new section, optional)

Consider a new `## Model selection` section after "How it all connects" that documents the three independent control surfaces so future readers don't have to reverse-engineer them from code. Suggested skeleton:

```markdown
## Model selection

Three independent layers decide which model + effort a spawned teammate actually runs:

| Layer | Source | Controls | Precedence |
|---|---|---|---|
| Frontmatter | `~/.claude/agents/<name>.md` | `model:` (opus/sonnet/full ID) + `effort:` default | lowest |
| DEBUG trap | `~/.bashrc` `__xbreed_effort_trap` | `CLAUDE_CODE_EFFORT_LEVEL` env var, role-keyword match | overrides frontmatter effort |
| `CLAUDE_CODE_SUBAGENT_MODEL` env | user shell | model override for every subagent | overrides everything (rarely set) |

Current tier map (2026-04-17):

- `*the-judge*` → xhigh
- `*critic*` / `*connector*` / `*planner*` → high
- `cco-*` / `ccs-*` / `cdx-*` / `g-*` → medium
- unmapped → NOMATCH (trap leaves env unset; CC uses frontmatter effort)

Codex dispatch lanes (src/ask.rs build_codex_ask_with_loadout):

- `--spark` → gpt-5.3-codex-spark + reasoning=low
- `--review` / `-R` → gpt-5.4 (full) + fast_mode
- default → gpt-5.4-mini + fast_mode + reasoning=high
```

---

## Out-of-scope / known caveats (don't fix in the same PR)

- **`src/mailbox.rs`** has an uncommitted mod from earlier on this branch (M4 test panic-path fix using `try_send` loop + `COMPACT_PENDING` decrement on disconnect). Unrelated to this refactor; leave alone unless explicitly asked.
- **`test.sh` / `test2.sh`** are untracked in the repo root. User's WIP — leave alone.
- **`data/bench-phase-c.paths`** untracked — benchmark data from prior session.
- **`docs/reports/shim-critique-a3-negcoverage-2026-04-17.md`** untracked — R2 critic report from the prior per-teammate-effort-shim work.
- **Per-subagent fast mode is not available.** Claude Code fast mode (`/fast`, `"fastMode": true` in settings.json) is session-level only per [code.claude.com/docs/en/fast-mode](https://code.claude.com/docs/en/fast-mode). The subagent frontmatter reference (name/description/tools/disallowedTools/model/permissionMode/maxTurns/skills/mcpServers/hooks/memory/background/effort/isolation/color/initialPrompt) has no `fast` field. This is why distiller landed at sonnet-medium instead of opus-4-6-low-fast — the "fast flag enabled" ask isn't reachable with current Claude Code.
- **R2 shim work (`feedback_teammate_mode_effort_caveat.md`) still valid** — the DEBUG trap shim remains the authoritative per-teammate effort control. Its open findings (I1 parent-side witness leak, I3 `$BASHPID` semantics comment) are cosmetic/documentation-only; the trap's correctness for child-side effort wiring is solid.

---

## Quick sanity checks before editing `docs/command-flows.md`

```bash
# Confirm 13 xbreed agents:
ls ~/.claude/agents/*.md | grep -v the-musketeer | grep -v the-puppeteer | wc -l   # expects 13

# Confirm 3-level gemini auth cascade:
grep -A3 'pub enum GeminiAuth' src/ask.rs   # expects only OAuthProfile + OAuthDefault

# Confirm codex three-lane logic:
grep -E 'CODEX_MINI_MODEL|CODEX_DEFAULT_MODEL|CODEX_SPARK_MODEL' src/ask.rs

# Confirm xask --review plumbing:
bash scripts/xask -d -R codex "probe" | grep REVIEW   # expects: REVIEW: true

# Confirm effort trap role-keyword ordering:
sed -n '234,266p' ~/.bashrc

# Confirm shared reference state:
grep -c 'gemini-rate-limited' ~/.claude/commands/references/xbreed-shared.md   # expects 0
grep -c 'Mandatory connector on every round' ~/.claude/commands/references/xbreed-shared.md   # expects 1
```

All six should return the expected values. If any of them drift, inspect before editing — the session may have landed additional changes after this handoff was written.

---

## Reference — relevant memory files (live as of this handoff)

- `feedback_unified_tier_scheme.md` — canonical tier map + codex lanes
- `feedback_connector_every_round.md` — mandatory connector rule
- `feedback_oauth_exclusive_code.md` — API-key code retirement
- `feedback_connector_gemini_high.md` — connector LOCKED to gemini high (pre-existing, still valid)
- `feedback_teammate_mode_effort_caveat.md` — DEBUG trap shim installation + open R2 findings
- `feedback_yolo_routing.md` — yolo / danger-full-access defaults through xask
- `user_oauth_exclusive.md` — user's OAuth-everywhere stance
- `feedback_recompile_on_change.md` — rebuild + install discipline
- `feedback_no_safety_theater.md` — execute-on-clear-instructions stance

---

## PR message template (if opening a PR for the command-flows.md update)

```
docs(command-flows): sync to unified opus-medium + codex-mini + OAuth-only

Refreshes docs/command-flows.md to match src/ask.rs and the 2026-04-17
tier scheme landed in commit 0ac5571:

- gemini auth cascade: 5-level (OAuth + API key) → 3-level OAuth-only
- codex dispatch: single "gpt-5.4 + fast_mode" branch → three lanes
  (spark / review / default-mini) via new `xask -R` flag
- agent count reference: 8 → 13 xbreed-managed definitions
- /xbgst sequence diagram: reviewer retargeted to `xask -R codex`;
  connector added as mandatory Phase-2 lane
- mandatory-connector-every-round rule explicit under /xgs and /xbgst
- new "Model selection" section documenting the three control layers
  (frontmatter / DEBUG trap / CLAUDE_CODE_SUBAGENT_MODEL)

No code changes.
```

---

## Empirical verification during execution (2026-04-17)

Executor (`ccs-executor-flows`) ran all six handoff sanity checks (lines 186-208) before starting the `docs/command-flows.md` rewrite. Observed state:

- **Agent count drift (handoff miscount):** Handoff line 138 prose enumerates 14 agent names but labels them as "13 xbreed agent definitions"; line 189 grep expects 13. Actual `ls ~/.claude/agents/*.md | grep -v the-musketeer | grep -v the-puppeteer | wc -l` returns **14**. Resolution: the `docs/command-flows.md` rewrite lands `14 xbreed-managed definitions` (with a parenthetical note that `the-musketeer` and `the-puppeteer` are user-invoked on demand, not xbreed-orchestrated, and are therefore excluded from the count).
- **All 6 sanity checks verified green:**
  - `ls ~/.claude/agents/*.md | grep -v the-musketeer | grep -v the-puppeteer | wc -l` → **14** (handoff expected 13; see drift note above).
  - `grep -A3 'pub enum GeminiAuth' src/ask.rs` → enum carries only `OAuthProfile(String)` + `OAuthDefault` variants (confirmed `ApiKey` variant removed).
  - `grep -E 'CODEX_MINI_MODEL|CODEX_DEFAULT_MODEL|CODEX_SPARK_MODEL' src/ask.rs` → `CODEX_SPARK_MODEL = "gpt-5.3-codex-spark"`, `CODEX_DEFAULT_MODEL = "gpt-5.4"`, `CODEX_MINI_MODEL = "gpt-5.4-mini"`; dispatch dispatches spark→SPARK, review→DEFAULT (full), otherwise→MINI.
  - `bash scripts/xask -d -R codex "probe" | grep REVIEW` → `REVIEW: true`.
  - `sed -n '234,266p' ~/.bashrc` → `__xbreed_effort_trap` with role-keyword case block: `*the-judge*` → xhigh, `*critic*|*connector*|*planner*` → high, `cco-*|ccs-*|cdx-*|g-*` → medium, catch-all → NOMATCH. First-match-wins ordering confirmed (role patterns precede prefix patterns).
  - `grep -c 'gemini-rate-limited' ~/.claude/commands/references/xbreed-shared.md` → **0**; `grep -c 'Mandatory connector on every round' ~/.claude/commands/references/xbreed-shared.md` → **1**.
- **No other drift found** between handoff claims and live state as of this execution timestamp (2026-04-17).
