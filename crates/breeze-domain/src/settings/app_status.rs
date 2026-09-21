#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AppStatus {
    Blocked,
    Spared,
    Ignored,
}
