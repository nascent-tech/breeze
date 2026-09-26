use crate::outcome::interruption_door::InterruptionDoor;
use core::time::Duration;

// `planned` : la pause en cours au moment du sort (rythme actif, allongement de dette
// compris s'il a été remboursé), jamais le rythme du cycle suivant.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BreakOutcome {
    Served {
        planned: Duration,
    },
    ValidatedByAbsence,
    Interrupted {
        planned: Duration,
        unserved: Duration,
        door: InterruptionDoor,
    },
}

impl BreakOutcome {
    // Temps de pause réellement tenu : la pause entière si servie, la part tenue si
    // interrompue, rien si le cycle a été validé par une absence.
    pub fn held(self) -> Duration {
        match self {
            BreakOutcome::Served { planned } => planned,
            BreakOutcome::ValidatedByAbsence => Duration::ZERO,
            BreakOutcome::Interrupted {
                planned, unserved, ..
            } => planned.saturating_sub(unserved),
        }
    }
}
