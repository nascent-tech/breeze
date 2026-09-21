use breeze_domain::Instant;
use breeze_ports::{SessionSignals, SessionSignalsPort};

// Faute d'un vrai relevé du temps d'inactivité de l'OS, l'utilisateur est réputé
// actif à l'instant courant : le gel d'inactivité ne se déclenche jamais. Sûr par
// défaut — mieux vaut ne pas geler que geler à tort. Un adaptateur par plateforme
// (temps d'inactivité réel) le remplacera.
pub struct NullSessionSignals;

impl SessionSignalsPort for NullSessionSignals {
    fn poll(&mut self, now: Instant) -> SessionSignals {
        SessionSignals { last_input: now }
    }
}
