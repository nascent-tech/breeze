#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CommandError {
    BreakDue,
    NotSuspendable,
    NotSuspended,
    NotInterruptible,
    // Application de la liste de sécurité (§10.6) : son statut n'est pas éditable.
    LockedApp,
}
