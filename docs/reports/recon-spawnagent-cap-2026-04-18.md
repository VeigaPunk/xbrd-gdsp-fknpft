# RECON: codex-spark parallel-spawn cap — 2026-04-18

**Agent:** cdx-revenger-spawnagent-cap  
**Mission:** locate the empirically-observed 6-parallel-spawn cap from codex-swarm-mini-0418  
**Verdict: neither Path A nor Path B applies — cap is prompt-driven, not architectural**

---

## Primary-source findings

### (a) Cap location anchors

**`~/.codex/config.toml` — NO cap present**  
```toml
# [features] block, lines 30-33
[features]
fast_mode = true
enable_request_compression = true
tool_suggest = true
```
`multi_agent`, `max_parallel`, `concurrency`, `max_threads` — none present.  
`codex features list` runtime output: `enable_fanout` = `false` (line 12), `multi_agent` = `true` (line 24).  
No `[agents]` table exists. Prior-mission claim of `[agents].max_threads=10` is stale lore (see `docs/reports/noninteractive-leverage-0418-closure-2026-04-18.md:60`).

**`scripts/xask` — NO parallelism injection**  
`scripts/xask:62` — builds only `--spark/--review/--full/--effort/--json/-o` flags.  
`scripts/xask:214` — forwards to `xbreed ask codex`. No `-p`, no `features.*`, no concurrency flags.

**`src/ask.rs` — NO parallelism injection**  
`src/ask.rs:45` — builds: `codex exec --skip-git-repo-check --color never --ephemeral --sandbox danger-full-access -c approval_policy="never"`  
`src/ask.rs:72-75` — spark adds: `-m gpt-5.3-codex-spark -c model_reasoning_effort=low`  
`src/ask.rs:480-489` — effort override only. Zero parallel-cap flags anywhere in the dispatch path.

**`codex` binary internals — NO hardcoded 6 constant**  
Binary version: `codex-cli 0.121.0` (Rust, musl, at `.../@openai/codex-linux-x64/.../codex`).  
`strings` scan: exposed `core/src/tools/parallel.rs`, `core/src/tools/handlers/multi_agents/*.rs`, `agent job concurrency: job_id= max_threads=` — debug format strings, no numeric `6` literal for a cap.  
No `max_parallel` config key is surfaced in any binary string or help output.

**Server-side OAuth tier — NOT a cap**  
Disproven empirically: same OAuth-backed install, same session, fanned out 12 — see probe results below.

---

## (b) Probe results — the 6 was prompt behavior, not a limit

Direct probe: `codex exec -m gpt-5.3-codex-spark --json "Orchestrate 12 parallel shell probes..."`  
**Result: 12 `item.started ... command_execution` events before first `item.completed`** — simultaneous starts.

xbreed path probe: `xbreed ask codex --spark --json "<12-parallel prompt>"`  
**Result: `STARTED_LINES=12`** — identical behavior through the full dispatch stack.

`multi_agent` isolation: `--disable multi_agent` → 12, `--enable multi_agent` → 12. Feature toggle has no effect on shell-tool fanout count.

**Conclusion:** The "6 parallel probes" observed in codex-swarm-mini-0418 was the model's heuristic response to the specific prompt phrasing in that session, not a config-reachable or binary-hardcoded architectural limit. The model already fans out to whatever N is requested in the prompt.

---

## (c) Concrete recipe

No config change needed. To get 12-way fanout from codex-spark in xask dispatches, the orchestrating prompt must explicitly request 12 parallel probes. The existing labrat-swarm dispatch templates (wherever they invoke `xask --spark codex "Orchestrate N parallel..."`) should set N=12 in the prompt text.

If the orchestrating judge/planner was generating "Orchestrate 6 parallel probes" as boilerplate, that is the only lever to change. No `~/.codex/config.toml` edit, no `scripts/xask` patch, no `src/ask.rs` change, no wrapper needed.

**Practical one-liner verification:**
```bash
xask --spark codex "Orchestrate 12 parallel probes: run 'echo probe-N' for N in 1..12, all simultaneously, report count of started probes"
# Expect: 12 simultaneous starts in JSONL stream
```

---

## (d) Trendsetter verdict

**CONSUMER** — no patching of codex internals, no shim, no workaround.  
The model accepts the requested parallelism directly from the prompt. xbreed is a pure consumer of codex's native capability surface.

---

## Path determination

| Path | Condition | Applies? |
|------|-----------|----------|
| **A** — lift cap via user-reachable config or xbreed runtime | Cap in `~/.codex/config.toml` or `scripts/xask`/`src/ask.rs` | **No** — no cap present in any of these locations |
| **B** — xbreed double-dispatch wrapper | Cap is binary-internal, not user-config-reachable | **No** — cap is not binary-internal; it's a model heuristic |

**Actual finding:** neither path. The 6 was emergent model behavior from prompt phrasing. Adjust the prompt, not the infrastructure.

---

## Unknowns

- True upper ceiling beyond 12 not probed. May be model-dependent or session-dependent.
- Full Rust source of shipped binary not available; internal semaphore limit (if any) above 12 not confirmed.

---

## xask gate status

`xask -R -F codex` invocation failed with exit code 1 (codex process returned non-zero). Full findings captured via Monitor stream before exit. Layer 3 fallback: in-session RECON completed (config read, binary strings, scripts/ask.rs grep). Core findings confirmed through both paths independently.
