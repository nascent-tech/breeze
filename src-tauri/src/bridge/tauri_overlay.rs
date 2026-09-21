use super::monitor_cache::display_id_of;
use breeze_ports::{DisplayId, OverlayCapability, OverlaySurfacesPort, SurfaceId, SurfaceKind};
use std::collections::HashMap;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub struct TauriOverlay {
    app: AppHandle,
    next_surface: u64,
    requested: HashMap<SurfaceId, String>,
}

impl TauriOverlay {
    pub fn new(app: AppHandle) -> Self {
        TauriOverlay {
            app,
            next_surface: 0,
            requested: HashMap::new(),
        }
    }
}

impl OverlaySurfacesPort for TauriOverlay {
    fn cover_display(&mut self, display: DisplayId, kind: SurfaceKind) -> SurfaceId {
        self.next_surface += 1;
        let surface = SurfaceId(self.next_surface);
        let label = label_for(display, surface);
        self.requested.insert(surface, label.clone());
        let app = self.app.clone();
        let dispatched = self.app.run_on_main_thread(move || {
            if let Err(error) = build_surface(&app, &label, display, kind) {
                eprintln!("breeze: overlay {label} not created: {error}");
            }
        });
        if let Err(error) = dispatched {
            eprintln!("breeze: overlay not dispatched: {error}");
        }
        surface
    }

    fn dismiss_all(&mut self) {
        let labels: Vec<String> = self.requested.values().cloned().collect();
        self.requested.clear();
        if labels.is_empty() {
            return;
        }
        let app = self.app.clone();
        let dispatched = self.app.run_on_main_thread(move || {
            for label in labels {
                if let Some(window) = app.get_webview_window(&label) {
                    if let Err(error) = window.destroy() {
                        eprintln!("breeze: overlay {label} not destroyed: {error}");
                    }
                }
            }
        });
        if let Err(error) = dispatched {
            eprintln!("breeze: overlay dismissal not dispatched: {error}");
        }
    }

    fn capability(&self) -> OverlayCapability {
        OverlayCapability::PlainFullscreen
    }
}

fn label_for(display: DisplayId, surface: SurfaceId) -> String {
    format!("overlay-{}-{}", display.0, surface.0)
}

fn kind_query(kind: SurfaceKind) -> &'static str {
    match kind {
        SurfaceKind::Hardcore => "hardcore",
        SurfaceKind::Veil => "veil",
    }
}

fn build_surface(
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
    WebviewWindowBuilder::new(app, label, url)
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
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_label_carries_both_the_display_and_the_surface() {
        assert_eq!(label_for(DisplayId(3), SurfaceId(17)), "overlay-3-17");
    }

    #[test]
    fn each_kind_maps_to_its_query_value() {
        assert_eq!(kind_query(SurfaceKind::Veil), "veil");
        assert_eq!(kind_query(SurfaceKind::Hardcore), "hardcore");
    }
}
