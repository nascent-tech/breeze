use breeze_domain::AppId;
use breeze_ports::{FramesUnobservable, SessionSignals, WindowFrame};

// Ce que l'hôte a relevé du système juste avant un poll, hors de tout verrou : l'instant de
// la dernière saisie, l'application au premier plan et les fenêtres visibles.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Observation {
    pub signals: SessionSignals,
    pub foreground: Option<AppId>,
    pub windows: Result<Vec<WindowFrame>, FramesUnobservable>,
}

impl Observation {
    pub fn frames_observable(&self) -> bool {
        self.windows.is_ok()
    }
}
