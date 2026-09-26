use super::Cycle;
use crate::clock::Instant;
use crate::cycle::absence::{absence_verdict, Absence, AbsenceVerdict};
use crate::cycle::countdown::Countdown;
use crate::cycle::state::CycleState;
use crate::debt::Settlement;
use crate::outcome::BreakOutcome;
use crate::settings::Weekday;
use core::time::Duration;

impl Cycle {
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

    // Réveil de veille : le verdict d'absence d'abord (une pause entamée va à son terme),
    // puis, si les heures actives ont été quittées pendant la veille, le travail s'éteint ;
    // le calendrier observé ensuite le relance sur un cycle neuf.
    pub fn return_from_sleep(
        &mut self,
        absence: Absence,
        now: Instant,
        (weekday, minute_of_day): (Weekday, u16),
    ) -> AbsenceVerdict {
        self.count_sleep_against_suspension(absence.lasted);
        let verdict = self.return_from_absence(absence, now);
        let stayed_active =
            self.configured_rhythm()
                .is_active_throughout(weekday, minute_of_day, absence.lasted);
        if !stayed_active {
            self.observe_calendar(now, false);
        }
        verdict
    }

    // Une suspension vise une heure murale (« demain 6 h ») : la veille, que l'horloge
    // monotone ne compte pas, doit la rapprocher d'autant.
    fn count_sleep_against_suspension(&mut self, slept: Duration) {
        if let CycleState::Suspended { resume_at, frozen } = self.state {
            self.state = CycleState::Suspended {
                resume_at: resume_at.minus(slept),
                frozen,
            };
        }
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
}
