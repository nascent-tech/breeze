use super::Cycle;
use crate::clock::Instant;
use crate::constants::{NOTICE, RETURN_HOLD};
use crate::cycle::break_mode::BreakMode;
use crate::cycle::countdown::Countdown;
use crate::cycle::state::CycleState;
use crate::outcome::BreakOutcome;
use core::time::Duration;

impl Cycle {
    pub fn tick(&mut self, now: Instant) {
        while self.advance_once(now) {}
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

    pub(super) fn enter_break(&mut self, from: Instant) -> bool {
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

    pub(super) fn enter_returning(&mut self, from: Instant) -> bool {
        let Some(deadline) = from.checked_plus(RETURN_HOLD) else {
            return false;
        };
        self.outcomes.push(BreakOutcome::Served);
        self.state = CycleState::Returning { deadline };
        true
    }

    pub(super) fn enter_next_work(&mut self, from: Instant) -> bool {
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
