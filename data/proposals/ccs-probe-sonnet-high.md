PROPOSAL: Per-Teammate Effort Propagation via Effort-Bucketed Multi-Session Architecture

---

## Root Cause (brief)

`src/sync.rs:29` forces `teammateMode: "tmux"`. CC reads `CLAUDE_CODE_EFFORT_LEVEL` at **session initialization** — not per-request, not per-pane. All teammates spawned in one session inherit one effort level from the parent env at init. There is no user-space hook into CC's per-pane spawn sequence that could inject a different env var per teammate. R3 Gap 3 confirmed this open.

---

## (a) Design Sketch

**Mechanism: Effort-Bucketed Multi-Session Launch**

Group teammates into effort tiers before spawn; launch one CC session per unique tier; the primary (judge) session coordinates via cross-session SendMessage.

### New: `src/team.rs`

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EffortTier { XHigh, High, Medium }

pub struct TeammateSpec {
    pub name: String,
    pub model: String,
    pub effort: EffortTier,
}

/// Clusters specs by tier; returns (tier, [specs]) pairs in spawn order.
pub fn bucket_by_effort(specs: &[TeammateSpec]) -> Vec<(EffortTier, Vec<&TeammateSpec>)> {
    use std::collections::BTreeMap;
    let mut map: BTreeMap<String, Vec<&TeammateSpec>> = BTreeMap::new();
    for s in specs {
        map.entry(format!("{:?}", s.effort)).or_default().push(s);
    }
    // ... convert to (EffortTier, Vec<&TeammateSpec>) pairs
}
```

### Modified: `src/sync.rs` — new function

```rust
pub fn materialize_per_tier_settings(tier: EffortTier, policy_path: &Path) -> Value {
    let mut settings = materialize_claude_settings(policy_path);
    let tier_str = match tier {
        EffortTier::XHigh => "xhigh",
        EffortTier::High  => "high",
        EffortTier::Medium => "medium",
    };
    // Inject tier into settings env block so outer session launches with it
    settings["env"]["CLAUDE_CODE_EFFORT_LEVEL"] = json!(tier_str);
    settings
}
```

### Spawn sequence (CLI / `src/main.rs`)

```bash
# xbreed generates one settings file per tier, then launches:
CLAUDE_CODE_EFFORT_LEVEL=medium  claude --settings ./generated/settings-medium.json  &
CLAUDE_CODE_EFFORT_LEVEL=high    claude --settings ./generated/settings-high.json    &
# primary judge session (xhigh) is the outer caller — no extra launch needed
```

Each bucketed session runs its own `TeamCreate` for its subset of teammates. The judge SendMessages across sessions by teammate name (CC routes by name globally if `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` is active — **unverified assumption; see failure modes**).

---

## (b) Failure Modes

| Mode | How to detect | Rollback |
|---|---|---|
| **Cross-session SendMessage silently drops** — CC may not route DMs across separate session processes | Teammate never replies; DM audit shows no receipt | Fall back to shared mailbox file (`src/mailbox.rs`) as inter-session relay |
| **Pane-cap amplification** — N tiers × M teammates blows the per-window pane cap faster | `xbreed precheck pane-cap --team-size N` underestimates (doesn't account for N extra session panes) | Update precheck formula: add `tier_count` as extra overhead; re-run with adjusted cap |
| **Multi-session cleanup race** — TeamDelete on tier-sessions before judge session exits leaves orphan tmux windows | `xbreed-cleanup --stale` catches them | Enforce cleanup order: bucket sessions first, judge session last; add to DESPAWN protocol |
| **Settings file collision** — concurrent sessions write to the same `generated/claude-settings.json` | File corruption or stale tier | Write tier-suffixed files: `settings-high.json`, `settings-medium.json`; never overwrite shared path |
| **env var ignored at tier session** — `CLAUDE_CODE_EFFORT_LEVEL` in settings `env` block may not be honored if the launching shell already has it set differently | Teammate `printenv` in tier session shows wrong value | Use shell-level prefix (`CLAUDE_CODE_EFFORT_LEVEL=medium claude ...`) not settings injection as primary path; settings injection is belt-and-suspenders |

---

## (c) Implementation Cost

| File | Change | LoC delta |
|---|---|---|
| `src/team.rs` | New: `EffortTier` enum + `TeammateSpec` + `bucket_by_effort` | +120 |
| `src/sync.rs` | New: `materialize_per_tier_settings` + tier-suffixed write helper | +35 |
| `src/cli.rs` | New `--effort-aware` flag on `spawn`/`team` subcommand; parse tier from teammate name prefix (cco-→high, ccs-→varies, g-→high) | +25 |
| `src/main.rs` | Wire bucket loop + per-tier session launch | +20 |
| `tests/team_effort_bucket.rs` | New: bucket_by_effort unit tests + settings-per-tier output assertions | +65 |
| **Total** | 4 files modified, 1 new test file | **~265 LoC** |

**Ceiling label:** Runtime-tier enforcement with documented ceiling. The critical unverified assumption (cross-session SendMessage routing) is Protocol-tier until empirically confirmed. If SendMessage doesn't cross session boundaries, the design degrades to mailbox-relay, which is Build/CI-tier only for the relay mechanism.

**Lowest-friction alternative (protocol-tier only):** Encode effort in the teammate name prefix per current convention + add a CLAUDE.md directive to emit effort-correlated reasoning depth. Zero code. Already tried; already non-operative for API-level compute allocation.
