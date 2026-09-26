#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DegradedReason {
    // Les cadres des fenêtres n'étaient pas observables au premier instant de la pause.
    FramesUnobservable,
    // Plus de fenêtres bloquées visibles que Breeze n'en voile une à une (§ contrat 7).
    TooManyWindows,
}
