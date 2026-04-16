# xask protocol reference

> Contamination-aware template dispatch for cross-model orchestration.  
> All dispatches route through `xbreed ask` — clean suppression, loadout injection, auth cascade, and godspeed forwarding are always-on.

---

## 1. Synopsis

```
xask [-d] [-s <scope>] [-r] [--spk] [-e <level>] <model> "<query>" ["<context>"] ["<skill>"]
```

`<model>` is one of `gemini`, `codex`, or `claude`.  
`<context>` defaults to `"No prior context."`.  
`<skill>` defaults to `"godspeed"`.

---

## 2. Flag dictionary

| Long form | Short | Description | Models | Default |
|-----------|-------|-------------|--------|---------|
| `--debug` | `-d` | Print constructed prompt and exit (dry run). Matches gemini's own `-d/--debug`. | all | `false` |
| `--scope` | `-s` | Scope boundary injected into `{{SCOPE_BOUNDARY}}` in the dispatch template. Note: shadows gemini's `-s/--sandbox` (consumed by xask, no runtime conflict). | all | `"entire project"` |
| `--rich` | `-r` | Restore `includeDirectoryTree: true` in a PID-namespaced gemini settings copy. Note: shadows gemini's `-r/--resume`. | gemini | `false` (tree suppressed) |
| `--spark` | `--spk` | Pin codex to `gpt-5.3-codex-spark` + `model_reasoning_effort=low`. Mutually exclusive with `--effort` on codex. | codex | `false` |
| `--effort` | `-e` | Reasoning effort level. **Codex/claude only** — gemini silently ignores this flag (warning emitted). Note: shadows gemini's `-e/--extensions` (consumed by xask, no dispatch conflict, but alias is meaningless on gemini path). See per-model mapping below. | claude, codex | unset |
| `--direct` | — | **Deprecated.** No-op (suppression is always-on). Use `--effort` to control reasoning level. | — | — |

### `--effort` per-model mapping

| Model | Maps to |
|-------|---------|
| `claude` | `--effort <level>` (native Claude flag) |
| `codex` | `-c model_reasoning_effort=<level>` (config key, not a flag) |
| `gemini` | Warning emitted; flag ignored. Use `thinkingBudget` in the prompt template instead. |

**Validated effort levels (codex):** `low`, `medium`, `high`, `xhigh`. Level `none` is not validated by xbreed and may fail at codex runtime.

---

## 3. Built-in behaviors (always-on, not user-exposed)

These are injected by xask/xbreed regardless of user flags.

### Gemini

| Behavior | Mechanism | Why |
|----------|-----------|-----|
| `--approval-mode yolo` | Hardcoded in `build_gemini_with_auth` | Prevents stdin hang when gemini prompts for tool call approval in non-interactive dispatch |
| `includeDirectoryTree: false` | Default gemini settings in clean mode | Suppresses directory context for epistemic cleanliness |
| PID-namespaced `HOME` for `--rich` | `gemini_rich_setup()` in xask | Prevents concurrent `--rich` calls from stomping each other's `settings.json` |
| OAuth creds copied to temp HOME | `gemini_rich_setup()` | Auth still works from the temp HOME during rich mode |

### Codex

| Behavior | Mechanism | Why |
|----------|-----------|-----|
| `-a never` | `build_codex_ask_with_loadout` | Prevents interactive approval prompts |
| `--skip-git-repo-check` | `build_codex_ask_with_loadout` | Avoids repo detection noise in headless dispatch |
| `-c include_permissions_instructions=false` | `build_codex_ask_with_loadout` | Suppression |
| `-c include_apps_instructions=false` | `build_codex_ask_with_loadout` | Suppression |
| `-c include_environment_context=false` | `build_codex_ask_with_loadout` | Suppression |
| `-c features.fast_mode=true` | `build_codex_ask_with_loadout` (non-spark only) | Faster output on gpt-5.4 family; omitted on spark (gpt-5.3-codex-spark) |
| `-c model_reasoning_effort=low` | `build_codex_ask_with_loadout` (spark only) | Hard-wired to low on spark path |

**Note (v0.120.0):** `include_skills_instructions` and `include_plugins_instructions` are not available in the current codex release — no further suppression keys exist.

### All models

| Behavior | Mechanism | Why |
|----------|-----------|-----|
| `\| godspeed` appended to prompt | xask line 71-73 | Forwards godspeed posture through text for codex-exec paths where no `--with` skill mechanism exists |
| Loadout injection via `--with <skill>` | `xbreed ask` Rust layer | Injects skill files (e.g. `godspeed`) via model-native mechanism (see dispatch table) |

---

## 4. Model dispatch table

| Model | xask routes to | Rust function | Loadout injection method |
|-------|---------------|---------------|--------------------------|
| `gemini` | `xbreed ask gemini` | `build_gemini_with_auth` + `dispatch` | Prompt prepend: `<loadout>\n\n---\n\n<prompt>` |
| `codex` | `xbreed ask codex` | `build_codex_ask_with_loadout` + `dispatch` | `-c developer_instructions=<toml-quoted-string>` |
| `claude` | `xbreed ask claude` | `build_claude_ask_with_loadout` + `dispatch` | `--append-system-prompt <loadout>` |

### Gemini model

Pinned to `gemini-3.1-pro-preview` (input id). The gemini CLI routes this internally to `gemini-3.1-pro-preview-customtools` for OAuth users with Gemini 3.1 launched. Do **not** use the `-customtools` id as input — it is a routing output and 404s on both auth paths (`isVisible: false` in gemini-cli's `defaultModelConfigs.ts`, verified 2026-04-11).

### Codex model

Default: `gpt-5.4` family (xbreed does not pin a non-spark model id; codex CLI uses its own default).  
Spark: `gpt-5.3-codex-spark` (pinned via `-m` flag).

---

## 5. Auth

### Gemini — OAuth-first cascade (5 levels, sequential)

```
1. OAuthProfile("primary")   — HOME override to ~/.config/xbreed/gemini-profiles/primary/
2. OAuthProfile("fallback")  — HOME override to ~/.config/xbreed/gemini-profiles/fallback/
3. OAuthDefault              — real HOME, no API key injection, uses ~/.gemini/oauth_creds.json
4. ApiKey(<primary>)         — GEMINI_API_KEY from .env.local
5. ApiKey(<fallback>)        — GEMINI_API_KEY_FALLBACK from .env.local
```

Cascades on: `429`, `401`, `403`, `PERMISSION_DENIED`, `UNAUTHENTICATED`, `API_KEY_INVALID`.  
Non-retriable errors bail immediately (no cascade).

OAuth is preferred: subscription-gated model variants (e.g. customtools routing) require an OAuth session. API keys have stricter per-account QPS limits.

**Setup:** `gemini login` (default OAuth) or `HOME=~/.config/xbreed/gemini-profiles/primary gemini login` (named profile).

### Codex — ChatGPT OAuth

```
codex login
```

Requires a ChatGPT Plus/Pro/Enterprise subscription or API key. xbreed does not manage codex OAuth.

### Claude — claude login

```
claude login
```

xbreed does not manage Claude auth.

---

## 6. Naming convention

Agent and teammate names use a prefix that signals where reasoning lives:

| Prefix | Target model | Examples |
|--------|-------------|---------|
| `g-` | Gemini | `g-scout-research`, `g-connector-axes` |
| `cdx-` | Codex | `cdx-labrat-probe`, `cdx-reviewer-security` |
| `ccs-` | Claude Code (Sonnet) | `ccs-executor-docs`, `ccs-simplifier-refactor` |
| `cco-` | Claude Code (Opus) | `cco-judge`, `cco-distiller` |

The prefix is the xask delegation target, not the model running the agent (which is always Claude). A `g-scout-*` agent's first tool call must be `xask gemini`.

---

## 7. Self-referential findings (probe: 2026-04-16)

Findings from gemini and codex probing their own CLI behavior — surfaced during a 2026-04-16 multi-model flag audit.

### Gemini self-report

- **Skill activation is LLM-native** — gemini uses an `activate_skill{}` tool call to load skills; the mechanism is inside the model, not a CLI flag. xbreed's loadout injection (prompt prepend) covers this path.
- **`--approval-mode yolo` required** — without it, gemini hangs on stdin waiting for tool call approval. Already always-on via `build_gemini_with_auth`.
- **OAuth active, API key fallback functional** — cascade works as documented. OAuth users with Gemini 3.1 get customtools routing automatically.
- **`-o/--output-format text|json|stream-json`** — available for structured headless output; not currently exposed via xask. Candidate future flag. Parallel to codex's `--json` + `-o/--output-last-message`.
- **`--include-directories`** — workspace expansion flag; `xask --rich` is the current workaround (mutates `includeDirectoryTree` in settings.json).
- **`-m/--model` override** — available but hardcoded in xbreed to `gemini-3.1-pro-preview`; not user-exposed via xask.

### Codex self-report (from `xask --spark codex` + `codex exec --help` + `src/ask.rs` direct read)

- **`features.fast_mode=true` confirmed** — correct key, gpt-5.4 non-spark path only. Spark always hard-wires `model_reasoning_effort=low`.
- **Effort is a `-c` config key, not a CLI flag** — codex exec has no `--effort` flag; xbreed maps `--effort <level>` → `-c model_reasoning_effort=<level>` (confirmed `src/ask.rs:407`).
- **Validated effort levels**: `low`, `medium`, `high`, `xhigh`. Level `none` not validated by xbreed; may fail at codex runtime.
- **`-e` shell alias gap (now closed)** — xask previously only parsed `--effort` long form; `xbreed ask` Rust CLI already had `-e`. Shell-layer parity restored by this update.
- **No additional suppression keys** — `include_skills_instructions` / `include_plugins_instructions` not available in v0.120.0.
- **Unused headless flags (candidates)**: `--ephemeral` (no session persistence), `--json` (JSONL event stream), `-o/--output-last-message` (write final response to file). Not currently exposed via xask.

### Alias shadow warnings

> **Flag namespace boundary:** xask short aliases are consumed by the xask shell layer and are **never forwarded to the underlying CLI**. The native gemini `-s/-r/-e` flags remain inaccessible through xask by design. If you need a gemini-native flag not exposed by xask, call `xbreed ask gemini` directly.

These xask short aliases shadow gemini's own native flags. There is **no runtime conflict**, but users should be aware of the cognitive collision if they also use the gemini CLI directly:

| xask alias | Shadows gemini flag | gemini meaning | Applies to gemini path? |
|------------|--------------------|----|---|
| `-s` | `-s/--sandbox` | Run in sandbox mode | No (scope is xask-layer) |
| `-r` | `-r/--resume` | Resume a prior session | No (rich mode is xask-layer) |
| `-e` | `-e/--extensions` | Load extension files | **No — effort is codex/claude only; gemini ignores it with a warning** |
| `-d` | `-d/--debug` | Debug output (same semantics ✓) | Yes — debug is xask-layer, model-agnostic |
