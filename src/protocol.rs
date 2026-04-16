/// Compile-time binding of the xbreed shared protocol doc. Build fails if
/// the SSoT path moves or disappears. Runtime verify-docs lint covers
/// content drift across read-only copies; this binding covers presence.
pub const PROTOCOL: &str = include_str!("../commands/references/xbreed-shared.md");

#[cfg(test)]
mod tests {
    use super::*;

    fn headings() -> Vec<&'static str> {
        PROTOCOL.lines().filter(|l| l.starts_with("## ")).collect()
    }

    // Known limitation (R4 hardening item): these tests assert heading *presence*,
    // not section body non-emptiness. A gutted section keeps the heading → FP still
    // possible. Fix: per-section min-body-line checks or pulldown-cmark parse.

    #[test]
    fn protocol_has_axis_profile_section() {
        assert!(
            headings().contains(&"## Axis → Profile Mapping"),
            "missing '## Axis → Profile Mapping' heading"
        );
    }

    #[test]
    fn protocol_has_xask_gate_section() {
        assert!(
            headings().contains(&"## xask Gate (4 layers)"),
            "missing '## xask Gate (4 layers)' heading"
        );
    }

    #[test]
    fn protocol_has_pareto_evidence_section() {
        assert!(
            headings().contains(&"## Pareto Filter Evidence Schema"),
            "missing '## Pareto Filter Evidence Schema' heading"
        );
    }

    #[test]
    fn protocol_has_minimum_section_count() {
        assert!(
            headings().len() >= 10,
            "expected ≥10 top-level sections, got {}",
            headings().len()
        );
    }
}
