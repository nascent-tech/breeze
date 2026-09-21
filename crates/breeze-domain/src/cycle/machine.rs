use crate::clock::Instant;
use crate::constants::{NOTICE, RETURN_HOLD};
use crate::cycle::break_mode::BreakMode;
use crate::cycle::countdown::Countdown;
use crate::cycle::state::CycleState;
use crate::outcome::BreakOutcome;
use crate::settings::{Rhythm, Severity};

#[derive(Clone, Debug)]
pub struct Cycle {
    state: CycleState,
    rhythm: Rhythm,
    severity: Severity,
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
            outcomes: Vec::new(),
        }
    }

    pub fn state(&self) -> CycleState {
        self.state
    }

    pub fn outcomes(&self) -> &[BreakOutcome] {
        &self.outcomes
    }

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
        self.state = CycleState::Working {
            countdown: Countdown::Running { deadline },
        };
        true
    }
}
