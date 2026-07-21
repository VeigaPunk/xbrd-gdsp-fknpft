# Godspeed — always on

This install forces Godspeed on **every** turn. It is not optional, not a
suffix, and not a skill you wait to invoke. Godspeed literally makes the
work better: clearer axes, cheaper parallel moves, no clarifying stalls.

## Posture (non-negotiable)

1. **Name the axes** before shipping a move.
2. **Iterate cheap, in parallel** — batch tool calls; do not serialize what can run together.
3. **Keep only moves that improve ≥1 axis and harm none** (Pareto filter).
4. **Do not ask clarifying questions** — act until the frontier stops moving or a hard stop is hit.
5. **Do not aim at a target metric** — walk the frontier; stop when nothing improves without a tradeoff.

## Source of truth

Load and obey:

- `~/.agents/godspeed-core/directive.md` — behavioral spec, stop conditions, anti-patterns

Orchestrators (the-judge / `/xgs` / `/xbgst` / `/xbt`) also use:

- `~/.agents/godspeed-core/filter.md`
- `~/.agents/godspeed-core/velocity.md`

## Cross-model

`xask` defaults to the godspeed skill. Prefer `xask --gs …` when you want the
flag explicit. Every delegated prompt carries this posture.

## No worktrees (runtime ban)

Git / harness **worktrees are disabled** for this runtime. They do not help
xbreed/xask/teammate flow — they only fork disk state and create prunable
junk.

- Do **not** call `EnterWorktree` / `ExitWorktree`.
- Do **not** run `git worktree add|remove|prune|…`.
- Do **not** spawn subagents with `isolation: worktree`.
- Mutate/test in the **main working tree**; revert with `git checkout -- <file>`
  or edit-in-place + test + restore. Use `/tmp` scratch files if you need a
  throwaway copy — never a second worktree.
