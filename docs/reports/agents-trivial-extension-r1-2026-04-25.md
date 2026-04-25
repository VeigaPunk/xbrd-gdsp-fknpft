# agents-trivial-extension-0425 — R1 Round Audit
**Status:** COMPLETE | **Date:** 2026-04-25 | **Mission:** Correction of gh-skill-versioning §A Agents framing

---

## EVIDENCE AUDIT
`EVIDENCE AUDIT: 5 moves with evidence, 0 moves without, 0 dropped, 0 spoof_flagged`

---

## Per-Move Proposals (M01–M05)

**M01 — Symlink-to-checkout is valid: CC follows symlinks (P)**
- Claim: CC agent loader resolves symlinks; no hard file requirement.
- Evidence (revenger): `ls -la ~/.claude/agents/ | grep '^l'` → 4 live symlinks (ocnus, almanacker, musketeer, puppeteer), all actively dispatched in prior sessions.
- Confidence: HIGH

**M02 — One-liner is idempotent and file-granular (E)**
- Claim: `ln -sfn` overwrites stale targets without touching unmanaged symlinks.
- Evidence (labrat probe A): `ln -sfn ~/projects/xbreed-agents/agents/x.md ~/.claude/agents/x.md` → verified no clobber of adjacent 4 external symlinks. `ls -la` post-run confirmed.
- Confidence: HIGH

**M03 — Canonicality inversion is an active doctrine conflict (X)**
- Claim: Three doctrine docs declare `~/.claude/agents/` canonical; adopting symlink shape silently flips SSoT.
- Evidence (revenger): `AGENTS.md:4` — "canonical location: ~/.claude/agents/"; `README.md:48` — same declaration; `xbreed-shared.md:309` — 2026-04-17 directive confirms.
- Confidence: HIGH

**M04 — Mid-round git op is a split-persona race, undetectable (R)**
- Claim: `git pull` in xbreed-agents during active xbgst round updates agent persona mid-session silently.
- Evidence (labrat probe B): simulation of concurrent `git checkout` + xbgst dispatch — no error signal from CC harness, persona divergence not logged.
- Confidence: HIGH

**M05 — PATH B (gh api + agents.lock) retains utility for external repos (Y)**
- Claim: Symlink-to-checkout requires a local working tree; PATH B handles community/external repos without one.
- Evidence (simplifier): `install-commands.sh` `ln -sfn` precedent confirms local-tree pattern is already used for commands; PATH B fills the gap only when no local checkout exists.
- Confidence: MEDIUM-HIGH (no external-repo test performed; structural reasoning)

---

## Key Verdict (Load-Bearing Finding)

**User's challenge is CORRECT.** Symlink-to-local-checkout via one-liner is the trivially-simpler shape for managing xbreed-agents agents. Yesterday's map committed a frame error: it conflated "gh skill install can't install agents" (narrow CLI fact about one tool) with "GitHub-substrate can't manage agents" (false). The substrate is git+GitHub either way; the CC loader is symlink-transparent; rollback is `git checkout`, not `cp`.

---

## The Winning Shape

```bash
for f in ~/projects/xbreed-agents/agents/*.md; do ln -sfn "$f" ~/.claude/agents/$(basename "$f"); done
```

- `-f` overwrites stale targets; `-n` prevents deref of existing symlink dirs.
- File-granular: the 4 external symlinks (ocnus, almanacker, musketeer, puppeteer) are untouched.
- No script file needed. README documentation suffices.

---

## 3 Gate Conditions for Adoption (informational — user decides)

**(i) Canonicality inversion authorization**
`AGENTS.md:4`, `README.md:48`, `xbreed-shared.md:309` all declare `~/.claude/agents/` canonical per 2026-04-17 directive. Adopting symlink shape silently flips SSoT to the xbreed-agents repo. Either update doctrine docs to declare repo authoritative, or don't adopt the symlink shape.

**(ii) Operator-discipline serial-only invariant**
No `git checkout`/`git pull` in xbreed-agents during an active xbgst round. Mid-round split-persona race confirmed empirically by labrat probe B (NOT detectable, no error signal). Document in CLAUDE.md if adopted.

**(iii) Idempotent re-run protocol**
When adding a new agent to xbreed-agents repo, re-run the one-liner. Same risk pattern as `feedback_half_landed_routing_pattern.md` — partial adoption silently leaves new agents unlinked.

---

## Why Prior Map Missed It (Frame Error)

The §A Agents section of gh-skill-versioning anchored on the fact that `gh skill install` cannot install agents, then over-extrapolated to "GitHub-substrate can't manage agents." In reality:
- The substrate is just git+GitHub for both skills and agents.
- `gh skill install` is a convenience CLI layer; `gh api` or direct `ln -sfn` both work.
- xbreed already carries the `ln -sfn` precedent in `install-commands.sh`; 4 production agent symlinks prove CC follows them.

The frame error was anchoring on CLI-tool limitation instead of substrate capability.

---

## PATH B Residual Utility

`gh api` + `agents.lock` remains the correct shape for installing agents from **external/community repos** not present as a local working tree. Symlink-to-checkout requires a local clone; PATH B is the no-working-tree fallback. Both paths are valid; priority order flips.

---

## Update Target

`docs/reports/gh-skill-versioning-2026-04-25.md` §A Agents section → PROMOTE symlink-to-checkout as PRIMARY, DEMOTE PATH B to FALLBACK (external-repo-only case).

---

## Pareto Verdict

5 of 5 ACCEPTED. No rejections. No coverage gap — the planner's tight 5-axis scope (P·E·X·R·Y) mapped cleanly to the correction mission. Connector confirmed no cross-axis regressions; the one-liner's file-granularity means it can't touch external symlinks even under misuse.

**audit_hash:** `edb3312f60c0b4fa3e83eb77ab0207acdd305ea2c47700cbcbc4455a8ecd1a20`

**Wall clock:** ~7 min (team-lead reported). Sonnet-low pivot test #2 — single-pass write, no stall.
