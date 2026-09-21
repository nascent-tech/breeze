#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RhythmError {
    WorkOutOfBounds,
    PauseOutOfBounds,
    PauseLongerThanWork,
    NoActiveDay,
    DegenerateRange,
}
