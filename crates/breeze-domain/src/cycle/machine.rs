use crate::clock::Instant;
use crate::command_error::CommandError;
use crate::constants::{IDLE_FREEZE, NOTICE, RETURN_HOLD};
use crate::cycle::absence::{absence_verdict, Absence, AbsenceVerdict};
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

    pub fn change_rhythm(&mut self, rhythm: Rhythm) -> Result<(), CommandError> {
        if self.break_is_due() {
            return Err(CommandError::BreakDue);
        }
        self.pending_rhythm = (rhythm != self.rhythm).then_some(rhythm);
        Ok(())
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

    pub fn return_from_absence(&mut self, absence: Absence, now: Instant) -> AbsenceVerdict {
        let verdict = absence_verdict(self.state, self.rhythm, absence);
        match verdict {
            AbsenceVerdict::Nothing => {}
            AbsenceVerdict::CycleValidated => {
                if self.enter_next_work(now) {
                    self.outcomes.push(BreakOutcome::ValidatedByAbsence);
                }
            }
            AbsenceVerdict::BreakServed => {
                self.enter_returning(now);
            }
            AbsenceVerdict::BreakStartsAtWake => {
                self.enter_break(now);
            }
            AbsenceVerdict::PhaseContinues { remaining } => self.reanchor(remaining, now),
        }
        verdict
    }

    fn reanchor(&mut self, remaining: Duration, now: Instant) {
        let Some(deadline) = now.checked_plus(remaining) else {
            return;
        };
        self.state = match self.state {
            CycleState::Working {
                countdown: Countdown::Running { .. },
            } => CycleState::Working {
                countdown: Countdown::Running { deadline },
            },
            CycleState::Notice { .. } => CycleState::Notice { deadline },
            CycleState::BreakActive { severity, mode, .. } => CycleState::BreakActive {
                deadline,
                severity,
                mode,
            },
            CycleState::Returning { .. } => CycleState::Returning { deadline },
            CycleState::Inactive
            | CycleState::Suspended { .. }
            | CycleState::Working {
                countdown: Countdown::Frozen { .. } | Countdown::Due { .. },
            } => self.state,
        };
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
        if let Some(rhythm) = self.pending_rhythm.take() {
            self.rhythm = rhythm;
        }
        if let Some(severity) = self.pending_severity.take() {
            self.severity = severity;
        }
        let Some(deadline) = from.checked_plus(self.rhythm.work().as_duration()) else {
            return false;
        };
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
