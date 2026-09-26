use crate::snapshot::cycle_phase::CyclePhase;
use crate::snapshot::veil_mode::VeilMode;
use breeze_domain::{
    BreakMode, BreakOutcome, Countdown, Cycle, CycleState, FreezeReason, Instant, Severity,
};
use core::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CycleSnapshot {
    pub phase: CyclePhase,
    pub deadline: Option<Instant>,
    // Restant d'un décompte de travail gelé par inactivité (sans échéance tant qu'il dort).
    pub frozen_remaining: Option<Duration>,
    pub frozen_reason: Option<FreezeReason>,
    // Tenue de la pause Simple en cours ; `None` hors pause Simple.
    pub veil_mode: Option<VeilMode>,
    pub severity: Severity,
    pub chosen_severity: Severity,
    pub rhythm_pending: bool,
    pub served_breaks: u32,
    pub debt_minutes: u16,
}

impl CycleSnapshot {
    pub fn of(cycle: &Cycle) -> Self {
        let served = cycle
            .outcomes()
            .iter()
            .filter(|outcome| matches!(outcome, BreakOutcome::Served { .. }))
            .count();
        let (phase, deadline) = project(cycle.state());
        CycleSnapshot {
            phase,
            deadline,
            frozen_remaining: frozen_remaining(cycle.state()),
            frozen_reason: cycle.freeze_reason(),
            veil_mode: veil_mode(cycle.state()),
            severity: cycle.severity(),
            chosen_severity: cycle.chosen_severity(),
            rhythm_pending: cycle.rhythm_pending(),
            served_breaks: u32::try_from(served).unwrap_or(u32::MAX),
            debt_minutes: cycle.debt().minutes(),
        }
    }
}

fn veil_mode(state: CycleState) -> Option<VeilMode> {
    match state {
        CycleState::BreakActive {
            severity: Severity::Simple,
            mode: BreakMode::Nominal,
            ..
        } => Some(VeilMode::Windows),
        CycleState::BreakActive {
            severity: Severity::Simple,
            mode: BreakMode::Degraded(_),
            ..
        } => Some(VeilMode::FullScreen),
        _ => None,
    }
}

fn frozen_remaining(state: CycleState) -> Option<Duration> {
    match state {
        CycleState::Working {
            countdown: Countdown::Frozen { remaining },
        } => Some(remaining),
        _ => None,
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
