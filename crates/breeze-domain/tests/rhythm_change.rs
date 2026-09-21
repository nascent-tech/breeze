use breeze_domain::constants::{NOTICE, RETURN_HOLD};
use breeze_domain::{
    ActiveDays, CommandError, Countdown, Cycle, CycleState, Instant, Minutes, Rhythm, Severity,
};

fn rhythm(work: u16, pause: u16) -> Rhythm {
    Rhythm::new(Minutes(work), Minutes(pause), None, ActiveDays::everyday()).unwrap()
}

#[test]
fn a_rhythm_change_applies_at_the_next_cycle_not_the_current_one() {
    let first = rhythm(50, 10);
    let work = first.work().as_duration();
    let pause = first.pause().as_duration();
    let mut cycle = Cycle::start(first, Severity::Simple, Instant::EPOCH);

    cycle.change_rhythm(rhythm(25, 5)).unwrap();
    assert_eq!(cycle.configured_rhythm().work(), Minutes(25));
    assert_eq!(cycle.rhythm(), first);

    let first_break = Instant::EPOCH.plus(work).plus(NOTICE);
    cycle.tick(first_break);
    match cycle.state() {
        CycleState::BreakActive { deadline, .. } => {
            assert_eq!(deadline, first_break.plus(pause));
        }
        other => panic!("expected the current break to keep the old pause, got {other:?}"),
    }

    let next_work = first_break.plus(pause).plus(RETURN_HOLD);
    cycle.tick(next_work);
    match cycle.state() {
        CycleState::Working {
            countdown: Countdown::Running { deadline },
        } => assert_eq!(deadline, next_work.plus(rhythm(25, 5).work().as_duration())),
        other => panic!("expected Working on the new rhythm, got {other:?}"),
    }
    assert_eq!(cycle.configured_rhythm(), rhythm(25, 5));
    assert_eq!(cycle.rhythm(), rhythm(25, 5));
}

#[test]
fn the_configured_rhythm_is_the_current_one_until_a_change() {
    let cycle = Cycle::start(rhythm(50, 10), Severity::Simple, Instant::EPOCH);
    assert_eq!(cycle.configured_rhythm().pause(), Minutes(10));
}

#[test]
fn a_rhythm_change_is_refused_while_a_break_is_due() {
    let first = rhythm(50, 10);
    let mut cycle = Cycle::start(first, Severity::Simple, Instant::EPOCH);

    cycle.tick(Instant::EPOCH.plus(first.work().as_duration()));

    assert_eq!(
        cycle.change_rhythm(rhythm(25, 5)),
        Err(CommandError::BreakDue)
    );
    assert_eq!(cycle.configured_rhythm(), first);
}
