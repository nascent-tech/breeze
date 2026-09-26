use breeze_domain::constants::{IDLE_FREEZE, NOTICE};
use breeze_domain::{
    ActiveDays, AppId, AppStatus, AppStatuses, Countdown, Cycle, CycleState, FreezeReason, Instant,
    Minutes, Rhythm, SafetyList, Severity, SparedApps,
};
use core::time::Duration;

const ONE_MIN: Duration = Duration::from_secs(60);
const TEN_MIN: Duration = Duration::from_secs(600);

fn id(raw: &str) -> AppId {
    AppId::parse(raw).unwrap()
}

fn music() -> AppId {
    id("com.apple.Music")
}

fn editor() -> AppId {
    id("com.microsoft.VSCode")
}

fn rhythm() -> Rhythm {
    Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday()).unwrap()
}

fn work() -> Duration {
    rhythm().work().as_duration()
}

fn cycle() -> Cycle {
    let chosen = SparedApps::from_pairs([(music(), AppStatus::Ignored)]);
    let apps = AppStatuses::new(chosen, SafetyList::default());
    Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH).with_app_statuses(apps)
}

fn deadline_of(cycle: &Cycle) -> Option<Instant> {
    match cycle.state() {
        CycleState::Working {
            countdown: Countdown::Running { deadline },
        } => Some(deadline),
        _ => None,
    }
}

fn frozen_remaining(cycle: &Cycle) -> Option<Duration> {
    match cycle.state() {
        CycleState::Working {
            countdown: Countdown::Frozen { remaining },
        } => Some(remaining),
        _ => None,
    }
}

#[test]
fn an_ignored_app_in_front_freezes_the_work_countdown() {
    let mut cycle = cycle();
    let at = Instant::EPOCH.plus(TEN_MIN);

    cycle.observe_foreground(at, Some(&music()));

    assert_eq!(frozen_remaining(&cycle), Some(work() - TEN_MIN));
    assert_eq!(cycle.freeze_reason(), Some(FreezeReason::IgnoredApp));
}

#[test]
fn another_app_coming_in_front_resumes_the_countdown_where_it_stopped() {
    let mut cycle = cycle();
    let froze_at = Instant::EPOCH.plus(TEN_MIN);
    cycle.observe_foreground(froze_at, Some(&music()));

    let back = froze_at.plus(TEN_MIN);
    cycle.observe_activity(back);
    assert!(
        frozen_remaining(&cycle).is_some(),
        "typing inside the ignored app does not count as work"
    );
    cycle.observe_foreground(back, Some(&editor()));

    assert_eq!(deadline_of(&cycle), Some(back.plus(work() - TEN_MIN)));
    assert_eq!(cycle.freeze_reason(), None);
}

#[test]
fn a_spared_or_blocked_app_in_front_counts_as_work() {
    let mut cycle = cycle();
    cycle.observe_foreground(Instant::EPOCH.plus(ONE_MIN), Some(&editor()));
    assert_eq!(deadline_of(&cycle), Some(Instant::EPOCH.plus(work())));
}

#[test]
fn an_unknown_foreground_identity_counts_as_work() {
    let mut cycle = cycle();
    cycle.observe_foreground(Instant::EPOCH.plus(ONE_MIN), None);
    assert_eq!(deadline_of(&cycle), Some(Instant::EPOCH.plus(work())));
}

#[test]
fn an_ignored_app_never_freezes_a_countdown_that_has_already_run_out() {
    let mut cycle = cycle();
    let due = Instant::EPOCH.plus(work());

    cycle.observe_foreground(due, Some(&music()));
    cycle.tick(due);

    assert!(matches!(cycle.state(), CycleState::Notice { .. }));
}

#[test]
fn an_ignored_app_does_not_touch_a_break() {
    let mut cycle = cycle();
    let break_at = Instant::EPOCH.plus(work()).plus(NOTICE);
    cycle.tick(break_at);
    cycle.observe_foreground(break_at, Some(&music()));
    assert!(matches!(cycle.state(), CycleState::BreakActive { .. }));
}

#[test]
fn ignoring_the_front_app_mid_cycle_does_not_freeze_this_cycle() {
    let chosen = SparedApps::new();
    let mut cycle = Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH)
        .with_app_statuses(AppStatuses::new(chosen, SafetyList::default()));
    cycle.set_app_status(music(), AppStatus::Ignored).unwrap();

    cycle.observe_foreground(Instant::EPOCH.plus(TEN_MIN), Some(&music()));

    assert!(
        deadline_of(&cycle).is_some(),
        "the weakening waits for the next cycle"
    );
}

#[test]
fn ceasing_to_ignore_the_front_app_thaws_at_the_next_reading() {
    let mut cycle = cycle();
    let froze_at = Instant::EPOCH.plus(TEN_MIN);
    cycle.observe_foreground(froze_at, Some(&music()));

    cycle.set_app_status(music(), AppStatus::Spared).unwrap();
    let later = froze_at.plus(ONE_MIN);
    cycle.observe_foreground(later, Some(&music()));

    assert_eq!(deadline_of(&cycle), Some(later.plus(work() - TEN_MIN)));
}

#[test]
fn idleness_over_an_ignored_app_keeps_the_countdown_frozen_until_both_end() {
    let mut cycle = cycle();
    let froze_at = Instant::EPOCH.plus(TEN_MIN);
    cycle.observe_activity(froze_at);
    cycle.observe_foreground(froze_at, Some(&music()));
    let idle_at = froze_at.plus(IDLE_FREEZE);
    cycle.freeze_if_idle(idle_at);
    assert_eq!(frozen_remaining(&cycle), Some(work() - TEN_MIN));

    let switched = idle_at.plus(ONE_MIN);
    cycle.observe_foreground(switched, Some(&editor()));
    assert_eq!(
        cycle.freeze_reason(),
        Some(FreezeReason::Idle),
        "still idle: the other app alone does not resume the countdown"
    );

    let typed = switched.plus(ONE_MIN);
    cycle.observe_activity(typed);
    assert_eq!(deadline_of(&cycle), Some(typed.plus(work() - TEN_MIN)));
}

#[test]
fn an_idle_freeze_reports_idle_as_its_reason() {
    let mut cycle = cycle();
    cycle.freeze_if_idle(Instant::EPOCH.plus(IDLE_FREEZE));
    assert_eq!(cycle.freeze_reason(), Some(FreezeReason::Idle));
}

#[test]
fn a_running_countdown_has_no_freeze_reason() {
    assert_eq!(cycle().freeze_reason(), None);
}
