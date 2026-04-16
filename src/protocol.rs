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
    fn parse_sections(doc: &str) -> Vec<(String, usize)> {
        let mut sections: Vec<(String, usize)> = Vec::new();
        let mut current_heading: Option<String> = None;
        let mut body_count: usize = 0;

        for line in doc.lines() {
            if let Some(title) = line.strip_prefix("## ") {
                if let Some(h) = current_heading.take() {
                    sections.push((h, body_count));
                }
                current_heading = Some(title.trim().to_string());
                body_count = 0;
            } else if current_heading.is_some() && !line.trim().is_empty() {
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
