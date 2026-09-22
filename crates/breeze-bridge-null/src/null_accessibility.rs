use breeze_ports::{AccessibilityPermissionPort, PermissionStatus};

// Sans permission d'Accessibilité réelle (OS sans adaptateur), l'état est inconnu
// et la demande est un non-op. Le produit dégrade honnêtement : Mode Simple pose
// alors un seul voile plein écran plutôt qu'un voile par fenêtre.
pub struct NullAccessibility;

impl AccessibilityPermissionPort for NullAccessibility {
    fn status(&self) -> PermissionStatus {
        PermissionStatus::Unknown
    }

    fn request(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_status_is_unknown() {
        assert_eq!(NullAccessibility.status(), PermissionStatus::Unknown);
    }
}
