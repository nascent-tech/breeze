// Comment se tient une pause Simple en cours : un voile par fenêtre bloquée, ou un voile
// plein écran par moniteur (§10.5).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VeilMode {
    Windows,
    FullScreen,
}
