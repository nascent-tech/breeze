use crate::cycle::degraded_reason::DegradedReason;

// Mode d'une pause (§10.5), fixé à son premier instant. Nominal en Mode Simple = un voile
// par fenêtre bloquée ; Dégradé = un voile plein écran par moniteur.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BreakMode {
    Nominal,
    Degraded(DegradedReason),
}
