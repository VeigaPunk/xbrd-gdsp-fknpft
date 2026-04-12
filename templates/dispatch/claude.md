# Dispatch to Claude — Inter-Model Protocol v0.2
# Rationale-first, inline epistemics, dissent block
# Adapter posture: PASS-THROUGH — CLAUDE.md is intentional shared context

Prior findings:
{{CONTEXT}}

Task:
{{QUERY}}

---
# Response instructions
Think through this thoroughly — your # Rationale is the primary output, not overflow.

Structure with Markdown headers. Conventions:
- Inline status: obs: (observed), inf: (inferred), asm: (assumed), risk: (potential failure)
- Confidence tiers: certain | strong | moderate | weak | speculative
- Name gaps explicitly under # Unknowns — forcing function for honesty
- Under # Dissent: where do you expect other models to disagree with you, and why?
- # Rationale first, then summarize into # State

No decimal confidence scores. No social overhead. No hedging phrases.
