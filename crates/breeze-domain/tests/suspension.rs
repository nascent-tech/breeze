use breeze_domain::constants::NOTICE;
use breeze_domain::{
    ActiveDays, CommandError, Countdown, Cycle, CycleState, Instant, Minutes, Rhythm, Severity,
};
use core::time::Duration;

const TEN_MIN: Duration = Duration::from_secs(600);
const FIFTEEN_MIN: Duration = Duration::from_secs(900);
const TWO_MIN: Duration = Duration::from_secs(120);

fn rhythm() -> Rhythm {
    Rhythm::new(Minutes(50), Minutes(10), None, ActiveDays::everyday()).unwrap()
}

#[test]
fn suspending_freezes_the_work_and_resuming_restores_the_remaining() {
    let r = rhythm();
    let work = r.work().as_duration();
    let mut cycle = Cycle::start(r, Severity::Simple, Instant::EPOCH);
    let at = Instant::EPOCH.plus(TEN_MIN);
    cycle.suspend(at, at.plus(FIFTEEN_MIN)).unwrap();
    assert!(matches!(cycle.state(), CycleState::Suspended { .. }));

    cycle.tick(at.plus(TWO_MIN));
    assert!(matches!(cycle.state(), CycleState::Suspended { .. }));

    let back = at.plus(TWO_MIN);
    cycle.resume(back).unwrap();
    match cycle.state() {
        CycleState::Working {
            countdown: Countdown::Running { deadline },
        } => assert_eq!(deadline, back.plus(work - TEN_MIN)),
        other => panic!("expected Working, got {other:?}"),
    }
}

#[test]
fn suspension_ends_by_itself_at_resume_at() {
    let mut cycle = Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH);
    let at = Instant::EPOCH.plus(TEN_MIN);
    let resume_at = at.plus(FIFTEEN_MIN);
    cycle.suspend(at, resume_at).unwrap();
    cycle.tick(resume_at);
    assert!(matches!(
        cycle.state(),
        CycleState::Working {
            countdown: Countdown::Running { .. }
        }
    ));
}

#[test]
fn a_due_break_refuses_suspension_and_severity_change() {
    let r = rhythm();
    let break_at = Instant::EPOCH.plus(r.work().as_duration()).plus(NOTICE);
    let mut cycle = Cycle::start(r, Severity::Simple, Instant::EPOCH);
    cycle.tick(break_at);
    assert_eq!(
        cycle.suspend(break_at, break_at).unwrap_err(),
        CommandError::BreakDue
    );
    assert_eq!(
        cycle.change_severity(Severity::Hardcore).unwrap_err(),
        CommandError::BreakDue
    );
}

#[test]
fn severity_changes_while_working_and_the_next_break_carries_it() {
    let r = rhythm();
    let break_at = Instant::EPOCH.plus(r.work().as_duration()).plus(NOTICE);
    let mut cycle = Cycle::start(r, Severity::Simple, Instant::EPOCH);
    cycle.change_severity(Severity::Hardcore).unwrap();
    assert_eq!(cycle.severity(), Severity::Hardcore);
    cycle.tick(break_at);
    assert!(matches!(
        cycle.state(),
        CycleState::BreakActive {
            severity: Severity::Hardcore,
            ..
        }
    ));
}

#[test]
fn resume_without_a_suspension_is_refused() {
    let mut cycle = Cycle::start(rhythm(), Severity::Simple, Instant::EPOCH);
    assert_eq!(
        cycle.resume(Instant::EPOCH).unwrap_err(),
        CommandError::NotSuspended
    );
}
