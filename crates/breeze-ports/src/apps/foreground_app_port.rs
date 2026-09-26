use breeze_domain::AppId;

// L'identité de l'application au premier plan — jamais son titre ni son contenu (§10.6).
// `None` = identité inconnue : tout usage compte alors comme du travail (§10.5).
pub trait ForegroundAppPort {
    fn foreground_app(&mut self) -> Option<AppId>;
}
