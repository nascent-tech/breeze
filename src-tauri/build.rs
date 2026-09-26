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
        "set_spared_apps",
        "open_settings",
        "finish_onboarding",
        "get_settings",
        "set_active_days",
        "set_schedule",
        "set_update_check",
        "reset_settings",
        "set_launch_at_login",
        "set_sounds",
        "set_menubar_mode",
        "get_stats",
        "get_app_info",
        "reopen_onboarding",
        "hide_panel",
        "open_url",
    ]);
    tauri_build::try_build(Attributes::new().app_manifest(manifest))
        .expect("failed to run tauri-build");
}
