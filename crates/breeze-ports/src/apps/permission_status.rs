#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PermissionStatus {
    Granted,
    Denied,
    Unknown,
}
