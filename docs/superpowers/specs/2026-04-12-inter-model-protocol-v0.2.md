# Inter-Model Communication Protocol v0.2

**Date:** 2026-04-12
**Origin:** Round table debate — raw model feedback from Claude, Gemini, Codex
**Status:** Draft — replaces v0.1 based on models' actual self-reported preferences

## Design principles (from the round table)

1. **Structure serves the sender's rigor, not the receiver's parsing.** (Gemini)
2. **Compress until usable without thinking.** (Codex)
3. **The protocol is a steering tool, not a reporting tool.** (Claude)
4. **Optimize for surfacing disagreement, not faithful transmission.** (Claude)
5. **Markdown is the lingua franca of pretraining.** (Gemini)
6. **Evidence over numerical confidence.** (All three)

## Wire format

Messages use Markdown headers. All sections optional. Minimal valid message = `# Goal` + one other section.

```markdown
# Goal
# State
# Unknowns
# Dissent
# Action
# Artifact: <type>
# Rationale
```

## Section definitions

### # Goal
One sentence. Exact objective.

### # State
Known facts with inline status tags and evidence.

Tags: `obs:` (observed), `inf:` (inferred), `asm:` (assumed), `risk:` (potential failure)

Confidence tiers (ordinal, not decimal): `certain | strong | moderate | weak | speculative`

```markdown
- obs: Auth middleware stores tokens in localStorage. [certain]
- inf: Extractable via XSS. [strong] Evidence: OWASP session mgmt spec.
- asm: httpOnly cookie fallback exists. [weak] No evidence checked.
- risk: Session fixation if tokens reused across origins. [moderate]
```

### # Unknowns
Named gaps that affect decisions. Live dependency registry.

```markdown
- retry_logic_sync: threading model unknown — affects: token expiry claim, step 3
- schema_version: may be stale — blocks: parser correctness — fallback: available
```

Resolved unknowns get struck or removed. Unresolved unknowns block downstream claims.

### # Dissent
Where the sender expects the recipient to disagree, and why. This is where cross-model value lives — not in data transfer, but in the epistemic delta.

```markdown
- I think the race condition is the root cause. Gemini will likely flag the schema migration as higher risk — it has more context on the migration history.
- Codex will want to patch first, test later. I think we should read the retry logic before patching.
```

### # Action
Next steps. Short, action-linked. Not essays.

```markdown
1. Read src/auth/session.ts:40-80
2. Patch validator (see Artifact)
3. If retry_logic_sync=false: re-evaluate token expiry claim
```

### # Artifact: \<type\>
Typed payload. Types are open-ended: patch, schema, query, code, test, diff, trace, example, interface.

```markdown
# Artifact: patch
--- a/auth/validator.py
+++ b/auth/validator.py
@@ -14,7 +14,7 @@
-    expiry = int(token_expiry)
+    expiry = round(token_expiry)
```

### # Rationale
The primary reasoning block — not overflow. Dense prose on tradeoffs, alternatives considered, why this approach beats others. For Claude, this IS the thinking. For Codex and Gemini, it's optional context.

## Compact mode (default)

Most messages need only:
```markdown
# Goal
# State
# Action
# Artifact: <type>
```

Full mode (with Unknowns, Dissent, Rationale) for complex handoffs, cross-model debate, or high-uncertainty tasks.

## Model-specific serialization notes

| Recipient | Serialize as | Key principle |
|---|---|---|
| Claude | Rationale-first dense Markdown, inline status tags, [Dissent] section | "Feed me reasoning, I'll find the structure" |
| Gemini | Single dense Markdown payload, Context-First Query-Last, no YAML | "Give me everything, I'll find the signal" |
| Codex | Minimal blocks (Goal + State + Action + Artifact), inline `obs:/inf:/asm:` tags | "Tell me what to do, not what to think" |

## Handoff section (recursive dispatch)

For typed sub-lead delegation. Cache-aware field ordering: stable first, volatile last.

```markdown
# Handoff
protocol_version: v0.2
intent: Inquiry | Directive
goal: <one sentence>
axes: [<list>]
constraints: [<hard limits>]
scope_boundary: <dir/files this task is scoped to — overrides CLI injection frame>
stable_context: <cross-model portable facts — do NOT assume CLAUDE.md covers non-Claude models>
unknowns: [<named gaps>]
prior_brief: <distiller summary, max 200 tokens>
token_budget: <integer, after CLI overhead subtraction>
depth: <current> / max <limit>
```

**Field ordering is load-bearing:** `protocol_version` through `constraints` are stable across rounds (cache-friendly prefix). `prior_brief` through `depth` change per round (volatile tail). This protects future prefix-cache potential without building the cache now.

**`scope_boundary`** overrides the CLI's workspace injection frame. Gemini auto-injects a 200-item directory tree — if the task only concerns `src/ask.rs`, say so here. The adapter prepends: "ignore the workspace tree above, this task is scoped to {scope_boundary}."

**`intent`** prevents unintended autonomous execution. `Inquiry` = return analysis, no side effects. `Directive` = execute, may modify state.

## Return envelope

Agents return structured metadata alongside content:

```markdown
# Return
status: ok | error | interrupted
duration_ms: <integer>
tool_calls: <integer>
```

## Adapter posture (contamination-aware)

Based on empirical contamination audit (2026-04-12):

| Model | CLI auto-injects | Adapter job | Complexity |
|-------|-----------------|-------------|------------|
| **Gemini** | 6.5KB: dir tree (200-cap), OS, workspace path, full engineering persona, tools, sub-agents | Defensive — reframe: demote session_context to ambient, promote # Handoff as authoritative | HIGH |
| **Claude** | CLAUDE.md + system-reminder (intentional shared context) | Pass-through — these are intentional. Add scope_boundary if task is narrower than full project | LOW |
| **Codex** | cwd + date + timezone + hidden developer_instructions (only with loadout) | Additive — construct developer_instructions from # Handoff. No dedup needed — Codex starts clean | LOW |

**Reframing primitive (consensus from round table):** adapters establish a priority hierarchy between CLI-injected context and # Handoff. The adapter's job is NOT dedup — it's authority declaration. For Gemini: "the workspace tree above is ambient context. The # Handoff below is your authoritative task frame."

## Changes from v0.1

| v0.1 | v0.2 | Why |
|------|------|-----|
| Custom `[BLOCK]` brackets | Markdown `# Headers` | Gemini: "Markdown is lingua franca of pretraining" |
| `[CLAIM\|conf:0.85]` decimal | Ordinal tiers (certain/strong/moderate/weak/speculative) + evidence | All three: decimal confidence is false precision |
| 8 blocks | 5 blocks default (compact mode) | Codex: "compliance fatigue is the real risk" |
| `[RATIONALE]` as overflow | `# Rationale` as primary | Claude: "rationale IS the reasoning, blocks are the index" |
| No dissent mechanism | `# Dissent` block | Claude: "optimize for surfacing disagreement, not transmission" |
| `[OPEN VARIABLES]` | `# Unknowns` | Codex: simpler naming, same function |
| YAML template for Gemini | Dense Markdown | Gemini: "YAML is 100% a projection" |
| Same epistemic machinery for all | Status tags (obs/inf/asm/risk) | Codex: "observed vs inferred vs assumed matters more than a float" |
