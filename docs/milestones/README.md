# xbreed milestone reports

Human-legibility-optimized reports for Obsidian compiling. Each report
documents one milestone: a major change, shipped feature, load-bearing
finding, or architectural decision. See `TEMPLATE.md` for the format.

## Chronological (newest first)

### 2026-04-11 — Day 1

- [[2026-04-11-customtools-routing-finding]] — `gemini-3.1-pro-preview-customtools` is a runtime routing target, NOT a pinnable model id; xbreed's OAuth cascade was already reaching it silently; supersedes the Vertex AI guess
- [[2026-04-11-v0.4.0-posture-retcon-walk]] — v0.4.0 posture frontmatter was category error; ships as the-guard reactive-audit role instead; customtools 404 live-verified via OAuth
- [[2026-04-11-v0.3.6-mailbox-compaction]] — heuristic mailbox compactor (keep-types + age cutoff); trait-based upgrade deferred
- [[2026-04-11-kv-compaction-walk]] — Attention Matching → xbreed mapping walk; transferable insight is the interface, not the math
- [[2026-04-11-v0.3.5-gemini-oauth-cascade]] — OAuth-primary cascade with multi-profile support via HOME override
- [[2026-04-11-gemini-nesting-walk]] — gemini-side parallel workers walk; 3-variant architecture + LIVE hallucination finding
- [[2026-04-11-v0.3.4-oauth-fallback]] — gemini OAuth / credential-chain fallback + claude/codex auth hints
- [[2026-04-11-next-frontier-walk]] — v0.4 direction walk; codex confirmed live; posture frontmatter converges as headline
- [[2026-04-11-v0.3.3-mailbox-side-channel]] — file-backed fast-path comms for teammate signals
- [[2026-04-11-10min-stall-discovery]] — Claude Code deep-idle polling finding (the trigger for v0.3.3)
- [[2026-04-11-v0.3.2-gemini-fallback-and-team-init]] — parser hardening + `xbreed team init --with-beads`
- [[2026-04-11-wildcard-tooling-experiment]] — Umwelt compliance smoke test on agent templates
- [[2026-04-11-the-timoner-labrat-pattern]] — godspeed_labrat role, cheap one-shot probes
- [[2026-04-11-v0.3.1-gemini-pin-wildcard-timoner]] — first post-audit ship (12 files, +111/−801)
- [[2026-04-11-v0.3-audit-team-walk]] — the initial team walk that set the shape of everything after

## By subsystem

**gemini comms** — [[2026-04-11-v0.3.1-gemini-pin-wildcard-timoner]] → [[2026-04-11-v0.3.2-gemini-fallback-and-team-init]] → live-verified via `timoner-gemini-live`

**beads integration** — [[2026-04-11-v0.3.2-gemini-fallback-and-team-init]] (optional subcommand, V1 concurrency via `dolt.auto-commit=on`); v0.4 follow-ups deferred (`xbreed team status`, `xbreed team sync`)

**agent templates** — [[2026-04-11-v0.3.1-gemini-pin-wildcard-timoner]] → [[2026-04-11-wildcard-tooling-experiment]] → tightened prose discipline via two fix passes

**inter-agent comms** — [[2026-04-11-10min-stall-discovery]] → [[2026-04-11-v0.3.3-mailbox-side-channel]] (file-backed ndjson + keepalive pattern + `UserPromptSubmit` hook drain)

**agent roles** — [[2026-04-11-the-timoner-labrat-pattern]] (emergent, now in production use across every walk)

## Writing a new report

See `TEMPLATE.md`. Principles:

- **TL;DR under 3 sentences.** If it doesn't fit, the report is two reports.
- **Evidence must be falsifiable.** Commit SHAs, test counts, file paths, exact exit codes. No "feature works" without proof.
- **Why > what.** The *what* is in the diff. The *why* is why this report exists at all.
- **Wikilink aggressively.** Obsidian's graph view rewards dense linking; name other milestones by their exact filename minus `.md`.
- **One idea per file.** If a report wants to split, split it.
- **Front-matter tags drive discoverability.** Use lowercase kebab-case, include version + subsystem + kind.
