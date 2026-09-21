mod commands;
mod transitions;

use crate::clock::Instant;
use crate::constants::IDLE_FREEZE;
use crate::cycle::countdown::Countdown;
use crate::cycle::state::CycleState;
use crate::outcome::BreakOutcome;
use crate::settings::{Rhythm, Severity};

#[derive(Clone, Debug)]
pub struct Cycle {
    state: CycleState,
    rhythm: Rhythm,
    pending_rhythm: Option<Rhythm>,
    severity: Severity,
    pending_severity: Option<Severity>,
    last_activity: Instant,
    outcomes: Vec<BreakOutcome>,
}

impl Cycle {
    pub fn start(rhythm: Rhythm, severity: Severity, now: Instant) -> Self {
        let deadline = now.plus(rhythm.work().as_duration());
        Cycle {
            state: CycleState::Working {
                countdown: Countdown::Running { deadline },
            },
            rhythm,
            pending_rhythm: None,
            severity,
            pending_severity: None,
            last_activity: now,
            outcomes: Vec::new(),
        }
    }

    pub fn state(&self) -> CycleState {
        self.state
    }

    pub fn severity(&self) -> Severity {
        self.severity
    }

    pub fn chosen_severity(&self) -> Severity {
        self.pending_severity.unwrap_or(self.severity)
    }

    pub fn configured_rhythm(&self) -> Rhythm {
        self.pending_rhythm.unwrap_or(self.rhythm)
    }

    pub fn rhythm(&self) -> Rhythm {
        self.rhythm
    }

    pub fn outcomes(&self) -> &[BreakOutcome] {
        &self.outcomes
    }

    pub fn observe_activity(&mut self, at: Instant) {
        self.last_activity = at;
        let CycleState::Working {
            countdown: Countdown::Frozen { remaining },
        } = self.state
        else {
            return;
        };
        let Some(deadline) = at.checked_plus(remaining) else {
            return;
        };
        self.state = CycleState::Working {
            countdown: Countdown::Running { deadline },
        };
    }

    pub fn freeze_if_idle(&mut self, now: Instant) {
        let CycleState::Working {
            countdown: Countdown::Running { deadline },
        } = self.state
        else {
            return;
        };
        if now.has_reached(deadline) {
            return;
        }
        let freeze_at = self.last_activity.plus(IDLE_FREEZE);
        if now < freeze_at {
            return;
        }
        self.state = CycleState::Working {
            countdown: Countdown::Frozen {
                remaining: deadline.elapsed_since(freeze_at),
            },
        };
    }
}
