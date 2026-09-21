#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CyclePhase {
    Inactive,
    Working,
    Notice,
    Break,
    Returning,
}
