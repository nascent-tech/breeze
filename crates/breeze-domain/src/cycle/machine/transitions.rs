use super::Cycle;
use crate::clock::Instant;
use crate::constants::{NOTICE, PAUSE_MAX, RETURN_HOLD};
use crate::cycle::break_mode::BreakMode;
use crate::cycle::countdown::Countdown;
use crate::cycle::state::CycleState;
use crate::debt::Settlement;
use crate::outcome::BreakOutcome;
use crate::settings::Minutes;
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
                self.enter_returning(deadline, Settlement::Repaid)
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
        let pause = self.rhythm.pause().as_duration();
        // La dette rembourse en allongeant la pause, sans dépasser la durée de travail
        // ni la borne haute de pause (§9.2, §12.2).
        let ceiling = Minutes(self.rhythm.work().count().min(PAUSE_MAX)).as_duration();
        let extension = self.debt.extension_within(ceiling.saturating_sub(pause));
        let Some(deadline) = from.checked_plus(pause.saturating_add(extension)) else {
            return false;
        };
        self.debt.absorb(extension);
        self.state = CycleState::BreakActive {
            deadline,
            severity: self.severity,
            mode: BreakMode::Nominal,
        };
        true
    }

    pub(super) fn enter_returning(&mut self, from: Instant, settlement: Settlement) -> bool {
        let Some(deadline) = from.checked_plus(RETURN_HOLD) else {
            return false;
        };
        let pause = self.rhythm.pause().as_duration();
        let planned = match settlement {
            Settlement::Repaid => {
                let lent = self.debt.absorbed();
                self.debt.settle();
                pause.saturating_add(lent)
            }
            Settlement::Frozen => {
                self.debt.freeze();
                pause
            }
        };
        self.outcomes.push(BreakOutcome::Served { planned });
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
        let work = self.rhythm.work().as_duration();
        self.enter_running(from, work)
    }

    fn resume_from_suspension(&mut self, now: Instant, frozen: Duration) -> bool {
        self.enter_running(now, frozen)
    }

    // Tout retour en [TRAVAIL] avec un décompte neuf redémarre l'horloge d'inactivité :
    // sinon `freeze_if_idle` gèlerait un restant calculé depuis une activité d'avant le
    // décompte, plus long que la durée de travail elle-même.
    pub(super) fn enter_running(&mut self, from: Instant, remaining: Duration) -> bool {
        let Some(deadline) = from.checked_plus(remaining) else {
            return false;
        };
        self.last_activity = from;
        self.state = CycleState::Working {
            countdown: Countdown::Running { deadline },
        };
        true
    }
}
