# Test Brief — Recent xbreed Features (R1-honoring frame)

**Spec:** prove xbreed's recent features hold under load by writing tests whose passing is *evidence*, not narrative.
**Frame:** xbreed's design commitments interpreted through **umwelt** (von Uexküll), **metis** (Scott), and **anti-scientific-forestry** (Scott) lenses, with R1's structural-uniqueness honesty preserved. The lenses describe — they do **not** prove. Superiority is not asserted; structural uniqueness is.
**Author:** wwkd posture, 2026-04-16. Inputs: gemini 6-axis plan (b3b0gxbtn.output), advisor framing (Layer 0), data-walk-verified ground truth.
**Handoff:** next session executes via `/xbgst | godspeed`.

---

## Phase 0 — Data Walk (done before any milestone was written)

Inspected before drafting:

| Claim | Source | Verified state |
|---|---|---|
| `src/protocol.rs` exists with `include_str!` SSoT binding | gemini | ✅ `src/protocol.rs:4` `pub const PROTOCOL: &str = include_str!("../commands/references/xbreed-shared.md");` |
| REQUIRED_SECTIONS sentinels present | gemini | ✅ `src/protocol.rs:21+` 11 sections (gate, dispatch, blinding, spawn, despawn, …) |
| W3 ghost-leak fix: 3-arm reap | self | ✅ `src/ask.rs:395-420` Ok arm `child.wait()`, Timeout arm `child.kill()+child.wait()`, Disconnected arm `child.kill()+child.wait()` |
| `THINKING_BUDGET` mapping in `scripts/xask` | self | ✅ `scripts/xask:83-87` low=512 / medium=4096 / high=8192 / xhigh=16384 |
| gemini template substitution | self | ✅ `templates/dispatch/gemini.md:8` `# ThinkingBudget: {{THINKING_BUDGET}}` |
| Yolo flags on codex builder | self | ✅ `src/ask.rs:100,106-107,110` `--skip-git-repo-check`, `--sandbox danger-full-access`, `approval_policy="never"` |
| Yolo flag on gemini builder | self | ✅ `src/ask.rs:285` `--approval-mode yolo` (preserved) |
| `verify-docs.sh` lint exists | self | ✅ `scripts/verify-docs.sh` extracts SSoT canonical, diffs 5 mirror copies |

### Spec/reality divergences in gemini's plan (caught at Phase 0, not propagated)

Gemini's verification commands include three falsifiable invocations:

1. **`cargo test --test protocol_sentinels`** — does NOT exist as integration test. The sentinels are inline `#[cfg(test)] mod tests` in `src/protocol.rs`. Correct invocation: `cargo test --lib protocol::tests`.
2. **`cargo run --bin xask -- --dry-run`** — `--dry-run` flag does NOT exist. `xask` is a shell script, not a Rust bin. Correct flag: `xask --debug` (per `scripts/xask:8`).
3. **`cargo test --test mutation_routing_thresholds`** — does NOT exist; mutation-tester is a markdown agent template (`templates/agents/mutation-tester.md`). Correct verification: `grep -F` on the template's selection rule.

Gemini's verification commands are aspirational, not data-walked. Brief uses corrected commands throughout. **This is itself evidence for the umwelt frame** — gemini's *world-model* of xbreed does not match xbreed's *world*; the corrections are the lens working in real time.

---

## Theoretical Framing (descriptive, not evidential)

**M7 ACH verdict: H_C (partial).** 5 load-bearing, 2 overdetermined, 1 dropped as decorative. Audit trail: `docs/reports/m7-framing-audit-2026-04-16.md`.

The lenses below are interpretations of design commitments already in the codebase. They are not arguments that xbreed is superior — they are the language in which xbreed's structural uniqueness becomes legible. Each lens carries an **adversarial gate** (Milestone 7) that the brief must pass before the lens earns its keep.

### Umwelt (von Uexküll) — every model has its own world

Each model exposes only what its umwelt permits. xbreed's design commitment: route by umwelt, do not flatten.

- **Cross-model divergence mandate** (`commands/references/xbreed-shared.md`): different models bring different priors and tooling; collapsing them to a single voice destroys signal.
- **Axis → Profile Mapping** (`xbreed-shared.md:124-148`): each axis routes to the model whose umwelt fits.
- **Raw-quote gate** (xask Layer 2) *(overdetermined — umwelt preservation + generic provenance discipline)*: preserves the source model's wording — it is the umwelt-respecting transport.

### Metis (Scott) — local, situated knowledge resists standardization

Local know-how (which agent caught what error, which axis_family carries which evidence shape) cannot be flattened into a generic framework without loss.

- **Per-axis_family evidence schema** (`xbreed-shared.md:131-148`): each axis_family declares its own evidence shape (executor needs failing+passing test output; scout needs file:line citation; planning is artifact-only). One schema would erase distinctions.
- **Filter-exempt allowlist as closed enum keyed on axis_family** (`xbreed-shared.md:146`): not free-text — the enum is the institutional memory of which axes carry executable evidence.
- **Distiller spot-check** (`xbreed-shared.md:162`): bare path citations rejected; literal-substring match required. Local verification, not central trust.

### Anti-scientific-forestry (Scott) — legibility-by-flattening creates fragility

Forestry-by-monoculture optimizes for one axis (board-feet); the forest collapses when one axis fails. xbreed's design commitment: enforce at the layer that catches the failure, not the layer that asks politely.

- **Enforcement Tiers** (`xbreed-shared.md` Enforcement Tiers): Build/CI > Runtime > Protocol. Protocol-only rules (the polite layer) are explicitly weaker than build-time and runtime checks.
- **Pareto Filter Evidence Schema dropping moves without evidence** (`xbreed-shared.md:133`): "dropped, not scored" — no compliance theatre.
- **Materiality rule + Round-2-always-runs** (xbgst exit condition): clean synthesis ≠ frontier halt; structural improvement is the gate.
- **`include_str!` SSoT binding** (`src/protocol.rs:4`) *(overdetermined — anti-forestry build-tier enforcement + Rust-idiomatic compile-time binding)*: forces the SSoT into the **build** tier (rustc fails if the path moves), not the protocol tier (a comment that hopes nobody renames the file).

---

## Milestones (each with executable verification gate)

### M1 — Skeleton: timeout reaps a real child (W3 fix)

**Does:** prove the `xask-timeout` path actually reaps the spawned child process — no zombies, no orphans.
**Touches:** `src/ask.rs:357-421` (`execute_with_timeout`).
**Out-of-scope:** any cross-CLI integration; just the timeout machinery on `/usr/bin/sleep`.
**Gate (executable):**
```bash
# Spawn a sleep we know will outlive the timeout. Assert: timeout fires, no orphan.
cd /home/vhpnk/projects/xbrd-gdsp-fknpft
cat > /tmp/m1_test.rs <<'EOF'
// integration test wired via tests/timeout_reap.rs
use std::process::Command;
use std::time::Duration;
use xbreed::ask::execute_with_timeout;
#[test]
fn timeout_reaps_child() {
    let mut c = Command::new("sleep"); c.arg("30");
    let pid_before = std::process::id();
    let r = execute_with_timeout(c, Duration::from_secs(1));
    assert!(r.is_err());
    assert!(format!("{:?}", r.err().unwrap()).contains("xask-timeout"));
    // pgrep should find zero stray sleeps owned by this pid
    let out = Command::new("pgrep").args(["-P", &pid_before.to_string(), "sleep"])
                                    .output().unwrap();
    assert!(out.stdout.is_empty(), "orphan sleep child not reaped");
}
EOF
mv /tmp/m1_test.rs tests/timeout_reap.rs
cargo test --test timeout_reap timeout_reaps_child
```
**Why this milestone first:** silent-failure axis. A timeout path that *says* it reaps but doesn't is the textbook anti-forestry failure: legible at the protocol level (comment claims reap), invisible at the runtime level (process leaks).

### M2 — Overfit one real instance: SSoT compile binding

**Does:** prove `cargo check` fails when `commands/references/xbreed-shared.md` is renamed/removed (forces SSoT into build tier).
**Touches:** `src/protocol.rs:4`.
**Gate (executable):**
```bash
cd /home/vhpnk/projects/xbrd-gdsp-fknpft
mv commands/references/xbreed-shared.md /tmp/ssot.bak
cargo check 2>&1 | grep -q "couldn't read.*xbreed-shared"; STATUS=$?
mv /tmp/ssot.bak commands/references/xbreed-shared.md
[ $STATUS -eq 0 ] && echo PASS || (echo FAIL; exit 1)
```
**Why this milestone:** anti-scientific-forestry concretized. The commit that "tidies up" the docs path now fails the build, not a lint nobody runs.

### M3 — REQUIRED_SECTIONS sentinels catch heading drift

**Does:** prove inline tests in `src/protocol.rs` catch removal of any of the 11 contracted sections.
**Touches:** `src/protocol.rs:21+` (`REQUIRED_SECTIONS`).
**Gate (executable):**
```bash
cd /home/vhpnk/projects/xbrd-gdsp-fknpft
cargo test --lib protocol::tests
# Then mutation: rename one heading and assert test fails.
sed -i.bak 's|^## xask Gate (4 layers)|## xask Gate (renamed)|' \
  commands/references/xbreed-shared.md
! cargo test --lib protocol::tests 2>&1 | grep -q "xask Gate (4 layers)" && echo "MUT-FAIL EXPECTED"
mv commands/references/xbreed-shared.md.bak commands/references/xbreed-shared.md
cargo test --lib protocol::tests   # baseline restored
```
**Why this milestone:** metis concretized. Each section is institutional memory; the test makes the memory load-bearing.

### M4 — verify-docs.sh catches drift across mirrors

**Does:** prove `scripts/verify-docs.sh` flags when one of the 5 mirror copies disagrees with SSoT on connector routing.
**Touches:** `scripts/verify-docs.sh`, `AGENTS.md`, `templates/agents/connector.md`.
**Gate (executable):**
```bash
cd /home/vhpnk/projects/xbrd-gdsp-fknpft
./scripts/verify-docs.sh && echo "BASELINE PASS"
# Mutate one mirror to claim codex
sed -i.bak 's|xask --effort high gemini|xask --effort high codex|' AGENTS.md
! ./scripts/verify-docs.sh && echo "MUTATION CAUGHT"
mv AGENTS.md.bak AGENTS.md
./scripts/verify-docs.sh && echo "BASELINE RESTORED"
```
**Why this milestone:** metis + anti-forestry. Each mirror has local context; the lint refuses central-trust on any single copy.

### M5 — gemini --effort actually substitutes ThinkingBudget

**Does:** prove `xask --effort {low,medium,high,xhigh} gemini` produces the matching numeric budget in the rendered prompt (and fails loud on bad input rather than silently dropping).
**Touches:** `scripts/xask:77-95`, `templates/dispatch/gemini.md:8`.
**Gate (executable):**
```bash
cd /home/vhpnk/projects/xbrd-gdsp-fknpft
bash tests/xask_effort_substitution.sh   # already exists; verify it covers all 4 tiers
# Direct probe: each effort produces the expected number, default produces "default"
for pair in low:512 medium:4096 high:8192 xhigh:16384; do
  tier="${pair%%:*}"; expect="${pair##*:}"
  out=$(xask --debug --effort "$tier" gemini "ping" 2>&1 | grep -oE "ThinkingBudget: [0-9a-z]+")
  echo "$tier -> $out (expect $expect)"
  echo "$out" | grep -q ": $expect$" || { echo "FAIL $tier"; exit 1; }
done
# Bad effort fails loud
xask --effort bogus gemini "ping" 2>&1 | grep -q "invalid effort" && echo "FAIL-LOUD OK"
```
**Why this milestone:** umwelt concretized. The flag doesn't lie — it converts an abstract tier into the model's native unit.

### M6 — Yolo routing preserved across both code paths (regression sentinel)

**Does:** prove the yolo flags survive any future refactor of `build_codex_ask_with_loadout` and the gemini builder. This is a regression sentinel — the user's directive (`feedback_yolo_routing.md`) is operationalized as a test, not a comment.
**Touches:** `src/ask.rs:99-110` (codex), `src/ask.rs:285` (gemini), `src/ask.rs:564-606` (existing assertions).
**Gate (executable):**
```bash
cd /home/vhpnk/projects/xbrd-gdsp-fknpft
cargo test --lib ask::tests::codex_ask_empty_loadout_has_suppression_and_approval_flags
cargo test --lib ask::tests::codex_ask_spark_adds_model_and_low_effort
# New: assert gemini builder includes --approval-mode yolo
cargo test --lib ask::tests 2>&1 | grep -q "gemini.*yolo" || \
  echo "GAP: add gemini yolo assertion test (currently only inferred from src grep)"
grep -F -- '--approval-mode' src/ask.rs | grep -F -- 'yolo' >/dev/null && echo "SRC-OK"
```
**Why this milestone:** anti-forestry concretized. The user's permissive routing decision lives in three layers — comment, frontmatter, test. Removing the comment is free; removing the test breaks the build.

### M7 — Adversarial axis: does the umwelt frame survive its own gate?

**Does:** spawn `cco-critic` with the heuer-planning skill loaded at Layer 0 to attack the theoretical frame itself. Question on the table: *"Does umwelt apply to model outputs, or is it post-hoc narrative dressed as analysis?"* If the frame cannot survive ACH (Analysis of Competing Hypotheses), it is jettisoned from the brief and M1-M6 are kept on their own structural merits.
**Touches:** the framing section above; nothing in the codebase.
**Gate (process, not bash):**
```
Spawn (during /xbgst godspeed):
  cco-critic-frame  (opus 4.7 high, on_spawn_skill: heuer-planning)
Brief: "Apply heuer ACH to: 'umwelt / metis / anti-forestry are descriptive lenses
        for xbreed's design commitments, not evidence of superiority.'
        Hypothesis A: lenses are load-bearing — predict observable design choices.
        Hypothesis B: lenses are post-hoc narrative — fit any system with dispatch.
        Hypothesis C: lenses partially apply — load-bearing for some commitments,
                      decorative for others.
        Score each hypothesis against the 8 commitments in M1-M6's Why-blocks.
        Output: H_A | H_B | H_C consistency matrix + verdict."

PASS condition: verdict supports H_A or H_C with named load-bearing commitments
                AND named decorative ones (so the brief drops the decorative ones).
FAIL condition: verdict is H_B → the framing section is removed wholesale; M1-M6
                are kept; the brief is republished as "Test Brief — Recent xbreed
                Features (structural)" with no theoretical section.
```
**Why this milestone:** the lens earns its keep or it leaves. R1's correction explicitly warned against using theoretical vocabulary as armor. This milestone is the armor-detector.

---

## Generalization order (after M1-M7 pass)

- M8: cross-model divergence regression sentinel (probe gemini and codex with same `xask --effort high <model>` payload; assert outputs differ on a known-divergent prompt; if they ever converge, raw-quote gate is leaking).
- M9: per-axis_family evidence schema enforcement test (spawn distiller with a malformed proposal; assert drop count > 0).
- M10: judge blinding audit_hash determinism (re-running synthesis on same inputs yields same SHA-256).

These are not gated on this brief — they belong to the next plan.

---

## Polish (last)

- M11: report formatting + cross-link to `xbreed-superiority-r1-2026-04-16.md` errata.
- M12: CI wiring — M1-M6 join `make verify`.

---

## Codex input — appended verbatim (umwelt preservation)

Codex returned 7 tests after the brief's M1-M7 were drafted. Per the umwelt frame, codex's voice is preserved raw (no re-narration in my voice) and then mapped to brief slots. Three of the seven add genuine gaps the rest of the brief missed.

**Source:** `bc3ipx8r9.output` (XASK_TIMEOUT_SECS=240 retry).

> Given the current suite, the biggest remaining gaps are end-to-end transport and the chatty-child race. I'd add these 7 tests.
>
> 1. `execute_with_timeout_drains_both_pipes_without_deadlock`; target: src/ask.rs:375, src/ask.rs:395. Observable: run a child that writes `131072` bytes of `O` to stdout and `131072` bytes of `E` to stderr, then exits `0`; assert `output.status.success()`, `output.stdout.len() == 131072`, `output.stderr.len() == 131072`, and every stdout byte is `b'O'` while every stderr byte is `b'E'`. Verify: `cargo test ask::tests::execute_with_timeout_drains_both_pipes_without_deadlock -- --exact`.
>
> 2. `execute_with_timeout_kills_chatty_child_on_timeout`; target: src/ask.rs:406, src/ask.rs:407, src/ask.rs:408. Observable: child records `$$`, then loops forever writing to both stdout and stderr; assert the error contains `xask-timeout`, the pid file is populated, and after a short settle window `/proc/<pid>` does not exist. This is the real ghost-leak regression trap across all three mpsc arms. Verify: `cargo test ask::tests::execute_with_timeout_kills_chatty_child_on_timeout -- --exact`.
>
> 3. `protocol_is_exactly_bound_to_shared_md_ssot`; target: src/protocol.rs:4. Observable: assert `PROTOCOL == std::fs::read_to_string("commands/references/xbreed-shared.md").unwrap()`. This catches the subtle failure mode where `include_str!` points at the wrong file but the required headings still happen to exist. Verify: `cargo test protocol::tests::protocol_is_exactly_bound_to_shared_md_ssot -- --exact`.
>
> 4. `parse_sections_ignores_fenced_heading_lookalikes`; target: src/protocol.rs:37. Observable: feed `parse_sections()` synthetic markdown containing one real `## xask Gate (4 layers)`, one fenced-code line `## xask Gate (4 layers)`, and one `### xask Gate (4 layers)`; assert the parsed count for that heading is exactly `1` and the body-line count only reflects real prose. This hardens the REQUIRED_SECTIONS sentinel against false duplicates. Verify: `cargo test protocol::tests::parse_sections_ignores_fenced_heading_lookalikes -- --exact`.
>
> 5. `xask_gemini_effort_transport_preserves_budget_and_flag`; target: scripts/xask:82, scripts/xask:183, templates/dispatch/gemini.md:8. Observable: with a fake `xbreed` stub, run `scripts/xask -e xhigh gemini "probe"`; assert captured argv contains `ask gemini --with godspeed --effort xhigh`, and the final prompt contains `# Effort: xhigh` and `# ThinkingBudget: 16384` with no literal `{{EFFORT}}` or `{{THINKING_BUDGET}}` left behind. Verify: `bash tests/xask_gemini_effort_transport.sh`.
>
> 6. `ask_gemini_uses_yolo_and_no_native_effort_flag`; target: src/ask.rs:274, src/ask.rs:285. Observable: through `xbreed ask gemini --effort high ...` with a fake `gemini` binary, assert argv contains the adjacent pair `--approval-mode`, `yolo`, and does not contain any literal `--effort` token. That proves the budget stays prompt-side while yolo stays CLI-side. Verify: `cargo test --test ask_with_loadout ask_gemini_uses_yolo_and_no_native_effort_flag -- --exact`.
>
> 7. `ask_codex_route_preserves_full_unlock_contract`; target: src/ask.rs:97, src/ask.rs:110. Observable: through `xbreed ask codex --effort high "say hi"` with a fake `codex` binary, assert argv contains `exec`, `--skip-git-repo-check`, the adjacent pair `--sandbox`, `danger-full-access`, `approval_policy=\"never\"`, `model_reasoning_effort=high`, and that the last arg is exactly `say hi`. This is the end-to-end check that the yolo route survives real CLI plumbing. Verify: `cargo test --test ask_with_loadout ask_codex_route_preserves_full_unlock_contract -- --exact`.

### Mapping to brief slots

| Codex # | Maps to | Status |
|---|---|---|
| 1 (chatty-child pipe drain) | **NEW M1.5** — gemini missed this; pipe-buffer deadlock is the silent-failure axis at the concurrency layer. Strict add. |
| 2 (chatty-child timeout reap) | **Replaces M1** — codex's `/proc/<pid>` check is more rigorous than my `pgrep -P`. Use codex's variant. |
| 3 (PROTOCOL == fs::read_to_string) | **NEW M2.5** — catches `include_str!` pointing at wrong file with coincidentally-valid headings. Genuine gap in M2+M3. Strict add. |
| 4 (fenced-heading parse) | **NEW M3.5** — assumes a `parse_sections()` helper that may not yet exist; first sub-task: confirm the helper, refactor inline test out of `mod tests` if needed, then add this test. **Risk:** invention if helper doesn't exist. |
| 5 (gemini transport argv + budget) | **Strengthens M5** — adds argv assertion + literal-template-absence assertion. Use codex's variant. |
| 6 (gemini yolo + no --effort) | **Strengthens M6** — adds the SEPARATION assertion (budget=prompt-side, yolo=CLI-side). Use codex's variant. |
| 7 (codex full unlock end-to-end) | **Strengthens M6** — through `xbreed ask codex`, not just `build_codex_ask_with_loadout`. Use codex's variant. |

### Cross-model divergence observation (load-bearing for the umwelt frame)

| Axis | Gemini lens | Codex lens |
|---|---|---|
| **What's at risk** | Drift between docs and engine | Pipe-buffer deadlock + binding misdirection |
| **Test style** | Bash one-liners with mutate/restore | Rust integration tests with byte-exact assertions |
| **Ground truth source** | Aspirational (invents `--dry-run`, `protocol_sentinels`) | Verified (reads actual file:line; uses real flags) |
| **Adversarial depth** | Single mutation per axis | Pathological inputs (131072 bytes, fenced lookalikes) |

This divergence is itself M7's evidence: **two models with different umwelten produce non-overlapping test gaps**. Collapsing them to one voice would destroy the gap-coverage. The brief preserves both as co-equal inputs.

## Inputs and provenance

- **gemini 6-axis plan:** `/tmp/.../tasks/b3b0gxbtn.output` — used as a starting point; verification commands rewritten after data-walk caught three non-existent invocations.
- **advisor (Layer 0):** framing pushback — preserved R1's structural-uniqueness honesty, instantiated as the M7 adversarial axis.
- **R1 superiority report:** `docs/reports/xbreed-superiority-r1-2026-04-16.md` — frame is in continuity with R1's correction, not a regression to it.
- **xbreed SSoT:** `commands/references/xbreed-shared.md` — read directly for axis_family enum, evidence schema, enforcement tiers.

## Handoff

Next session runs `/xbgst | godspeed` on this brief. Expected axes (Round 0): silent-failure-W3, ssot-build-binding, mirror-drift, gemini-effort-substitution, yolo-regression-sentinel, framing-armor-detection, plus whatever codex adds when its retry lands.
