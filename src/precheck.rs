/// Minimum acceptable pane height in rows (matches adaptive-panes.sh MIN_ROWS).
pub const MIN_ROWS: u32 = 8;

/// Result of a pane-cap preflight check.
#[derive(Debug, PartialEq)]
pub enum CapResult {
    /// Spawn is safe — practical cap remains at or above MIN_ROWS.
    Ok,
    /// Spawn would push the practical cap below MIN_ROWS.
    Fail {
        panes_in_use: u32,
        cap: u32,
        team_size: u32,
    },
    /// tmux is not running / not reachable — check skipped (fail-open).
    TmuxUnavailable,
}

/// Pure cap computation.
///
/// `practical_cap = WIN_H − (current_panes + team_size − 1)`
/// Fails when `practical_cap < MIN_ROWS`.
pub fn compute_cap(win_h: u32, current_panes: u32, team_size: u32) -> CapResult {
    let total = current_panes.saturating_add(team_size);
    let practical_cap = win_h.saturating_sub(total.saturating_sub(1));
    if practical_cap < MIN_ROWS {
        CapResult::Fail {
            panes_in_use: current_panes,
            cap: practical_cap,
            team_size,
        }
    } else {
        CapResult::Ok
    }
}

/// Run preflight against the live tmux session.
///
/// Returns `TmuxUnavailable` (exit 0) when tmux is not present or not in a session.
pub fn run(team_size: u32) -> anyhow::Result<CapResult> {
    let current_panes = match get_pane_count() {
        Err(_) => return Ok(CapResult::TmuxUnavailable),
        Ok(n) => n,
    };
    let win_h = match get_window_height() {
        Err(_) => return Ok(CapResult::TmuxUnavailable),
        Ok(h) => h,
    };
    Ok(compute_cap(win_h, current_panes, team_size))
}

fn get_pane_count() -> anyhow::Result<u32> {
    // Per-window scope: the cap formula is derived from one window's height being
    // split across panes inside that window. `-a` would over-count across sessions.
    let output = std::process::Command::new("tmux")
        .args(["list-panes"])
        .output()
        .map_err(|_| anyhow::anyhow!("tmux not found"))?;
    if !output.status.success() {
        return Err(anyhow::anyhow!("tmux list-panes failed"));
    }
    let count = String::from_utf8_lossy(&output.stdout)
        .lines()
        .count()
        .try_into()
        .unwrap_or(0);
    Ok(count)
}

fn get_window_height() -> anyhow::Result<u32> {
    let output = std::process::Command::new("tmux")
        .args(["display-message", "-p", "#{window_height}"])
        .output()
        .map_err(|_| anyhow::anyhow!("tmux not found"))?;
    let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
    s.parse::<u32>()
        .map_err(|e| anyhow::anyhow!("parse window height '{s}': {e}"))
}
