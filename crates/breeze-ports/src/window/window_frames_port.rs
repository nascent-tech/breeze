use crate::window::{FramesUnobservable, WindowFrame};

pub trait WindowFramesPort {
    // Les fenêtres ordinaires visibles des autres applications, de l'avant vers l'arrière.
    // Rapide (quelques millisecondes) : l'hôte l'appelle à chaque tick, hors verrou.
    fn visible_windows(&mut self) -> Result<Vec<WindowFrame>, FramesUnobservable>;
}
