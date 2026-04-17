PROPOSAL: Per-teammate effort propagation via PATH-shim on `claude` binary

## (a) Design sketch

**Core mechanism:** Insert an xbreed-owned shim ahead of the real `claude` binary on PATH. The shim runs *inside each teammate pane* (not the outer session), reads the teammate's identity, maps prefix → effort tier via config, exports `CLAUDE_CODE_EFFORT_LEVEL` into its own env, then `exec`s the real claude. Because the env var is set pre-exec, the child claude process reads it at session-init as if the user had typed `CLAUDE_CODE_EFFORT_LEVEL=... claude` manually.

**New CLI subcommand:** `xbreed teammate-shim` — when `argv[0]` basename resolves to `claude`, it acts as the shim; when invoked as itself, it installs/uninstalls the symlink.

```rust
// src/teammate_shim.rs
pub fn run_as_shim(argv: Vec<String>) -> ! {
    let name = teammate_name_from_env()      // CLAUDE_TEAMMATE_NAME (CC 2.1+) or scan argv
        .or_else(|| teammate_name_from_argv(&argv));
    let tier = name.as_deref().and_then(tier_from_prefix);
    let real = resolve_real_claude();        // PATH scan, skip self inode
    let mut cmd = Command::new(&real);
    cmd.args(&argv[1..]);
    if let Some(t) = tier { cmd.env("CLAUDE_CODE_EFFORT_LEVEL", t); }
    cmd.env("XBREED_SHIM_ACTIVE", "1");      // re-entry guard
    audit_log(&name, &tier, &real);          // ~/.xbreed/logs/shim.ndjson
    cmd.exec(); unreachable!()
}

fn tier_from_prefix(name: &str) -> Option<&'static str> {
    match () {
        _ if name.starts_with("cco-advisor")   => Some("max"),
        _ if name.starts_with("cco-the-judge") => Some("xhigh"),
        _ if name.starts_with("cco-")          => Some("high"),
        _ if name.starts_with("ccs-distiller") 
          || name.starts_with("ccs-simplifier")
          || name.starts_with("ccs-scribe")    => Some("medium"),
        _                                       => None,   // session default
    }
}
```

**Shim installation (sync-side, src/sync.rs):**
- `xbreed sync` creates `~/.xbreed/bin/claude` → symlink to `xbreed` binary (argv[0] = `claude` triggers shim mode in `main.rs`).
- `materialize_claude_settings` prepends `~/.xbreed/bin` to the `env.PATH` in the teammate settings block (CC propagates settings.env to teammate procs per R3 `/proc/$PPID/environ` evidence).
- Tier map externalised in `config/teammate-tiers.yaml` (prefix → tier rows); shim loads once at start.

## (b) Failure modes

| Mode | Symptom | Detection | Rollback |
|---|---|---|---|
| CC doesn't set `CLAUDE_TEAMMATE_NAME` nor pass name in argv | `tier = None` always, session-default applies | Audit log shows `name=null` for every spawn | Pure passthrough — no harm; remove PATH entry |
| CC resolves claude via absolute path (bypasses PATH) | Shim never fires, silent no-op | Audit log empty after spawns | Same — shim inert, remove entry |
| Real claude resolved back to shim (exec loop) | Fork bomb / fast crash | `XBREED_SHIM_ACTIVE=1` guard aborts w/ exit 77 | Guard catches; remove shim |
| Tier map stale for new prefixes | New `cco-foo` role runs at session default | Manual diff of `ls templates/agents/` vs tier keys | Add entry + re-sync |
| `exec` fails (ENOEXEC, missing claude) | Teammate pane dies | tmux pane shows "exec failed: ..." | PATH unshim one-liner: `xbreed sync --no-effort-shim` |
| Shim latency adds to spawn cold-start | Slow TeamCreate | Audit log timestamps vs tmux pane-start | Per-pane overhead <5ms; non-issue unless batch>20 |
| Settings.env not inherited by tmux pane | Shim never on PATH | `printenv PATH` inside pane | Fall back to outer-shell PATH export |

**Empirical gap (R3 Gap 3 inheritance):** still unresolved — if CC reads effort before spawning teammate shell (harness internal), env-var injection post-spawn is too late. Requires session-restart probe; mitigation is session-wide `CLAUDE_CODE_EFFORT_LEVEL` fallback (R3 documented workaround) retained as failsafe.

## (c) Implementation cost

| File | Action | LoC |
|---|---|---|
| `src/teammate_shim.rs` | NEW — shim logic, tier map loader, audit log | ~120 |
| `src/sync.rs` | Add PATH entry + shim-symlink install in `materialize_claude_settings` | +25 |
| `src/cli.rs` | Add `teammate-shim install/uninstall/dry-run` subcommand | +35 |
| `src/main.rs` | argv[0]-basename dispatch to shim mode | +15 |
| `src/lib.rs` | Export module | +2 |
| `config/teammate-tiers.yaml` | NEW — prefix → tier table | ~30 |
| `tests/teammate_shim.rs` | NEW — prefix mapping, env injection, loop guard, dry-run | ~140 |
| `commands/references/xbreed-shared.md` | Amend §Session Effort Configuration — add per-teammate path | +15 |

**Total:** ~380 LoC net across 8 files. **New tests:** 8 (prefix map 4 cases, exec-loop guard, real-claude resolution, missing-tier passthrough, dry-run mode). **Risk tier:** Runtime-tier with documented ceiling — bypassable if user exports a different PATH in their shell rc, but default xbreed-sync'd path closes the ergonomic gap for the common case.

**Ship-order:** shim module + tests → sync PATH injection → dry-run flag first (logs tier without setting env) → flip on after one session of clean audit logs → update R3 Gap 3 to CLOSED.