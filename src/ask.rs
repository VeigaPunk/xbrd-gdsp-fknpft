use crate::loadout::Loadout;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Two-slot holder for Gemini API keys loaded from `.env.local`.
///
/// Kept as distinct slots (not a flattened Vec) so that an empty primary
/// value does NOT silently promote the fallback to primary — the retry path
/// preserves the user's declared primary/fallback intent.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GeminiKeys {
    pub primary: Option<String>,
    pub fallback: Option<String>,
}


/// Reads `.env.local` from cwd.
pub fn load_gemini_keys() -> GeminiKeys {
    load_gemini_keys_from(Path::new(".env.local"))
}

/// Reads a `.env.local`-style file and extracts GEMINI_API_KEY /
/// GEMINI_API_KEY_FALLBACK. Parser behavior:
/// - Strips UTF-8 BOM at start of file
/// - Skips blank lines and full-line comments (`# ...`)
/// - Splits each line on the first `=`, trims whitespace from both sides
///   (handles `KEY=value`, `KEY =value`, `KEY= value`, `KEY = value`)
/// - Strips matched single- or double-quotes around the value
/// - Strips inline comments (first `#` preceded by whitespace in unquoted values)
/// - Discards empty values (`KEY=` or `KEY=""`) — leaves the slot as None
pub fn load_gemini_keys_from(path: &Path) -> GeminiKeys {
    let Ok(content) = std::fs::read_to_string(path) else {
        return GeminiKeys::default();
    };
    let content = content.strip_prefix('\u{FEFF}').unwrap_or(&content);
    let mut out = GeminiKeys::default();
    for line in content.lines() {
        let Some((key, value)) = parse_env_line(line) else {
            continue;
        };
        if value.is_empty() {
            continue;
        }
        match key.as_str() {
            "GEMINI_API_KEY" => out.primary = Some(value),
            "GEMINI_API_KEY_FALLBACK" => out.fallback = Some(value),
            _ => {}
        }
    }
    out
}

fn parse_env_line(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim_start();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    let (raw_key, raw_value) = trimmed.split_once('=')?;
    let key = raw_key.trim().to_string();
    if key.is_empty() {
        return None;
    }
    Some((key, clean_env_value(raw_value)))
}

fn clean_env_value(raw: &str) -> String {
    let v = raw.trim();
    if let Some(inner) = v.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        return inner.to_string();
    }
    if let Some(inner) = v.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')) {
        return inner.to_string();
    }
    // Unquoted: strip inline comment (first `#` preceded by whitespace).
    let mut end = v.len();
    let mut prev_was_space = false;
    for (i, ch) in v.char_indices() {
        if prev_was_space && ch == '#' {
            end = i;
            break;
        }
        prev_was_space = ch.is_whitespace();
    }
    v[..end].trim().to_string()
}

pub fn build_claude_ask_with_loadout(prompt: &str, loadout: &Loadout) -> Command {
    let mut c = Command::new("claude");
    c.arg("-p").arg(prompt);
    if !loadout.is_empty() {
        c.arg("--append-system-prompt").arg(loadout.to_concat());
    }
    c
}

/// Build a codex Command with loadout injection. NOTE: does NOT append the
/// prompt — caller must append it AFTER any `-c` flags (effort, etc.) since
/// `codex exec` treats the prompt as a trailing positional arg.
pub fn build_codex_ask_with_loadout(loadout: &Loadout) -> Command {
    let mut c = Command::new("codex");
    c.arg("exec");
    if !loadout.is_empty() {
        // codex -c value is parsed as TOML. A JSON-serialized string (double-quoted,
        // with \n / \" / \\ escapes) is also a valid TOML basic string.
        let toml_quoted = serde_json::to_string(&loadout.to_concat())
            .expect("serde_json::to_string of a String never fails");
        c.arg("-c").arg(format!("developer_instructions={toml_quoted}"));
    }
    c
}

/// The gemini model xbreed pins for gemini calls. Use this as the INPUT id;
/// the gemini CLI handles the final model selection via its internal routing.
///
/// DO NOT pin `gemini-3.1-pro-preview-customtools` here — it is a routing
/// OUTPUT, not an input, and 404s on both OAuth and API-key paths when used
/// as input (`isVisible: false` in gemini-cli's `defaultModelConfigs.ts`).
/// The CLI's `getUseCustomToolModelSync()` silently routes this preview id
/// → customtools when `authType === AuthType.USE_GEMINI` (OAuth) AND the
/// account has Gemini 3.1 launched. Live-verified 2026-04-11 via a 4-probe
/// truth table (`gemini-research` + `gemini-probe` walk — see
/// docs/milestones/2026-04-11-customtools-routing-finding.md).
///
/// Consequence: the v0.3.5 OAuth-first cascade is already optimal for
/// customtools access. OAuth users with Gemini 3.1 launched automatically
/// get customtools via routing; API-key fallback users get base preview
/// (still functional, loses the tool-selection optimizations). No xbreed
/// change needed to reach customtools.
pub const GEMINI_DEFAULT_MODEL: &str = "gemini-3.1-pro-preview";

// ========================================================================
// v0.3.5 — Gemini auth cascade with multi-profile OAuth
// ========================================================================

/// Auth method for a single gemini dispatch attempt.
///
/// The cascade in `dispatch()` tries these in order (OAuth-first by design):
///
///   1. `OAuthProfile("primary")`   — HOME override to
///      `~/.config/xbreed/gemini-profiles/primary/` so gemini reads that
///      profile's `.gemini/oauth_creds.json` instead of the user's default
///   2. `OAuthProfile("fallback")`  — same mechanism, fallback profile
///   3. `OAuthDefault`              — user's real HOME, no API key env
///      injection; gemini uses `~/.gemini/oauth_creds.json`
///   4. `ApiKey(<primary>)`         — `GEMINI_API_KEY` from `.env.local`
///   5. `ApiKey(<fallback>)`        — `GEMINI_API_KEY_FALLBACK` from `.env.local`
///
/// OAuth is preferred because subscription-gated model variants (e.g. the
/// `-customtools` preview) require an OAuth session, not an API key, and
/// free-tier API keys have stricter per-account QPS limits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeminiAuth {
    /// Named OAuth profile. xbreed overrides HOME to
    /// `~/.config/xbreed/gemini-profiles/<name>/` so gemini CLI reads the
    /// profile's `.gemini/oauth_creds.json` file.
    OAuthProfile(String),
    /// Default OAuth from the user's real `~/.gemini/oauth_creds.json`.
    /// Does NOT override HOME; strips any `GEMINI_API_KEY` env injection.
    OAuthDefault,
    /// API key injected via the `GEMINI_API_KEY` env var on the subprocess.
    ApiKey(String),
}

impl GeminiAuth {
    /// Short human-readable label for logging / error messages.
    pub fn label(&self) -> String {
        match self {
            Self::OAuthProfile(name) => format!("oauth:{name}"),
            Self::OAuthDefault => "oauth:default".to_string(),
            Self::ApiKey(_) => "api-key".to_string(),
        }
    }
}

/// Root directory for named OAuth profiles. Each profile is a subdirectory
/// that contains a `.gemini/` subdir with `oauth_creds.json` + `settings.json`.
/// xbreed spawns gemini with `HOME` pointing at `<root>/<profile>/`.
pub fn gemini_profiles_root() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home)
            .join(".config")
            .join("xbreed")
            .join("gemini-profiles")
    } else {
        PathBuf::from(".xbreed/gemini-profiles")
    }
}

/// Check whether a profile directory has a populated oauth_creds.json.
pub fn gemini_profile_exists(name: &str) -> bool {
    gemini_profiles_root()
        .join(name)
        .join(".gemini")
        .join("oauth_creds.json")
        .exists()
}

/// Check whether the user's default `~/.gemini/` has an oauth_creds.json file.
fn default_gemini_oauth_exists() -> bool {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home)
            .join(".gemini")
            .join("oauth_creds.json")
            .exists()
    } else {
        false
    }
}

/// Build the ordered cascade chain of auth methods to try.
///
/// Returns only auth methods that plausibly *could* work based on disk +
/// `.env.local` state. Iterates: named OAuth profiles (primary, fallback)
/// → default OAuth → API key primary → API key fallback.
pub fn gemini_auth_chain() -> Vec<GeminiAuth> {
    let mut chain = Vec::new();

    // OAuth profiles — check each named profile dir in priority order
    for name in ["primary", "fallback"] {
        if gemini_profile_exists(name) {
            chain.push(GeminiAuth::OAuthProfile(name.to_string()));
        }
    }

    // Default OAuth — user's real ~/.gemini/
    if default_gemini_oauth_exists() {
        chain.push(GeminiAuth::OAuthDefault);
    }

    // API keys — last-resort fallback
    let keys = load_gemini_keys();
    if let Some(primary) = keys.primary {
        chain.push(GeminiAuth::ApiKey(primary));
    }
    if let Some(fallback) = keys.fallback {
        chain.push(GeminiAuth::ApiKey(fallback));
    }

    chain
}

/// Build a gemini Command configured for a specific auth method.
///
/// The env manipulation per variant:
/// - `OAuthProfile(name)`: sets `HOME=<profile-dir>`, env_removes `GEMINI_API_KEY`
/// - `OAuthDefault`: env_removes `GEMINI_API_KEY` only (inherits real HOME)
/// - `ApiKey(k)`: sets `GEMINI_API_KEY=k` on the subprocess
pub fn build_gemini_with_auth(prompt: &str, loadout: &Loadout, auth: &GeminiAuth) -> Command {
    let mut c = Command::new("gemini");
    let final_prompt = if loadout.is_empty() {
        prompt.to_string()
    } else {
        format!("{}\n\n---\n\n{}", loadout.to_concat(), prompt)
    };
    c.arg("-m").arg(GEMINI_DEFAULT_MODEL).arg("-p").arg(final_prompt);

    match auth {
        GeminiAuth::OAuthProfile(name) => {
            let profile_home = gemini_profiles_root().join(name);
            c.env("HOME", &profile_home);
            c.env_remove("GEMINI_API_KEY");
        }
        GeminiAuth::OAuthDefault => {
            c.env_remove("GEMINI_API_KEY");
        }
        GeminiAuth::ApiKey(key) => {
            c.env("GEMINI_API_KEY", key);
        }
    }
    c
}


/// Tightened quota detector. Matches specific, unambiguous rate-limit signals.
fn is_quota_error(stderr: &[u8]) -> bool {
    let s = String::from_utf8_lossy(stderr);
    s.contains("RESOURCE_EXHAUSTED")
        || s.contains("status: 429")
        || s.contains("HTTP 429")
        || s.contains("code: 429")
        || s.contains("Quota exceeded")
        || s.contains("rate limit exceeded")
}

/// Auth-failure detector. Distinct from quota exhaustion — triggers the
/// cascade advance in `dispatch()` for gemini and the auth-hint error
/// message for claude/codex.
fn is_auth_error(stderr: &[u8]) -> bool {
    let s = String::from_utf8_lossy(stderr);
    s.contains("401")
        || s.contains("403")
        || s.contains("PERMISSION_DENIED")
        || s.contains("API key not valid")
        || s.contains("API_KEY_INVALID")
        || s.contains("UNAUTHENTICATED")
        || s.contains("authentication failed")
        || s.contains("set an Auth method")
}

pub fn dispatch(cli: &str, prompt: &str, loadout: &Loadout, effort: Option<&str>) -> Result<String> {
    if cli == "gemini" {
        if effort.is_some() {
            eprintln!("warning: --effort is ignored for gemini (no native flag; use thinkingBudget in prompt template instead)");
        }
        let chain = gemini_auth_chain();
        if chain.is_empty() {
            anyhow::bail!(
                "gemini: no auth methods available. Set up at least one of:\n  \
                 - default OAuth:       run `gemini login` (populates ~/.gemini/oauth_creds.json)\n  \
                 - named OAuth profile: HOME=~/.config/xbreed/gemini-profiles/primary gemini login\n  \
                 - API key:             add GEMINI_API_KEY=<key> to .env.local at the repo root"
            );
        }

        let mut last_stderr: Vec<u8> = Vec::new();
        let mut last_code: Option<i32> = None;
        let mut last_label = String::new();

        for auth in &chain {
            let mut cmd = build_gemini_with_auth(prompt, loadout, auth);
            let output = cmd
                .output()
                .map_err(|e| anyhow::anyhow!("failed to execute gemini (auth={}): {e}", auth.label()))?;
            if output.status.success() {
                return Ok(String::from_utf8_lossy(&output.stdout).to_string());
            }
            last_stderr = output.stderr.clone();
            last_code = output.status.code();
            last_label = auth.label();

            // Cascade only on retriable errors (quota-class or auth-class).
            // Terminal errors (network, invalid prompt, malformed model arg)
            // bail immediately without wasting the remaining auth levels.
            if !is_quota_error(&output.stderr) && !is_auth_error(&output.stderr) {
                anyhow::bail!(
                    "gemini failed at auth={} with non-retriable error (exit {:?}): {}",
                    last_label,
                    last_code,
                    String::from_utf8_lossy(&last_stderr)
                );
            }
        }

        anyhow::bail!(
            "gemini failed at every auth level ({} tried, last={}, exit {:?}): {}\n\
             hint: verify ~/.gemini/oauth_creds.json, \
             ~/.config/xbreed/gemini-profiles/*/.gemini/oauth_creds.json, \
             or .env.local GEMINI_API_KEY values",
            chain.len(),
            last_label,
            last_code,
            String::from_utf8_lossy(&last_stderr)
        );
    }

    let mut cmd = match cli {
        "claude" => {
            let mut c = build_claude_ask_with_loadout(prompt, loadout);
            if let Some(e) = effort {
                c.arg("--effort").arg(e);
            }
            c
        }
        "codex" => {
            let mut c = build_codex_ask_with_loadout(loadout);
            if let Some(e) = effort {
                c.arg("-c").arg(format!("model_reasoning_effort={e}"));
            }
            // Prompt MUST be the last positional arg for codex exec —
            // all -c flags must come before it.
            c.arg(prompt);
            c
        }
        other => anyhow::bail!("unknown cli: {other} (expected claude|codex|gemini)"),
    };
    let output = cmd
        .output()
        .map_err(|e| anyhow::anyhow!("failed to execute {cli}: {e} (is it on PATH?)"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Auth-class errors get a clearer hint pointing to the CLI's own
        // credential chain. xbreed does not manage claude/codex OAuth —
        // those CLIs own their own auth (subscription login, etc.).
        if is_auth_error(stderr.as_bytes()) {
            let hint = match cli {
                "claude" => "run `claude login` to authenticate",
                "codex" => {
                    "run `codex login` to sign in with your ChatGPT Plus/Pro/Enterprise subscription or API key"
                }
                _ => "check CLI authentication",
            };
            anyhow::bail!("{cli}: authentication failed — {hint}\nstderr: {stderr}");
        }
        anyhow::bail!("{cli} failed (exit {:?}): {}", output.status.code(), stderr);
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmd_args(c: &Command) -> Vec<String> {
        c.get_args().map(|a| a.to_string_lossy().to_string()).collect()
    }

    fn loadout_with(body: &str) -> Loadout {
        use std::fs;
        use tempfile::tempdir;
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join("skills");
        fs::create_dir_all(&dir).unwrap();
        let skill_dir = dir.join("testskill");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(skill_dir.join("SKILL.md"), body).unwrap();
        let l = Loadout::resolve_with_paths(&["testskill".to_string()], &[dir]).unwrap();
        drop(tmp);
        l
    }

    fn write_env(body: &str) -> (tempfile::TempDir, std::path::PathBuf) {
        use std::fs;
        use tempfile::tempdir;
        let tmp = tempdir().unwrap();
        let p = tmp.path().join(".env.local");
        fs::write(&p, body).unwrap();
        (tmp, p)
    }

    #[test]
    fn claude_ask_empty_loadout_matches_v0_1_behavior() {
        let c = build_claude_ask_with_loadout("hello", &Loadout::empty());
        assert_eq!(c.get_program().to_string_lossy(), "claude");
        assert_eq!(cmd_args(&c), vec!["-p", "hello"]);
    }

    #[test]
    fn codex_ask_empty_loadout_matches_v0_1_behavior() {
        let mut c = build_codex_ask_with_loadout(&Loadout::empty());
        c.arg("hello"); // caller appends prompt after -c flags
        assert_eq!(c.get_program().to_string_lossy(), "codex");
        assert_eq!(cmd_args(&c), vec!["exec", "hello"]);
    }

    #[test]
    fn claude_ask_with_loadout_adds_append_system_prompt() {
        let l = loadout_with("BE FAST");
        let c = build_claude_ask_with_loadout("hello", &l);
        let args = cmd_args(&c);
        assert_eq!(args[0], "-p");
        assert_eq!(args[1], "hello");
        assert_eq!(args[2], "--append-system-prompt");
        assert!(args[3].contains("BE FAST"));
        assert!(args[3].contains("## testskill"));
    }

    #[test]
    fn codex_ask_with_loadout_uses_developer_instructions_override() {
        let l = loadout_with("BE FAST");
        let mut c = build_codex_ask_with_loadout(&l);
        c.arg("hello"); // caller appends prompt after -c flags
        let args = cmd_args(&c);
        assert_eq!(args[0], "exec");
        assert_eq!(args[1], "-c");
        assert!(args[2].starts_with("developer_instructions="));
        assert!(args[2].contains("BE FAST"));
        let value = args[2].trim_start_matches("developer_instructions=");
        assert!(value.starts_with('"') && value.ends_with('"'));
        assert_eq!(args[3], "hello");
    }

    #[test]
    fn dispatch_rejects_unknown_cli() {
        let l = Loadout::empty();
        let err = dispatch("unknown-cli", "hello", &l, None).unwrap_err();
        assert!(err.to_string().contains("unknown cli"));
    }

    #[test]
    fn load_gemini_keys_simple() {
        let (_tmp, p) = write_env("GEMINI_API_KEY=primary-key\nGEMINI_API_KEY_FALLBACK=fallback-key\n");
        let keys = load_gemini_keys_from(&p);
        assert_eq!(keys.primary.as_deref(), Some("primary-key"));
        assert_eq!(keys.fallback.as_deref(), Some("fallback-key"));
    }

    #[test]
    fn load_gemini_keys_handles_crlf_bom_quotes_and_trailing_ws() {
        let content = "\u{FEFF}GEMINI_API_KEY=\"primary-key\"  \r\nGEMINI_API_KEY_FALLBACK=fallback-key \r\n";
        let (_tmp, p) = write_env(content);
        let keys = load_gemini_keys_from(&p);
        assert_eq!(keys.primary.as_deref(), Some("primary-key"));
        assert_eq!(keys.fallback.as_deref(), Some("fallback-key"));
    }

    #[test]
    fn load_gemini_keys_handles_spaces_around_equals() {
        let (_tmp, p) = write_env("GEMINI_API_KEY = primary\nGEMINI_API_KEY_FALLBACK =fallback\n");
        let keys = load_gemini_keys_from(&p);
        assert_eq!(keys.primary.as_deref(), Some("primary"));
        assert_eq!(keys.fallback.as_deref(), Some("fallback"));
    }

    #[test]
    fn load_gemini_keys_strips_inline_comments() {
        let (_tmp, p) = write_env(
            "GEMINI_API_KEY=primary # my primary key\nGEMINI_API_KEY_FALLBACK=fallback#no-space-preserved\n",
        );
        let keys = load_gemini_keys_from(&p);
        assert_eq!(keys.primary.as_deref(), Some("primary"));
        assert_eq!(keys.fallback.as_deref(), Some("fallback#no-space-preserved"));
    }

    #[test]
    fn load_gemini_keys_skips_full_line_comments_and_blanks() {
        let (_tmp, p) = write_env(
            "# top comment\n\nGEMINI_API_KEY=primary\n  # indented comment\nGEMINI_API_KEY_FALLBACK=fallback\n\n",
        );
        let keys = load_gemini_keys_from(&p);
        assert_eq!(keys.primary.as_deref(), Some("primary"));
        assert_eq!(keys.fallback.as_deref(), Some("fallback"));
    }

    #[test]
    fn load_gemini_keys_empty_primary_does_not_promote_fallback() {
        let (_tmp, p) = write_env("GEMINI_API_KEY=\nGEMINI_API_KEY_FALLBACK=only-one\n");
        let keys = load_gemini_keys_from(&p);
        assert_eq!(keys.primary, None);
        assert_eq!(keys.fallback.as_deref(), Some("only-one"));
    }

    #[test]
    fn load_gemini_keys_missing_file_returns_default() {
        let keys = load_gemini_keys_from(std::path::Path::new("/nonexistent-file-abc-123"));
        assert_eq!(keys, GeminiKeys::default());
    }

    #[test]
    fn is_quota_error_matches_specific_signals_only() {
        assert!(super::is_quota_error(b"error: RESOURCE_EXHAUSTED quota exceeded"));
        assert!(super::is_quota_error(b"HTTP 429 Too Many Requests"));
        assert!(super::is_quota_error(b"status: 429"));
        assert!(super::is_quota_error(b"code: 429"));
        assert!(super::is_quota_error(b"Quota exceeded for service"));
        assert!(super::is_quota_error(b"rate limit exceeded"));
        assert!(!super::is_quota_error(b"account has no quota allocation"));
        assert!(!super::is_quota_error(b"quota: ok"));
    }

    #[test]
    fn is_auth_error_matches_auth_signals() {
        assert!(super::is_auth_error(b"HTTP 401 Unauthorized"));
        assert!(super::is_auth_error(b"403 Forbidden"));
        assert!(super::is_auth_error(b"PERMISSION_DENIED"));
        assert!(super::is_auth_error(b"API key not valid. Please pass a valid API key."));
        assert!(super::is_auth_error(b"API_KEY_INVALID"));
        assert!(super::is_auth_error(b"UNAUTHENTICATED"));
        assert!(super::is_auth_error(b"authentication failed"));
        assert!(super::is_auth_error(
            b"Please set an Auth method in your /tmp/xbreed-oauth-probe/.gemini/settings.json"
        ));
        assert!(!super::is_auth_error(b"HTTP 500 Internal Server Error"));
        assert!(!super::is_auth_error(b"connection refused"));
        assert!(!super::is_auth_error(b"request timed out"));
    }

    #[test]
    fn build_gemini_oauth_default_removes_env_var() {
        let loadout = Loadout::empty();
        let cmd = build_gemini_with_auth("hello", &loadout, &GeminiAuth::OAuthDefault);
        let has_removed = cmd
            .get_envs()
            .any(|(k, v)| k == std::ffi::OsStr::new("GEMINI_API_KEY") && v.is_none());
        assert!(has_removed, "OAuth fallback must env_remove GEMINI_API_KEY");
    }

    // ====================================================================
    // v0.3.5 — GeminiAuth cascade tests
    // ====================================================================

    #[test]
    fn gemini_auth_label_formats() {
        assert_eq!(GeminiAuth::OAuthProfile("alice".into()).label(), "oauth:alice");
        assert_eq!(GeminiAuth::OAuthDefault.label(), "oauth:default");
        assert_eq!(GeminiAuth::ApiKey("sk-abc".into()).label(), "api-key");
    }

    #[test]
    fn build_gemini_with_auth_oauth_default_strips_env_var_no_home_override() {
        let loadout = Loadout::empty();
        let cmd = build_gemini_with_auth("hello", &loadout, &GeminiAuth::OAuthDefault);
        let has_removed = cmd
            .get_envs()
            .any(|(k, v)| k == std::ffi::OsStr::new("GEMINI_API_KEY") && v.is_none());
        assert!(has_removed, "OAuthDefault must env_remove GEMINI_API_KEY");
        // OAuthDefault must NOT touch HOME — inherit the caller's real HOME
        let touches_home = cmd
            .get_envs()
            .any(|(k, _)| k == std::ffi::OsStr::new("HOME"));
        assert!(!touches_home, "OAuthDefault must NOT override HOME");
    }

    #[test]
    fn build_gemini_with_auth_profile_overrides_home_and_strips_key() {
        let loadout = Loadout::empty();
        let cmd = build_gemini_with_auth(
            "hello",
            &loadout,
            &GeminiAuth::OAuthProfile("primary".into()),
        );
        // HOME must be set to the profile dir
        let home_override = cmd
            .get_envs()
            .find(|(k, _)| k == &std::ffi::OsStr::new("HOME"))
            .and_then(|(_, v)| v);
        assert!(home_override.is_some(), "OAuthProfile must set HOME");
        let home_str = home_override.unwrap().to_string_lossy();
        assert!(
            home_str.contains(".config/xbreed/gemini-profiles/primary"),
            "HOME override must point at the profile dir, got: {home_str}"
        );
        // GEMINI_API_KEY must be explicitly removed
        let has_removed = cmd
            .get_envs()
            .any(|(k, v)| k == std::ffi::OsStr::new("GEMINI_API_KEY") && v.is_none());
        assert!(has_removed, "OAuthProfile must env_remove GEMINI_API_KEY");
    }

    #[test]
    fn build_gemini_with_auth_api_key_injects_env_var() {
        let loadout = Loadout::empty();
        let cmd = build_gemini_with_auth(
            "hello",
            &loadout,
            &GeminiAuth::ApiKey("test-key-xyz".into()),
        );
        let has_key = cmd.get_envs().any(|(k, v)| {
            k == std::ffi::OsStr::new("GEMINI_API_KEY")
                && v == Some(std::ffi::OsStr::new("test-key-xyz"))
        });
        assert!(has_key, "ApiKey auth must set GEMINI_API_KEY on the Command");
        // ApiKey should NOT touch HOME
        let touches_home = cmd
            .get_envs()
            .any(|(k, _)| k == std::ffi::OsStr::new("HOME"));
        assert!(!touches_home, "ApiKey auth must NOT override HOME");
    }

    #[test]
    fn gemini_profiles_root_is_under_xbreed_config() {
        let root = gemini_profiles_root();
        let root_str = root.to_string_lossy();
        assert!(
            root_str.ends_with(".config/xbreed/gemini-profiles")
                || root_str.ends_with(".xbreed/gemini-profiles"),
            "profiles root must be under .config/xbreed or fallback .xbreed, got: {root_str}"
        );
    }
}
