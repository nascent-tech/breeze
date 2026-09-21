use crate::snapshot::cycle_phase::CyclePhase;
use breeze_domain::{BreakOutcome, Countdown, Cycle, CycleState, Instant, Severity};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CycleSnapshot {
    pub phase: CyclePhase,
    pub deadline: Option<Instant>,
    pub severity: Option<Severity>,
    pub served_breaks: u32,
}

impl CycleSnapshot {
    pub fn of(cycle: &Cycle) -> Self {
        let served = cycle
            .outcomes()
            .iter()
            .filter(|outcome| matches!(outcome, BreakOutcome::Served))
            .count();
        let (phase, deadline, severity) = project(cycle.state());
        CycleSnapshot {
            phase,
            deadline,
            severity,
            served_breaks: u32::try_from(served).unwrap_or(u32::MAX),
        }
    }
}

fn project(state: CycleState) -> (CyclePhase, Option<Instant>, Option<Severity>) {
    match state {
        CycleState::Inactive => (CyclePhase::Inactive, None, None),
        CycleState::Working {
            countdown: Countdown::Running { deadline },
        } => (CyclePhase::Working, Some(deadline), None),
        CycleState::Working { .. } => (CyclePhase::Working, None, None),
        CycleState::Notice { deadline } => (CyclePhase::Notice, Some(deadline), None),
        CycleState::BreakActive {
            deadline, severity, ..
        } => (CyclePhase::Break, Some(deadline), Some(severity)),
        CycleState::Returning { deadline } => (CyclePhase::Returning, Some(deadline), None),
    }
}
