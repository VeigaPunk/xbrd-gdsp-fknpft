---
date: 2026-04-11
tags: [finding, gemini, customtools, routing, oauth, model-selection, substrate, retcon]
related: [[2026-04-11-v0.4.0-posture-retcon-walk]], [[2026-04-11-v0.3.5-gemini-oauth-cascade]], [[2026-04-11-next-frontier-walk]]
status: finding (settings.json + GEMINI.md + src/ask.rs comment retconned; no Rust behavior change)
commit: (pending)
---

# customtools-routing-finding — the variant is a routing target, not a pinnable input

## TL;DR

`gemini-3.1-pro-preview-customtools` is a real, released Gemini model (1M context, released 2026-02-25, specialized for tool-selection reliability), **but it is NOT a valid `-m` flag input**. It is a silent runtime routing target: gemini-cli's `getUseCustomToolModelSync()` routes `gemini-3.1-pro-preview` → customtools when `authType === AuthType.USE_GEMINI` (OAuth) AND the account has Gemini 3.1 launched. Pinning the customtools id as input 404s on BOTH OAuth and API-key paths because it has `isVisible: false` in the CLI's model catalog. xbreed's v0.3.5 OAuth-first cascade is already optimal for customtools access.

## What the prior walk reported (wrong)

[[2026-04-11-v0.4.0-posture-retcon-walk]] shipped this framing:

> "customtools variant is gated above the gemini-CLI surface entirely (likely Vertex AI only). Do NOT attempt to pin it again without a new substrate change."

This was wrong on the "Vertex AI" guess, wrong on "gated above the CLI surface," and wrong on "do not pin again." The variant is reachable — just not via the flag xbreed was trying.

## What's actually true

gemini-cli computes the effective model at runtime via `getUseCustomToolModelSync()` in `packages/core/src/config/config.ts`:

```ts
getUseCustomToolModelSync(): boolean {
  const useGemini3_1 = this.getGemini31LaunchedSync();
  const authType = this.contentGeneratorConfig?.authType;
  return useGemini3_1 && authType === AuthType.USE_GEMINI;
}
```

When that returns `true`, `modelIdResolutions` in `packages/core/src/config/defaultModelConfigs.ts` maps `gemini-3.1-pro-preview` → `gemini-3.1-pro-preview-customtools` via the condition `{ useCustomTools: true }`. The `-m` flag is the INPUT to routing; the customtools id is the OUTPUT.

## Smoking-gun evidence

### User's `~/.gemini/settings.json` had the pin as the default model

```json
"model": { "name": "gemini-3.1-pro-preview-customtools" }
```

When the user ran naked `gemini` (TUI), settings.json → `-m gemini-3.1-pro-preview-customtools` → backend 404 → "this model is not available, your admin might've disabled..." error. The 404 fires because the pinned id is `isVisible: false` in the input catalog.

### 4-probe truth table

Run by `gemini-probe` (builder teammate) + cross-verified lead-side:

| # | cmd | auth | verdict | why |
|---|---|---|---|---|
| 1 | `gemini -m gemini-3.1-pro-preview -p "..."` | OAuth (settings.json default) | **PASS** | CLI routes to customtools under OAuth (silent) |
| 2 | `gemini -m gemini-3.1-pro-preview-customtools -p "..."` | OAuth | **404** | id is routing output, not input |
| 3 | `GEMINI_API_KEY=X gemini -m gemini-3.1-pro-preview-customtools -p "..."` | API key | **404** | same; API key doesn't activate routing either |
| 4 | `target/debug/xbreed ask gemini --with godspeed "..."` | xbreed OAuth cascade | **PASS** | OAuth-first → routing kicks in → customtools |

Probes 1 and 4 both succeed because they are silently running the customtools variant under the hood, courtesy of gemini-cli's runtime routing logic. xbreed has been calling customtools this entire session without knowing it.

### Source citations (via `gemini-research` scout)

- `packages/core/src/config/defaultModelConfigs.ts` — defines the `gemini-3.1-pro-preview-customtools` entry with `isVisible: false` + the routing condition `{ useCustomTools: true }` pointing `gemini-3.1-pro-preview` at it
- `packages/cli/src/config/config.ts:802` — CLI precedence: `argv.model || process.env['GEMINI_MODEL'] || settings.model?.name` — command-line `-m` wins over env var wins over settings.json
- `packages/cli/src/ui/components/ModelDialog.tsx`, `packages/cli/src/acp/acpClient.ts` — both pass `useCustomTools: useCustomToolModel` into model resolution context
- `packages/core/src/availability/policyHelpers.ts` — reads the flag and passes into `resolveModel()`
- `packages/core/src/billing/billing.test.ts` — test confirms `isOverageEligibleModel('gemini-3.1-pro-preview-customtools')` returns false (it's a routing-only id)
- `.github/workflows/evals-nightly.yml` — customtools is in the nightly eval set (confirms it's a live production model, not deprecated)
- `docs/changelogs/latest.md` — "Fix dynamic model routing for gemini 3.1 pro to customtools model" — prior bug in the routing code

### openrouter.ai self-description

URL: https://openrouter.ai/google/gemini-3.1-pro-preview-customtools

- 1,048,576 token context
- Input: text/audio/image/video/file; output: text only
- Provider: Google AI Studio
- Pricing: $2/M input, $12/M output (tiered higher above 200K)
- Released 2026-02-25
- Described as a **specialized variant of Gemini 3.1 Pro that improves tool selection to prevent bash-tool overuse** — a behavioral optimization for function-calling reliability

This matches the user's handout claim from [[2026-04-11-next-frontier-walk]] that customtools is "superior for orchestrator-executor dynamics."

## Why it matters for xbreed

- **v0.3.5 OAuth-first cascade is already optimal.** OAuth users with Gemini 3.1 launched automatically get customtools via routing. API-key fallback users get base preview (still functional, loses the tool-selection optimizations but preserves availability under quota exhaustion).
- **Do NOT pin the customtools id in `src/ask.rs`.** The prior retcon comment implied it might work via some path. Accurate framing: it's routing output, not input.
- **The user's settings.json pin was counterproductive.** It caused TUI 404s where routing would otherwise get them customtools seamlessly. Fix: pin `gemini-3.1-pro-preview` in settings.json and let routing happen.

## Fixes applied in this walk

- **F1** — `~/.gemini/settings.json` `model.name`: `customtools` → `preview`. TUI error fixed; routing still delivers customtools to OAuth user.
- **F2** — `~/.gemini/GEMINI.md` prose: updated the `**Model:**` line to say preview with routing explanation. Preserves user's aspirational intent while being accurate.
- **F3** — `src/ask.rs` `GEMINI_DEFAULT_MODEL` comment: rewritten with accurate routing explanation + cross-link to this doc.
- **F4** — [[2026-04-11-v0.4.0-posture-retcon-walk]] gets an UPDATE cross-link pointing here.
- **F5** — v0.4.1 candidate: `xbreed team status` detects settings.json pinning routing-output model ids (and warns the user).
- **F6** — **CANCELLED.** Pinning customtools in xbreed was a category error; the CLI handles it.

## Gemini CLI substrate findings (worth v0.5 scope)

From `gemini-research`'s docs dive (github.com/google-gemini/gemini-cli):

- **Settings precedence:** command-line args (layer 7) > env vars (layers 6–5) > settings files (layers 4–1). xbreed's `-m` override is the right layer.
- **`modelConfigs.overrides` system** — per-scope model config in settings.json with `match` conditions. The user's settings.json already has a `godspeed_explorer` scope skeleton. This could be a v0.5 integration point for xbreed to emit per-teammate model overrides.
- **Gemini CLI has native hooks, skills, extensions subcommand surfaces** — parallel to Claude Code. `gemini hooks migrate` literally migrates Claude Code hooks into gemini format. xbreed could ship a bi-directional sync.
- **xbreed's `~/.agents/skills/` is the SAME dir gemini-cli reads skills from.** `gemini skills list` shows `godspeed`, `godspeed-team`, `agent-browser` all enabled. Shared ecosystem — unintentional but useful. Worth formalizing as a first-class integration surface.
- **Gemini CLI has a `--policy` / `--admin-policy` flag + policy engine** (`--allowed-tools` is deprecated in favor of it). Native tool-scope enforcement on the gemini side. xbreed could bridge to this instead of reinventing posture frontmatter on Claude Code's side.

## Links

- [[2026-04-11-v0.4.0-posture-retcon-walk]] — the walk that first hit the 404 with the wrong framing (Vertex AI guess superseded here)
- [[2026-04-11-v0.3.5-gemini-oauth-cascade]] — the cascade that now has a documented routing-aware justification
- [[2026-04-11-next-frontier-walk]] — where customtools was first flagged as v0.3.7 candidate; the claim "superior for orchestrator-executor dynamics" is now corroborated by openrouter's product description
