use breeze_domain::Severity;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PersistedState {
    pub work_minutes: u16,
    pub pause_minutes: u16,
    pub severity: Severity,
    pub served_breaks: u32,
    pub debt_seconds: u32,
    pub debt_recorded_at_unix: u64,
    pub active_days: u8,
    // Plage horaire : bornes en minutes depuis minuit, None = plage désactivée.
    pub schedule_start: Option<u16>,
    pub schedule_end: Option<u16>,
}
