use tauri_build::{AppManifest, Attributes};

fn main() {
    // Déclarer les commandes applicatives les soumet à l'ACL : sans une permission
    // `allow-*` explicite, une fenêtre ne peut pas les invoquer. Les surfaces de pause
    // n'obtiennent ainsi que `get_snapshot` — rien qui puisse écourter la pause.
    let manifest = AppManifest::new().commands(&[
        "get_snapshot",
        "suspend",
        "resume",
        "interrupt_break",
        "set_severity",
        "set_rhythm",
        "quit",
        "list_installed_apps",
        "set_app_status",
        "request_accessibility",
        "accessibility_status",
        "open_settings",
    ]);
    tauri_build::try_build(Attributes::new().app_manifest(manifest))
        .expect("failed to run tauri-build");
}
