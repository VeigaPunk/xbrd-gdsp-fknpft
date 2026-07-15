use xbreed::precheck::{compute_cap, CapResult, MAX_TEAM_SIZE, MIN_ROWS};

#[test]
fn constants_correct() {
    assert_eq!(MIN_ROWS, 8);
    assert_eq!(MAX_TEAM_SIZE, 12);
}

#[test]
fn hard_cap_exceeded_returns_error() {
    let result = compute_cap(200, 0, 42);
    match result {
        CapResult::HardCapExceeded { requested, max } => {
            assert_eq!(requested, 42);
            assert_eq!(max, MAX_TEAM_SIZE);
        }
        other => panic!("expected HardCapExceeded, got {other:?}"),
    }
}

#[test]
fn zero_team_size_always_ok_nonempty_window() {
    // team_size=0: no new panes, always safe
    assert_eq!(compute_cap(24, 3, 0), CapResult::Ok);
}

#[test]
fn fresh_session_single_pane_ok() {
    // current_panes=0, adding 1 in a large window
    assert_eq!(compute_cap(46, 0, 1), CapResult::Ok);
}

#[test]
fn window_already_too_small_fails_regardless() {
    // WIN_H < MIN_ROWS: even team_size=1 fails
    // practical_cap = 4 - (0 + 1 - 1) = 4 < 8
    let result = compute_cap(4, 0, 1);
    assert!(
        matches!(result, CapResult::Fail { .. }),
        "expected Fail for window smaller than MIN_ROWS, got {result:?}"
    );
}

#[test]
fn exactly_at_cap_boundary_ok() {
    // total = current + team = 1+1 = 2; practical_cap = WIN_H - (2-1) = WIN_H - 1
    // WIN_H=9: practical_cap = 8 = MIN_ROWS → still Ok
    assert_eq!(compute_cap(9, 1, 1), CapResult::Ok);
}

#[test]
fn one_below_cap_boundary_fails() {
    // WIN_H=8: practical_cap = 8 - (1+1-1) = 8-1 = 7 < 8 → Fail
    let result = compute_cap(8, 1, 1);
    assert!(
        matches!(result, CapResult::Fail { .. }),
        "expected Fail for practical_cap < MIN_ROWS, got {result:?}"
    );
}

#[test]
fn fail_carries_team_size() {
    // Verify the Fail variant carries the requested team_size
    let result = compute_cap(8, 5, 10);
    match result {
        CapResult::Fail { team_size, .. } => assert_eq!(team_size, 10),
        other => panic!("expected Fail, got {other:?}"),
    }
}

#[test]
fn fail_carries_panes_in_use() {
    let result = compute_cap(8, 5, 10);
    match result {
        CapResult::Fail { panes_in_use, .. } => assert_eq!(panes_in_use, 5),
        other => panic!("expected Fail, got {other:?}"),
    }
}

#[test]
fn large_team_in_normal_window_hits_hard_cap_first() {
    let result = compute_cap(46, 1, 40);
    match result {
        CapResult::HardCapExceeded { requested, max } => {
            assert_eq!(requested, 40);
            assert_eq!(max, MAX_TEAM_SIZE);
        }
        other => panic!(
            "expected HardCapExceeded for 40-pane batch in 46-row window, got {other:?}"
        ),
    }
}

#[test]
fn geometry_failure_can_apply_when_team_size_within_hard_cap() {
    let result = compute_cap(10, 1, 12);
    match result {
        CapResult::Fail { panes_in_use, cap, team_size } => {
            assert_eq!(panes_in_use, 1);
            assert_eq!(cap, 0);
            assert_eq!(team_size, 12);
        }
        other => panic!("expected Fail for capped geometry case, got {other:?}"),
    }
}

#[test]
fn reasonable_team_in_normal_window_ok() {
    // 46-row window, 1 pane, spawning 5 teammates
    // total = 6; practical_cap = 46 - 5 = 41 >= 8 → Ok
    assert_eq!(compute_cap(46, 1, 5), CapResult::Ok);
}
