use breeze_ports::{FramesUnobservable, WindowFrame, WindowFramesPort};

// Sans adaptateur de la plateforme, les cadres sont inobservables : une pause Simple se
// tient alors en voile plein écran par moniteur (§10.5).
pub struct NullWindowFrames;

impl WindowFramesPort for NullWindowFrames {
    fn visible_windows(&mut self) -> Result<Vec<WindowFrame>, FramesUnobservable> {
        Err(FramesUnobservable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_frames_are_unobservable() {
        assert_eq!(NullWindowFrames.visible_windows(), Err(FramesUnobservable));
    }
}
