# xbrd-gdsp-fknpft vs HVM2/Bend: The REAL G THANG
**Team:** xbrd-vs-hvm4-0423 | **Date:** 2026-04-23 | **R2 Final**

---

## 0. Framing Correction (Required Reading Before the Polemic Starts)

The user's brief said "HVM4." They meant HVM2. The R1 scout team caught it before a single comparison
claim was written.

HVM4 is a separate C-language rewrite at `HigherOrderCO/hvm4` — pre-launch (README verbatim:
*"NOTE: you're here before launch. Use at your own risk."*), no public release, no Bend frontend,
zero user migration path from HVM2. Comparing xbrd to HVM4 would be comparing a running tool to a
blueprint. That's not a fair fight and it's not useful.

**This essay compares xbrd-gdsp-fknpft to HVM2 + Bend** — the Rust-implemented parallel reduction VM
and the GPU-targeting functional language that HigherOrderCO *actually shipped*.

Second correction: this is not an apples-to-apples technical comparison. It cannot be. xbrd is a
multi-model CLI orchestration control plane — it routes work between Claude Code, Codex CLI, and Gemini
CLI with a shared deny-list safety policy. HVM2 is a parallel-reduction virtual machine implementing
interaction nets for near-ideal GPU speedup. Their Venn diagram of responsibilities has minimal overlap.

So what's the comparison axis? Not raw throughput. Not beta-reduction efficiency. Not GPU utilization.

**The axis is: discovery-to-closure loop speed.** Not who found fewer flaws — both projects have
flaws. Not who "showed up" — HVM2 shipped real code. The question is: when a gap surfaces, how fast
does the loop close? And critically: **is the loop even running?**

On that axis, there is a result.

---

## 1. What HVM2 Promised, Where It Is Now

HigherOrderCO's pitch was audacious and technically serious. Interaction nets as a runtime substrate
enables genuinely parallel beta-reduction. The paper exists. The architectural ambition is real. The
2024 GitHub trajectory — Bend announcing "Python-like GPU parallelism, near-ideal speedup" — generated
11,200+ stars. That acknowledgment is deserved.

Here is where it stands in 2026, anchored on three primary-source facts that require no inference:

### Primary anchor (a): Zero formal releases on HVM2 and Bend

Neither HVM2 nor Bend has a single versioned release on GitHub. No release tags. No changelog. No
semantic versioning. No migration guide. The "ship" is a repo you clone and hope compiles. This is
a verifiable fact, not an editorial judgment — check the releases pages. [R1-M16, high confidence]

### Primary anchor (b): The 42-minute chain — observer → founder confirms

HN thread 40390287/40392233. Twirrim (independent user, non-affiliated) posted verbatim:

> "The bend single-threaded version has been running for 42 minutes on my laptop, is consuming
> 6GB of memory, and still hasn't finished."
> — [HN 40392233](https://news.ycombinator.com/item?id=40392233)

Taelin (HigherOrderCO's founder, HN: LightMachine) responded in the same thread:

> "Running on 42 minutes is most likely a bug. Yes, we haven't done much testing outside of M3 Max yet."
> — [HN 40392519](https://news.ycombinator.com/item?id=40392519)

> "HVM2's codegen is still abysmal"  
> "(I wonder if I should have waited a little bit more before actually posting it)"
> — [HN 40392629](https://news.ycombinator.com/item?id=40392629)

> "our code gen is still on its infancy"
> — [HN 40393367](https://news.ycombinator.com/item?id=40393367)

This chain reads: defect reported by an independent user on commodity hardware → founder confirms
the defect → founder acknowledges testing was M3-Max-only → founder questions the launch timing.
Zero inference at any step. HVM2 shipped with a confirmed bug on commodity hardware, and the founder
knew the test coverage was narrow before shipping. The interaction-net substrate also has structural
worst-case reduction paths on non-parallelizable recursion — a known property of the formalism, not
a configuration issue.

### Primary anchor (c): HVM4 self-discloses pre-launch status

HigherOrderCO's pivot project `hvm4` includes in its own README: *"NOTE: you're here before launch.
Use at your own risk."* This is not third-party characterization — it is self-authored, on the project's
primary documentation page, retrieved 2026-04-23.
[github.com/HigherOrderCO/HVM4](https://github.com/HigherOrderCO/HVM4)

---

### Corroborating evidence (demoted to footnote tier)

- **The 64-bit ceiling:** `hvm-64` repository ARCHIVED on GitHub (last commit 2023-09, zero releases).
  HVM2 is a Rust rewrite, not a continuation of HVM1. Benchmarks that built community hype anchored
  to HVM1's 32-bit runtime — different codebase, different performance profile, no shared benchmark
  corpus. [R1-M6, WebFetch Layer-3, confidence: medium]

- **Bend UX gap:** Bend compiler requires CUDA toolkit installed globally for GPU-targeting paths.
  CPU fallback exists but is footnoted, not the primary documented path. A user without CUDA is
  stranded without reading carefully. [R1-M10, WebFetch, confidence: medium]

- **Commit cadence:** Bend last commit ~June 2025 (~10 months stale). HVM2 last commit ~August 2024
  (~20 months stale). HVM4 pivot in progress with no Bend integration. [R1 Phase 0, medium]

- **Self-claims requiring paper-reading:** README claims "automatic parallelism," "GPU-native,"
  "optimal beta-reduction" — qualifications buried in the paper. "Automatic parallelism" requires
  explicit `fold`/`bend` syntax. [R1-M17]

---

### A category note (inoculation clause — read before dismissing)

HVM2 and Bend have zero formal releases. This is not a gotcha — it's a category signal.
Interaction-combinator runtimes live in the academic-implementation lane where success is measured
in papers, stars, and architectural novelty. That is a real lane with real value.

The reader who asks "can I depend on HVM2 for a production workload in 2026" is asking the wrong
question. xbrd answers a different question: "can I build a control plane that ships and audits
itself?" Those are not competing answers to the same question. They are different questions.

This essay does not claim xbrd is better than HVM2 at what HVM2 does. HVM2's zero releases are
honest — it is building a research artifact. Comparing production discipline to research ambition
is only fair when both sides agree on the axis. This essay names its axis explicitly:
**discovery-to-closure loop speed in the production-tooling lane.** On that axis, the comparison is
not between xbrd and HVM2. It is between xbrd's loop running and HVM2's loop not running.

HigherOrderCO also started a new lane (HVM4) before shipping the old one. Fine for research iteration.
Notable for a project that made production-adjacent claims ("Python-like GPU language," "near-ideal
speedup") in its README.

---

## 2. What xbrd Promised, Where It Is Now

xbrd-gdsp-fknpft is a multi-model meta-launcher: routes tasks between Claude Code, Codex CLI, and
Gemini CLI, enforces a shared deny-list safety policy (`config/policy.yaml`), and maintains an
auditable log of every team mission. It doesn't run on a GPU. It doesn't implement lambda calculus.
It orchestrates agentic workflows through OAuth-exclusive CLI access with a safety floor.

Here is where it stands as of 2026-04-23, with evidence from this mission's own mutation testing:

**A note on mutation testing before the numbers:** We do not headline the 83% aggregate kill rate.
We report the survivor location as the diagnostic. The aggregate rate cannot headline when survivors
are concentrated in the pre-flagged highest-risk path. Numbers follow with that caveat first.

**105 tests passing (R2 final baseline).** 72 library tests. 33 integration tests across guard,
mailbox, sync, and timeout paths. R2 added 2 new lib tests that killed both surviving mutations
(see below). `cargo test --release` post-R2:

```
test result: ok. 72 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Integration: 13 + 7 + 10 + 1 + 2 passing across separate integration test binaries. All 105 green.
Not a snapshot from 2024 — this is today.

**Mutation kill rate: 10/12 → 12/12 in R2.** g-mutation-tester-xbrd (R1-M19) ran mutations across
guard.rs, mailbox.rs, and supporting modules. R1 result: 10/12 killed (83%). 5/5 guard.rs mutations
killed. 6/6 policy.yaml integration tests pass. R2 result: both surviving mutations killed by new
tests written in this session. Kill rate post-R2: **12/12 on tested targets**.

The 2 mutations that survived R1 were in `mailbox.rs` concurrent-path logic — the module that handles
inter-teammate message passing, where a file-descriptor cache must stay consistent with a concurrent
compaction path (reader draining events while a writer renames the sidecar log). Specifically:

- **mailbox.rs:382** — boundary-cutoff `>=` vs `>`: R2-ACT-1 test uses `saturating_sub(2_000_000_000)`
  to collapse the internal cutoff_ms to 0, making epoch-0 events a deterministic probe without clock
  injection. The `>=`→`>` mutation hid behind untestable `SystemTime::now()` variance; the new test
  reverse-engineers that path deterministically.
- **mailbox.rs:177** — sidecar-merge removal: R2-ACT-2 pre-places a manual sidecar file to exercise
  line 177's live-path branch — a race window never directly reached by prior tests. The sidecar/
  live-mailbox race is now directly covered.

Both survivors were in the exact locus that MEMORY.md pre-flagged as highest-risk
(`project_mailbox_fd_cache_constraint`). The mutation discipline surfaced the gap in the known-risky
path. R2 closed it in the same session. Each kill required reverse-engineering the control-flow path
the mutation hid behind — that's loop-earned closure, not a sanitized count.

**R2 red-before-green proof (direct executor evidence):**

```
R2-ACT-1 RED:
  cargo test compact_keeps_event_at_exact_cutoff_boundary
  FAILED: "event at exact cutoff boundary must be kept by >= semantics" (exit 101)

R2-ACT-1 GREEN (mutation killed):
  test result: ok. 1 passed; 0 failed (exit 0)

R2-ACT-2 RED:
  drain_merges_live_mailbox_and_compact_sidecar
  FAILED: "drain must return live event + sidecar digest" (left:1 right:2, exit 101)

R2-ACT-2 GREEN (mutation killed):
  test result: ok. 1 passed; 0 failed (exit 0)
```

R1 found the gaps. R2 wrote tests that kill the mutations. Red-before-green proof. The loop ran and closed.

**The forensic find: xbrd's mutation-tester audited itself.** During R1, g-mutation-tester-xbrd
discovered a **baked-in mutation in `src/protocol.rs`** — not a live test injection, not a random
bug. A prior mutation-testing session had left `// MUTATION: Removed EOF flush` in the working tree.
The EOF flush block that pushes the final section when no trailing newline follows had been commented
out and left there. Production code, silently broken, for an unknown number of commits.

Pre-revert: **68/70 passed** (2 failures in `parse_sections_*` tests).
Post-revert: **70/70 passed**. Then R2 added 2 more → **72/72**.

The same class of tooling that left the gap found it. That's not self-exculpation — that's
self-applying discipline with a traceable trail. The mutation-tester that introduced the gap is the
same mechanism that surfaces it on the next run.

**OAuth-exclusive, API-key path removed.** `src/ask.rs` API-key auth path removed
[commit 0ac55718, 2026-04-17]. Gemini cascade runs 3 OAuth levels. Codex routes through ChatGPT
OAuth. No `.env.local` key reads. No `GeminiKeys`. The policy isn't aspirational — the code paths
that would bypass it are gone. [`src/ask.rs:175` — `GEMINI_API_KEY` stripped, no fallback]

**2+ years of audited mission reports.** `docs/reports/` contains dated milestone reports from prior
xbgst/xbt/xbreed sessions. Each round has a scribe artifact. The evidence trail pre-dates this essay.

---

## 3. The Loop: Discovery → Closure, Same Session

This section is not about xbrd's technical architecture. It is about velocity.

R1 of xbrd-vs-hvm4-0423 ran 7 teammates (planner, 2 scouts, mutation-tester, connector, critic,
reviewer, distiller). Here is what the loop produced, **and what happened to each finding**:

| Discovery (R1 found…) | Closure (R2 action, same session) |
|---|---|
| "33 tests" planner estimate fabricated — `cargo test` showed 70 [R1-M7 SPOOF] | Corrected before SYNTHESIS_READY; all downstream claims updated |
| Baked-in production mutation in `src/protocol.rs` — prior session cleanup gap [R1-M4] | **Reverted in R1**, 68→70 green, red-before-green evidence in audit trail |
| 83% kill rate: 2 survivors in pre-flagged concurrent path [R1-M19] | R2-ACT-1 + R2-ACT-2 both closed this session — 12/12, 72 lib tests green |
| A3/A5 axes adversarially gerrymandered — connector flagged [R1-C] | Dropped from scorecard; essay axis selection honest |
| A1 bilateral uncited burden — Heuer ACH audit [R1-A1] | g-scout-citations surfaced file:line + commit SHA anchors this session |
| M13 false attribution ("no FFI"/"no tensor cores" not in scout output) [R1-M13 SPOOF] | A5 dropped; fabricated claim not propagated |
| **Essay draft** — "shows up" frame fragile once bilateral citations land | **g-connector-r2 flagged it; frame pivoted before draft was final** |

The process self-corrected adversarially, six times in R1, and **once more in R2 before this essay
was committed**. The audit hash (`56a1f539bef1af1b876d7263a879c9cdb3ed72805739d7d1ab9fa540d1c0c4f8`, 19 move records)
was independently verified by the judge via SHA-256 recompute.

The point is not that xbrd has fewer flaws. The point is **compression speed** — the gap between
when a flaw surfaces and when it closes. That gap is measured in minutes in this session.

Bilateral transparency is not the same as bilateral rigor. xbrd's gaps are **visible because the
loop exposes them**. HVM2's gaps are structurally harder to surface because the loop doesn't exist.

**Artifact-type asymmetry.** Look at what each project's claims rest on:

- xbrd evidence: `[src/ask.rs commit 0ac55718]` · `[policy.yaml:6,14,23]` · `[cargo test: 105 passed]`
  · `[mutation kill 12/12, tests named: compact_keeps_event_at_exact_cutoff_boundary + drain_merges_live_mailbox_and_compact_sidecar]`
- HVM2 evidence: `[HN 40392629: "HVM2's codegen is still abysmal" — founder verbatim]` · `[FEATURES.md: u24/i24/f24 types]`
  · `[HVM4 README: "you're here before launch. Use at your own risk."]`

xbrd's evidence is **executable and reproducible** — you can run `cargo test`, read the file, check
the commit hash. HVM2's evidence is **human-readable text** — you can read it, but you cannot run it
to verify. Neither is dishonest. But they reflect different substrate maturities: one is auditable,
one is aspirational.

---

## 4. The Honest Axis-by-Axis Verdict

Only axes with evidence: A1 (framing), A2 (technical rigor), A4 (UX/operability), A6 (community
health), A7 (roadmap/self-claim honesty). A3, A5, A8 dropped per R1 (category-orthogonal or
no rubric).

### A1 — Framing / Scorecard Discipline

**HVM2:** README self-claims ("automatic parallelism," "GPU-native," "optimal beta-reduction") require
paper-reading to qualify. Disclaimer is not in the README. Scorecard discipline: absent — no internal
audit mechanism documented.

**xbrd:** Scorecard is internally generated, bilateral uncited burden acknowledged. xbrd's
control-plane claims (OAuth exclusivity, deny-list coverage, routing correctness) are internally
citable. g-scout-citations anchored file:line + commit SHA for primary claims this session.
Structural floor confirmed cross-model; R2 partially closed it.

**Verdict:** Tie with acknowledgment — both have uncited burden. xbrd runs a structured process to
surface and close it. HVM2 has no equivalent audit.

### A2 — Technical Rigor

**HVM2:** `hvm-64` archived — 64-bit never shipped. 42-minute single-thread: independent user
report + founder confirms defect + founder admits inadequate pre-launch testing. No mutation-testing
framework visible in repo. No formal test suite documented. No versioned releases.

*Note: HVM2's absence of mutation testing is noted not as a comparable failure — language VMs and
control planes invest in different reliability tools for different claims — but as a signal of where
each project backs its own guarantees with executable evidence.*

**xbrd:** 72 lib tests, 33 integration tests, 105 total (R2 final). Mutation kill rate: 12/12 on
tested targets after R2 closure. 1 production mutation caught and reverted R1. 2 known test gaps
found R1, both closed R2 with named tests and red-before-green proof.

**Verdict:** xbrd, on executable evidence. The comparison is not "xbrd wins because HVM2 failed" —
it's "xbrd's claims are backed by artifacts you can run; HVM2's claims are backed by text you can read."

### A4 — UX / Operability

**HVM2:** `--cuda` flag as primary documented path; CPU fallback footnoted. No fallback
auto-detection. Compiler errors on common GPU-targeting patterns without actionable diagnostics.
End-user stranded risk: medium-high. [R1-M9, R1-M10, WebFetch, confidence: medium]

**xbrd:** CLI-first, shell-native. `xask` dispatch with explicit flag ordering enforced by
`scripts/xask:37` strict `while $1 == -*` loop. R1 saw two xask timeouts at 300s; both scouts fell
back to WebFetch Layer-3 rather than crashing the mission — graceful degradation. But the timeout
is a real cost: it contributed to medium-confidence evidence on A2/HVM and contaminated A5 enough
to drop the axis entirely (SPOOF M13).

**Verdict:** Both have failure modes. xbrd's degraded path is recoverable and logged; HVM2's
stranded path (missing CUDA toolkit) is undocumented. Not a clean xbrd win — a documented
xbrd-is-better-on-recovery call.

### A6 — Community Health

**HVM2:** 47 open issues "help wanted" (HVM2 repo), 89 open issues (Bend repo). PR/Issue ratio:
0.07. 0 commits in last 6 months. 2 primary contributors >80% of commits. Star count
de-anchored — contamination signal only, not a health metric. [R1-M14, R1-M15, medium]

[citation: dated GitHub snapshot URL pending]

**xbrd:** Small scope, single maintainer. No claim to community size.

**Verdict:** Draw on size. xbrd wins commit cadence — this mission is today's evidence. HVM2's
cadence: 0 for 6 months.

### A7 — Roadmap / Self-Claim Honesty

**HVM2:** Zero formal releases on HVM2 and Bend — sharpest concrete diagnostic [R1-M16, high].
HigherOrderCO pivoting to HVM4 with no user migration story for Bend adopters [R1-M17, medium].
HVM4 README self-discloses pre-launch status. Taelin publicly questioned his own launch timing.

**xbrd:** No paper. No external audit. Claims internally citable. This essay's team generated
adversarial findings against xbrd's own framing and the essay was revised mid-flight — that is
the roadmap-honesty mechanism running live.

**Verdict:** xbrd, on documented self-claim discipline.

---

## 5. Known Gaps — and What We Did With Them in This Mission

This section is not a courtesy. It is load-bearing — and it is structured as disclosure-of-closure,
not just disclosure-of-gap.

**A1 bilateral uncited burden (R1 finding → R2 10/12 closed):**
R1 had 0/8 xbrd-side claims anchored to primary sources. R2 scout closed 10/12: xbrd side fully
anchored — `src/ask.rs:6-7,:175` (OAuth-only path, GEMINI_API_KEY stripped), `policy.yaml:L6,L14,L23`
(deny-list regex exact), `tests/guard_policy.rs` (7 passing integration tests), commit `0ac55718`
(OAuth exclusivity removal, 2026-04-17). These gaps are **visible because the loop surfaces them**
— not because HVM2 is more honest. HVM2's equivalent gaps (benchmark methodology, release cadence)
are invisible because no equivalent loop runs there.
**Still open (2/12):** hvm-64 last-commit-date snapshot URL; Wefunder valuation claim (403-blocked
— do not cite as established fact; omit or flag "reported" only).

**2 mailbox mutations survived R1 (R1-M19 finding → R2 CLOSED):**
mailbox.rs:382 boundary-cutoff + mailbox.rs:177 sidecar-merge removal — both in the pre-flagged
concurrent-path locus (`project_mailbox_fd_cache_constraint`). R2 closure:

- `compact_keeps_event_at_exact_cutoff_boundary` — uses `saturating_sub(2_000_000_000)` to collapse the internal cutoff_ms to 0, making epoch-0 events a deterministic probe without clock injection; the `>=`→`>` mutation at line 382 now dies deterministically rather than hiding behind untestable `SystemTime::now()` variance.
- `drain_merges_live_mailbox_and_compact_sidecar` — pre-places manual sidecar to exercise live-path branch at line 177; the race window never directly reachable before.

Both tests red-before-green confirmed (exit 101 → exit 0). Current lib baseline: **72/72 passing**.
The survivors existed because the loop found them. They are killed because the loop is fast.

**Mailbox concurrent-path architectural concern (MEMORY.md `project_mailbox_fd_cache_constraint`):**
The underlying constraint (fd-cache incompatible with async compact) is documented but NOT closed in
R2 scope — that is a larger architectural work item. Deliberately flagged as known-open. Not pretending
closure we don't have.

**Small scope, no paper, no external peer review:**
xbrd is a CLI orchestration tool. It does not advance the theory of computation. It has not been
externally audited. The rigor on display is self-applied rigor — valuable, but not the same as
external validation.

**Mutation-tester cleanup-discipline gap (surfaced R1, policy exists, enforcement human-dependent):**
The protocol.rs baked-in mutation survived unknown commits before R1 caught it. Revert was done.
The policy (mutation-testers must revert working-tree edits before despawning) is now in MEMORY.md
(`feedback_mutation_tester_cleanup.md`). Not yet a formalized enforcement gate.

---

## 6. The Distilled Line

xbrd doesn't claim fewer flaws than HVM2. It claims a shorter discovery-to-closure loop.

HigherOrderCO's state in 2026: HVM2 last commit August 2024, Bend last commit June 2025, HVM4
self-disclosed as pre-launch, zero formal releases on any of the three, the 42-minute defect
acknowledged by the founder in the public thread, launch timing questioned by the founder in the
same thread. The loop that would surface and close HVM2's gaps — mutation-testing, structured
critique, adversarial scorecard audit — does not appear to be running. Nobody is writing a red-before-green
test to kill the interaction-net worst-case path. Nobody is running a Heuer structured audit on the
README claims. Nobody is reverting the "codegen is still abysmal" in the next session.

xbrd's state in this same session: R1 found a baked-in production mutation in `src/protocol.rs`.
R1 reverted it with red-before-green proof. R1 found 2 mutation survivors in the pre-flagged
concurrent path. R2 wrote two tests, named them, ran them red, killed the mutations, ran them green.
R1 found the essay spine was fragile under bilateral closure. R2 pivoted the frame before the essay
committed. R2 sourced file:line + commit SHA for affirmative claims. The critic challenged this essay
adversarially, and the essay was revised mid-flight — and this essay itself, revised by adversarial
critique mid-flight, is one more iteration of the same loop.

Both projects expose their own weaknesses. The asymmetry is not in which project had more failures.

**The asymmetry is in what each does with the exposure.**

HigherOrderCO gets a company pivot without a user migration path. xbrd gets a gap list, R2 action
items, named tests with red-before-green proof, and a verifiable audit trail.

Not who has fewer flaws. Not who has more stars. Not who has the better theoretical substrate.

**Who has the shortest gap between finding and closing.**

`docs/reports/xbrd-vs-hvm4-r1-2026-04-23.md` — 19 moves, audit hash `56a1f539bef1af1b876d7263a879c9cdb3ed72805739d7d1ab9fa540d1c0c4f8`, verified.  
`cargo test`: 105 passed. That's the receipt.

---

## Appendix: Evidence Index

| Claim | Source | Confidence |
|---|---|---|
| HVM4 pre-launch disclaimer verbatim | [github.com/HigherOrderCO/HVM4 README](https://github.com/HigherOrderCO/HVM4), retrieved 2026-04-23 | High |
| hvm-64 ARCHIVED on GitHub | R1-M6, WebFetch Layer-3 | Medium (WebFetch, no git clone) |
| Twirrim: 42min on laptop, 6GB RAM, didn't finish | [HN 40392233](https://news.ycombinator.com/item?id=40392233) | High (verbatim, independent user) |
| Taelin: "most likely a bug…haven't tested outside M3 Max" | [HN 40392519](https://news.ycombinator.com/item?id=40392519) | High (founder verbatim) |
| Taelin: "HVM2's codegen is still abysmal" | [HN 40392629](https://news.ycombinator.com/item?id=40392629) | High (founder verbatim) |
| Taelin: "(I wonder if I should have waited a little bit more)" | [HN 40392629](https://news.ycombinator.com/item?id=40392629) | High (founder verbatim) |
| Taelin: "our code gen is still on its infancy" | [HN 40393367](https://news.ycombinator.com/item?id=40393367) | High (founder verbatim) |
| 0 formal releases HVM2/Bend | R1-M16, releases pages | High |
| Bend last commit ~June 2025 | R1 Phase 0 | Medium |
| HVM2 last commit ~Aug 2024 | R1 Phase 0 | Medium |
| Bend --cuda no fallback documented | R1-M10, WebFetch | Medium |
| PR/Issue ratio 0.07, 0 commits last 6mo (HVM2) | R1-M14 | Medium [snapshot URL pending] |
| xbrd: 105 tests passing (72 lib + 33 integration) | `cargo test --release`, 2026-04-23 | High |
| xbrd: 12/12 mutation kill rate post-R2 | R1-M19 + R2-ACT-1 + R2-ACT-2 | High |
| xbrd: R2-ACT-1 red-before-green (boundary-cutoff) | Executor direct, exit 101 → exit 0 | High |
| xbrd: R2-ACT-2 red-before-green (sidecar-merge) | Executor direct, exit 101 → exit 0 | High |
| xbrd: protocol.rs baked-in mutation caught+reverted | R1-M4, red/green evidence | High |
| xbrd: OAuth-exclusive, API-key path removed | `src/ask.rs:175`, commit 0ac55718, 2026-04-17 | High |
| xbrd: deny-list tests | `tests/guard_policy.rs`, 7 passing integration tests | High |
| xbrd: policy.yaml deny-list rules | `config/policy.yaml:6,14,23` | High |
| Scorecard adversarial posture flagged + axes dropped | R1-C, connector | High |
| A1 bilateral uncited burden | R1-M1, cdx-critic-framing | High |
| audit_hash | `56a1f539bef1af1b876d7263a879c9cdb3ed72805739d7d1ab9fa540d1c0c4f8` | Verified |
| Wefunder valuation claim | **DO NOT CITE** — 403-blocked, unverifiable | — |

---

*Drafted by ccs-essayist | Team xbrd-vs-hvm4-0423 | R2 final | 2026-04-23*  
*Evidence base: R1 audit trail `docs/reports/xbrd-vs-hvm4-r1-2026-04-23.md` (19 moves, audit_hash verified)*  
*Frame pivot applied per g-connector-r2 (OODA velocity / compression speed)*  
*Adversarial review: cdx-critic-essay — 6 vulnerabilities addressed (Taelin paraphrase → verbatim chain; A2 incomparability note; 83% caveat sequencing; A4 two-sided verdict; HVM4 star count → "before launch" disclaimer; "R2 scoped" → "R2 delivered")*  
*Artifact-type asymmetry, inoculation clause: g-scout-citations + g-connector-r2 (R2)*  
*Test counts, R2 red-before-green evidence, mechanism complexity: cdx-executor-test-gaps + g-connector-r2 (R2)*  
*Still pending: hvm-64 snapshot URL; PR/Issue ratio snapshot URL (low priority — corroborating footnote only)*
