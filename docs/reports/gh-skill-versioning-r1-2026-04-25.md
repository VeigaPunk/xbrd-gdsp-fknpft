# gh-skill-versioning-0425 — Round 1 Report

**Status:** COMPLETE | **Date:** 2026-04-25 | **Round:** 1
**Mission slug:** `gh-skill-versioning`
**audit_hash:** `3a0737cfd4279d1b6a2d663db20f8547d77bcf33bb230fc1f0f347b8d444b386`

---

## 1. Round Overview

- **Team:** `gh-skill-versioning-0425` (10 members)
- **Mission:** Adopt `gh skill` as substrate for version pinning + audit, extended beyond skills to agents/commands/memory
- **Axes (8):** F · M · A · T · I · S · E · R
- **Wall time:** ~13 min
- **xask targets:**
  - gemini medium → scout
  - gemini high cap-6k → connector
  - codex `-R` → reviewer + critic + heuer-Layer-0
  - codex `-R -F` → revenger
  - codex `--spark` → labrat (initial 4/4 false-negative due to spark env mismatch; self-corrected to 4/4 POSITIVE)
- **Pareto verdict:** 7 of 7 ACCEPTED

`EVIDENCE AUDIT: 7 moves with evidence, 0 moves without, 0 dropped, 1 spoof_flagged`

---

## 2. Per-Teammate Proposals (M01–M07)

### M01 — Native `gh skill install` substrate (skills only)
- **AXIS:** F (feature surface)
- **CLAIM:** `gh skill {install,publish,list,view,remove,upgrade}` is real and supports `@<tag>`/`@<sha>`/`--pin` versioning; only `name` + `description` are required frontmatter fields.
- **EVIDENCE:** `gh skill --help` (judge primary-source, this session) lists install/publish/list/view/remove/upgrade subcommands. `gh skill publish --help` lists `--dry-run` (no `--immutable`). Validation accepts `name` + `description` only.
- **REJECTED-ALT:** scout-overclaim variant with `--immutable`, `gh skill verify`, `gh skill audit`, `.skill-lock.json`, mandatory `version:` — all stripped (see §7).
- **Confidence:** HIGH (primary-source confirmed in-session)

### M02 — Agents PATH B: xbreed-owned `gh api` + `agents.lock`
- **AXIS:** A (adoption / scope fit)
- **CLAIM:** `gh skill` does not host agents; xbreed owns the fetch via `gh api repos/<owner>/<repo>/contents/agents/<name>.md?ref=<tag>` plus a TOML `agents.lock`. Symlink-to-local-checkout fallback already production for 4 agents.
- **EVIDENCE:** `gh skill` subcommand list (no `agent`/`gh agent` surface); existing `~/.claude/agents/` symlink topology (judge env).
- **REJECTED-ALT:** Repurpose `gh skill` to host agent files (rejected: validator requires skill-shaped frontmatter, would force schema-spoofing).
- **Confidence:** HIGH

### M03 — Commands no-op (already version-tracked in xbreed)
- **AXIS:** S (substrate fit) / E (effort)
- **CLAIM:** Commands live in xbreed repo with `include_str!` Build/CI binding; further versioning is redundant.
- **EVIDENCE:** Repo layout `commands/` → `include_str!` references in src/; CI binding already gates command-template drift.
- **REJECTED-ALT:** Mirror commands into a `gh skill`-published repo (rejected: zero benefit, adds release overhead).
- **Confidence:** HIGH

### M04 — Memory: GATE_FAIL for full substrate move
- **AXIS:** M (memory) / R (risk)
- **CLAIM:** Migrating MEMORY.md / per-feedback files into a versioned substrate fails on (a) index-contract drift already live, (b) path-slug coupling locking substrate to exact filesystem layout, (c) verbatim memory versioning is the wrong abstraction.
- **EVIDENCE:** Yesterday's `map-feedback-memory-r1` report flagged 2 missing CAT entries; project-memory dir uses path-slug-keyed indirection that any substrate rewrite would break.
- **REJECTED-ALT:** `gh skill publish` per-feedback-file (rejected: schema spoof + write amplification).
- **Confidence:** HIGH

### M05 — Selective memory elevation (recommendation, not action)
- **AXIS:** M (memory) / I (information topology)
- **CLAIM:** MUST-tier memory entries should be promoted into agent prompts / `shared.md`; convention-tier entries stay in MEMORY.md. NOT a verbatim memory-versioning move.
- **EVIDENCE:** Existing `shared.md` already encodes orchestration MUSTs; precedent for elevation (godspeed posture, OAuth-exclusive).
- **REJECTED-ALT:** Mass-elevate everything (rejected: shared.md bloat, lifecycle mismatch).
- **Confidence:** MEDIUM (recommendation; no policy hardening per `feedback_no_policy_hardening.md`)

### M06 — Dedicated `xbreed-skills` repo (Tier 1 future)
- **AXIS:** F / S
- **CLAIM:** When curated skills emerge worth pinning, create `xbreed-skills` repo and use `gh skill install` natively.
- **EVIDENCE:** `gh skill publish` requires a dedicated skill-shaped repo (publish scope verified via `--help`).
- **REJECTED-ALT:** Publish skills from xbreed monorepo subdirectory (rejected: validator scope is repo-root-shaped).
- **Confidence:** MEDIUM (conditional on curated-skill emergence)

### M07 — `xbreed agents sync` Rust subcommand (Tier 2 future)
- **AXIS:** A / E
- **CLAIM:** Implement `xbreed agents sync` using `gh api` + `agents.lock` TOML for tag/sha-pinned agent fetch. Belongs in xbreed Rust surface (per `feedback_prefer_rust_over_python`).
- **EVIDENCE:** `gh api` content-fetch with `?ref=<tag>` is the canonical GitHub-pinned-fetch primitive; existing xbreed CLI substrate is clap-derive-based.
- **REJECTED-ALT:** Bash wrapper around `gh api` (rejected: home-language is Rust; CLI-shaped per CLAUDE.md).
- **Confidence:** HIGH (design); LOW (delivery — out-of-scope for this round)

---

## 3. F-axis Correction Loop

The F-axis (feature surface of `gh skill`) traversed three states before resolution:

1. **Scout overclaim (gemini medium).** Initial proposal listed `--immutable`, `gh skill verify`, `gh skill audit`, `--dry-run` for install, `.skill-lock.json`, and mandatory `version:` field. Plausible-sounding feature surface; none of it survived primary-source check.
2. **Labrat false-negative (codex --spark).** First labrat probe returned 4/4 NEGATIVE — appearing to confirm scout's framing. Root cause: spark sandbox lacked `gh` v2.90.0 (present in judge env). Self-corrected on second probe to 4/4 POSITIVE.
3. **Judge primary-source resolution.** Judge ran `gh skill --help` and `gh skill publish --help` directly; resolved the conflict. Real surface: `install/publish/list/view/remove/upgrade`; `--dry-run` is publish-only; only `name`+`description` required.

**Lessons:**
- **(a) Gemini hallucinates feature surfaces under research prompts.** `--immutable`, `verify`, `audit`, `.skill-lock.json` were all internally consistent inventions.
- **(b) codex-spark sandbox ≠ judge env.** Tools present in CC parent (gh v2.90.0) can be absent in spark; negative labrat results from spark must be treated as inconclusive until tool-presence confirmed.
- **(c) Primary-source > peer cross-correction.** Judge's `--help` invocation broke the tie that scout-vs-labrat could not resolve internally. Aligns with `feedback_critic_hallucination.md` — even strong critics fabricate state assertions.
- **(d) Scout's structural framing was correct.** Tree-SHA / frontmatter-metadata-injection model was right; specific artifact names (`.skill-lock.json`, `version:`) were wrong. Don't discard structurally-sound proposals over surface-hallucination.

---

## 4. Per-Class Adoption Verdicts

| Class | Verdict | Mechanism |
|-------|---------|-----------|
| **Skills** | PASS — native | `gh skill install` + dedicated repo; pin via `@<tag>` / `@<sha>` / `--pin`. Required frontmatter: `name` + `description`. |
| **Agents** | PASS — xbreed-owned | PATH B: `gh api repos/<owner>/<repo>/contents/agents/<name>.md?ref=<tag>` + `agents.lock` TOML. Symlink-to-local-checkout fallback (already prod for 4 agents). |
| **Commands** | PASS — no-op | Already in xbreed repo + `include_str!` Build/CI binding. |
| **Memory** | GATE_FAIL (substrate) | Index-contract drift + path-slug coupling + wrong-abstraction. RECOMMENDATION: index-fix prerequisite + selective MUST-tier elevation to agents/skills/shared.md (NOT verbatim memory versioning). |

---

## 5. Conflicts (4 verified)

### (i) F-axis self-correction loop
- **Status:** RESOLVED
- Scout claims (`--immutable`, `verify`, `audit`, `.skill-lock.json`, mandatory `version:`) refuted by judge `gh skill --help` / `gh skill publish --help` primary-source.

### (ii) Connector audit-runtime split vs judge partial refutation
- **Status:** CLOSED for native skills; OPEN for non-skills under bare-git
- Connector posited audit/runtime substrate split. Judge: not applicable inside `gh skill` (audit metadata is frontmatter-injected, not separate file). For agents/commands fetched via bare `gh api`, the audit-runtime question remains open as a design space.

### (iii) Critic trendsetter trigger vs labrat symlink confirmation
- **Status:** DOWNGRADED
- Critic: symlink-to-local-checkout might require trendsetter-disqualifying client-side patching. Labrat (post-correction): symlinks work natively in `~/.claude/agents/`; no shim required. Critic concern dismissed on empirical grounds.

### (iv) `gh skill publish` scope vs commands-only versioning
- **Status:** CONSENSUS — dedicated skills repo required
- `gh skill publish` validator is repo-root-shaped; cannot publish from xbreed monorepo subdirectory. Resolved by Tier 1 recommendation (`xbreed-skills` repo when curated skills emerge).

---

## 6. Pareto Verdict per Move

| Move | Axis | Verdict |
|------|------|---------|
| M01 | F | **ACCEPT** (with spoof-strip — see §7) |
| M02 | A | **ACCEPT** |
| M03 | S/E | **ACCEPT** |
| M04 | M/R | **ACCEPT** (GATE_FAIL is the verdict) |
| M05 | M/I | **ACCEPT** (recommendation-tier) |
| M06 | F/S | **ACCEPT** (conditional Tier 1) |
| M07 | A/E | **ACCEPT** (Tier 2 design) |

7/7 ACCEPT. 0 dropped. 1 spoof-flagged (M01 partial).

---

## 7. Spoof Flags

**M01 partial.** Six scout-introduced hallucinations stripped per judge primary-source override (`gh skill --help` / `gh skill publish --help`):

| Spoofed claim | Reality |
|---------------|---------|
| `--immutable` flag | NOT in `gh skill publish` flag set |
| `gh skill verify` subcommand | NOT a subcommand |
| `gh skill audit` subcommand | NOT a subcommand |
| `--dry-run` for install | NOT an install flag (publish has it) |
| `.skill-lock.json` lockfile | Does NOT exist; metadata in frontmatter |
| Mandatory `version:` frontmatter field | FALSE; only `name` + `description` required per validator |

---

## 8. Optimization Routes Surveyed (informational only)

Per `feedback_no_policy_hardening.md`, no auto-actions. Surfaced for user decision:

**Prerequisites (un-block before Tier 1+):**
- (a) Fix MEMORY.md CAT drift — add the 2 missing entries OR delete those 2 files (per yesterday's `map-feedback-memory-r1`).
- (b) Update `feedback_teammate_mode_effort_caveat.md` (stale per yesterday's C3a).
- (c) Address C3d (`no_schedule_suggestions` vs `schedule` skill conflict).

**Tier 1 (skills):**
- (d) Create dedicated `xbreed-skills` repo when curated skills emerge; use `gh skill install` natively.

**Tier 2 (agents):**
- (e) Implement `xbreed agents sync` Rust subcommand using `gh api` + `agents.lock`; create `xbreed-agents` repo.

**Tier 3 (commands):**
- (f) Stay in xbreed monorepo (already version-tracked via `include_str!`).

**Tier 4 (memory):**
- (g) Selective elevation: MUST-tier entries promoted to agents/`shared.md`; rest stay convention-tier in MEMORY.md.

---

## 9. audit_hash

```
3a0737cfd4279d1b6a2d663db20f8547d77bcf33bb230fc1f0f347b8d444b386
```

---

## 10. Coverage Gaps (open)

Four gaps surfaced by distiller; partial closure this round:

1. **`--immutable` flag verification** — CLOSED. Judge `gh skill publish --help` confirmed absence.
2. **`agentskills.io/specification` not fetched** — OPEN. External spec page not retrieved this round.
3. **Signed-commit verification** — OPEN. Whether `gh skill install` validates commit signatures not probed.
4. **Frontmatter-injection verbatim fields** — OPEN. Beyond `name` + `description` (required), the full metadata-injection field list not enumerated.

---

## Links

- **Plan / framing:** team-lead brief, gh-skill-versioning-0425 R1
- **Prior context:** `docs/reports/gh-skill-versioning-2026-04-25.md` (pre-R1 audit)
- **Related:** `docs/reports/map-feedback-memory-r1-2026-04-25.md` (M04 prerequisite)
- **Next:** R2 (if dispatched) — close gaps 2/3/4; or Tier-1 pickup on `xbreed-skills` repo creation.
