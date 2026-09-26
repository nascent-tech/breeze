use breeze_domain::constants::{NOTICE, WINDOW_VEIL_CAP};
use breeze_domain::{
    Absence, ActiveDays, BreakMode, Cycle, CycleState, DegradedReason, Instant, Minutes, Rhythm,
    Severity,
};
use core::time::Duration;

const ONE_SEC: Duration = Duration::from_secs(1);

fn rhythm() -> Rhythm {
    Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday()).unwrap()
}

fn due_at() -> Instant {
    Instant::EPOCH.plus(rhythm().work().as_duration())
}

fn break_at() -> Instant {
    due_at().plus(NOTICE)
}

fn mode_of(cycle: &Cycle) -> Option<BreakMode> {
    match cycle.state() {
        CycleState::BreakActive { mode, .. } => Some(mode),
        _ => None,
    }
}

fn simple_break(frames_observable: bool) -> Cycle {
    let mut cycle = Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH);
    cycle.observe_frames(frames_observable);
    cycle.tick(break_at());
    cycle
}

#[test]
fn a_simple_break_with_observable_frames_veils_window_by_window() {
    assert_eq!(mode_of(&simple_break(true)), Some(BreakMode::Nominal));
}

#[test]
fn a_simple_break_without_observable_frames_falls_back_to_full_screen() {
    assert_eq!(
        mode_of(&simple_break(false)),
        Some(BreakMode::Degraded(DegradedReason::FramesUnobservable))
    );
}

#[test]
fn a_hardcore_break_ignores_the_frames_capability() {
    let mut cycle = Cycle::start(rhythm(), Severity::Hardcore, Instant::EPOCH);
    cycle.observe_frames(false);
    cycle.tick(break_at());
    assert_eq!(mode_of(&cycle), Some(BreakMode::Nominal));
}

#[test]
fn the_capability_read_during_the_notice_is_the_one_that_counts() {
    let mut cycle = Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH);
    cycle.observe_frames(false);
    cycle.tick(due_at());
    cycle.observe_frames(true);
    cycle.tick(break_at());
    assert_eq!(mode_of(&cycle), Some(BreakMode::Nominal));
}

#[test]
fn a_capability_lost_during_the_break_does_not_change_its_mode() {
    let mut cycle = simple_break(true);
    cycle.observe_frames(false);
    cycle.tick(break_at().plus(ONE_SEC));
    assert_eq!(mode_of(&cycle), Some(BreakMode::Nominal));
}

#[test]
fn a_capability_regained_during_the_break_does_not_change_its_mode() {
    let mut cycle = simple_break(false);
    cycle.observe_frames(true);
    cycle.tick(break_at().plus(ONE_SEC));
    assert_eq!(
        mode_of(&cycle),
        Some(BreakMode::Degraded(DegradedReason::FramesUnobservable))
    );
}

#[test]
fn a_break_starting_at_wake_reads_the_capability_of_that_instant() {
    let mut cycle = Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH);
    let slept_at = due_at().minus(ONE_SEC);
    cycle.observe_frames(true);
    let absence = Absence {
        began_at: slept_at,
        lasted: Duration::from_secs(120),
    };
    cycle.return_from_absence(absence, due_at().plus(ONE_SEC));
    assert_eq!(mode_of(&cycle), Some(BreakMode::Nominal));
}

#[test]
fn up_to_the_cap_every_blocked_window_keeps_its_own_veil() {
    let mut cycle = simple_break(true);
    cycle.observe_blocked_windows(WINDOW_VEIL_CAP);
    assert_eq!(mode_of(&cycle), Some(BreakMode::Nominal));
}

#[test]
fn beyond_the_cap_the_break_falls_back_to_full_screen_for_good() {
    let mut cycle = simple_break(true);
    cycle.observe_blocked_windows(WINDOW_VEIL_CAP + 1);
    assert_eq!(
        mode_of(&cycle),
        Some(BreakMode::Degraded(DegradedReason::TooManyWindows))
    );

    cycle.observe_blocked_windows(1);
    assert_eq!(
        mode_of(&cycle),
        Some(BreakMode::Degraded(DegradedReason::TooManyWindows)),
        "closing windows afterwards does not restore the window veils"
    );
}

#[test]
fn many_windows_never_degrade_a_hardcore_break() {
    let mut cycle = Cycle::start(rhythm(), Severity::Hardcore, Instant::EPOCH);
    cycle.observe_frames(true);
    cycle.tick(break_at());
    cycle.observe_blocked_windows(WINDOW_VEIL_CAP * 2);
    assert_eq!(mode_of(&cycle), Some(BreakMode::Nominal));
}

#[test]
fn the_next_break_reads_the_capability_afresh() {
    let r = rhythm();
    let mut cycle = simple_break(true);
    cycle.observe_blocked_windows(WINDOW_VEIL_CAP + 1);
    let next_break = break_at()
        .plus(r.pause().as_duration())
        .plus(breeze_domain::constants::RETURN_HOLD)
        .plus(r.work().as_duration())
        .plus(NOTICE);
    cycle.tick(next_break);
    assert_eq!(mode_of(&cycle), Some(BreakMode::Nominal));
}
