use breeze_ports::PresentationLockPort;

// Hors macOS : aucune présentation d'application à verrouiller (§5.3).
pub struct NullPresentationLock;

impl PresentationLockPort for NullPresentationLock {
    fn lock(&mut self) {}

    fn hold(&mut self) {}

    fn release(&mut self) {}
}
