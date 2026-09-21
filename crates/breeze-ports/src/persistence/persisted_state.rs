use breeze_domain::Severity;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PersistedState {
    pub work_minutes: u16,
    pub pause_minutes: u16,
    pub severity: Severity,
    pub served_breaks: u32,
}
