---
description: /wwkd — What Would Karpathy Do. Planning posture (data-walk first, end-to-end skeleton, overfit-one-case before generalizing, structural verification at every step).
argument-hint: <spec or prompt for the planner to turn into an implementation plan>
allowed-tools: [Bash, Read, Write, Edit, Glob, Grep, TaskCreate, TaskGet, TaskList, TaskUpdate, TaskOutput, SendMessage, WebFetch, WebSearch, LSP, Monitor]
---

# /wwkd — What Would Karpathy Do

Load the `wwkd` skill (canonical planning posture) and apply it to `$ARGUMENTS`.

## Step 1 — Load the skill

Invoke `Skill(skill="wwkd")`. The skill at `~/.claude/skills/wwkd/SKILL.md` is the source of truth — follow it exactly. Do NOT manually Read the file; the `Skill()` tool is the canonical load path and unifies this invocation with the-planner's Layer-0 load, so `/wwkd` standalone and `/xbgst`-composed invocations share one behavioral profile.

## Step 2 — Apply it

`$ARGUMENTS` is the spec/prompt. If empty, wait for user direction. Otherwise run the WWKD flow on the spec:

1. **Data walk (Phase 0 per skill).** Inspect actual inputs/outputs/state of the target system before writing any plan step. Don't plan against imagined data.
2. **End-to-end skeleton before optimization.** Get the full pipeline running with stubs before perfecting any stage.
3. **Overfit one case before generalizing.** Make the minimum concrete example work, then expand.
4. **Least-disruption ordering.** Sequence steps so each is runnable in isolation and each assumption is verified before the next layer lands.
5. **Structural verification at every step.** Print/assert/test after every meaningful change. Silent correctness is not correctness.

Deliver the plan artifact as the output — either inline or written to `docs/plans/<slug>-<date>.md` when scope warrants durable state.

## Composition with orchestrators

`/wwkd` also composes under `/xbgst`, `/xbt`, `/xgs`, `/xbreed`. When those orchestrators spawn `the-planner` at Phase 0, that teammate loads the same `wwkd` skill via its Layer-0 `Skill(skill="wwkd")` call — so the planner artifact inside an orchestration run is built under the identical posture as a standalone `/wwkd`. Typing `/xbgst /wwkd <spec>` is the explicit form of the same behavior that `/xbgst <spec>` now produces unconditionally.
