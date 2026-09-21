use crate::clock::Instant;
use crate::cycle::countdown::Countdown;
use crate::cycle::state::CycleState;
use crate::settings::Rhythm;
use core::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Absence {
    pub began_at: Instant,
    pub lasted: Duration,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AbsenceVerdict {
    Nothing,
    CycleValidated,
    BreakServed,
    BreakStartsAtWake,
    PhaseContinues { remaining: Duration },
}

pub fn absence_verdict(state: CycleState, rhythm: Rhythm, absence: Absence) -> AbsenceVerdict {
    let lasted = absence.lasted;
    if lasted.is_zero() {
        return AbsenceVerdict::Nothing;
    }
    let pause = rhythm.pause().as_duration();
    match state {
        CycleState::Inactive | CycleState::Suspended { .. } => AbsenceVerdict::Nothing,
        CycleState::Working {
            countdown: Countdown::Frozen { .. },
        } => {
            if lasted > pause {
                AbsenceVerdict::CycleValidated
            } else {
                AbsenceVerdict::Nothing
            }
        }
        CycleState::Working {
            countdown: Countdown::Running { deadline },
        } => {
            let remaining = deadline.elapsed_since(absence.began_at);
            if lasted > pause {
                AbsenceVerdict::CycleValidated
            } else if lasted > remaining {
                AbsenceVerdict::BreakStartsAtWake
            } else {
                AbsenceVerdict::PhaseContinues {
                    remaining: remaining.saturating_sub(lasted),
                }
            }
        }
        CycleState::Working {
            countdown: Countdown::Due { .. },
        } => {
            if lasted > pause {
                AbsenceVerdict::CycleValidated
            } else {
                AbsenceVerdict::BreakStartsAtWake
            }
        }
        CycleState::Notice { deadline } => {
            let remaining = deadline.elapsed_since(absence.began_at);
            if lasted > pause {
                AbsenceVerdict::CycleValidated
            } else if lasted > remaining {
                AbsenceVerdict::BreakStartsAtWake
            } else {
                AbsenceVerdict::PhaseContinues {
                    remaining: remaining - lasted,
                }
            }
        }
        CycleState::BreakActive { deadline, .. } => {
            let remaining = deadline.elapsed_since(absence.began_at);
            if lasted > remaining {
                AbsenceVerdict::BreakServed
            } else {
                AbsenceVerdict::PhaseContinues { remaining }
            }
        }
        CycleState::Returning { deadline } => AbsenceVerdict::PhaseContinues {
            remaining: deadline.elapsed_since(absence.began_at),
        },
    }
}
