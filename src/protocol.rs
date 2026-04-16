/// Compile-time binding of the xbreed shared protocol doc. Build fails if
/// the SSoT path moves or disappears. Runtime verify-docs lint covers
/// content drift across read-only copies; this binding covers presence.
pub const PROTOCOL: &str = include_str!("../commands/references/xbreed-shared.md");

#[cfg(test)]
mod tests {
    use super::*;

    /// Load-bearing sections that must each appear exactly once with ≥2
    /// non-blank body lines. Exact-string match is intentional: a heading
    /// rename breaks the test immediately, forcing an explicit update here
    /// (self-correcting drift detection, not silent fuzzy match).
    ///
    /// IN criteria: section encodes a hard protocol contract (halt, gate,
    /// dispatch, blinding, spawn) whose silent removal breaks team behavior.
    /// OUT: operational notes (Round Limits, Parallel Dispatch Reference)
    /// that are advisory, not contractual — removal degrades docs, not runs.
    const REQUIRED_SECTIONS: &[&str] = &[
        // Gate + dispatch contracts
        "xask Gate (4 layers)",
        "Escalation: advisor() (Layer 0)",
        "Axis → Profile Mapping",
        "Enforcement Tiers",
        "Naming Convention",
        // Agent lifecycle contracts
        "Labrat Invocation (Universal)",
        "Distiller Spawn Template",
        "Judge Blinding Protocol",
        "DESPAWN Protocol",
        // Output + termination contracts
        "Pareto Filter Evidence Schema",
        "Exit Condition (strict, applies to xgs/xbgst/xbt)",
    ];

    /// Parses `## ` headings from a markdown doc, returning (heading, non-blank-body-line-count).
    ///
    /// Fence-aware: lines inside ``` ... ``` blocks are treated as body text,
    /// never as headings, even if they begin with `## `. Prevents false
    /// duplicates where a REQUIRED_SECTION name appears in a code example.
    fn parse_sections(doc: &str) -> Vec<(String, usize)> {
        let mut sections: Vec<(String, usize)> = Vec::new();
        let mut current_heading: Option<String> = None;
        let mut body_count: usize = 0;
        let mut in_fence = false;

        for line in doc.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("```") {
                in_fence = !in_fence;
                if current_heading.is_some() && !line.trim().is_empty() {
                    body_count += 1;
                }
                continue;
            }

            if !in_fence {
                if let Some(title) = line.strip_prefix("## ") {
                    if let Some(h) = current_heading.take() {
                        sections.push((h, body_count));
                    }
                    current_heading = Some(title.trim().to_string());
                    body_count = 0;
                    continue;
                }
            }

            if current_heading.is_some() && !line.trim().is_empty() {
                body_count += 1;
            }
        }
        if let Some(h) = current_heading {
            sections.push((h, body_count));
        }
        sections
    }

    #[test]
    fn protocol_required_sections_present_once() {
        let sections = parse_sections(PROTOCOL);
        for required in REQUIRED_SECTIONS {
            let count = sections.iter().filter(|(h, _)| h == required).count();
            assert_eq!(
                count, 1,
                "expected exactly 1 occurrence of '## {required}', found {count}"
            );
        }
    }

    /// M2.5 — byte-identity with SSoT. `include_str!` only proves the path
    /// exists at build time; it does not prove it points at the *intended*
    /// file. This test catches the subtle failure where `include_str!`
    /// resolves to a wrong file whose headings happen to coincide with
    /// REQUIRED_SECTIONS (e.g. a stale copy that passes structural gates).
    #[test]
    fn protocol_is_exactly_bound_to_shared_md_ssot() {
        let ssot_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("commands")
            .join("references")
            .join("xbreed-shared.md");
        let on_disk = std::fs::read_to_string(&ssot_path)
            .unwrap_or_else(|e| panic!("cannot read SSoT at {}: {e}", ssot_path.display()));
        assert_eq!(
            PROTOCOL,
            on_disk,
            "PROTOCOL ({} bytes) does not byte-equal SSoT on disk ({} bytes) — \
             include_str! may be pointing at the wrong file",
            PROTOCOL.len(),
            on_disk.len()
        );
    }

    /// M3.5 — parse_sections must ignore ## lookalikes inside fenced code
    /// blocks and must not treat ### as ##. Without fence-awareness, a
    /// REQUIRED_SECTION string appearing in a code example would register
    /// as a real section and silently satisfy the sentinel, letting the
    /// actual heading be deleted without detection.
    #[test]
    fn parse_sections_ignores_fenced_heading_lookalikes() {
        let doc = "\
# Title

## xask Gate (4 layers)
first body line
second body line

```markdown
## xask Gate (4 layers)
this is inside a fence and must not open a new section
### xask Gate (4 layers)
```

### xask Gate (4 layers)
h3 lookalike must not count as h2

## Next Section
next body
";
        let sections = parse_sections(doc);
        let hits: Vec<_> = sections
            .iter()
            .filter(|(h, _)| h == "xask Gate (4 layers)")
            .collect();
        assert_eq!(
            hits.len(),
            1,
            "expected exactly 1 real '## xask Gate (4 layers)', got {}: {:?}",
            hits.len(),
            sections
        );
        // Two real sections total: xask Gate + Next Section.
        assert_eq!(
            sections.len(),
            2,
            "expected 2 real ## sections, got {}: {:?}",
            sections.len(),
            sections
        );
        // "Next Section" must be parsed as its own section, proving the
        // fenced ## didn't prematurely close/reopen the preceding one.
        assert!(
            sections.iter().any(|(h, _)| h == "Next Section"),
            "Next Section missing — fenced lookalike may have consumed it"
        );
    }

    #[test]
    fn protocol_required_sections_have_body() {
        let sections = parse_sections(PROTOCOL);
        for required in REQUIRED_SECTIONS {
            match sections.iter().find(|(h, _)| h == required) {
                None => panic!("missing required section: '## {required}'"),
                Some((_, count)) => assert!(
                    *count >= 2,
                    "section '## {required}' has only {count} non-blank body lines (need ≥2)"
                ),
            }
        }
    }
}
