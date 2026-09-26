use super::{screen_surface, window_veil};
use breeze_ports::{
    DisplayId, OverlayCapability, OverlaySurfacesPort, Rect, SurfaceId, SurfaceKind, WindowId,
};
use std::collections::HashMap;
use tauri::{AppHandle, Manager};

// Une surface demandée : son label, et pour un voile de fenêtre la fenêtre qu'il couvre.
struct Requested {
    label: String,
    above: Option<WindowId>,
}

// Chaque opération est expédiée au thread principal sans l'attendre : l'expédition se fait
// sous le verrou du scheduler (le ticker appelle le port depuis `poll`), l'exécution a lieu
// plus tard, hors verrou, sur la boucle d'événements.
pub struct TauriOverlay {
    app: AppHandle,
    next_surface: u64,
    requested: HashMap<SurfaceId, Requested>,
}

impl TauriOverlay {
    pub fn new(app: AppHandle) -> Self {
        TauriOverlay {
            app,
            next_surface: 0,
            requested: HashMap::new(),
        }
    }

    fn issue(
        &mut self,
        above: Option<WindowId>,
        label_of: impl FnOnce(SurfaceId) -> String,
    ) -> (SurfaceId, String) {
        self.next_surface += 1;
        let surface = SurfaceId(self.next_surface);
        let label = label_of(surface);
        let requested = Requested {
            label: label.clone(),
            above,
        };
        self.requested.insert(surface, requested);
        (surface, label)
    }

    fn on_main_thread(&self, what: &'static str, task: impl FnOnce(&AppHandle) + Send + 'static) {
        let app = self.app.clone();
        if let Err(error) = self.app.run_on_main_thread(move || task(&app)) {
            eprintln!("breeze: overlay {what} not dispatched: {error}");
        }
    }

    // À chaque tick : chaque voile de fenêtre est ramené juste au-dessus de sa cible, que
    // l'utilisateur a pu faire passer devant en cliquant l'app bloquée. Une seule expédition
    // pour tous les voiles ; rien quand il n'y en a aucun.
    pub fn keep_window_veils_above_targets(&self) {
        let veils = window_veils(&self.requested);
        if veils.is_empty() {
            return;
        }
        self.on_main_thread("restacking", move |app| {
            for (label, target) in veils {
                if let Err(error) = window_veil::keep_above(app, &label, target) {
                    eprintln!("breeze: window veil {label} not restacked: {error}");
                }
            }
        });
    }
}

impl OverlaySurfacesPort for TauriOverlay {
    fn cover_display(&mut self, display: DisplayId, kind: SurfaceKind) -> SurfaceId {
        let (surface, label) = self.issue(None, |surface| label_for(display, surface));
        self.on_main_thread("creation", move |app| {
            if let Err(error) = screen_surface::build(app, &label, display, kind) {
                eprintln!("breeze: overlay {label} not created: {error}");
            }
        });
        surface
    }

    fn cover_window(&mut self, window: WindowId, frame: Rect) -> SurfaceId {
        let (surface, label) =
            self.issue(Some(window), |surface| window_label_for(window, surface));
        self.on_main_thread("window veil", move |app| {
            if let Err(error) = window_veil::build(app, &label, window, frame) {
                eprintln!("breeze: window veil {label} not created: {error}");
            }
        });
        surface
    }

    fn reframe(&mut self, surface: SurfaceId, frame: Rect) {
        let Some(label) = self.requested.get(&surface).map(|r| r.label.clone()) else {
            return;
        };
        self.on_main_thread("move", move |app| {
            if let Err(error) = window_veil::reframe(app, &label, frame) {
                eprintln!("breeze: window veil {label} not moved: {error}");
            }
        });
    }

    fn dismiss(&mut self, surface: SurfaceId) {
        let Some(Requested { label, .. }) = self.requested.remove(&surface) else {
            return;
        };
        self.on_main_thread("dismissal", move |app| destroy(app, &label));
    }

    fn dismiss_all(&mut self) {
        let labels: Vec<String> = self.requested.drain().map(|(_, r)| r.label).collect();
        if labels.is_empty() {
            return;
        }
        self.on_main_thread("dismissal", move |app| {
            for label in labels {
                destroy(app, &label);
            }
        });
    }

    fn capability(&self) -> OverlayCapability {
        OverlayCapability::PlainFullscreen
    }
}

fn destroy(app: &AppHandle, label: &str) {
    if let Some(window) = app.get_webview_window(label) {
        if let Err(error) = window.destroy() {
            eprintln!("breeze: overlay {label} not destroyed: {error}");
        }
    }
}

// Les voiles de fenêtre demandés et la fenêtre que chacun couvre ; les surfaces plein
// écran, au-dessus de tout, n'ont rien à suivre.
fn window_veils(requested: &HashMap<SurfaceId, Requested>) -> Vec<(String, WindowId)> {
    requested
        .values()
        .filter_map(|requested| Some((requested.label.clone(), requested.above?)))
        .collect()
}

fn label_for(display: DisplayId, surface: SurfaceId) -> String {
    format!("overlay-{}-{}", display.0, surface.0)
}

// Même préfixe `overlay-` : la capacité IPC des surfaces de pause les couvre aussi.
fn window_label_for(window: WindowId, surface: SurfaceId) -> String {
    format!("overlay-w{}-{}", window.0, surface.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_window_veils_are_kept_above_a_target() {
        let requested = HashMap::from([
            (
                SurfaceId(1),
                Requested {
                    label: label_for(DisplayId(1), SurfaceId(1)),
                    above: None,
                },
            ),
            (
                SurfaceId(2),
                Requested {
                    label: window_label_for(WindowId(77), SurfaceId(2)),
                    above: Some(WindowId(77)),
                },
            ),
        ]);
        assert_eq!(
            window_veils(&requested),
            vec![("overlay-w77-2".to_owned(), WindowId(77))]
        );
    }

    #[test]
    fn a_label_carries_both_the_display_and_the_surface() {
        assert_eq!(label_for(DisplayId(3), SurfaceId(17)), "overlay-3-17");
    }

    #[test]
    fn a_window_veil_label_stays_under_the_overlay_capability() {
        assert_eq!(
            window_label_for(WindowId(4242), SurfaceId(5)),
            "overlay-w4242-5"
        );
    }
}
