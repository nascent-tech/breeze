use crate::snapshot::cycle_phase::CyclePhase;
use breeze_domain::{BreakOutcome, Countdown, Cycle, CycleState, Instant, Severity};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CycleSnapshot {
    pub phase: CyclePhase,
    pub deadline: Option<Instant>,
    pub severity: Severity,
    pub served_breaks: u32,
    pub debt_minutes: u16,
}

impl CycleSnapshot {
    pub fn of(cycle: &Cycle) -> Self {
        let served = cycle
            .outcomes()
            .iter()
            .filter(|outcome| matches!(outcome, BreakOutcome::Served))
            .count();
        let (phase, deadline) = project(cycle.state());
        CycleSnapshot {
            phase,
            deadline,
            severity: cycle.severity(),
            served_breaks: u32::try_from(served).unwrap_or(u32::MAX),
            debt_minutes: cycle.debt().minutes(),
        }
    }
}

fn project(state: CycleState) -> (CyclePhase, Option<Instant>) {
    match state {
        CycleState::Inactive => (CyclePhase::Inactive, None),
        CycleState::Working {
            countdown: Countdown::Running { deadline },
        } => (CyclePhase::Working, Some(deadline)),
        CycleState::Working { .. } => (CyclePhase::Working, None),
        CycleState::Notice { deadline } => (CyclePhase::Notice, Some(deadline)),
        CycleState::BreakActive { deadline, .. } => (CyclePhase::Break, Some(deadline)),
        CycleState::Returning { deadline } => (CyclePhase::Returning, Some(deadline)),
        CycleState::Suspended { resume_at, .. } => (CyclePhase::Suspended, Some(resume_at)),
    }
}
