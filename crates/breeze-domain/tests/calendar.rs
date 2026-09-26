use breeze_domain::constants::{NOTICE, RETURN_HOLD};
use breeze_domain::{
    ActiveDays, Countdown, Cycle, CycleState, Instant, Minutes, NextStart, OffHours, Rhythm,
    Severity, TimeRange, Weekday,
};
use core::time::Duration;

const NINE: u16 = 9 * 60;
const SIX_PM: u16 = 18 * 60;
const TEN_PM: u16 = 22 * 60;
const TWO_AM: u16 = 2 * 60;
const WEEKDAYS: u8 = 0b0001_1111;
const MONDAY_ONLY: u8 = 0b0000_0001;
const TEN_MIN: Duration = Duration::from_secs(600);

fn rhythm_with(schedule: Option<TimeRange>, days: u8) -> Rhythm {
    Rhythm::new(
        Minutes(50),
        Minutes(10),
        schedule,
        ActiveDays::from_mask(days).unwrap(),
    )
    .unwrap()
}

fn office_hours() -> Rhythm {
    rhythm_with(
        Some(TimeRange::from_minutes(NINE, SIX_PM).unwrap()),
        WEEKDAYS,
    )
}

fn night_shift(days: u8) -> Rhythm {
    rhythm_with(Some(TimeRange::from_minutes(TEN_PM, TWO_AM).unwrap()), days)
}

fn is_working(cycle: &Cycle) -> bool {
    matches!(cycle.state(), CycleState::Working { .. })
}

#[test]
fn without_a_schedule_every_minute_of_an_active_day_counts() {
    let rhythm = rhythm_with(None, WEEKDAYS);
    assert!(rhythm.is_active_at(Weekday::Monday, 0));
    assert!(rhythm.is_active_at(Weekday::Friday, 23 * 60 + 59));
    assert_eq!(
        rhythm.off_hours_at(Weekday::Saturday, NINE),
        Some(OffHours::InactiveDay)
    );
}

#[test]
fn outside_the_schedule_of_an_active_day_is_off_hours() {
    let rhythm = office_hours();
    assert!(rhythm.is_active_at(Weekday::Tuesday, NINE));
    assert_eq!(
        rhythm.off_hours_at(Weekday::Tuesday, SIX_PM),
        Some(OffHours::OutsideSchedule)
    );
    assert_eq!(
        rhythm.off_hours_at(Weekday::Tuesday, NINE - 1),
        Some(OffHours::OutsideSchedule)
    );
}

#[test]
fn an_inactive_day_wins_over_the_schedule_as_the_reason() {
    assert_eq!(
        office_hours().off_hours_at(Weekday::Sunday, 20 * 60),
        Some(OffHours::InactiveDay)
    );
}

#[test]
fn the_hours_after_midnight_belong_to_the_day_the_range_began() {
    let rhythm = night_shift(MONDAY_ONLY);
    assert!(rhythm.is_active_at(Weekday::Monday, TEN_PM));
    assert!(rhythm.is_active_at(Weekday::Tuesday, 60));
    assert_eq!(
        rhythm.off_hours_at(Weekday::Monday, 60),
        Some(OffHours::InactiveDay)
    );
}

#[test]
fn the_next_start_is_later_today_before_the_range_opens() {
    assert_eq!(
        office_hours().next_start_after(Weekday::Wednesday, 7 * 60),
        NextStart {
            days_ahead: 0,
            weekday: Weekday::Wednesday,
            minute_of_day: NINE,
        }
    );
}

#[test]
fn the_next_start_is_tomorrow_once_the_range_has_closed() {
    assert_eq!(
        office_hours().next_start_after(Weekday::Wednesday, 19 * 60),
        NextStart {
            days_ahead: 1,
            weekday: Weekday::Thursday,
            minute_of_day: NINE,
        }
    );
}

#[test]
fn the_next_start_skips_the_weekend() {
    assert_eq!(
        office_hours().next_start_after(Weekday::Friday, 19 * 60),
        NextStart {
            days_ahead: 3,
            weekday: Weekday::Monday,
            minute_of_day: NINE,
        }
    );
}

#[test]
fn without_a_schedule_the_next_start_is_midnight_of_the_next_active_day() {
    assert_eq!(
        rhythm_with(None, WEEKDAYS).next_start_after(Weekday::Saturday, NINE),
        NextStart {
            days_ahead: 2,
            weekday: Weekday::Monday,
            minute_of_day: 0,
        }
    );
}

#[test]
fn a_single_active_day_already_started_comes_back_a_week_later() {
    assert_eq!(
        rhythm_with(None, MONDAY_ONLY).next_start_after(Weekday::Monday, 60),
        NextStart {
            days_ahead: 7,
            weekday: Weekday::Monday,
            minute_of_day: 0,
        }
    );
}

#[test]
fn working_goes_inactive_as_soon_as_the_hours_end() {
    let mut cycle = Cycle::start(office_hours(), Severity::Simple, Instant::EPOCH);
    cycle.observe_calendar(Instant::EPOCH.plus(TEN_MIN), false);
    assert_eq!(cycle.state(), CycleState::Inactive);
}

#[test]
fn a_frozen_countdown_also_goes_inactive() {
    let mut cycle = Cycle::start(office_hours(), Severity::Simple, Instant::EPOCH);
    cycle.freeze_if_idle(Instant::EPOCH.plus(TEN_MIN));
    assert!(matches!(
        cycle.state(),
        CycleState::Working {
            countdown: Countdown::Frozen { .. }
        }
    ));
    cycle.observe_calendar(Instant::EPOCH.plus(TEN_MIN), false);
    assert_eq!(cycle.state(), CycleState::Inactive);
}

#[test]
fn a_started_break_runs_to_its_end_then_the_cycle_goes_inactive() {
    let rhythm = office_hours();
    let work = rhythm.work().as_duration();
    let pause = rhythm.pause().as_duration();
    let mut cycle = Cycle::start(rhythm, Severity::Simple, Instant::EPOCH);
    let break_starts = Instant::EPOCH.plus(work).plus(NOTICE);
    cycle.tick(break_starts);

    cycle.observe_calendar(break_starts, false);
    assert!(matches!(cycle.state(), CycleState::BreakActive { .. }));

    let back = break_starts.plus(pause).plus(RETURN_HOLD);
    cycle.tick(back);
    assert!(is_working(&cycle));
    cycle.observe_calendar(back, false);
    assert_eq!(cycle.state(), CycleState::Inactive);
}

#[test]
fn the_notice_is_not_cut_short_by_the_end_of_the_hours() {
    let rhythm = office_hours();
    let notice_at = Instant::EPOCH.plus(rhythm.work().as_duration());
    let mut cycle = Cycle::start(rhythm, Severity::Simple, Instant::EPOCH);
    cycle.tick(notice_at);
    cycle.observe_calendar(notice_at, false);
    assert!(matches!(cycle.state(), CycleState::Notice { .. }));
}

#[test]
fn a_suspension_stays_suspended_outside_the_hours() {
    let mut cycle = Cycle::start(office_hours(), Severity::Simple, Instant::EPOCH);
    let resume_at = Instant::EPOCH.plus(TEN_MIN);
    cycle.suspend(Instant::EPOCH, resume_at).unwrap();
    cycle.observe_calendar(Instant::EPOCH, false);
    assert!(matches!(cycle.state(), CycleState::Suspended { .. }));
}

#[test]
fn returning_into_the_hours_starts_a_fresh_work_countdown() {
    let rhythm = office_hours();
    let mut cycle = Cycle::start(rhythm, Severity::Simple, Instant::EPOCH);
    cycle.observe_calendar(Instant::EPOCH, false);

    let morning = Instant::EPOCH.plus(Duration::from_secs(15 * 3600));
    cycle.observe_calendar(morning, true);
    assert_eq!(
        cycle.state(),
        CycleState::Working {
            countdown: Countdown::Running {
                deadline: morning.plus(rhythm.work().as_duration()),
            },
        }
    );
}

#[test]
fn the_fresh_cycle_takes_the_pending_rhythm_and_severity() {
    let mut cycle = Cycle::start(office_hours(), Severity::Hardcore, Instant::EPOCH);
    let shorter = Rhythm::new(
        Minutes(25),
        Minutes(5),
        office_hours().schedule(),
        ActiveDays::from_mask(WEEKDAYS).unwrap(),
    )
    .unwrap();
    cycle.change_rhythm(shorter).unwrap();
    cycle.change_severity(Severity::Simple).unwrap();
    assert!(cycle.rhythm_pending());

    cycle.observe_calendar(Instant::EPOCH, false);
    cycle.observe_calendar(Instant::EPOCH.plus(TEN_MIN), true);

    assert_eq!(cycle.rhythm(), shorter);
    assert!(!cycle.rhythm_pending());
    assert_eq!(cycle.severity(), Severity::Simple);
}

#[test]
fn inside_the_hours_the_calendar_leaves_a_running_cycle_alone() {
    let mut cycle = Cycle::start(office_hours(), Severity::Simple, Instant::EPOCH);
    let before = cycle.state();
    cycle.observe_calendar(Instant::EPOCH.plus(TEN_MIN), true);
    assert_eq!(cycle.state(), before);
}

#[test]
fn an_interval_that_reaches_the_end_of_the_schedule_leaves_the_active_hours() {
    let rhythm = office_hours();
    assert!(!rhythm.is_active_throughout(Weekday::Tuesday, SIX_PM - 10, TEN_MIN * 2));
    assert!(rhythm.is_active_throughout(Weekday::Tuesday, NINE, TEN_MIN * 6));
}

#[test]
fn without_a_schedule_every_day_active_nothing_is_ever_left() {
    let rhythm = rhythm_with(None, 0b0111_1111);
    let three_days = Duration::from_secs(3 * 24 * 3600);
    assert!(rhythm.is_active_throughout(Weekday::Sunday, 23 * 60, three_days));
}

#[test]
fn a_night_shift_stays_active_past_midnight_into_an_inactive_day() {
    let rhythm = night_shift(MONDAY_ONLY);
    let two_hours = Duration::from_secs(2 * 3600);
    assert!(rhythm.is_active_throughout(Weekday::Monday, 23 * 60, two_hours));
}
