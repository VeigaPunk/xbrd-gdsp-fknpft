# Multi-Agent Memory Extension — Design Sketch
**Surfaced by:** mgd-agents-memlift-r1 Round 1/2 (memory-lift experiment)
**Status:** design-doc only, NOT for implementation in this session
**Author:** the-judge synthesis of ccs-scout-anthropic + ccs-scout-quiroga + g-connector-crosslinks + cdx-reviewer-gaps + ccs-distiller
**Cognitive grounding:** Quiroga, *The Forgetting Machine* (alexandria/neurology/books/forgetting-machine.md)
**Anthropic reference:** code.claude.com/docs/en/memory (primary source); claude.com/blog/claude-managed-agents-memory

---

## Why this sketch exists

The memlift-r1 experiment confirmed the Anthropic managed-agents memory pattern runs on our VPS substrate unchanged — one bash write, one bash read, `~/.claude/projects/<project>/memory/MEMORY.md`, bit-exact readback. **That's the single-agent pattern.**

xbrd-gdsp-fknpft runs N parallel teammates. The single-agent pattern doesn't address the coordination problem N-agent systems have. This sketch names the extension points, grounds them in primary sources, and defers implementation to a future planning session with a tighter spec.

---

## Two-threshold architecture

```
                    ┌─────────────────────────────┐
Teammate writes ──► │  T1 staging gate             │ ──► findings table
                    │  (GNW / consciousness gate   │     (episodic, SQLite,
                    │   analog, Quiroga-grounded)  │      mission-scoped)
                    │                              │
                    │  Quiroga line 47: only       │
                    │  GNW-ignited content gets    │
                    │  durable encoding            │
                    └─────────────────────────────┘
                                  │
                                  │ Pareto filter (global axis view)
                                  ▼
                    ┌─────────────────────────────┐
                    │  T2 consolidation gate       │ ──► MEMORY.md + topic files
                    │  (Anthropic-fidelity layer)  │     (semantic, markdown,
                    │  Judge = write authority     │      readable by every
                    │                              │      agent via standard
                    │  Anthropic primary source:   │      file tools)
                    │  MEMORY.md = index of dir    │
                    └─────────────────────────────┘
```

**T1 (missing today):** staging gate for teammate writes. Current substrate has no way for a teammate to propose a finding without direct write to `findings`; Pareto filter runs after-the-fact. Adding T1 means Pareto becomes intake-gate rather than rejection-after-write.

**T2 (partially present today):** the Phase-0 hook reads `findings` and injects into judge context; MEMORY.md hierarchy is judge-maintained. Missing: the index file + per-axis topic files on the Anthropic auto-memory path. memlift-r1 M1 confirmed the substrate works on VPS; extension is content, not substrate.

---

## Open decisions (blocking implementation)

### C4 — T1 substrate

| Option | Pros | Cons |
|--------|------|------|
| **A — SQLite `findings_staging` table** | Consistent substrate (same DB as `findings`). Strong dedup/UNIQUE-gate properties. Joins cheap. | Schema complexity ↑. Less Anthropic-substrate-faithful. |
| **B — `.xbreed/mailbox/judge/proposals/<teammate>_<round>.md` markdown** | Zero schema change. Anthropic-substrate-faithful (filesystem+markdown). Inspectable with standard tools. Extends existing mailbox. | Requires flush-on-read lifecycle discipline. Concurrent-write safety leans on fs atomicity. |

**Resolution path:** a dedicated planning session with an axis probe — run both substrates on a single round with one real mission and measure: (a) dedup behavior under concurrent writes, (b) inspectability, (c) lines-of-code delta, (d) Phase-0 read impact. Do NOT pick by taste alone.

### C5 — scope remediation path

| Option | Pros | Cons |
|--------|------|------|
| **A — `ADD COLUMN scope TEXT DEFAULT ''` to findings** | One DDL. Keeps everything in one table. Schema reflects scope as a first-class field. | Less Anthropic-faithful. Pre-primary-source proposal — superseded. |
| **B — Per-session topic files** (`data/memory/<axis_id>.md`) | Directly matches Anthropic primary-source pattern ("topic files on demand"). No schema change. Agents read via standard file tools. | Requires conventions around who writes per-axis files. Harder to cross-query vs SQL. |

**Lean:** B. Primary-source-faithful; B2 (ADD COLUMN inserted_at) still covers the temporal-sort gap the original scope-column proposal implicitly addressed.

---

## Read-path extensions (Quiroga-grounded)

### Concept-token augmentation (Move E in distiller brief)

```sql
SELECT axis_id, observable, direction
FROM findings
WHERE mission = ?
ORDER BY
  (observable LIKE '%' || ?1 || '%' OR axis_id LIKE '%' || ?1 || '%') DESC,
  round DESC
LIMIT FINDINGS_LIMIT;
```

- Concept tokens sourced from **axis labels of the current round** (specific vocabulary, not free text). Prevents generic-word LIKE flooding.
- No cross-mission traversal (MEMORY.md hierarchy preserved).
- Addresses: high-relevance round-N-25 finding invisible to temporal-only ORDER BY.
- **Source:** forgetting-machine.md line 43 (cue-completion: "Future cue matching any part reactivates the full trace"); line 35 (concept cells strip format, retain semantic identity); line 33 (sparse distributed coding balances capacity vs interference).

### Cue-retrieved annotation (Move F)

Findings surfaced by LIKE-boost (not by Pareto promotion) tagged in judge context with `[cue-retrieved, not Pareto-promoted]`. Dual function:

1. **Write-path integrity.** Re-promotion requires fresh Pareto evaluation. Surfacing ≠ encoding.
2. **Judge cognitive hygiene.** Phase-0 context mixes Pareto-promoted rows (structural truth) with cue-boosted rows (episodic texture). Without the tag, the judge can't tell architecture from associated context.

**Source:** forgetting-machine.md line 47 (GNW ignition prerequisite for durable encoding).

---

## The FINDINGS_LIMIT constant (Move B)

`FINDINGS_LIMIT = 20` is the Quiroga sparse-coding budget, not a perf knob.

Quoting forgetting-machine.md line 33: *"Each concept is represented by a small population of MTL cells (on the order of 1–2% of recorded neurons)... sparse-but-distributed architecture balances storage capacity against interference."*

The 20-row ceiling is the substrate's expression of that sparseness. Stream B blocker report recommends extracting the constant — this sketch is the conceptual justification.

---

## Multi-agent coordination rationale (connector)

Without T1 staging, exposing raw teammate writes to `findings` creates four simultaneous breaks:

1. **Split-brain state.** Teammates use `findings` as a low-latency broadcast bus, bypassing `.xbreed/mailbox/`; the mailbox (peer DMs) and the findings table (structural truth) merge into one overloaded channel.
2. **Pareto frontier gaming.** A teammate optimizing `axis_id=performance` writes a finding that degrades `axis_id=security`. The stopping condition triggers prematurely on false local optima. Pareto requires a global view.
3. **Phase-0 context eviction.** With O(N_teammates × rounds) writes vs O(1) judge writes, the LIMIT budget saturates fast. Session-resume inherits a fragmented world model.
4. **Semantic overwrite via flooding.** Additive-only holds structurally (no DELETE/UPDATE) but breaks logically — contradictory rows for the same `axis_id` create race conditions.

**Source:** g-connector-crosslinks gemini-high xask (memlift-r1 R1 artifact).

---

## NOT in scope of this sketch

- Implementation of T1 staging (needs C4 resolved + dedicated planning session)
- Per-session topic files (M2+ work after MEMORY.md index lands)
- Cross-scope Phase-0 hook (reviewer-self-deferred M5; needs trendsetter token list first)
- Mailbox flush-on-read lifecycle (conditional on C4 resolving toward mailbox substrate)
- Rust-level changes (this sketch is entirely at the bash/SQLite/filesystem layer)

---

## Planning inputs required before Stream C executes

1. **C4 resolution** — judge arbitration on SQLite staging table vs mailbox markdown files. Empirical probe recommended.
2. **C5 resolution** — scope remediation path (column vs topic files). Current lean: topic files.
3. **Axis-label token list** — what vocabulary counts as a "concept token" for the LIKE-boost. Judge supplies per round.
4. **Round cadence + actual row count** — empirical check before setting any FINDINGS_LIMIT override.
5. **Stream B blockers resolved** — B1/B2/B3 from `xbreed-memory-blockers-2026-04-24.md` must land before T1 staging is built on top of the current substrate.

---

## Evidence lineage

- **ccs-scout-anthropic** — primary source acquired (code.claude.com/docs/en/memory); ghost API stricken; MEMORY.md-as-index pattern established.
- **ccs-scout-quiroga** — cognitive framework mapping; line citations for concept cells (35), cue-completion (43), GNW ignition (47), sparse coding (33); active forgetting and consciousness gate identified as ALREADY SATISFIED by current substrate.
- **g-connector-crosslinks** — cross-axis analysis; four second-order breaks named; ghost-API-correction confirms staging-tier-as-markdown is Anthropic-substrate-faithful; two-threshold framing.
- **cdx-reviewer-gaps** — gap ranking; actual anthropic-fidelity gaps identified post-primary-source (no MEMORY.md index, no scope hierarchy, no topic files); earlier WAL-durability framing retracted.
- **cdx-labrat-m1-smoke** — VPS empirical confirmation of Stream B blockers; substrate reachability proven.
- **ccs-distiller** — 11 raw moves → 8 unique; conflict table; Stream A/B/C separation.
- **cdx-executor-m1** — M1 gate PASS on VPS; bit-exact readback of Anthropic pattern on `~/.claude/projects/<project>/memory/` substrate.
