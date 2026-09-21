use super::Cycle;
use crate::clock::Instant;
use crate::command_error::CommandError;
use crate::cycle::absence::{absence_verdict, Absence, AbsenceVerdict};
use crate::cycle::countdown::Countdown;
use crate::cycle::state::CycleState;
use crate::debt::Settlement;
use crate::outcome::{BreakOutcome, InterruptionDoor};
use crate::settings::{Rhythm, Severity};
use core::time::Duration;

impl Cycle {
    pub fn change_rhythm(&mut self, rhythm: Rhythm) -> Result<(), CommandError> {
        if self.break_is_due() {
            return Err(CommandError::BreakDue);
        }
        self.pending_rhythm = (rhythm != self.rhythm).then_some(rhythm);
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
        if !self.enter_running(now, frozen) {
            return Err(CommandError::NotSuspended);
        }
        Ok(())
    }

    pub fn interrupt_break(&mut self, now: Instant) -> Result<(), CommandError> {
        // L'échéance fait autorité, pas la fréquence du poll : une pause déjà due est
        // servie, pas interrompue (§8.5 « comptée prise, la confirmation se ferme sans effet »).
        self.tick(now);
        let CycleState::BreakActive {
            deadline,
            severity: Severity::Hardcore,
            ..
        } = self.state
        else {
            return Err(CommandError::NotInterruptible);
        };
        let unserved = deadline.elapsed_since(now);
        if !self.enter_next_work(now) {
            return Err(CommandError::NotInterruptible);
        }
        self.record_interruption(unserved, InterruptionDoor::HardcoreExitGesture);
        Ok(())
    }

    // Terminaison propre (§10.2) : le raccourci système (Quit) ou le menu de l'icône
    // (TrayMenu). Une pause due mais jamais commencée crédite sa durée réglée entière,
    // une pause en cours son restant. Idempotente : après coup l'état est [TRAVAIL].
    pub fn terminate(&mut self, now: Instant, door: InterruptionDoor) {
        self.tick(now);
        let unserved = match self.state {
            CycleState::Notice { .. } => self.rhythm.pause().as_duration(),
            CycleState::BreakActive { deadline, .. } => deadline.elapsed_since(now),
            _ => return,
        };
        if self.enter_next_work(now) {
            self.record_interruption(unserved, door);
        }
    }

    pub(super) fn record_interruption(&mut self, unserved: Duration, door: InterruptionDoor) {
        if door.charges_debt() {
            self.debt.credit(unserved);
        } else {
            self.debt.freeze();
        }
        self.outcomes
            .push(BreakOutcome::Interrupted { unserved, door });
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
                self.enter_returning(now, Settlement::Frozen);
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
}
