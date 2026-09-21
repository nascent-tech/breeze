use crate::outcome::interruption_door::InterruptionDoor;
use core::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BreakOutcome {
    Served,
    ValidatedByAbsence,
    Interrupted {
        unserved: Duration,
        door: InterruptionDoor,
    },
}
