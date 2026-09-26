// Pourquoi un décompte de travail est gelé : l'inactivité (§10.1) ou une application
// Ignorée au premier plan (§8.3).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FreezeReason {
    Idle,
    IgnoredApp,
}
