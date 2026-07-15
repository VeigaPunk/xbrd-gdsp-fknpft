use crate::loadout::Loadout;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::Command;

// OAuth-exclusive + single-path (user directives 2026-04-17 API-key removal,
// 2026-04-19 cascade collapse): xbreed reads ONLY `~/.gemini/oauth_creds.json`
// for gemini (Sign-in-with-Google subscription) and delegates `~/.codex/auth.json`
// management to the codex CLI itself (ChatGPT subscription). No named-profile
// cascade, no API-key fallback, no quota-aware retry, no health canary. See
// `user_oauth_exclusive.md` + `feedback_oauth_exclusive_code.md` auto-memory.

/// Build a codex Command with loadout injection and clean-dispatch suppression.
///
/// Five lanes select the codex model family:
/// - `spark=true`            → [`CODEX_SPARK_MODEL`] + `model_reasoning_effort=low`
///   (no fast_mode). Labrat probes, cheap/fast/expendable.
/// - `gpt55=true` (and not spark) → [`CODEX_55_MODEL`] (`gpt-5.6-sol`) +
///   `features.fast_mode=true`. Added 2026-04-24 for the xbrd-exec bench
///   (xask-arm gpt-5.6-sol measurement). Orthogonal to review/full — those are
///   5.5-family lanes; `gpt55` short-circuits them.
/// - `review=true, full=true` → [`CODEX_FULL_MODEL`] (`gpt-5.6-sol` full, 1.05M ctx) +
///   `features.fast_mode=true`. The escape hatch (user directive 2026-04-18) for
///   the-revenger RECON where stitching codebase-scale evidence needs the larger
///   context window.
/// - `review=true` (no full)  → [`CODEX_MINI_MODEL`] (`gpt-5.6-sol`, 400K ctx) +
///   `features.fast_mode=true`. Review lane default per 2026-04-18 directive.
/// - otherwise                → [`CODEX_MINI_MODEL`] + `features.fast_mode=true`.
///   Default non-spark lane; mini handles execution/labrat/scout/etc.
///
/// `spark` short-circuits all other lanes (labrat probes are cheaper than
/// reviews and should beat review in the rare spark+review case). `gpt55`
/// short-circuits review/full (explicit model pin wins over review-lane
/// defaulting). `full` without `review` is a no-op.
///
/// Always applies contamination-suppression flags (`--skip-git-repo-check` +
/// `include_permissions/apps/environment_context=false`) for epistemic
/// equivalence across models. When `json` is true, passes `--json` for
/// structured output. When `output_last_message` is `Some(path)`, passes
/// `-o <path>` to write the final assistant message to disk.
///
/// NOTE: does NOT append the prompt — caller must append it AFTER any `-c`
/// flags (effort, etc.) since `codex exec` treats the prompt as a trailing
/// positional arg.
pub fn build_codex_ask_with_loadout(
    loadout: &Loadout,
    spark: bool,
    review: bool,
    full: bool,
    gpt55: bool,
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
    } else if gpt55 {
        // Explicit gpt-5.6-sol lane — short-circuits review/full (those are
        // 5.5-family). fast_mode enabled for parity with mini/full lanes
        // so `xask --gpt55 -e <effort> codex` is the canonical xbreed
        // entry point for 5.5 xask-arm dispatches.
        c.arg("-m").arg(CODEX_55_MODEL);
        c.arg("-c").arg("features.fast_mode=true");
    } else if review && full {
        // -R -F escape hatch: full gpt-5.6-sol (1.05M ctx) for the-revenger RECON
        // where the larger context window earns the cost. User directive 2026-04-18.
        c.arg("-m").arg(CODEX_FULL_MODEL);
        c.arg("-c").arg("features.fast_mode=true");
    } else {
        // Default + review-default lanes both route to mini (400K ctx) + fast_mode.
        // User directive 2026-04-18 — review lane migrated to mini; escape hatch
        // via --full/-F above when RECON-class context is needed.
        c.arg("-m").arg(CODEX_MINI_MODEL);
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

/// The codex model used for the `-R -F` escape hatch — full `gpt-5.6-sol`,
/// codex's full-capacity model in v0.120.0 (1.05M context window). Reserved
/// for the-revenger RECON tasks stitching codebase-scale evidence where
/// mini's 400K ceiling would silently truncate. User directive 2026-04-18.
pub const CODEX_FULL_MODEL: &str = "gpt-5.6-sol";

/// The codex model used for the default non-spark lane — `gpt-5.6-sol`,
/// introduced as the standing default 2026-04-17 and extended to the review
/// lane default 2026-04-18. Handles execution/labrat/scout/planning/synthesis
/// AND reviewer/critic/sentinel work at a fraction of the cost and round-time
/// of full 5.4, while still supporting `features.fast_mode=true`. The-revenger
/// RECON escalates via `-R -F` to reach [`CODEX_FULL_MODEL`] for the larger
/// context window.
pub const CODEX_MINI_MODEL: &str = "gpt-5.6-sol";

/// The codex model reached via `xask --gpt55 codex` — `gpt-5.6-sol`. Added
/// 2026-04-24 so the xbrd-exec bench can measure xask-arm latency/throughput
/// for 5.5 and compute Δ_wrap (xask wrapper overhead) by comparison against
/// the raw `codex exec -m gpt-5.6-sol` arm already benched. Supports all four
/// effort levels (low/medium/high/xhigh) via the standard `-e` flag — fast_mode
/// enabled to mirror the mini/full lanes' default posture.
pub const CODEX_55_MODEL: &str = "gpt-5.6-sol";

// ========================================================================
// Gemini OAuth — single default path (2026-04-19 cascade collapse)
// ========================================================================

/// Check whether the user's default `~/.gemini/oauth_creds.json` exists.
/// This is the single auth precondition for gemini dispatch — no named
/// profiles, no API-key fallback.
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

/// Build a gemini Command using the default OAuth path.
///
/// Strips any inherited `GEMINI_API_KEY` to force the OAuth path — user
/// directive (`user_oauth_exclusive.md`) is OAuth-exclusive. Does not
/// override HOME; gemini reads `~/.gemini/oauth_creds.json` natively.
pub fn build_gemini(prompt: &str, loadout: &Loadout) -> Command {
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
        .arg("yolo")
        .env_remove("GEMINI_API_KEY");
    c
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

#[allow(clippy::too_many_arguments)]
pub fn dispatch(
    cli: &str,
    prompt: &str,
    loadout: &Loadout,
    effort: Option<&str>,
    spark: bool,
    review: bool,
    full: bool,
    gpt55: bool,
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
        if !default_gemini_oauth_exists() {
            anyhow::bail!(
                "gemini: OAuth credentials not found at ~/.gemini/oauth_creds.json. \
                 Run `gemini login` to sign in with your Google account."
            );
        }
        let cmd = build_gemini(prompt, loadout);
        let output =
            execute_with_timeout(cmd, timeout).map_err(|e| anyhow::anyhow!("gemini: {e}"))?;
        if output.status.success() {
            return Ok(String::from_utf8_lossy(&output.stdout).to_string());
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        if is_auth_error(stderr.as_bytes()) {
            anyhow::bail!(
                "gemini: authentication failed — run `gemini login` to refresh \
                 ~/.gemini/oauth_creds.json.\nstderr: {stderr}"
            );
        }
        anyhow::bail!("gemini failed (exit {:?}): {stderr}", output.status.code());
    }

    let cmd = match cli {
        "codex" => {
            let mut c = build_codex_ask_with_loadout(
                loadout,
                spark,
                review,
                full,
                gpt55,
                json,
                output_last_message,
            );
            if spark {
                warn_codex_spark_effort(effort);
            } else if let Some(e) = effort {
                c.arg("-c").arg(format!("model_reasoning_effort={e}"));
            } else if !review {
                // Default (mini) lane: user directive 2026-04-17 — reasoning high
                // unless the caller overrides via `-e/--effort`. Review lane
                // inherits codex's own default (xhigh per ~/.codex/config.toml)
                // so the extra capacity isn't starved by a mid-tier effort cap.
                c.arg("-c").arg("model_reasoning_effort=high");
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
                    "run `codex login` to sign in with your ChatGPT Plus/Pro/Enterprise subscription"
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

    #[test]
    fn codex_ask_default_lane_uses_mini_model() {
        // Default lane (spark=false, review=false, full=false) = gpt-5.6-sol + fast_mode.
        // User directive 2026-04-17 — mini is the standing default; 2026-04-18
        // extended mini to the review lane default, with -R -F as escape hatch
        // to full 5.4 for the-revenger RECON.
        let mut c = build_codex_ask_with_loadout(
            &Loadout::empty(),
            false,
            false,
            false,
            false,
            false,
            None,
        );
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
        // --color never: codex's TTY-color autodetection misfires in headless
        // dispatch (emits ANSI escapes that poison downstream parsers). Always-on.
        assert!(args.contains(&"--color".to_string()));
        assert!(args.contains(&"never".to_string()));
        // Default lane pins CODEX_MINI_MODEL (gpt-5.6-sol) explicitly.
        assert!(args.contains(&"-m".to_string()));
        assert!(args.contains(&CODEX_MINI_MODEL.to_string()));
        // Yolo / allow-all-tools sandbox unlock — see feedback_yolo_routing.md
        assert!(args.contains(&"--sandbox".to_string()));
        assert!(args.contains(&"danger-full-access".to_string()));
        // json=false: --json must NOT appear in argv
        assert!(!args.contains(&"--json".to_string()));
        assert_eq!(*args.last().unwrap(), "hello");
    }

    #[test]
    fn codex_ask_review_default_uses_mini_model() {
        // Review lane default (review=true, full=false) routes to gpt-5.6-sol
        // per user directive 2026-04-18 (migrated from prior full-review defaults).
        let mut c =
            build_codex_ask_with_loadout(&Loadout::empty(), false, true, false, false, false, None);
        c.arg("review this").arg(""); // second arg is a no-op placeholder
        let args = cmd_args(&c);
        assert!(args.contains(&"-m".to_string()));
        assert!(args.contains(&CODEX_MINI_MODEL.to_string()));
        assert!(args.contains(&"features.fast_mode=true".to_string()));
    }

    #[test]
    fn codex_ask_review_full_flag_uses_full_model() {
        // -R -F escape hatch (review=true, full=true) routes to full gpt-5.6-sol
        // for the-revenger RECON with 1.05M context window. User directive
        // 2026-04-18.
        let mut c =
            build_codex_ask_with_loadout(&Loadout::empty(), false, true, true, false, false, None);
        c.arg("recon this").arg("");
        let args = cmd_args(&c);
        assert!(args.contains(&"-m".to_string()));
        assert!(
            args.contains(&CODEX_FULL_MODEL.to_string()),
            "-R -F must pin -m gpt-5.6-sol (full) for the-revenger RECON: {args:?}"
        );
        assert!(
            !args.contains(&CODEX_MINI_MODEL.to_string()),
            "-R -F must NOT route to mini: {args:?}"
        );
        assert!(args.contains(&"features.fast_mode=true".to_string()));
    }

    #[test]
    fn codex_ask_full_without_review_is_noop() {
        // --full/-F without --review is a no-op; routes to mini default.
        let mut c =
            build_codex_ask_with_loadout(&Loadout::empty(), false, false, true, false, false, None);
        c.arg("hello");
        let args = cmd_args(&c);
        assert!(args.contains(&CODEX_MINI_MODEL.to_string()));
        assert!(
            !args.contains(&CODEX_FULL_MODEL.to_string()),
            "--full without --review must NOT promote to full model: {args:?}"
        );
    }

    #[test]
    fn codex_ask_gpt55_lane_uses_55_model_with_fast_mode() {
        // --gpt55 routes to gpt-5.6-sol with fast_mode enabled. Effort is applied
        // by dispatch() via -c model_reasoning_effort=<e> (not this function).
        // Added 2026-04-24 for xbrd-exec bench xask-arm measurement.
        let mut c =
            build_codex_ask_with_loadout(&Loadout::empty(), false, false, false, true, false, None);
        c.arg("probe-55");
        let args = cmd_args(&c);
        assert!(args.contains(&"-m".to_string()));
        assert!(
            args.contains(&CODEX_55_MODEL.to_string()),
            "--gpt55 must pin -m gpt-5.6-sol: {args:?}"
        );
        assert!(
            !args.contains(&CODEX_MINI_MODEL.to_string()),
            "--gpt55 must NOT route to mini: {args:?}"
        );
        assert!(args.contains(&"features.fast_mode=true".to_string()));
        assert_eq!(*args.last().unwrap(), "probe-55");
    }

    #[test]
    fn codex_ask_spark_short_circuits_gpt55() {
        // --spark wins over --gpt55 (spark is the cheapest lane; explicit short-circuit
        // preserves "labrat probes beat every other lane" semantics).
        let mut c =
            build_codex_ask_with_loadout(&Loadout::empty(), true, false, false, true, false, None);
        c.arg("probe");
        let args = cmd_args(&c);
        assert!(args.contains(&CODEX_SPARK_MODEL.to_string()));
        assert!(
            !args.contains(&CODEX_55_MODEL.to_string()),
            "spark must short-circuit --gpt55: {args:?}"
        );
    }

    #[test]
    fn codex_ask_gpt55_short_circuits_review_and_full() {
        // --gpt55 short-circuits review/full (those route to 5.6 family; explicit
        // gpt-5.6-sol model pin wins over review-lane defaulting).
        let mut c =
            build_codex_ask_with_loadout(&Loadout::empty(), false, true, true, true, false, None);
        c.arg("probe");
        let args = cmd_args(&c);
        assert!(args.contains(&CODEX_55_MODEL.to_string()));
        assert!(
            !args.contains(&CODEX_MINI_MODEL.to_string()),
            "--gpt55 must short-circuit -R -F (no mini path): {args:?}"
        );
    }

    #[test]
    fn codex_ask_spark_adds_model_and_low_effort() {
        let mut c =
            build_codex_ask_with_loadout(&Loadout::empty(), true, false, false, false, false, None);
        c.arg("probe"); // caller appends prompt
        let args = cmd_args(&c);
        assert!(args.contains(&"-m".to_string()));
        assert!(args.contains(&CODEX_SPARK_MODEL.to_string()));
        assert!(args.contains(&"model_reasoning_effort=low".to_string()));
        // fast_mode is non-spark Codex lanes only — must NOT be present on spark path
        assert!(!args.contains(&"features.fast_mode=true".to_string()));
        // Yolo sandbox applies to spark too — labrats need all-tool access
        assert!(args.contains(&"--sandbox".to_string()));
        assert!(args.contains(&"danger-full-access".to_string()));
        assert_eq!(*args.last().unwrap(), "probe");
    }

    #[test]
    fn codex_ask_with_loadout_uses_developer_instructions_override() {
        let l = loadout_with("BE FAST");
        let mut c = build_codex_ask_with_loadout(&l, false, false, false, false, false, None);
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
        let err = dispatch(
            "unknown-cli",
            "hello",
            &l,
            None,
            false,
            false,
            false,
            false,
            false,
            None,
        )
        .unwrap_err();
        assert!(err.to_string().contains("unknown cli"));
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
    // Gemini OAuth single-path tests (2026-04-19 cascade collapse)
    // ====================================================================

    #[test]
    fn build_gemini_strips_gemini_api_key_and_does_not_override_home() {
        let loadout = Loadout::empty();
        let cmd = build_gemini("hello", &loadout);
        let has_removed = cmd
            .get_envs()
            .any(|(k, v)| k == std::ffi::OsStr::new("GEMINI_API_KEY") && v.is_none());
        assert!(
            has_removed,
            "build_gemini must env_remove GEMINI_API_KEY to force OAuth path"
        );
        // Default OAuth path must NOT touch HOME — inherit the caller's real HOME
        let touches_home = cmd
            .get_envs()
            .any(|(k, _)| k == std::ffi::OsStr::new("HOME"));
        assert!(
            !touches_home,
            "build_gemini must NOT override HOME (default OAuth reads ~/.gemini/)"
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
            false,
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
