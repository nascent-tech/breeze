use super::monitor_cache::display_id_of;
use super::native_window;
use breeze_ports::{DisplayId, SurfaceKind};
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};

// Surface plein écran d'un moniteur : bouclier Hardcore opaque, ou voile Simple dégradé.
// Au-dessus de tout et sur tous les bureaux, espaces plein écran compris. À appeler sur le
// thread principal.
fn kind_query(kind: SurfaceKind) -> &'static str {
    match kind {
        SurfaceKind::Hardcore => "hardcore",
        SurfaceKind::Veil => "veil",
    }
}

pub fn build(
    app: &AppHandle,
    label: &str,
    display: DisplayId,
    kind: SurfaceKind,
) -> tauri::Result<()> {
    let monitors = app.available_monitors()?;
    let Some(monitor) = monitors
        .iter()
        .find(|monitor| display_id_of(monitor) == display)
    else {
        eprintln!("breeze: display {} vanished before its overlay", display.0);
        return Ok(());
    };
    // Coordonnées logiques au facteur d'échelle DU MONITEUR cible : les setters physiques
    // convertiraient avec le facteur de la fenêtre (créée sur l'écran principal), ce qui
    // décale la fenêtre en DPI mixte et laisse l'écran externe découvert.
    let scale = monitor.scale_factor();
    let position = monitor.position().to_logical::<f64>(scale);
    let size = monitor.size().to_logical::<f64>(scale);
    let url = WebviewUrl::App(format!("overlay.html?kind={}", kind_query(kind)).into());
    let window = WebviewWindowBuilder::new(app, label, url)
        .title("Breeze — pause")
        .decorations(false)
        .shadow(false)
        .resizable(false)
        .minimizable(false)
        .maximizable(false)
        .closable(false)
        .fullscreen(false)
        .always_on_top(true)
        .visible_on_all_workspaces(true)
        .skip_taskbar(true)
        .transparent(matches!(kind, SurfaceKind::Veil))
        .focused(true)
        .position(position.x, position.y)
        .inner_size(size.width, size.height)
        .build()?;
    // Aussi sur l'espace d'une app en plein écran natif : une app passée en plein écran
    // n'échappe pas à la pause.
    native_window::join_full_screen_spaces(&window)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_kind_maps_to_its_query_value() {
        assert_eq!(kind_query(SurfaceKind::Veil), "veil");
        assert_eq!(kind_query(SurfaceKind::Hardcore), "hardcore");
    }
}
