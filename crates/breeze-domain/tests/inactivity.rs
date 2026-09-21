use breeze_domain::constants::IDLE_FREEZE;
use breeze_domain::{ActiveDays, Countdown, Cycle, CycleState, Instant, Minutes, Rhythm, Severity};
use core::time::Duration;

const TEN_MIN: Duration = Duration::from_secs(600);
const A_LONG_WHILE: Duration = Duration::from_secs(6000);

fn rhythm() -> Rhythm {
    Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday()).unwrap()
}

fn is_frozen(cycle: &Cycle) -> bool {
    matches!(
        cycle.state(),
        CycleState::Working {
            countdown: Countdown::Frozen { .. }
        }
    )
}

#[test]
fn work_freezes_once_the_idle_threshold_passes() {
    let mut cycle = Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH);
    cycle.freeze_if_idle(Instant::EPOCH.plus(IDLE_FREEZE));
    assert!(is_frozen(&cycle));
}

#[test]
fn recent_activity_keeps_the_work_running() {
    let mut cycle = Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH);
    let now = Instant::EPOCH.plus(IDLE_FREEZE).plus(TEN_MIN);
    cycle.observe_activity(now);
    cycle.freeze_if_idle(now);
    assert!(matches!(
        cycle.state(),
        CycleState::Working {
            countdown: Countdown::Running { .. }
        }
    ));
}

#[test]
fn activity_after_a_freeze_thaws_and_keeps_the_remaining() {
    let r = rhythm();
    let work = r.work().as_duration();
    let mut cycle = Cycle::start(r, Severity::Simple, Instant::EPOCH);
    let froze_at = Instant::EPOCH.plus(IDLE_FREEZE);
    cycle.freeze_if_idle(froze_at);
    assert!(is_frozen(&cycle));

    let back = froze_at.plus(TEN_MIN);
    cycle.observe_activity(back);
    match cycle.state() {
        CycleState::Working {
            countdown: Countdown::Running { deadline },
        } => assert_eq!(deadline, back.plus(work - IDLE_FREEZE)),
        other => panic!("expected Running, got {other:?}"),
    }
}

#[test]
fn a_frozen_phase_never_reaches_a_break_on_its_own() {
    let mut cycle = Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH);
    cycle.freeze_if_idle(Instant::EPOCH.plus(IDLE_FREEZE));
    cycle.tick(Instant::EPOCH.plus(A_LONG_WHILE));
    assert!(is_frozen(&cycle));
}

#[test]
fn idleness_never_freezes_once_the_work_deadline_is_reached() {
    let r = rhythm();
    let work = r.work().as_duration();
    let mut cycle = Cycle::start(r, Severity::Simple, Instant::EPOCH);
    cycle.freeze_if_idle(Instant::EPOCH.plus(work));
    assert!(
        matches!(
            cycle.state(),
            CycleState::Working {
                countdown: Countdown::Running { .. }
            }
        ),
        "a due break is never turned into a frozen phase"
    );
    cycle.tick(Instant::EPOCH.plus(work));
    assert!(matches!(cycle.state(), CycleState::Notice { .. }));
}

#[test]
fn only_a_running_work_phase_freezes_not_the_notice() {
    let r = rhythm();
    let notice_at = Instant::EPOCH.plus(r.work().as_duration());
    let mut cycle = Cycle::start(r, Severity::Simple, Instant::EPOCH);
    cycle.tick(notice_at);
    cycle.freeze_if_idle(notice_at.plus(IDLE_FREEZE));
    assert!(matches!(cycle.state(), CycleState::Notice { .. }));
}

fn at(n: u64) -> Instant {
    Instant::at_secs(n)
}

#[test]
fn a_freeze_right_after_a_served_break_never_exceeds_the_work_duration() {
    let mut cycle = Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH);
    // Un cycle complet (50/10) jusqu'au travail suivant : 3000 travail, 60 préavis,
    // 600 pause, 3 retour → nouveau travail à 3663.
    cycle.tick(at(3663));
    assert!(matches!(
        cycle.state(),
        CycleState::Working {
            countdown: Countdown::Running { .. }
        }
    ));
    // Inactif depuis le début du nouveau travail : le restant gelé ne peut pas dépasser
    // la durée de travail (sinon `last_activity` datait d'avant ce décompte).
    cycle.freeze_if_idle(at(3663).plus(IDLE_FREEZE));
    match cycle.state() {
        CycleState::Working {
            countdown: Countdown::Frozen { remaining },
        } => assert!(
            remaining <= rhythm().work().as_duration(),
            "restant gelé {remaining:?} > durée de travail"
        ),
        other => panic!("expected a frozen work, got {other:?}"),
    }
}

#[test]
fn a_freeze_right_after_resume_never_exceeds_the_frozen_remaining() {
    let mut cycle = Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH);
    // Suspendre tôt (à 60 s) : 2940 s de travail gelées. Reprendre bien plus tard.
    cycle.suspend(at(60), at(4000)).unwrap();
    cycle.resume(at(4000)).unwrap();
    cycle.freeze_if_idle(at(4000).plus(IDLE_FREEZE));
    match cycle.state() {
        CycleState::Working {
            countdown: Countdown::Frozen { remaining },
        } => assert!(
            remaining <= Duration::from_secs(2940),
            "restant gelé {remaining:?} > restant repris"
        ),
        other => panic!("expected a frozen work, got {other:?}"),
    }
}
