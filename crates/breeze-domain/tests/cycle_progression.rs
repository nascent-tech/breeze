use breeze_domain::constants::{NOTICE, RETURN_HOLD};
use breeze_domain::{
    ActiveDays, BreakOutcome, Countdown, Cycle, CycleState, Instant, Minutes, Rhythm, Severity,
};
use core::time::Duration;

const ONE_SEC: Duration = Duration::from_secs(1);
const LATE: Duration = Duration::from_secs(30);
const REPLAYED_CYCLES: u32 = 5;

fn rhythm() -> Rhythm {
    Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday()).unwrap()
}

#[test]
fn steps_through_each_state_in_order_and_records_one_served_break() {
    let r = rhythm();
    let work = r.work().as_duration();
    let pause = r.pause().as_duration();
    let notice_at = Instant::EPOCH.plus(work);
    let break_at = notice_at.plus(NOTICE);
    let return_at = break_at.plus(pause);
    let next_at = return_at.plus(RETURN_HOLD);

    let mut cycle = Cycle::start(r, Severity::Simple, Instant::EPOCH);
    assert!(matches!(
        cycle.state(),
        CycleState::Working {
            countdown: Countdown::Running { .. }
        }
    ));

    cycle.tick(Instant::EPOCH.plus(work - ONE_SEC));
    assert!(matches!(cycle.state(), CycleState::Working { .. }));

    cycle.tick(notice_at);
    assert!(matches!(cycle.state(), CycleState::Notice { .. }));

    cycle.tick(notice_at.plus(NOTICE - ONE_SEC));
    assert!(matches!(cycle.state(), CycleState::Notice { .. }));

    cycle.tick(break_at);
    assert!(matches!(cycle.state(), CycleState::BreakActive { .. }));

    cycle.tick(break_at.plus(pause - ONE_SEC));
    assert!(
        matches!(cycle.state(), CycleState::BreakActive { .. }),
        "a break is never cut short before its deadline"
    );

    cycle.tick(return_at);
    assert!(matches!(cycle.state(), CycleState::Returning { .. }));
    assert_eq!(cycle.outcomes(), &[BreakOutcome::Served]);

    cycle.tick(next_at);
    assert!(matches!(
        cycle.state(),
        CycleState::Working {
            countdown: Countdown::Running { .. }
        }
    ));
}

#[test]
fn a_late_tick_stretches_neither_the_notice_nor_the_break() {
    let r = rhythm();
    let work = r.work().as_duration();
    let pause = r.pause().as_duration();
    let notice_at = Instant::EPOCH.plus(work);
    let break_at = notice_at.plus(NOTICE);
    let mut cycle = Cycle::start(r, Severity::Simple, Instant::EPOCH);

    cycle.tick(notice_at.plus(LATE));
    match cycle.state() {
        CycleState::Notice { deadline } => assert_eq!(deadline, break_at),
        other => panic!("expected Notice anchored to break_at, got {other:?}"),
    }

    cycle.tick(break_at.plus(LATE));
    match cycle.state() {
        CycleState::BreakActive { deadline, .. } => assert_eq!(deadline, break_at.plus(pause)),
        other => panic!("expected BreakActive anchored to break_at + pause, got {other:?}"),
    }
}

#[test]
fn replaying_whole_cycles_credits_one_served_break_each() {
    let r = rhythm();
    let cycle_len = r.work().as_duration() + NOTICE + r.pause().as_duration() + RETURN_HOLD;
    let mut cycle = Cycle::start(r, Severity::Hardcore, Instant::EPOCH);

    cycle.tick(Instant::EPOCH.plus(cycle_len * REPLAYED_CYCLES));

    assert_eq!(cycle.outcomes().len(), REPLAYED_CYCLES as usize);
    assert!(matches!(
        cycle.state(),
        CycleState::Working {
            countdown: Countdown::Running { .. }
        }
    ));
}

#[test]
fn the_break_carries_the_chosen_severity() {
    let r = rhythm();
    let break_at = Instant::EPOCH.plus(r.work().as_duration()).plus(NOTICE);
    let mut cycle = Cycle::start(r, Severity::Hardcore, Instant::EPOCH);
    cycle.tick(break_at);
    assert!(matches!(
        cycle.state(),
        CycleState::BreakActive {
            severity: Severity::Hardcore,
            ..
        }
    ));
}
