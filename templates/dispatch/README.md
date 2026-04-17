# Dispatch Templates

Model-specific prompt templates for mediator agents dispatching tasks via `xbreed ask`.

## How to Use

1. Pick the template for your target model: `gemini.txt`, `codex.txt`, or `claude.txt`.
2. Replace `{{QUERY}}` with the actual task or question.
3. Replace `{{CONTEXT}}` with any prior findings, facts, or state the recipient needs. Leave empty if none.
4. Prepend the filled template to your `xbreed ask <model>` invocation.

## Template Design

Each template encodes the recipient model's preferred communication substrate, per the model-language debate findings and Inter-Model Communication Protocol v0.1:

| Template | Serialization | Rationale |
|---|---|---|
| `gemini.txt` | YAML backbone, single payload | Gemini's 1M context enables whole-state sync; YAML eliminates renegotiation turns |
| `codex.txt` | Typed-block protocol (the spec itself) | Codex routes attention by section header; structure degrades less than convention under pipeline stress |
| `claude.txt` | Typed prose with inline epistemics | Claude's cognition peaks in sequential token generation; per-assertion metadata replaces hedging |

All three instruct the recipient to return in protocol v0.1 format, ensuring mediator-parseable output regardless of which model produced it.

## Token Budget

Each template is under 500 tokens. Keep fills lean — pass only facts the recipient cannot derive.

## References

- Protocol spec: `docs/superpowers/specs/2026-04-12-inter-model-protocol-v0.1.md`
- Debate findings: `~/claudevault/Projects/the-crossbreeder/2026-04-12-model-language-debate.md`
