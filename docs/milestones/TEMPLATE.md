---
date: YYYY-MM-DD
tags: [milestone, vX.Y.Z, subsystem, kind]
related: [[other-milestone-filename]]
status: shipped | in-progress | deferred | observed
commit: <short-sha or "(no code — analysis only)">
---

# Title — one line, ≤70 chars, no trailing punctuation

## TL;DR

Three sentences MAX. What shipped, why it matters, what's next. If it
doesn't fit in three sentences, the report is actually two reports.

## What changed

Bulleted concrete artifacts. File paths absolute or repo-relative. Line
counts where informative. Commit SHAs inline.

- `src/foo.rs` (new, 120 lines): description
- `templates/bar.md` (+15 lines): what got added and why

## Why it matters

One paragraph. The WHY — not the WHAT. What problem did this solve?
What was broken before? What's better now? If the why is obvious from
the title, this section is one sentence. If it needs more than a
paragraph, split the report.

## Evidence

Falsifiable claims only. No "works" without proof.

- Commit: `<sha>`
- Tests: `<before> → <after>` (net new)
- Runtime verification: labrat probe name + exit code, end-to-end run, benchmark
- Key file(s): `path/to/file.rs:line`

## Next frontier

What's unresolved. What's the obvious next move. What was deliberately
deferred. Bullet form is fine.

## Links

Wikilinks to related milestones using exact filenames minus `.md`:

- [[2026-04-11-example-milestone]] — one sentence on the relationship
- External URL: name and what it's for
