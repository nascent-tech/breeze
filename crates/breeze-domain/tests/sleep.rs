use breeze_domain::constants::RETURN_HOLD;
use breeze_domain::{
    Absence, AbsenceVerdict, ActiveDays, BreakOutcome, Countdown, Cycle, CycleState, Instant,
    Minutes, Rhythm, Severity, TimeRange, Weekday,
};
use core::time::Duration;

const WEEKDAYS: u8 = 0b0001_1111;
const WORK: Duration = Duration::from_secs(3000);

fn secs(n: u64) -> Duration {
    Duration::from_secs(n)
}

fn at(n: u64) -> Instant {
    Instant::at_secs(n)
}

fn hours(n: u64) -> Duration {
    secs(n * 3600)
}

fn minute(hour: u16, minute: u16) -> u16 {
    hour * 60 + minute
}

fn office_hours() -> Rhythm {
    Rhythm::new(
        Minutes(50),
        Minutes(10),
        Some(TimeRange::from_minutes(minute(9, 0), minute(18, 0)).unwrap()),
        ActiveDays::from_mask(WEEKDAYS).unwrap(),
    )
    .unwrap()
}

fn all_day() -> Rhythm {
    Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday()).unwrap()
}

fn working_until(deadline: Instant) -> CycleState {
    CycleState::Working {
        countdown: Countdown::Running { deadline },
    }
}

// L'horloge monotone ne compte pas la veille : au réveil, elle n'a avancé que d'un tick.
fn sleep(began_at: Instant, lasted: Duration) -> (Absence, Instant) {
    (Absence { began_at, lasted }, began_at.plus(secs(1)))
}

#[test]
fn a_lid_closed_at_ten_to_six_and_opened_the_next_morning_starts_a_full_work_cycle() {
    // Cycle lancé mardi 17:10 ; capot fermé à 17:50, rouvert mercredi 9:10.
    let mut cycle = Cycle::start(office_hours(), Severity::Simple, Instant::EPOCH);
    let (absence, wake) = sleep(at(2400), hours(15) + secs(20 * 60));

    cycle.return_from_sleep(absence, wake, (Weekday::Tuesday, minute(17, 50)));
    assert_eq!(cycle.state(), CycleState::Inactive);
    cycle.observe_calendar(wake, true);

    assert_eq!(cycle.state(), working_until(wake.plus(WORK)));
}

#[test]
fn a_short_sleep_across_the_end_of_the_schedule_puts_the_work_to_rest() {
    let mut cycle = Cycle::start(office_hours(), Severity::Simple, Instant::EPOCH);
    let (absence, wake) = sleep(at(600), secs(5 * 60));

    let verdict = cycle.return_from_sleep(absence, wake, (Weekday::Tuesday, minute(17, 58)));

    assert!(matches!(verdict, AbsenceVerdict::PhaseContinues { .. }));
    assert_eq!(cycle.state(), CycleState::Inactive);
}

#[test]
fn a_sleep_that_stays_within_the_hours_keeps_the_countdown() {
    let mut cycle = Cycle::start(office_hours(), Severity::Simple, Instant::EPOCH);
    let (absence, wake) = sleep(at(600), secs(5 * 60));

    cycle.return_from_sleep(absence, wake, (Weekday::Tuesday, minute(10, 0)));

    assert_eq!(
        cycle.state(),
        working_until(wake.plus(secs(3000 - 600 - 300)))
    );
}

#[test]
fn a_sleep_longer_than_the_pause_during_work_validates_the_cycle() {
    let mut cycle = Cycle::start(all_day(), Severity::Simple, Instant::EPOCH);
    let (absence, wake) = sleep(at(1000), secs(15 * 60));

    let verdict = cycle.return_from_sleep(absence, wake, (Weekday::Tuesday, minute(14, 0)));

    assert_eq!(verdict, AbsenceVerdict::CycleValidated);
    assert_eq!(cycle.outcomes(), &[BreakOutcome::ValidatedByAbsence]);
    assert_eq!(cycle.state(), working_until(wake.plus(WORK)));
}

#[test]
fn a_hardcore_break_begun_before_sleep_is_served_when_the_wake_comes_after_its_deadline() {
    let mut cycle = Cycle::start(office_hours(), Severity::Hardcore, Instant::EPOCH);
    cycle.tick(at(3000));
    cycle.tick(at(3060));
    assert!(matches!(cycle.state(), CycleState::BreakActive { .. }));
    // Pause entamée mardi 17:55, veille jusqu'au lendemain matin.
    let (absence, wake) = sleep(at(3100), hours(15));

    let verdict = cycle.return_from_sleep(absence, wake, (Weekday::Tuesday, minute(17, 55)));

    assert_eq!(verdict, AbsenceVerdict::BreakServed);
    assert_eq!(
        cycle.outcomes(),
        &[BreakOutcome::Served { planned: secs(600) }]
    );
    assert_eq!(
        cycle.state(),
        CycleState::Returning {
            deadline: wake.plus(RETURN_HOLD)
        }
    );
}

#[test]
fn a_suspension_until_morning_resumes_on_time_despite_the_night_asleep() {
    let mut cycle = Cycle::start(all_day(), Severity::Simple, Instant::EPOCH);
    cycle.suspend(at(600), at(600).plus(hours(9))).unwrap();
    let (absence, wake) = sleep(at(700), hours(9));

    cycle.return_from_sleep(absence, wake, (Weekday::Tuesday, minute(22, 0)));
    cycle.tick(wake);

    assert_eq!(cycle.state(), working_until(wake.plus(secs(3000 - 600))));
}

#[test]
fn a_short_sleep_during_a_suspension_only_brings_its_end_closer() {
    let mut cycle = Cycle::start(all_day(), Severity::Simple, Instant::EPOCH);
    cycle.suspend(at(600), at(600).plus(hours(1))).unwrap();
    let (absence, wake) = sleep(at(700), secs(20 * 60));

    cycle.return_from_sleep(absence, wake, (Weekday::Tuesday, minute(14, 0)));

    assert_eq!(
        cycle.state(),
        CycleState::Suspended {
            resume_at: at(600).plus(hours(1)).minus(secs(20 * 60)),
            frozen: secs(3000 - 600),
        }
    );
}
