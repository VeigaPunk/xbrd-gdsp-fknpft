# Map — `gh skill` Adoption + Trendsetter Extension to Non-Skill Artifacts

**Mission:** `gh-skill-versioning-0425` · adopt GitHub's `gh skill` as substrate for version pinning + audit trace, extended beyond skills to agents/commands/memory
**Date:** 2026-04-25
**Method:** `/xbgst /wwkd | godspeed` · 1 round · 8 axes · 10 teammates · 7 of 7 moves accepted
**Companion:** yesterday's `map-feedback-memory-2026-04-25.md` (committed `72da14d`) — the Tier A/B/C taxonomy this map's ethos-elevation rule rests on
**audit_hash:** `3a0737cfd4279d1b6a2d663db20f8547d77bcf33bb230fc1f0f347b8d444b386`

---

## TL;DR

1. **`gh skill` is REAL.** gh CLI v2.90.0 (released 2026-04-16, **in preview**). Subcommands: `install / preview / publish / search / update`. Pinning via `@<tag>` or `@<commit-sha>` or `--pin`. Cross-host (Copilot / Claude Code / Cursor / Codex / Gemini / Antigravity). Spec: https://agentskills.io/specification.
2. **Native install path for Claude Code is `~/.claude/skills/`** — exactly where xbreed's existing skills live. **Zero layout reshape needed for skills.**
3. **`gh skill install` does NOT reach `~/.claude/agents/`** — only `~/.claude/skills/`. Agents/memory/commands need separate adoption paths.
4. **The user's trendsetter pivot ("extend beyond skills") is implementable trendsetter-compliantly via separate paths** — `gh skill` for skills natively; PATH B (`gh api` fetch + `agents.lock`) for agents; in-repo for commands; selective elevation for memory.
5. **Memory is GATE_FAIL for full substrate move** — index-contract migration is adaptation; path-slug coupling locks substrate to exact filesystem; live drift already happening (2 CAT-drifted files yesterday). **Recommendation: selective elevation, NOT verbatim versioning.**
6. **The F-axis correction loop is itself the load-bearing trust pattern**: scout overclaimed 6 specific feature surfaces (`.skill-lock.json`, `verify`/`audit` subcommands, `--immutable` flag, mandatory `version:` field, Sigstore attestations); labrat returned 4/4 false-negative due to codex-spark sandbox missing gh v2.90.0; judge primary-source `gh skill --help` resolved both. **Same pattern fired yesterday on the orphan-list overclaim.** Cross-pollination + primary-source verification is what makes the system robust against single-agent error.

---

## §F — Feature Mechanics (`gh skill`)

> **Source:** M01-feature (g-scout-feature-r1) + judge primary-source corrections + ccs-labrat-firetests-r1 self-corrected re-probe
> **Linchpin:** `gh skill --help` exits 0; subcommand list verified via direct Bash exec

### Confirmed mechanics (primary-source)

| Field | Value | Verification |
|---|---|---|
| Feature name | `gh skill` (gh CLI built-in subcommand) | `gh --version` → `2.90.0 (2026-04-16)` |
| Status | **Preview** ("subject to change without notice") | Help text |
| Subcommands | `install` · `preview` · `publish` · `search` · `update` | `gh skill --help` |
| Spec | https://agentskills.io/specification (cross-vendor open spec, NOT GitHub-proprietary) | Help text reference |
| Reference repo | `github/awesome-copilot` | Help examples; `gh skill preview github/awesome-copilot documentation-writer` returns real SKILL.md |
| Community ecosystem | `majiayu000/claude-skill-registry` (215★), `Mindrally/skills`, others | `gh skill search` returns real results |
| Install paths (host × scope) | See table below | `gh skill install --help` |
| Pinning | `@<semver-tag>` · `@<commit-sha>` · `--pin <ref>` flag | `gh skill install --help` |
| Pin enforcement | API-enforced server-side (HTTP 422 on invalid ref) | labrat empirical probe |
| Required frontmatter | `name`, `description` only | `gh skill publish --help` validation rules |
| Tracking metadata | Injected into LOCAL `SKILL.md` frontmatter on install | `gh skill install --help` ("tracking metadata injected into frontmatter") |
| Distribution | Raw repo fetch + GitHub Releases for tags + `agent-skills` topic on repo | `gh skill publish --help` |
| Auth | Ambient `gh auth` (OAuth Device Flow / PAT / `GH_TOKEN`) | help + xbreed inherits parent env without `env_clear()` |
| Files written | Local copies (NOT symlinks) at the host's install path | `gh skill install --help` ("files are copied (not symlinked)") |

### Multi-host install paths

| Host | Project scope | User scope |
|---|---|---|
| GitHub Copilot | `.agents/skills` | `~/.copilot/skills` |
| Claude Code | `.claude/skills` | **`~/.claude/skills`** ← xbreed's existing path |
| Cursor | `.agents/skills` | `~/.cursor/skills` |
| Codex | `.agents/skills` | `~/.codex/skills` |
| Gemini CLI | `.agents/skills` | `~/.gemini/skills` |
| Antigravity | `.agents/skills` | `~/.gemini/antigravity/skills` |

### Stripped scout overclaims (6 hallucinations, primary-source disconfirmed)

| Claim | Status | How disconfirmed |
|---|---|---|
| `.skill-lock.json` lockfile | **HALLUCINATED** | `gh skill install --help` says "tracking metadata injected into frontmatter"; no lock file mentioned anywhere |
| `gh skill verify` subcommand | **HALLUCINATED** | `gh skill --help` lists 5 subcommands; verify absent |
| `gh skill audit` subcommand | **HALLUCINATED** | Same as above |
| `--immutable` flag on publish | **HALLUCINATED** | `gh skill publish --help \| grep -i immutable` returns nothing (judge primary-source) |
| Sigstore + GitHub Artifact Attestations | **HALLUCINATED** | Same probe; `attest`/`sign` absent from publish flag set |
| Mandatory `version:` frontmatter field | **FALSE** | Validation rules: only `name` + `description` required; version comes from git tag at publish time |

This is **the second time in three days** the spoof-correction pattern has fired (yesterday: revenger 12-orphans → distiller 5-true; today: scout 6 hallucinations → judge primary-source). Documented as a load-bearing meta-pattern in §X.

---

## §M — Current State (xbreed Artifact Versioning Surface)

> **Source:** M02-recon (cdx-revenger-current-r1) + map-feedback-memory-2026-04-25.md ground truth
> **Linchpin:** Commands is the ONLY class with hard runtime drift gate (`include_str!` Build/CI bind)

| Class | In git? | Audit trace | Loader | Drift gate | Migration friction |
|---|---|---|---|---|---|
| **Commands** (7 `.md`) | ✅ in xbreed monorepo | Full `git log`/blame/diff + **`include_str!` compile-bind on `xbreed-shared.md`** | CC slash-discovery | **HARD** (build breaks on drift; `verify-docs.sh` + `make verify-docs`) | **LOWEST** — already correct substrate |
| **Skills** (6 dirs) | MIXED — `godspeed-mode` symlinked to upstream git repo at `/home/vhpnk/godspeed-mode/`; user-local skills untracked | Inherited via symlink at upstream; user-local: none | `xbreed ask --with` 3-path search; CC manifest + on-demand load | SOFT (load-time presence check only) | HIGH |
| **Agents** (18 `.md`) | ❌ NOT in any git repo (`~/.claude/` is fresh git with NO_HEAD, nothing committed); 4 live symlinks to out-of-repo dirs | Absent — no commit/blame/diff; mtime side-channel only | CC subagent discovery from `~/.claude/agents/`; commands explicitly hardload `the-judge.md` | SOFT (`axis_family_schema_check.sh` + `verify-docs.sh` connector-row mirror; not CI-wired) | VERY HIGH |
| **Memory** (47 `.md` + index) | ❌ NOT in any git repo | Absent — filesystem timestamps only | **INDEX-ONLY**: `MEMORY.md` is the gate; CC loads only entries listed there (yesterday H3 confirmed via labrat contra-indication probe) | NONE | **HIGHEST** |

### Cross-class coupling

- **Commands → Agents** (load-bearing): `commands/xbreed.md`, `xbgst.md`, `xgs.md` explicitly instruct sessions to `Read ~/.claude/agents/the-judge.md`. Half-landed routing trap documented in `feedback_half_landed_routing_pattern.md`.
- **Commands → Skills**: `xbreed-shared.md` references `godspeed` + `godspeed-mode` as operating protocol.
- **Agents → Skills**: `the-planner.md` Layer-0 loads `wwkd` then `godspeed-mode`.
- **Memory → All**: behavioral coupling only (shapes edits/docs); no structural load dependency.

### Live drift already happening (today, in current state)

- 2 memory files on disk **NOT** in `MEMORY.md` index — silent invisibility (per yesterday's CAT axis):
  - `feedback_agents_canonical_source.md` (has 9 inbound repo refs but never loads)
  - `feedback_no_claude_md_overhead.md` (zero inbound + not in index = doubly invisible)
- 4 live production symlinks in `~/.claude/agents/` (ocnus, almanacker, musketeer, puppeteer) — symlink-to-out-of-repo IS the existing pattern
- Memory path-slug encoding: `~/.claude/projects/-home-vhpnk-projects-xbrd-gdsp-fknpft/memory/` (deterministic + machine-specific; solo-dev = irrelevant; multi-machine = silent miss)

---

## §A — Adaptation Paths (Per-Class Adoption Shape)

> **Source:** synthesis of M01 (feature mechanics) + M02 (current state) + M03 (compat) + scout PATH A/B
> **Linchpin:** trendsetter-compliant ≠ uniform; different artifact classes require different mechanisms

### Skills — Native `gh skill install` (PASS)

```bash
# Pin to a specific tag
gh skill install <owner>/<repo> <skill-name>@v1.2.0

# Pin to a specific commit SHA
gh skill install <owner>/<repo> <skill-name>@a1b2c3d

# Or via flag
gh skill install <owner>/<repo> <skill-name> --pin v1.2.0
```

- Writes to `~/.claude/skills/<name>/SKILL.md`
- Tracking metadata (source repo + ref) injected into local frontmatter at install time
- `gh skill update` compares local tree SHA against remote
- `--pin` skips a skill from `gh skill update` (use `--unpin` to re-include)
- Required frontmatter: `name`, `description`
- Validation: skill-name matches dir-name; `allowed-tools` is string (not array); install metadata stripped if present

**Trendsetter check**: PASS. Native path; `~/.claude/skills/` is exactly where xbreed's skills already live; no layout adaptation.

**Caveat — dedicated repo required for publish**: `gh skill publish` creates a GitHub release on the **whole repo**, no subdirectory scoping. The xbreed monorepo (Rust binary at v0.4.0) cannot host published skills without semver collision. **Adoption shape: dedicated `xbreed-skills` repo if/when curated skills emerge.** The xbreed monorepo continues to host source code; a separate skills repo hosts versioned skill bundles.

### Agents — Manual `cp` Checkpoint (user-chosen 2026-04-25)

`~/.claude/agents/` stays canonical (per 2026-04-17 directive at `AGENTS.md:4`, `README.md:48`, `xbreed-shared.md:309`). The xbreed-agents repo is a **snapshot store**, not the source of truth. Plain `cp` workflow — no symlinks, no Rust code, no lockfile.

**Checkpoint** (when state is working well):
```bash
cd ~/projects/xbreed-agents
find ~/.claude/agents -maxdepth 1 -type f -name "*.md" -exec cp {} agents/ \;
git add . && git commit -m "checkpoint vX.Y.Z — <reason>"
git tag vX.Y.Z
git push origin main --tags
```

`-type f` skips the 4 external symlinks (they live in their own repos).

**Rollback** (didn't like the changes after testing):
```bash
cd ~/projects/xbreed-agents
git checkout vX.Y.Z
cp agents/*.md ~/.claude/agents/
```

`~/.claude/agents/` is overwritten with the tagged snapshot. The 4 external symlinks are untouched (they don't exist in the repo).

**Why not symlink-to-checkout** (rejected by user 2026-04-25): symlinks would require **inverting canonicality** — the repo would silently become authoritative, `~/.claude/agents/` would become a view. User explicitly chose to keep `~/.claude/agents/` canonical and use the repo for snapshots only. The mechanical-simplicity gain isn't worth the doctrine flip.

#### Fallback — PATH B (`gh api` Fetch + Lockfile)

For installing agents from EXTERNAL/community repos (NOT in your own xbreed-agents):

```bash
gh api repos/<owner>/<repo>/contents/agents/<name>.md?ref=<tag> \
  | jq -r .content | base64 -d > ~/.claude/agents/<name>.md
```

With an optional `agents.lock` TOML (per-agent `repo + ref + sha256`). New scope in `src/sync.rs`. Trendsetter-compliant only if motivated by an independent agent-versioning goal (per the discriminator in §S); pure gap-patching for `gh skill`'s limitation = disqualifying. **For your own 14 agents in xbreed-agents repo**: skip PATH B; the manual `cp` workflow above is what you wanted.

### Commands — In-Repo (No-Op, PASS)

Commands already live at `commands/*.md` in the xbreed monorepo. Already version-tracked via main repo semver. The `include_str!` Build/CI binding (`src/protocol.rs` references `xbreed-shared.md`) already enforces drift detection — drift breaks the build. **No migration needed.**

### Memory — Selective Elevation (NOT verbatim versioning)

> **Verdict:** GATE_FAIL for full substrate move. Recommend selective elevation of MUST-tier entries to existing version-tracked locations.

Memory has three structural blockers:
1. **MEMORY.md is the loader gate** (per yesterday H3) — substrate move must preserve OR replace this contract; replacing requires CC harness change (not user-reachable)
2. **Path-slug coupling** — substrate locks to `~/.claude/projects/-home-vhpnk-projects-xbrd-gdsp-fknpft/memory/`; moving = silent miss
3. **Live drift already happening** — 2 CAT-drifted entries prove the index-vs-disk gap is current state, not migration hypothetical

The trendsetter-compliant move is **NOT versioning memory as a class**. Instead, **elevate select MUST-tier entries to existing version-tracked locations** where their content type fits:

| Memory entry | Elevation target | Mechanism |
|---|---|---|
| `feedback_xask_flag_order.md` | Already enforced at `scripts/xask:37` (Runtime tier); memory is redundant narrative | Consolidate as inline comment in `scripts/xask`; archive memory file |
| `feedback_unified_tier_scheme.md` | Already in `xbreed-shared.md` "Axis → Profile Mapping" section | Memory becomes a pointer to shared.md anchor; archive narrative copy |
| `reference_godspeed_skills.md` (35 inbound refs — most-cited) | xbreed-shared.md godspeed-mode section + `~/.claude/skills/godspeed/SKILL.md` (already in skills) | Memory becomes pointer; canonical lives in skill |
| `feedback_critic_hallucination.md` (17 inbound refs) | Could elevate to xbreed-shared.md as a §Distiller-spot-check protocol section | Optional; lower priority |

For Tier B (docs-cited convention-laundered) and Tier C (orphan) entries: stay convention-tier. The yesterday-map's priority inversion finding holds — Tier B is the higher-blast remediation surface, but the right fix is **promotion to enforceable tiers** (Runtime/Build-CI), not version-controlled memory archives.

### Distribution channel disambiguation

| Channel | What it covers | Where it writes | Status |
|---|---|---|---|
| `gh skill install` | Skills only | `~/.claude/skills/` | Native, primary-source verified |
| `enabledPlugins` (settings.json) | Plugins only | Internal CC namespace | **NOT** the `{source, ref}` GitHub-pinning shape scout described — live schema is `name@registry: boolean` only (revenger primary-source); scout's PATH A is **disconfirmed at the schema level** |
| `gh api` fetch (PATH B) | Anything in any repo | xbreed-controlled paths | xbreed-owned tooling; new `src/sync.rs` scope |
| Symlink-to-checkout | Any local dir CC's loader scans | `~/.claude/{agents,skills,commands}/` | Already production for 4 agents |

---

## §T — Trust Model (Provenance vs Correctness)

> **Source:** M03-compat (cdx-reviewer-compat-r1) + M04-compose (g-connector-crossaxis-r1) + M05-ach (cdx-critic-migration-r1)
> **Linchpin:** CC harness performs ZERO hash/signature validation on loaded `.md` files

### Two distinct trust grades

- **Provenance-grade trust** (achievable today): git commit history + GitHub release tags + signed commits provide an audit trail of WHO changed WHAT and WHEN. Operator can `git log --show-signature -1` to verify.
- **Correctness-grade trust** (NOT achievable through `gh skill` alone): runtime guarantee that the loaded artifact matches the pinned version. Requires CC harness to validate content against frontmatter pin OR an out-of-band pre-load gate.

### Per-channel trust verdict

| Channel | Trust grade | Why |
|---|---|---|
| `gh skill install` (skills) | **Provenance + partial correctness** | Tracking metadata in local frontmatter; loader sees pin in the file it reads. **HOWEVER**: pull-once-cache (per labrat access timestamp evidence) — local file can be mutated post-install with no re-validation on next session load. Drift is possible, not just theoretical. |
| PATH B `gh api` fetch (agents) | **Provenance only** by default; correctness ADD-ON if xbreed implements pre-load hash check | Lockfile records sha256; xbreed-side verification possible but not built |
| Symlink-to-checkout | **Provenance only** | Symlink resolves to whatever HEAD is at; commit history is in the repo but loader doesn't consult it |
| In-repo commands | **Provenance + correctness** | `include_str!` Build/CI binding fails the build on drift |

### The "audit-runtime decoupling" concern (connector M04)

Original framing: "version pins live only in commit history; loader reads HEAD; audit and runtime diverge."

**Refinements** (post primary-source):
- For native `gh skill install`: **PARTIALLY MITIGATED** — frontmatter metadata injection puts the pin in the local file the loader reads. Audit and runtime are SAME-FILE coupled.
- For PATH B without lockfile verification: **GAP STANDS** — operator-discipline required.
- For symlink-to-checkout: **GAP STANDS** — pre-load `git verify-commit` is operator-discipline only.

### Goodhart-decoy (critic M05)

> Operator trusts GitHub history because "it looks more authoritative" → stops checking the live filesystem path → silent drift between tagged-version-in-history and actually-loaded-version.

This concern is REAL for non-skill artifacts under bare-git usage, and SOFTENED for native `gh skill` (frontmatter pin is loader-visible). Map names this honestly — adopting `gh skill` for skills closes one decoy surface but does NOT prevent the Tier B convention-laundering pattern from yesterday's map.

### CC harness validation gap

Reviewer M03 confirmed: **no CC-side signature/hash validation on loaded `.md` files**. A repo compromise (malicious PR merge, force-push, local filesystem write) injects behavior changes at next session load with **no harness-side tripwire**. Trust is git-provider-level, NOT harness-level.

This is **structurally bounded by CC's loader design**, not by xbreed's adoption choices. Mitigations are operator-side: signed commits + pre-session `git verify-commit HEAD` + branch-protection rules on the source repo.

---

## §I — Loader Compatibility (Path-Coupling)

> **Source:** M03-compat (cdx-reviewer-compat-r1) + M07-firetests (ccs-labrat) symlink probe
> **Linchpin:** every artifact class is path-coupled; symlink-at-exact-path is the only structurally compliant loader bridge

| Class | Path-coupling | Override knob? | Symlink works? |
|---|---|---|---|
| Memory | **PATH-COUPLED** to cwd-derived absolute path slug | NONE (no `memory_path`/`memory_url` in settings.json or env) | Yes, at exact slug path; orphans silent (H3) |
| Skills | PATH-COUPLED to `~/.claude/skills/<name>/SKILL.md` (or project equivalent) | NONE (`Skill()` has no remote URL deref) | Yes — `gh skill install` natively writes here |
| Agents | PATH-COUPLED to `~/.claude/agents/` and `.claude/agents/` | NONE (`--add-dir` does NOT extend agent discovery) | **EMPIRICALLY YES** — 4 live production symlinks confirm |
| Commands | PATH-FLEXIBLE (real files in xbreed repo per simplifier correction; CLAUDE.md "symlinks" label was stale) | — | Already in-repo; no symlink needed |

**Critical implication**: GitHub-hosted artifact + symlink-to-local-checkout is the **only** structurally compatible distribution shape for non-skill non-command classes. Sync-on-demand (network call per load) violates the index-only gate (memory) and adds latency. CC has no `*_path`/`*_url` config knob to point at remote repos directly.

---

## §S — Substrate Decision Per Class

> **Source:** M06-yagni (ccs-simplifier-r1) + judge synthesis
> **Linchpin:** `user_trendsetter_principle.md` red-flag list (a)–(d)

### Per-class substrate verdict

| Class | Substrate decision | Trendsetter check | Audit trace |
|---|---|---|---|
| Skills | Dedicated `xbreed-skills` (or similar) repo + `gh skill install` | PASS — native path, no adaptation | GitHub Releases + tags + frontmatter pin (loader-visible) |
| Agents | xbreed-owned `xbreed-agents` repo + PATH B `gh api` fetch + `agents.lock` | PASS — xbreed owns the tool; not adapting to gh skill | Repo git history + lockfile sha256 + (optional) pre-load hash check |
| Commands | Stay in xbreed monorepo (already there) | PASS — no migration | xbreed repo git history + `include_str!` Build/CI |
| Memory | **Do NOT version as a class.** Selectively elevate MUST-tier entries to other paths (xbreed-shared.md, scripts/xask comments, agents/skills) | Full substrate FAIL (adaptation). Selective elevation PASS (consolidation). | Inherited from elevation target |

### Why "expand the feature beyond skills" was technically misdiagnosed

The user's framing — "this feature should be expanded to versioning any relevant .md file" — implies extending `gh skill` itself. **`gh skill` is path-scoped to `~/.claude/skills/`**; extending it to `~/.claude/agents/` or memory would require CC harness changes (out-of-user-space) OR layout/semantic adaptation of agents/memory (trendsetter VIOLATION).

**The trendsetter-compliant version of the same intent**: use the same git+GitHub primitives `gh skill` uses (Releases, tags, content-addressing, pinning) for non-skill artifacts via `gh api` + xbreed-owned tooling. The end state is identical (version-pinned, audit-traced, GitHub-hosted), the framing differs (`gh skill` for skills only; bare git+GitHub for non-skills via xbreed PATH B).

This is a **consolidation move dressed as expansion**. Same substrate (GitHub), different tools per artifact class.

---

## §E — Ethos Elevation Criteria (Mechanism-Agnostic)

> **Source:** M06-yagni (ccs-simplifier-r1)
> **Linchpin:** blast-radius × cross-axis coupling — NOT inbound-ref count alone

The user's "memories that we deem relevant (to later be incorporated in the ethos of the flow)" maps to a 3-tier rule:

| Tier | Criterion | Examples (from yesterday's map) |
|---|---|---|
| **MUST elevate** | Protocol × LIVE + HIGH blast-radius if violated + HIGH cross-axis coupling | `feedback_xask_flag_order` (scripts/xask:37 — breaks ALL routing); `feedback_unified_tier_scheme` (whole-table effort-tier corruption, c725c2a-class); `reference_godspeed_skills` (35 inbound refs); `feedback_teammate_mode_effort_caveat` (29 inbound refs); `feedback_critic_hallucination` (17 inbound refs); `project_on_spawn_skill_dead_metadata` (22 inbound refs) |
| **MAY elevate** | Docs-cited Tier B; single-hop promotion path; contained blast-radius | `feedback_connector_every_round` (13 refs, single xbgst flow); `feedback_scribe_per_round` (11 refs, same scope); `feedback_xbgst_cdx_teammate` (10 refs); `feedback_half_landed_routing_pattern` (10 refs) |
| **DO NOT elevate** | Behavioral preference, no cross-axis coupling, OR operationally inert | `feedback_prefer_rust_over_python`; `feedback_no_safety_theater`; `feedback_wsl2_ext4_faster_tmpfs`; `feedback_no_remember_plugin` (settings.json is SSoT); `feedback_no_obsidian_mcp` |

**Anti-overfit check**: "Would this elevation rule still be worthwhile if `gh skill` disappeared?" YES — blast-radius × coupling is the right promotion gate regardless of mechanism. The rule names which memories deserve **promotion to enforceable tiers** (Runtime/Build-CI), not which deserve "version-controlled memory archive" status.

**Implication for the user's pivot**: the request "store memories in GitHub for audit" decomposes into:
- (a) MUST-elevate entries: promote to existing version-tracked locations (xbreed-shared.md, scripts/xask, agents/, skills/) — **already have audit trail there**
- (b) MAY-elevate entries: same promotion path, lower priority
- (c) DO NOT elevate: stay convention-tier; audit value not worth the layout cost

The "ethos of the flow" benefits more from elevation-to-Runtime than from version-controlled-memory-storage.

---

## §R — Risks (Adversarial)

> **Source:** M05-ach (cdx-critic-migration-r1) + M04-compose (g-connector-crossaxis-r1)
> **Linchpin:** trendsetter trigger NOT activated post-symlink-probe; Goodhart-decoy + monorepo publish-scope mismatch stand

| Risk | Severity | Mitigation |
|---|---|---|
| **Goodhart-decoy drift** — operator trusts GitHub history; live filesystem diverges silently | HIGH for non-skills under bare-git; MED for native gh skill (frontmatter pin loader-visible) | Pre-load hash-match gate (xbreed-side, not CC-side); signed commits + `git verify-commit` discipline |
| **`gh skill publish` whole-repo release scope** — incompatible with xbreed monorepo + commit-per-round pattern | HIGH | Dedicated `xbreed-skills` repo (NOT publish skills from xbreed monorepo) |
| **§C × §A coupling break** — agent substrate move without commands/ co-update silently breaks orchestration (`feedback_half_landed_routing_pattern.md`) | MED | Pre-flight grep: `grep -rn "the-judge\.md\|the-planner\.md" commands/` before declaring agent migration complete |
| **Memory index-contract drift** — already happening today (2 CAT-drifted entries) | HIGH (existing) | Index-fix prerequisite BEFORE any substrate move; per-CI gate (extend `verify-docs.sh`) |
| **Path-slug coupling** — solo-dev = irrelevant; multi-machine = silent miss | LOW (current solo-dev context) | Document explicit machine-specific assumption |
| **Convention-laundering compounded** — adding GitHub layer on top of existing index-vs-disk drift adds third authority (git history) without fixing the first two | MED | Fix yesterday's CAT axis FIRST (Phase 0 prerequisite); only then consider versioning |
| **CC harness no-validation** — out-of-scope for any user-space adoption choice | STRUCTURAL | Operator-discipline only; not closeable from user-space |

### Trendsetter compliance verdict

Per `user_trendsetter_principle.md:13-16` red-flag list:

| Adoption shape | Red-flag check | Verdict |
|---|---|---|
| `gh skill` for skills (dedicated repo) | (a) "client-side patching to accommodate missing capability"? NO — native path | PASS |
| PATH B for agents (xbreed-owned `gh api` + lockfile) | (a)? NO — xbreed already has `src/sync.rs` for settings.json materialization; agents fetch is a natural extension. (c) "introduces runtime deps not in stack"? NO — gh CLI already in stack | PASS |
| Commands in xbreed monorepo (no-op) | All red-flags N/A — no migration | PASS |
| Memory full substrate move | (a)? YES — index-contract + path-slug both require client-side accommodation. (d) "requires posture/persona to change"? YES — commit-per-round pattern incompatible with publish-immutable. | **VIOLATION** |
| Memory selective elevation | All red-flags N/A — consolidation, not expansion | PASS |

### What c725c2a-class regression looks like under adoption

Per yesterday's `map-flawless-why` §F: a docs-only rewire that touched both canonical (`xbreed-shared.md`) and mirror (`the-judge.md`) in lockstep, briefly making them agree on a worse contract. **Adoption parallel**: a `gh skill update` that pulls a malicious or buggy upstream version would land in `~/.claude/skills/<name>/SKILL.md` with no harness validation. The frontmatter pin update would be visible in git history of the upstream skills repo, but the operator would have to actively check before next session load. **Defense**: `--pin` to known-good ref; `git verify-commit` discipline on the upstream repo's tag; periodic `gh skill update --dry-run` before pulling.

---

## §X — Synthesis: What the User Actually Asked For

The user's request decomposes into THREE distinct sub-requests, each with a different trendsetter-compliant answer:

### 1. "Use the new GitHub feature for skill versioning"

**Answer**: PASS. Use `gh skill install` natively for skills with a dedicated `xbreed-skills` repo. Cross-host (Claude/Cursor/Codex/Gemini/Copilot/Antigravity) supported uniformly. Pin via `@<tag>` or `@<sha>` or `--pin`. Tracking metadata lives in local frontmatter (loader-visible).

### 2. "Store memories that we deem relevant (to later be incorporated in the ethos of the flow)"

**Answer**: TRENDSETTER PIVOT. Don't version memory as a class — selectively elevate MUST-tier entries to existing version-tracked locations (xbreed-shared.md, scripts/xask, agents/, skills/). The "ethos" is better served by promotion to enforceable tiers than by GitHub-versioned memory archives.

### 3. "Agents versioning, skill versioning, command versioning"

**Answer**: HETEROGENEOUS — different mechanism per class:
- **Skills**: native `gh skill install` (Tier 1)
- **Agents**: xbreed-owned PATH B with `gh api` + `agents.lock` (Tier 2, requires Rust scope add to `src/sync.rs`)
- **Commands**: already version-tracked in xbreed monorepo + `include_str!` Build/CI bind (Tier 3 = no-op)
- **Memory**: selective elevation only (Tier 4 = consolidation, not versioning)

**The trendsetter-compliant unified mental model**: "use GitHub Releases + git tags + content-addressing as the underlying primitives across all classes; pick the right tool per class (`gh skill` for skills, `gh api` + xbreed Rust for agents, in-repo for commands, elevation for memory)."

### F-axis correction loop as load-bearing meta-pattern

The scout-overclaim → labrat-false-negative → judge-primary-source resolution that fired this round is **not a bug; it's the system working as designed**. Cross-pollination + primary-source verification is what makes the godspeed Pareto walk robust against single-agent error.

Documented occurrences:
- 2026-04-24 (`map-flawless-why-r1`): connector self-flagged hallucinated specifics on c725c2a; spot-check caught
- 2026-04-25 morning (`map-feedback-memory-r1`): revenger 12-orphans → distiller 5-true via primary-source spot-check on revenger's own scratch artifact
- 2026-04-25 afternoon (this report): scout 6 hallucinations → judge `gh skill --help` → confirmed list

**The pattern is reliable enough to be load-bearing**. A new session reading this map should note: gemini under research prompts hallucinates plausible-sounding feature surfaces; codex-spark sandboxes can lack tools present in the parent session; primary-source > peer cross-correction. Per `feedback_critic_hallucination.md`, this remains the canonical correction protocol.

---

## §Z — Optimization Routes (informational)

Per `feedback_no_policy_hardening.md` and `feedback_no_safety_theater.md`, listed for completeness; **no actions taken**, user decides:

| # | Route | Phase | Closes |
|---|---|---|---|
| (a) | Fix yesterday's CAT drift: add 2 missing entries to MEMORY.md OR delete those 2 files | Prerequisite | Live index-vs-disk drift |
| (b) | Update `feedback_teammate_mode_effort_caveat.md` to match live `~/.bashrc` trap | Prerequisite | Yesterday's C3a stale-content |
| (c) | Address C3d (no_schedule_suggestions vs schedule skill manifest) — settings.json deny on /schedule OR PreToolUse hook | Prerequisite | Live Goodhart-decoy convention-laundered conflict |
| (d) | Create dedicated `xbreed-skills` repo if/when curated skills emerge; use `gh skill install` for native distribution | Tier 1 | Skills versioning + audit |
| (e) | Implement `xbreed agents sync` Rust subcommand using `gh api` + `agents.lock`; create `xbreed-agents` repo | Tier 2 | Agents versioning + audit + provenance |
| (f) | Selective elevation: promote MUST-tier memory entries to existing version-tracked paths (xbreed-shared.md sections, scripts/xask comments, agents/skills SSoT) | Tier 4 | Memory ethos elevation without trendsetter violation |
| (g) | Extend `verify-docs.sh` to cover MEMORY.md ↔ on-disk drift (CI gate) | Hardening | Future CAT drift |
| (h) | Operator-discipline: `git verify-commit` on upstream skills repo before `gh skill update`; `--pin` to known-good refs | Hardening | Goodhart-decoy in skills channel |

---

## §0 — What This Map Is NOT

- **Not a remediation plan.** Optimization routes (§Z) are informational; user decides actions.
- **Not a recommendation to migrate.** Per `feedback_no_safety_theater.md`, system is calibrated for solo dev; pre-flight ceremony is anti-value.
- **Not a critique of the user's trendsetter framing.** The user's intent is implementable; the map names which sub-requests admit which tools.
- **Not a final word on `gh skill`.** Feature is in PREVIEW ("subject to change without notice"). Specific subcommands and flags may evolve. Re-verify primary-source before any future adoption decision.
- **Not a unified-tool answer.** The trendsetter-compliant adoption is heterogeneous by class. Forcing uniformity would re-trigger the trendsetter violation.

---

## Appendix A — Round 1 Trace

- **Team**: `gh-skill-versioning-0425`, 10 members, ~13 min wall time
- **Phase 0**: `ccs-planner-r0` (sonnet · WWKD-loaded) — 9-milestone skeleton; v3 update post-revenger M03↔M04 swap
- **Phase 1 axes**: F · M · A · T · I · S · E · R (8 axes)
- **Phase 2 specialists** (7 dispatched in parallel):
  - `g-scout-feature-r1` (gemini medium) — F-axis primary-source research; 6 hallucinations later stripped
  - `cdx-revenger-current-r1` (codex -R -F, 1.05M ctx) — M+S current state RECON
  - `cdx-reviewer-compat-r1` (codex -R, scoped) — I+T loader compatibility
  - `g-connector-crossaxis-r1` (gemini high, 6k cap) — substrate × ethos × audit composition
  - `cdx-critic-migration-r1` (heuer-planning + codex -R) — adversarial ACH on H1-H4
  - `ccs-simplifier-r1` (CC native) — ethos elevation + minimal viable + trendsetter compliance
  - `ccs-labrat-probes-r1` (codex --spark) — empirical probes; **initial 4/4 false-negative due to spark sandbox env mismatch; self-corrected to 4/4 POSITIVE**
- **Phase 3 distiller**: `ccs-distiller` (sonnet · in-session) — 7-move synthesis with judge primary-source overrides preserved
- **Phase 4 scribe**: `ccs-scribe-r1` (**opus · low — first test of yesterday's frontmatter pivot**)
- **Pareto verdict**: 7 of 7 ACCEPTED; integrated R2 inside R1 via 5+ post-DESPAWN amendment cycles per teammate
- **Spoof corrections**: M01 (scout 6 hallucinations stripped); M07 (labrat 4/4 self-reversal)
- **Coverage gaps closed via primary-source**: `--immutable` flag (confirmed absent), gh skill subcommand list (5 verified)

## Appendix B — F-axis Correction Loop Forensics

Forensic chain (chronological):

1. **Phase 0 baseline**: planner notes "F-axis is unknown; team-lead training data does not cover this feature."
2. **Scout dispatch** with primary-source research instruction; gemini xask returns confident, detailed mechanics including `.skill-lock.json`, `gh skill verify`, `gh skill audit`, `--immutable`, mandatory `version:` field.
3. **Labrat dispatch** with empirical probes via codex-spark sandbox.
4. **Labrat returns 4/4 NEGATIVE** — gh CLI subcommand probe reports "unknown subcommand"; raw URLs 404 on guessed namespaces. Marks F-axis as "may not exist."
5. **Scout self-flags reliability concern**: "documentary findings come from gemini web search; labrat probes returned negative; possible interpretations: hallucination, wrong gh CLI version, internal feature."
6. **Critic + connector** begin building synthesis on the assumption that `gh skill` may not exist (Tier P vs Tier E partition; circuit-breaker framing).
7. **Judge primary-source intervention**: runs `gh --version` → 2.90.0; `gh skill --help` → exits 0 with 5 subcommands. Posts override.
8. **Labrat self-correction**: re-probes from direct Bash (not codex-spark), gets 4/4 POSITIVE. Identifies root cause: codex-spark sandbox lacked gh v2.90.0; spark misreported "unknown subcommand"; failed to verify shell environment before submitting.
9. **Scout primary-source revisits**: confirms 6 specific hallucinations from gemini's documentary findings. Gemini fabricated plausible-but-non-existent specifics (`.skill-lock.json`, verify/audit, --immutable, mandatory version, Sigstore).
10. **Distiller synthesis preserves judge override**: scout overclaims stripped from M01; labrat 4/4 POSITIVE folded as M07; F-axis HIGH confidence cross-model.

**Lessons distilled**:
- Gemini under research prompts hallucinates plausible feature surfaces. Treat gemini documentary findings as MED confidence pending primary-source verification.
- Codex-spark sandbox can lack tools present in the parent CC session. Future labrat probes on local-CLI surfaces should verify tool version in spark env first OR run from direct Bash in the parent session.
- Primary-source > peer cross-correction. Per `feedback_critic_hallucination.md`, the judge MUST primary-source verify content-state claims before letting downstream synthesis build on them.
- The correction loop is fast: ~3-4 minutes from initial conflict surface to resolution. The system is robust against single-agent error precisely because cross-pollination forces the conflict to surface, AND primary-source verification is cheap (one Bash invocation).

---

*This document was produced by the system mapping a feature it did not previously know existed. The fact that the mission completed with primary-source-verified mechanics, identified 6 specific gemini hallucinations, and produced a heterogeneous-by-class adoption shape that respects the user's own trendsetter principle is empirical evidence for the §X synthesis claim about cross-pollination + primary-source robustness.*
