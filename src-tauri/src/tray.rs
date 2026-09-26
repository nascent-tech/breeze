use crate::{panel, windows, TRAY_ID};
use tauri::image::Image;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle};

// Feuille monochrome noire sur fond transparent : en image « template », macOS la
// teinte lui-même (clair, sombre, surbrillance) comme toute icône de barre de menus.
const TEMPLATE_ICON: &[u8] = include_bytes!("../icons/tray-template.png");

fn on_menu(app: &AppHandle, id: &str) {
    match id {
        "open" => panel::show_panel(app, panel::tray_rect(app)),
        "settings" => windows::show_settings(app),
        "quit" => crate::graceful_quit(app),
        _ => {}
    }
}

fn on_icon(app: &AppHandle, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        rect,
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
    } = event
    {
        panel::toggle(app, Some(rect));
    }
}

// Clic gauche : le panneau, en popover sous l'icône. Clic droit : le menu.
// Pur adaptateur de l'enveloppe Tauri — aucune décision du domaine ici.
pub fn build(app: &App) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "open", "Ouvrir Breeze", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Réglages…", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quitter Breeze", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &settings, &quit])?;
    TrayIconBuilder::with_id(TRAY_ID)
        .icon(Image::from_bytes(TEMPLATE_ICON)?)
        .icon_as_template(true)
        .tooltip("Breeze")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| on_menu(app, event.id.as_ref()))
        .on_tray_icon_event(|tray, event| on_icon(tray.app_handle(), event))
        .build(app)?;
    Ok(())
}
