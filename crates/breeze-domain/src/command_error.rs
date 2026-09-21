#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CommandError {
    BreakDue,
    NotSuspendable,
    NotSuspended,
    NotInterruptible,
}
