// La session n'expose pas les cadres des fenêtres, ou leur relevé a échoué : une pause
// Simple commencée maintenant se tiendrait en plein écran (§10.5).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FramesUnobservable;
