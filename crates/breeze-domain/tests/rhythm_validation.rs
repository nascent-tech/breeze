use breeze_domain::{ActiveDays, Minutes, Rhythm, RhythmError, TimeRange, Weekday};

fn days() -> ActiveDays {
    ActiveDays::everyday()
}

#[test]
fn accepts_a_sane_rhythm() {
    assert!(Rhythm::new(Minutes(50), Minutes(10), None, days()).is_ok());
}

#[test]
fn rejects_a_pause_longer_than_the_work() {
    let err = Rhythm::new(Minutes(10), Minutes(20), None, days()).unwrap_err();
    assert_eq!(err, RhythmError::PauseLongerThanWork);
}

#[test]
fn rejects_work_outside_its_bounds() {
    let too_short = Rhythm::new(Minutes(WORK_UNDER), Minutes(1), None, days()).unwrap_err();
    let too_long = Rhythm::new(Minutes(WORK_OVER), Minutes(10), None, days()).unwrap_err();
    assert_eq!(too_short, RhythmError::WorkOutOfBounds);
    assert_eq!(too_long, RhythmError::WorkOutOfBounds);
}

#[test]
fn rejects_a_pause_outside_its_bounds() {
    let too_short = Rhythm::new(Minutes(50), Minutes(PAUSE_UNDER), None, days()).unwrap_err();
    let too_long = Rhythm::new(Minutes(180), Minutes(PAUSE_OVER), None, days()).unwrap_err();
    assert_eq!(too_short, RhythmError::PauseOutOfBounds);
    assert_eq!(too_long, RhythmError::PauseOutOfBounds);
}

#[test]
fn rejects_a_minute_outside_the_day() {
    assert_eq!(
        TimeRange::from_minutes(0, MINUTES_IN_DAY).unwrap_err(),
        RhythmError::DegenerateRange
    );
}

#[test]
fn rejects_an_empty_set_of_active_days() {
    assert_eq!(
        ActiveDays::from_mask(0).unwrap_err(),
        RhythmError::NoActiveDay
    );
}

#[test]
fn keeps_only_the_requested_days() {
    let only_monday = ActiveDays::from_mask(Weekday::Monday.bit()).unwrap();
    assert!(only_monday.contains(Weekday::Monday));
    assert!(!only_monday.contains(Weekday::Sunday));
}

#[test]
fn rejects_a_degenerate_time_range() {
    assert_eq!(
        TimeRange::from_minutes(NOON, NOON).unwrap_err(),
        RhythmError::DegenerateRange
    );
}

#[test]
fn a_range_crossing_midnight_is_accepted_and_answers_containment() {
    let night = TimeRange::from_minutes(EVENING, EARLY_MORNING).unwrap();
    assert!(night.crosses_midnight());
    assert!(night.contains(EVENING));
    assert!(night.contains(EARLY_MORNING - 1));
    assert!(!night.contains(NOON));
}

const WORK_UNDER: u16 = 4;
const WORK_OVER: u16 = 181;
const PAUSE_UNDER: u16 = 0;
const PAUSE_OVER: u16 = 61;
const NOON: u16 = 720;
const EVENING: u16 = 1320;
const EARLY_MORNING: u16 = 120;
const MINUTES_IN_DAY: u16 = 1440;
