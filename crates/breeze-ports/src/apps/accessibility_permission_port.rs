use crate::apps::permission_status::PermissionStatus;

pub trait AccessibilityPermissionPort: Send + Sync {
    fn status(&self) -> PermissionStatus;
    // Ouvre l'invite système ; ne bloque pas et ne garantit pas l'octroi.
    fn request(&self);
}
