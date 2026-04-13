use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;

/// A resolved loadout of one or more skills, ready to be injected into a target CLI.
#[derive(Debug, Default, Clone)]
pub struct Loadout {
    /// (skill name, file contents) pairs, in the order the user requested.
    entries: Vec<(String, String)>,
}

impl Loadout {
    /// Empty loadout — injection into a CLI should be a no-op.
    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Resolve skill names using the default search path:
    ///   1. $HOME/.agents/skills/<name>/SKILL.md
    ///   2. $HOME/.claude/skills/<name>/SKILL.md
    ///   3. $HOME/.config/xbreed/skills/<name>/SKILL.md
    pub fn resolve(names: &[String]) -> Result<Self> {
        let home = std::env::var("HOME").context("HOME is not set")?;
        let search_dirs = [
            PathBuf::from(format!("{home}/.agents/skills")),
            PathBuf::from(format!("{home}/.claude/skills")),
            PathBuf::from(format!("{home}/.config/xbreed/skills")),
        ];
        Self::resolve_with_paths(names, &search_dirs)
    }

    /// Resolve skill names using explicit search directories.
    /// Exposed for unit tests; production callers should use `resolve`.
    pub fn resolve_with_paths(names: &[String], search_dirs: &[PathBuf]) -> Result<Self> {
        let mut entries = Vec::with_capacity(names.len());
        for name in names {
            let path = find_skill(name, search_dirs).ok_or_else(|| {
                let attempted = search_dirs
                    .iter()
                    .map(|d| format!("  {}/{}/SKILL.md", d.display(), name))
                    .collect::<Vec<_>>()
                    .join("\n");
                anyhow!("skill not found: {name}\nsearched:\n{attempted}")
            })?;
            let body = std::fs::read_to_string(&path)
                .with_context(|| format!("failed to read skill {}: {}", name, path.display()))?;
            entries.push((name.clone(), body));
        }
        Ok(Self { entries })
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Render the full concat string. Empty loadout renders as an empty string.
    pub fn to_concat(&self) -> String {
        if self.entries.is_empty() {
            return String::new();
        }
        let names: Vec<&str> = self.entries.iter().map(|(n, _)| n.as_str()).collect();
        let mut out = format!("# xbreed loadout: {}\n\n", names.join(", "));
        for (i, (name, body)) in self.entries.iter().enumerate() {
            if i > 0 {
                out.push_str("\n---\n\n");
            }
            out.push_str(&format!("## {name}\n\n{body}"));
            if !body.ends_with('\n') {
                out.push('\n');
            }
        }
        out
    }
}

fn find_skill(name: &str, search_dirs: &[PathBuf]) -> Option<PathBuf> {
    for dir in search_dirs {
        let candidate = dir.join(name).join("SKILL.md");
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    fn write_skill(root: &Path, name: &str, body: &str) -> PathBuf {
        let dir = root.join(name);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("SKILL.md");
        fs::write(&path, body).unwrap();
        path
    }

    #[test]
    fn empty_loadout_renders_empty_string() {
        let l = Loadout::empty();
        assert!(l.is_empty());
        assert_eq!(l.to_concat(), "");
    }

    #[test]
    fn resolve_single_skill_from_first_dir() {
        let tmp = tempdir().unwrap();
        let dir_a = tmp.path().join("a");
        fs::create_dir_all(&dir_a).unwrap();
        write_skill(&dir_a, "godspeed", "GO FAST");

        let l = Loadout::resolve_with_paths(
            &["godspeed".to_string()],
            &[dir_a.clone(), tmp.path().join("b")],
        )
        .unwrap();

        let c = l.to_concat();
        assert!(c.contains("# xbreed loadout: godspeed"));
        assert!(c.contains("## godspeed"));
        assert!(c.contains("GO FAST"));
    }

    #[test]
    fn resolve_fallback_to_second_dir() {
        let tmp = tempdir().unwrap();
        let dir_a = tmp.path().join("a");
        let dir_b = tmp.path().join("b");
        fs::create_dir_all(&dir_a).unwrap();
        write_skill(&dir_b, "the-librarian", "curate");

        let l = Loadout::resolve_with_paths(
            &["the-librarian".to_string()],
            &[dir_a.clone(), dir_b.clone()],
        )
        .unwrap();
        assert!(l.to_concat().contains("curate"));
    }

    #[test]
    fn resolve_missing_returns_err_listing_attempted_paths() {
        let tmp = tempdir().unwrap();
        let err = Loadout::resolve_with_paths(
            &["nope".to_string()],
            &[tmp.path().join("a"), tmp.path().join("b")],
        )
        .unwrap_err();
        let msg = format!("{err:#}");
        assert!(msg.contains("skill not found: nope"));
        assert!(msg.contains("a/nope/SKILL.md"));
        assert!(msg.contains("b/nope/SKILL.md"));
    }

    #[test]
    fn concat_preserves_argument_order_and_has_separators() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("skills");
        write_skill(&dir, "alpha", "A-body");
        write_skill(&dir, "beta", "B-body");

        let l = Loadout::resolve_with_paths(&["beta".to_string(), "alpha".to_string()], &[dir])
            .unwrap();

        let c = l.to_concat();
        let beta_idx = c.find("## beta").unwrap();
        let alpha_idx = c.find("## alpha").unwrap();
        assert!(beta_idx < alpha_idx, "expected beta before alpha in concat");
        assert!(c.contains("\n---\n"), "expected separator between skills");
    }

    #[test]
    fn first_dir_wins_over_second() {
        let tmp = tempdir().unwrap();
        let dir_a = tmp.path().join("a");
        let dir_b = tmp.path().join("b");
        write_skill(&dir_a, "godspeed", "FROM A");
        write_skill(&dir_b, "godspeed", "FROM B");

        let l = Loadout::resolve_with_paths(&["godspeed".to_string()], &[dir_a, dir_b]).unwrap();
        let c = l.to_concat();
        assert!(c.contains("FROM A"));
        assert!(!c.contains("FROM B"));
    }
}
