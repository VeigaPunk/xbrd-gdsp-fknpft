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

/// Build a codex Command with loadout injection and clean-dispatch suppression.
///
/// Always applies contamination-suppression flags (`--skip-git-repo-check` +
/// `include_permissions/apps/environment_context=false`) for epistemic
/// equivalence across models. When `spark` is true, pins the model to
/// [`CODEX_SPARK_MODEL`] and forces `model_reasoning_effort=low`. When `json`
/// is true, passes `--json` to codex exec for structured output. When
/// `output_last_message` is Some(path), passes `-o <path>` to write the final
/// assistant message to disk.
///
/// NOTE: does NOT append the prompt — caller must append it AFTER any `-c`
/// flags (effort, etc.) since `codex exec` treats the prompt as a trailing
/// positional arg.
pub fn build_codex_ask_with_loadout(
    loadout: &Loadout,
    spark: bool,
    json: bool,
    output_last_message: Option<&Path>,
) -> Command {
    let mut c = Command::new("codex");
    c.arg("exec")
        .arg("--skip-git-repo-check")
        .arg("--color")
        .arg("never")
        .arg("--ephemeral")
        // Yolo / allow-all-tools: codex defaults to a sandbox; we unlock it
        // for headless xask dispatch (parity with gemini's --approval-mode yolo
        // at line ~279). User-locked policy: solo-dev workflow, all-tool
        // permission across xask-gated subprocesses. See feedback_yolo_routing.md.
        .arg("--sandbox")
        .arg("danger-full-access");

    // Contamination suppression + approval bypass — always-on for clean headless dispatch
    c.arg("-c").arg("approval_policy=\"never\"");
    c.arg("-c").arg("include_permissions_instructions=false");
    c.arg("-c").arg("include_apps_instructions=false");
    c.arg("-c").arg("include_environment_context=false");

    if json {
        c.arg("--json");
    }

    if let Some(path) = output_last_message {
        c.arg("-o").arg(path);
    }

    if spark {
        c.arg("-m").arg(CODEX_SPARK_MODEL);
        c.arg("-c").arg("model_reasoning_effort=low");
    } else {
        // Pin the default model explicitly (CODEX_DEFAULT_MODEL = "gpt-5.4"
        // per ~/.codex/config.toml SSoT). Previously inferred via fast_mode;
        // pinning makes a codex default-model change visible at argv audit
        // time rather than silently drifting.
        c.arg("-m").arg(CODEX_DEFAULT_MODEL);
        // fast_mode is a codex feature flag for gpt-5.4 family (default model).
        // Not applicable to spark (gpt-5.3-codex-spark).
        c.arg("-c").arg("features.fast_mode=true");
    }

    if !loadout.is_empty() {
        // codex -c value is parsed as TOML. A JSON-serialized string (double-quoted,
        // with \n / \" / \\ escapes) is also a valid TOML basic string.
        let toml_quoted = serde_json::to_string(&loadout.to_concat())
            .expect("serde_json::to_string of a String never fails");
        c.arg("-c")
            .arg(format!("developer_instructions={toml_quoted}"));
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

/// The codex model used for spark (cheap/fast/expendable) probes.
pub const CODEX_SPARK_MODEL: &str = "gpt-5.3-codex-spark";

/// The codex model used for non-spark dispatch (reviewer/critic/sentinel/
/// the-revenger/etc.). `gpt-5.4` is codex's native default in v0.120.0 per
/// `~/.codex/config.toml` SSoT (`# gpt-5.4 is the native default ... declared
/// for SSoT alignment with models.yaml`). We pin it explicitly via `-m` in
/// the non-spark branch for drift detection and argv audit clarity —
/// previously the model was only inferred server-side via `features.fast_mode
/// =true`, which makes regressions invisible until a codex version bump
/// changes the default.
pub const CODEX_DEFAULT_MODEL: &str = "gpt-5.4";

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
    c.arg("-m")
        .arg(GEMINI_DEFAULT_MODEL)
        .arg("-p")
        .arg(final_prompt)
        .arg("--approval-mode")
        .arg("yolo");

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
/// message for codex.
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

/// Warn when `--effort` is supplied alongside `--spark` for codex.
/// Returns true if a warning was emitted (effort is non-default for spark).
pub fn warn_codex_spark_effort(effort: Option<&str>) -> bool {
    if let Some(e) = effort {
        if e != "low" {
            eprintln!(
                "warning: --effort is ignored for codex spark (spark pins model_reasoning_effort=low)"
            );
            return true;
        }
    }
    false
}

/// Execute a `Command` with a wall-clock timeout.
///
/// Returns `Err` with an `"xask-timeout:"` marker when the process does not
/// complete within `timeout`. Without explicit child.kill(), the timed-out
/// subprocess is reparented to pid 1 and continues running — leaking process
/// slots, file descriptors, and (for gemini) burning API quota. Hence the
/// explicit Child::kill() + wait() sequence below.
///
/// **Bypass surface:** this timeout only applies to calls routed through
/// `dispatch()` → `src/ask.rs`. Agents invoking `gemini` directly via shell
/// (Bash tool, `Agent()` native) bypass it entirely. `XASK_TIMEOUT_SECS=0`
/// is treated as invalid and falls back to the default to prevent
/// accidental self-DoS.
///
/// **Default raised 2026-04-16:** 60s → 300s. User hit the 60s ceiling on
/// high-effort codex calls (xhigh reasoning on non-trivial prompts can
/// exceed 60s; see m7-framing-audit-2026-04-16.md which needed
/// XASK_TIMEOUT_SECS=540 for the ACH run). 300s is a safe ceiling that
/// still prevents runaway processes from hanging the harness indefinitely.
/// Override via `XASK_TIMEOUT_SECS=<seconds>` env var.
pub fn execute_with_timeout(
    mut cmd: std::process::Command,
    timeout: std::time::Duration,
) -> Result<std::process::Output> {
    use std::io::Read;

    cmd.stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    let mut child = cmd
        .spawn()
        .map_err(|e| anyhow::anyhow!("failed to spawn command: {e}"))?;

    let stdout_pipe = child.stdout.take().expect("stdout piped");
    let stderr_pipe = child.stderr.take().expect("stderr piped");

    let (tx, rx) = std::sync::mpsc::channel::<(Vec<u8>, Vec<u8>)>();

    // Two inner threads read stdout and stderr concurrently to avoid pipe-buffer
    // deadlock, then signal the outer thread which holds the Child handle.
    std::thread::spawn(move || {
        let (otx, orx) = std::sync::mpsc::channel();
        let (etx, erx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut v = Vec::new();
            let _ = std::io::BufReader::new(stdout_pipe).read_to_end(&mut v);
            let _ = otx.send(v);
        });
        std::thread::spawn(move || {
            let mut v = Vec::new();
            let _ = std::io::BufReader::new(stderr_pipe).read_to_end(&mut v);
            let _ = etx.send(v);
        });
        let out = orx.recv().unwrap_or_default();
        let err = erx.recv().unwrap_or_default();
        let _ = tx.send((out, err));
    });

    match rx.recv_timeout(timeout) {
        Ok((stdout, stderr)) => {
            let status = child
                .wait()
                .map_err(|e| anyhow::anyhow!("failed to wait for child: {e}"))?;
            Ok(std::process::Output {
                status,
                stdout,
                stderr,
            })
        }
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
            child.kill().ok();
            child.wait().ok();
            anyhow::bail!(
                "xask-timeout: command did not complete within {}s \
                 (set XASK_TIMEOUT_SECS env var to override)",
                timeout.as_secs()
            )
        }
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
            child.kill().ok();
            child.wait().ok();
            anyhow::bail!("xask-timeout: command worker thread disconnected unexpectedly")
        }
    }
}

pub fn dispatch(
    cli: &str,
    prompt: &str,
    loadout: &Loadout,
    effort: Option<&str>,
    spark: bool,
    json: bool,
    output_last_message: Option<&Path>,
) -> Result<String> {
    let timeout_secs = std::env::var("XASK_TIMEOUT_SECS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .filter(|&n| n > 0)
        .unwrap_or(300);
    let timeout = std::time::Duration::from_secs(timeout_secs);

    if cli == "gemini" {
        // --effort for gemini: mapped to thinkingBudget (low=512, medium=4096,
        // high=8192, xhigh=16384) and injected into the prompt template by
        // scripts/xask. Gemini-CLI has no native --effort flag, so we don't
        // pass it on the command line; the budget reaches the model as
        // prompt-text directive.
        let _ = effort;
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
            let cmd = build_gemini_with_auth(prompt, loadout, auth);
            let output = execute_with_timeout(cmd, timeout)
                .map_err(|e| anyhow::anyhow!("gemini (auth={}): {e}", auth.label()))?;
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

    let cmd = match cli {
        "codex" => {
            let mut c = build_codex_ask_with_loadout(loadout, spark, json, output_last_message);
            if spark {
                warn_codex_spark_effort(effort);
            } else if let Some(e) = effort {
                c.arg("-c").arg(format!("model_reasoning_effort={e}"));
            }
            // User directive: codex ALWAYS inherits the godspeed posture
            // through xask in its purest form. Structural guarantee at the
            // Rust dispatch layer — append "| godspeed" to the prompt if
            // the caller (scripts/xask or direct xbreed ask) hasn't already.
            // Idempotent: scripts/xask appends when SKILL=godspeed (default);
            // the check below avoids "| godspeed | godspeed" duplication.
            // Belt + suspenders: --with godspeed also injects the skill text
            // as -c developer_instructions, so codex sees the directive via
            // both channels.
            let final_prompt = if prompt.trim_end().ends_with("| godspeed") {
                prompt.to_string()
            } else {
                format!("{prompt} | godspeed")
            };
            // Prompt MUST be the last positional arg for codex exec —
            // all -c flags must come before it.
            c.arg(&final_prompt);
            c
        }
        other => anyhow::bail!("unknown cli: {other} (expected codex|gemini)"),
    };
    let output = execute_with_timeout(cmd, timeout)
        .map_err(|e| anyhow::anyhow!("failed to execute {cli}: {e} (is it on PATH?)"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Auth-class errors get a clearer hint pointing to the CLI's own
        // credential chain. xbreed does not manage codex OAuth —
        // those CLIs own their own auth (subscription login, etc.).
        if is_auth_error(stderr.as_bytes()) {
            let hint = match cli {
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
        c.get_args()
            .map(|a| a.to_string_lossy().to_string())
            .collect()
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
    fn codex_ask_empty_loadout_has_suppression_and_approval_flags() {
        let mut c = build_codex_ask_with_loadout(&Loadout::empty(), false, false, None);
        c.arg("hello"); // caller appends prompt after -c flags
        assert_eq!(c.get_program().to_string_lossy(), "codex");
        let args = cmd_args(&c);
        assert_eq!(args[0], "exec");
        assert_eq!(args[1], "--skip-git-repo-check");
        assert!(args.contains(&"approval_policy=\"never\"".to_string()));
        assert!(args.contains(&"include_permissions_instructions=false".to_string()));
        assert!(args.contains(&"include_apps_instructions=false".to_string()));
        assert!(args.contains(&"include_environment_context=false".to_string()));
        assert!(args.contains(&"features.fast_mode=true".to_string()));
        assert!(args.contains(&"--ephemeral".to_string()));
        // Non-spark path pins CODEX_DEFAULT_MODEL explicitly for drift detection —
        // codex's native default in v0.120.0 is gpt-5.4 per ~/.codex/config.toml.
        assert!(args.contains(&"-m".to_string()));
        assert!(args.contains(&CODEX_DEFAULT_MODEL.to_string()));
        // Yolo / allow-all-tools sandbox unlock — see feedback_yolo_routing.md
        assert!(args.contains(&"--sandbox".to_string()));
        assert!(args.contains(&"danger-full-access".to_string()));
        // json=false: --json must NOT appear in argv
        assert!(!args.contains(&"--json".to_string()));
        assert_eq!(*args.last().unwrap(), "hello");
    }

    #[test]
    fn codex_ask_spark_adds_model_and_low_effort() {
        let mut c = build_codex_ask_with_loadout(&Loadout::empty(), true, false, None);
        c.arg("probe"); // caller appends prompt
        let args = cmd_args(&c);
        assert!(args.contains(&"-m".to_string()));
        assert!(args.contains(&CODEX_SPARK_MODEL.to_string()));
        assert!(args.contains(&"model_reasoning_effort=low".to_string()));
        // fast_mode is gpt-5.4 only — must NOT be present on spark path
        assert!(!args.contains(&"features.fast_mode=true".to_string()));
        // Yolo sandbox applies to spark too — labrats need all-tool access
        assert!(args.contains(&"--sandbox".to_string()));
        assert!(args.contains(&"danger-full-access".to_string()));
        assert_eq!(*args.last().unwrap(), "probe");
    }

    #[test]
    fn codex_ask_with_loadout_uses_developer_instructions_override() {
        let l = loadout_with("BE FAST");
        let mut c = build_codex_ask_with_loadout(&l, false, false, None);
        c.arg("hello"); // caller appends prompt after -c flags
        let args = cmd_args(&c);
        assert_eq!(args[0], "exec");
        assert_eq!(args[1], "--skip-git-repo-check");
        // suppression flags at [2..7], then developer_instructions
        let dev_instr = args
            .iter()
            .find(|a| a.starts_with("developer_instructions="))
            .expect("developer_instructions flag missing");
        assert!(dev_instr.contains("BE FAST"));
        let value = dev_instr.trim_start_matches("developer_instructions=");
        assert!(value.starts_with('"') && value.ends_with('"'));
        assert_eq!(*args.last().unwrap(), "hello");
    }

    #[test]
    fn dispatch_rejects_unknown_cli() {
        let l = Loadout::empty();
        let err = dispatch("unknown-cli", "hello", &l, None, false, false, None).unwrap_err();
        assert!(err.to_string().contains("unknown cli"));
    }

    #[test]
    fn load_gemini_keys_simple() {
        let (_tmp, p) =
            write_env("GEMINI_API_KEY=primary-key\nGEMINI_API_KEY_FALLBACK=fallback-key\n");
        let keys = load_gemini_keys_from(&p);
        assert_eq!(keys.primary.as_deref(), Some("primary-key"));
        assert_eq!(keys.fallback.as_deref(), Some("fallback-key"));
    }

    #[test]
    fn load_gemini_keys_handles_crlf_bom_quotes_and_trailing_ws() {
        let content =
            "\u{FEFF}GEMINI_API_KEY=\"primary-key\"  \r\nGEMINI_API_KEY_FALLBACK=fallback-key \r\n";
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
        assert_eq!(
            keys.fallback.as_deref(),
            Some("fallback#no-space-preserved")
        );
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
        assert!(super::is_quota_error(
            b"error: RESOURCE_EXHAUSTED quota exceeded"
        ));
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
        assert!(super::is_auth_error(
            b"API key not valid. Please pass a valid API key."
        ));
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

    // ====================================================================
    // v0.3.5 — GeminiAuth cascade tests
    // ====================================================================

    #[test]
    fn gemini_auth_label_formats() {
        assert_eq!(
            GeminiAuth::OAuthProfile("alice".into()).label(),
            "oauth:alice"
        );
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
        assert!(
            has_key,
            "ApiKey auth must set GEMINI_API_KEY on the Command"
        );
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

    #[test]
    fn spark_with_effort_warns_and_drops() {
        assert!(super::warn_codex_spark_effort(Some("high")));
        assert!(super::warn_codex_spark_effort(Some("medium")));
        assert!(!super::warn_codex_spark_effort(Some("low")));
        assert!(!super::warn_codex_spark_effort(None));
    }

    #[test]
    fn execute_with_timeout_returns_err_on_slow_cmd() {
        let mut cmd = std::process::Command::new("sleep");
        cmd.arg("30");
        let result = super::execute_with_timeout(cmd, std::time::Duration::from_secs(1));
        let err = result.unwrap_err();
        assert!(
            err.to_string().contains("xask-timeout"),
            "expected xask-timeout error, got: {err}"
        );
    }

    #[test]
    fn execute_with_timeout_kills_child_on_timeout() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let pid_path = tmp.path().to_path_buf();

        let mut cmd = std::process::Command::new("bash");
        cmd.arg("-c")
            .arg(format!("echo $$ > {}; sleep 60", pid_path.display()));

        let result = super::execute_with_timeout(cmd, std::time::Duration::from_secs(1));
        assert!(result.is_err(), "expected timeout error");
        assert!(
            result.unwrap_err().to_string().contains("xask-timeout"),
            "error should carry xask-timeout marker"
        );

        // Poll up to 500 ms for bash to have written its PID (it does so immediately
        // at startup, but we race the kill window in the fixed impl).
        let mut child_pid: u32 = 0;
        for _ in 0..10 {
            if let Ok(s) = std::fs::read_to_string(&pid_path) {
                if let Ok(p) = s.trim().parse::<u32>() {
                    child_pid = p;
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        assert!(child_pid > 0, "child PID was never written to temp file");

        // Brief settle window for the OS to finalise the kill+wait.
        std::thread::sleep(std::time::Duration::from_millis(200));

        // /proc/<pid> must be absent — child killed and reaped, not a ghost.
        let still_alive = std::path::Path::new(&format!("/proc/{child_pid}")).exists();
        assert!(
            !still_alive,
            "child PID {child_pid} still present in /proc after timeout — ghost leak not fixed"
        );
    }

    #[test]
    fn dispatch_codex_path_reaches_timeout_wrapper() {
        use std::io::Write;
        use std::os::unix::fs::PermissionsExt;

        // Serialize env-mutating tests to avoid PATH race with parallel test threads.
        static PATH_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
        let _guard = PATH_LOCK
            .get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .unwrap();

        // Fake "codex" that hangs — ensures dispatch() must fire the timeout wrapper.
        let tmp = tempfile::tempdir().unwrap();
        let fake_codex = tmp.path().join("codex");
        {
            let mut f = std::fs::File::create(&fake_codex).unwrap();
            writeln!(f, "#!/bin/sh\nexec sleep 60").unwrap();
        }
        std::fs::set_permissions(&fake_codex, std::fs::Permissions::from_mode(0o755)).unwrap();

        let orig_path = std::env::var("PATH").unwrap_or_default();
        std::env::set_var("PATH", format!("{}:{}", tmp.path().display(), orig_path));
        std::env::set_var("XASK_TIMEOUT_SECS", "1");

        let result = super::dispatch(
            "codex",
            "test prompt",
            &super::Loadout::empty(),
            None,
            false,
            false,
            None,
        );

        std::env::set_var("PATH", &orig_path);
        std::env::remove_var("XASK_TIMEOUT_SECS");

        let err = result.unwrap_err();
        assert!(
            err.to_string().contains("xask-timeout"),
            "codex dispatch path did not invoke timeout wrapper: {err}"
        );
    }
}
