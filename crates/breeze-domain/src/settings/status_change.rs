// Quand un statut choisi entre en vigueur (§10.3) : tout de suite s'il renforce la
// contrainte ou la laisse égale, au cycle suivant s'il l'affaiblit.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StatusChange {
    AppliesNow,
    AppliesNextCycle,
}
