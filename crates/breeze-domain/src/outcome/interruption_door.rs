#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum InterruptionDoor {
    Quit,
    TrayMenu,
    HardcoreExitGesture,
    SuspensionOverrun,
    Crash,
}
