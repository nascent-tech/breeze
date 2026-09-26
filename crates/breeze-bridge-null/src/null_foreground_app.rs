use breeze_domain::AppId;
use breeze_ports::ForegroundAppPort;

// Sans adaptateur de la plateforme, l'identité au premier plan est inconnue : tout usage
// compte comme du travail et le statut Ignorée n'a aucun effet (§10.5).
pub struct NullForegroundApp;

impl ForegroundAppPort for NullForegroundApp {
    fn foreground_app(&mut self) -> Option<AppId> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_foreground_identity_is_unknown() {
        assert_eq!(NullForegroundApp.foreground_app(), None);
    }
}
