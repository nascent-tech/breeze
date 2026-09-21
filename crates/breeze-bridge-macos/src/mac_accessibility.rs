use breeze_ports::{AccessibilityPermissionPort, PermissionStatus};
use macos_accessibility_client::accessibility;

pub struct MacAccessibility;

impl AccessibilityPermissionPort for MacAccessibility {
    fn status(&self) -> PermissionStatus {
        if accessibility::application_is_trusted() {
            PermissionStatus::Granted
        } else {
            PermissionStatus::Denied
        }
    }

    fn request(&self) {
        // Déclenche l'invite système « Autoriser dans Réglages » ; le résultat se
        // lit ensuite via status(). Ne bloque pas, ne garantit rien.
        let _ = accessibility::application_is_trusted_with_prompt();
    }
}
