use breeze_domain::{
    ActiveDays, BreakOutcome, CommandError, Countdown, Cycle, CycleState, Instant,
    InterruptionDoor, Minutes, Rhythm, Severity,
};
use core::time::Duration;

fn rhythm() -> Rhythm {
    Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday()).unwrap()
}

fn at(n: u64) -> Instant {
    Instant::at_secs(n)
}

fn hardcore_break() -> Cycle {
    let mut cycle = Cycle::start(rhythm(), Severity::Hardcore, Instant::EPOCH);
    cycle.tick(at(3000));
    cycle.tick(at(3060));
    assert!(matches!(cycle.state(), CycleState::BreakActive { .. }));
    cycle
}

#[test]
fn the_gesture_interrupts_a_hardcore_break_straight_into_a_full_work_cycle() {
    let mut cycle = hardcore_break();

    cycle.interrupt_break(at(3300)).unwrap();

    // Comptée interrompue, pas servie, avec le temps de pause qui restait.
    assert_eq!(
        cycle.outcomes(),
        &[BreakOutcome::Interrupted {
            unserved: Duration::from_secs(360),
            door: InterruptionDoor::HardcoreExitGesture,
        }]
    );
    // Retour direct en travail, décompte plein, sans les 3 s de [RETOUR].
    match cycle.state() {
        CycleState::Working {
            countdown: Countdown::Running { deadline },
        } => assert_eq!(deadline, at(3300).plus(rhythm().work().as_duration())),
        other => panic!("expected a full working cycle, got {other:?}"),
    }
}

#[test]
fn a_break_whose_deadline_has_passed_is_served_not_interrupted() {
    let mut cycle = hardcore_break();

    // L'échéance de la pause (3060 + 600 = 3660) est atteinte : le geste ne peut plus rien.
    assert_eq!(
        cycle.interrupt_break(at(3660)),
        Err(CommandError::NotInterruptible)
    );
    assert_eq!(cycle.outcomes(), &[BreakOutcome::Served]);
    assert!(matches!(cycle.state(), CycleState::Returning { .. }));
}

#[test]
fn a_simple_break_has_no_exit_gesture() {
    let mut cycle = Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH);
    cycle.tick(at(3000));
    cycle.tick(at(3060));
    let before = cycle.state();

    assert_eq!(
        cycle.interrupt_break(at(3300)),
        Err(CommandError::NotInterruptible)
    );
    assert_eq!(cycle.state(), before);
    assert!(cycle.outcomes().is_empty());
}

#[test]
fn the_gesture_is_refused_outside_a_break() {
    let mut cycle = Cycle::start(rhythm(), Severity::Hardcore, Instant::EPOCH);

    assert_eq!(
        cycle.interrupt_break(at(10)),
        Err(CommandError::NotInterruptible)
    );
    assert!(matches!(cycle.state(), CycleState::Working { .. }));
}
