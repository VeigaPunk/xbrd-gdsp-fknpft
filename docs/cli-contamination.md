# CLI Contamination Reference

Context injection behavior for Gemini CLI and Codex CLI — what each injects by default, how to suppress it, and how `xask` manages clean vs rich dispatch.

## Problem

Both Gemini CLI and Codex CLI auto-inject workspace context into every session:
- **Gemini:** directory tree of CWD, GEMINI.md file contents (upward traversal to `.git`), persistent memory from `~/.gemini/memory/`, JIT injection of GEMINI.md from any directory the model's tools touch at runtime.
- **Codex:** system instructions (permissions, apps, environment), git repo context, `~/.codex/config.toml` personality and plugin config.

In cross-model orchestration (judge → gemini/codex), this injection contaminates epistemic equivalence — each model starts from different ambient state, making comparisons and Pareto filtering unreliable.

---

## Gemini CLI

### Default injection sources

| Source | Volume | Notes |
|---|---|---|
| CWD directory tree | ~6.5 KB typical | Recursive file listing of workspace |
| GEMINI.md traversal | variable | Upward from CWD to `.git` boundary marker |
| `~/.gemini/memory/` | variable | Persistent cross-session memory |
| JIT GEMINI.md injection | variable | v0.36.0+: injects on file touch at runtime |

### Suppression knobs (`~/.gemini/settings.json`)

```json
{
  "context": {
    "includeDirectoryTree": false,
    "memoryBoundaryMarkers": [],
    "fileName": "__disabled__",
    "fileFiltering": {
      "respectGeminiIgnore": true
    }
  }
}
```

| Key | Effect |
|---|---|
| `includeDirectoryTree: false` | Kills CWD directory tree (highest impact) |
| `memoryBoundaryMarkers: []` | Disables upward GEMINI.md traversal entirely |
| `fileName: "__disabled__"` | Prevents any GEMINI.md loading globally |
| `respectGeminiIgnore: true` | Honors `.geminiignore` for surgical path exclusion |

No flag exists to suppress context per-invocation. All control is via `settings.json`.

### xbreed profile behavior

`xbreed ask gemini` sets `HOME=~/.config/xbreed/gemini-profiles/primary` when that directory exists. If it does not exist (default), it falls back to the user's `~/.gemini/` — meaning `~/.gemini/settings.json` controls context injection for all `xbreed ask gemini` calls.

`GEMINI_SETTINGS` env var is **not honored** by either `xbreed` or `gemini` CLI. The env var approach in earlier xask versions was a no-op.

### Clean profile (`~/.gemini/profiles/dispatch-clean.json`)

This file exists but is not used by any dispatch path. It documents the target clean state:

```json
{
  "context": {
    "includeDirectoryTree": false,
    "memoryBoundaryMarkers": [],
    "fileName": "__disabled__",
    "fileFiltering": { "respectGeminiIgnore": true }
  }
}
```

The current `~/.gemini/settings.json` already has `includeDirectoryTree: false` as the default clean state.

### JIT injection (unresolved)

GEMINI.md files from directories touched at runtime are injected (v0.36.0+). No documented disable mechanism. Workaround: place `.geminiignore` at project root excluding known GEMINI.md parent dirs.

---

## Codex CLI

### Default injection sources

| Source | Notes |
|---|---|
| Permissions system instructions | Auto-injected system prompt block |
| Apps system instructions | Auto-injected system prompt block |
| Environment context | OS, shell, git repo info |
| `~/.codex/config.toml` | personality, plugins, project trust levels |

### Key flags (`codex exec`)

```
--skip-git-repo-check          Run outside a git repo (default sandbox: read-only)
-s, --sandbox <MODE>           read-only | workspace-write | danger-full-access
-a, --ask-for-approval <POL>   untrusted | on-request | never
-C, --cd <DIR>                 Set working root
--add-dir <DIR>                Add writable directory alongside workspace
-c <key=value>                 Override config.toml value inline
-p, --profile <NAME>           Load a named config profile
```

### Context suppression via `-c` flags

```bash
codex exec \
  --skip-git-repo-check \
  -c include_permissions_instructions=false \
  -c include_apps_instructions=false \
  -c include_environment_context=false \
  "$PROMPT"
```

Note: `include_*` key names are sourced from Codex help text examples; the binary does not expose a config schema. Unknown keys are silently ignored (TOML parser falls back to raw string). Confirm validity against current binary before relying on these for suppression.

### Default approval and sandbox

- `codex exec` default approval: `never`
- Default sandbox (no git repo): `read-only`
- Default sandbox (git repo): `workspace-write`

### Per-project trust (without editing config.toml)

```bash
codex exec -c 'projects."/path/to/project".trust_level="trusted"' "$PROMPT"
```

### `--full-auto` limitation

Only works interactively. Fails with "stdin is not a terminal" in piped/non-interactive contexts. Use `--sandbox workspace-write` instead for scripted runs.

---

## xask dispatch contract

`xask` is the contamination-aware dispatch wrapper at `~/.local/bin/xask` and `~/projects/the-crossbreeder/scripts/xask`.

### Modes

| Flag | Gemini behavior | Codex behavior |
|---|---|---|
| (none) — clean | Uses current `settings.json` (dir tree off) | Routes via `xbreed ask codex` with godspeed skill |
| `--rich` | Toggles `includeDirectoryTree: true`, dispatches, restores `false` on EXIT trap | Routes via `xbreed ask codex` (full context) |
| `--direct` | N/A | Bypasses xbreed: `codex exec --skip-git-repo-check -c ...` |

### Gemini rich mode note

The `--rich` path mutates `settings.json` in place and uses an EXIT trap to restore it. If the process is killed before the trap fires, `settings.json` will be left with `includeDirectoryTree: true`. Check and restore manually if dispatch hangs:

```bash
python3 -c "
import json
with open('/home/vhpnk/.gemini/settings.json') as f: d = json.load(f)
d.setdefault('context', {})['includeDirectoryTree'] = False
with open('/home/vhpnk/.gemini/settings.json', 'w') as f: json.dump(d, f, indent=2)
"
```

### Template dispatch

Templates at `~/projects/the-crossbreeder/templates/dispatch/{gemini,codex}.md` wrap the payload in Inter-Model Protocol v0.2 format before dispatch. Variables: `{{QUERY}}`, `{{CONTEXT}}`, `{{SCOPE_BOUNDARY}}`.

---

## Open issues

1. **Codex `-c include_*` keys unverified** — the binary has no string matches for `include_permissions_instructions`. Test with `codex exec -c include_permissions_instructions=false --debug "echo test"` to confirm suppression.
2. **JIT GEMINI.md injection** — no disable flag for runtime-touch injection. Monitor context size with `--debug` in Gemini.
3. **xbreed gemini profiles** — if `~/.config/xbreed/gemini-profiles/primary/` is created in the future, the profile's `~/.gemini/settings.json` must also have `includeDirectoryTree: false` for clean dispatch to work.
