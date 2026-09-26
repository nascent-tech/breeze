#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct LedgerDay {
    pub local_date: String,
    pub served: u32,
    pub interrupted: u32,
    pub validated: u32,
    pub served_seconds: u64,
    pub emergency_exits: u32,
    pub emergency_unserved_seconds: u64,
}
