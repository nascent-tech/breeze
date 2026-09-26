use breeze_domain::BreakOutcome;

// Une ligne du journal des pauses : le sort d'une pause, daté au jour LOCAL où il est
// tombé (les statistiques se lisent en jours de l'utilisateur, pas en jours UTC). Le
// temps tenu se déduit du sort lui-même (`BreakOutcome::held`).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LedgerEntry {
    pub ended_at_unix: u64,
    pub local_date: String,
    pub outcome: BreakOutcome,
}
