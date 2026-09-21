use crate::clock::Instant;
use crate::command_error::CommandError;
use crate::constants::{IDLE_FREEZE, NOTICE, RETURN_HOLD};
use crate::cycle::break_mode::BreakMode;
use crate::cycle::countdown::Countdown;
use crate::cycle::state::CycleState;
use crate::outcome::BreakOutcome;
use crate::settings::{Rhythm, Severity};
use core::time::Duration;

#[derive(Clone, Debug)]
pub struct Cycle {
    state: CycleState,
    rhythm: Rhythm,
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

    pub fn outcomes(&self) -> &[BreakOutcome] {
        &self.outcomes
    }

    pub fn observe_activity(&mut self, at: Instant) {
        self.last_activity = at;
        if let CycleState::Working {
            countdown: Countdown::Frozen { remaining },
        } = self.state
        {
            self.state = CycleState::Working {
                countdown: Countdown::Running {
                    deadline: at.plus(remaining),
                },
            };
        }
    }

    pub fn freeze_if_idle(&mut self, now: Instant) {
        let CycleState::Working {
            countdown: Countdown::Running { deadline },
        } = self.state
        else {
            return;
        };
        if now.elapsed_since(self.last_activity) < IDLE_FREEZE {
            return;
        }
        self.state = CycleState::Working {
            countdown: Countdown::Frozen {
                remaining: deadline.elapsed_since(now),
            },
        };
    }

    pub fn tick(&mut self, now: Instant) {
        while self.advance_once(now) {}
    }

    pub fn suspend(&mut self, now: Instant, resume_at: Instant) -> Result<(), CommandError> {
        let frozen = match self.state {
            CycleState::Working {
                countdown: Countdown::Running { deadline },
            } => deadline.elapsed_since(now),
            CycleState::Working {
                countdown: Countdown::Frozen { remaining },
            } => remaining,
            CycleState::Suspended { frozen, .. } => frozen,
            CycleState::Working {
                countdown: Countdown::Due { .. },
            }
            | CycleState::Notice { .. }
            | CycleState::BreakActive { .. }
            | CycleState::Returning { .. } => return Err(CommandError::BreakDue),
            CycleState::Inactive => return Err(CommandError::NotSuspendable),
        };
        self.state = CycleState::Suspended { resume_at, frozen };
        Ok(())
    }

    pub fn resume(&mut self, now: Instant) -> Result<(), CommandError> {
        let CycleState::Suspended { frozen, .. } = self.state else {
            return Err(CommandError::NotSuspended);
        };
        let Some(deadline) = now.checked_plus(frozen) else {
            return Err(CommandError::NotSuspended);
        };
        self.state = CycleState::Working {
            countdown: Countdown::Running { deadline },
        };
        Ok(())
    }

    pub fn change_severity(&mut self, severity: Severity) -> Result<(), CommandError> {
        if self.break_is_due() {
            return Err(CommandError::BreakDue);
        }
        match (self.severity, severity) {
            (Severity::Hardcore, Severity::Simple) => {
                self.pending_severity = Some(Severity::Simple);
            }
            _ => {
                self.severity = severity;
                self.pending_severity = None;
            }
        }
        Ok(())
    }

    fn break_is_due(&self) -> bool {
        matches!(
            self.state,
            CycleState::Working {
                countdown: Countdown::Due { .. }
            } | CycleState::Notice { .. }
                | CycleState::BreakActive { .. }
                | CycleState::Returning { .. }
        )
    }

    fn advance_once(&mut self, now: Instant) -> bool {
        match self.state {
            CycleState::Working {
                countdown: Countdown::Running { deadline },
            } if now.has_reached(deadline) => self.enter_notice(deadline),
            CycleState::Notice { deadline } if now.has_reached(deadline) => {
                self.enter_break(deadline)
            }
            CycleState::BreakActive { deadline, .. } if now.has_reached(deadline) => {
                self.enter_returning(deadline)
            }
            CycleState::Returning { deadline } if now.has_reached(deadline) => {
                self.enter_next_work(deadline)
            }
            CycleState::Suspended { resume_at, frozen } if now.has_reached(resume_at) => {
                self.resume_from_suspension(now, frozen)
            }
            _ => false,
        }
    }

    fn enter_notice(&mut self, from: Instant) -> bool {
        let Some(deadline) = from.checked_plus(NOTICE) else {
            return false;
        };
        self.state = CycleState::Notice { deadline };
        true
    }

    fn enter_break(&mut self, from: Instant) -> bool {
        let Some(deadline) = from.checked_plus(self.rhythm.pause().as_duration()) else {
            return false;
        };
        self.state = CycleState::BreakActive {
            deadline,
            severity: self.severity,
            mode: BreakMode::Nominal,
        };
        true
    }

    fn enter_returning(&mut self, from: Instant) -> bool {
        let Some(deadline) = from.checked_plus(RETURN_HOLD) else {
            return false;
        };
        self.outcomes.push(BreakOutcome::Served);
        self.state = CycleState::Returning { deadline };
        true
    }

    fn enter_next_work(&mut self, from: Instant) -> bool {
        let Some(deadline) = from.checked_plus(self.rhythm.work().as_duration()) else {
            return false;
        };
        if let Some(pending) = self.pending_severity.take() {
            self.severity = pending;
        }
        self.state = CycleState::Working {
            countdown: Countdown::Running { deadline },
        };
        true
    }

    fn resume_from_suspension(&mut self, now: Instant, frozen: Duration) -> bool {
        let Some(deadline) = now.checked_plus(frozen) else {
            return false;
        };
        self.state = CycleState::Working {
            countdown: Countdown::Running { deadline },
        };
        true
    }
}
