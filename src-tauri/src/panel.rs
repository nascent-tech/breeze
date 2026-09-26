use crate::TRAY_ID;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tauri::{AppHandle, LogicalPosition, Manager, Monitor, Rect, WebviewWindow, WindowEvent};

pub const PANEL_LABEL: &str = "panel";
const PANEL_WIDTH: f64 = 360.0;
const GAP_BELOW_ICON: f64 = 6.0;
const SCREEN_MARGIN: f64 = 8.0;
// Un clic sur l'icône retire d'abord le focus au panneau (qui se cache), puis arrive
// comme clic : sans cette fenêtre, le clic qui doit fermer rouvrirait aussitôt.
const REOPEN_GUARD: Duration = Duration::from_millis(300);

#[derive(Default)]
pub struct PanelFocus(Mutex<Option<Instant>>);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Bounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

// Centré sous l'icône, gardé dans l'écran qui la porte (coordonnées logiques).
pub fn origin_under(icon: Bounds, screen: Bounds, panel_width: f64) -> (f64, f64) {
    let centered = icon.x + icon.width / 2.0 - panel_width / 2.0;
    let lowest = screen.x + SCREEN_MARGIN;
    let highest = (screen.x + screen.width - panel_width - SCREEN_MARGIN).max(lowest);
    (
        centered.clamp(lowest, highest),
        icon.y + icon.height + GAP_BELOW_ICON,
    )
}

// Un écran en coordonnées logiques globales, avec son facteur d'échelle.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Screen {
    pub bounds: Bounds,
    pub scale: f64,
}

fn screen_of(monitor: &Monitor) -> Screen {
    let scale = monitor.scale_factor();
    let position = monitor.position().to_logical::<f64>(scale);
    let size = monitor.size().to_logical::<f64>(scale);
    Screen {
        bounds: Bounds {
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        },
        scale,
    }
}

fn contains(bounds: Bounds, x: f64, y: f64) -> bool {
    x >= bounds.x && x < bounds.x + bounds.width && y >= bounds.y && y < bounds.y + bounds.height
}

fn scaled_down(physical: Bounds, scale: f64) -> Bounds {
    Bounds {
        x: physical.x / scale,
        y: physical.y / scale,
        width: physical.width / scale,
        height: physical.height / scale,
    }
}

// Le rect de l'icône arrive en pixels physiques au facteur de SON écran, sans dire lequel :
// en DPI mixte, le même rect converti tombe dans deux écrans (1000 px = 1000 pt sur un
// écran @1x, ou 500 pt sur un écran @2x). Le curseur, lui, est en coordonnées logiques
// globales sans ambiguïté : il désigne l'écran à retenir parmi les candidats.
pub fn screen_for_icon(
    icon: Bounds,
    screens: &[Screen],
    cursor: Option<(f64, f64)>,
) -> Option<(Screen, Bounds)> {
    let candidates: Vec<(Screen, Bounds)> = screens
        .iter()
        .map(|screen| (*screen, scaled_down(icon, screen.scale)))
        .filter(|(screen, logical)| {
            let center = (
                logical.x + logical.width / 2.0,
                logical.y + logical.height / 2.0,
            );
            logical.width > 0.0 && contains(screen.bounds, center.0, center.1)
        })
        .collect();
    let under_cursor = cursor.and_then(|(x, y)| {
        candidates
            .iter()
            .find(|(screen, _)| contains(screen.bounds, x, y))
            .copied()
    });
    under_cursor.or_else(|| candidates.first().copied())
}

// Sur macOS, la position du curseur est rendue au facteur de l'écran principal.
fn cursor_logical(app: &AppHandle) -> Option<(f64, f64)> {
    let cursor = app.cursor_position().ok()?;
    let scale = app.primary_monitor().ok()??.scale_factor();
    Some((cursor.x / scale, cursor.y / scale))
}

fn place_under_icon(app: &AppHandle, icon: Rect) -> Option<LogicalPosition<f64>> {
    let screens: Vec<Screen> = app
        .available_monitors()
        .ok()?
        .iter()
        .map(screen_of)
        .collect();
    let position = icon.position.to_physical::<f64>(1.0);
    let size = icon.size.to_physical::<f64>(1.0);
    let physical = Bounds {
        x: position.x,
        y: position.y,
        width: size.width,
        height: size.height,
    };
    let (screen, logical) = screen_for_icon(physical, &screens, cursor_logical(app))?;
    let (x, y) = origin_under(logical, screen.bounds, PANEL_WIDTH);
    Some(LogicalPosition::new(x, y))
}

pub fn tray_rect(app: &AppHandle) -> Option<Rect> {
    let tray = app.tray_by_id(TRAY_ID)?;
    match tray.rect() {
        Ok(rect) => rect,
        Err(error) => {
            eprintln!("breeze: tray icon rect unavailable: {error}");
            None
        }
    }
}

fn panel(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window(PANEL_LABEL)
}

pub fn show_panel(app: &AppHandle, icon: Option<Rect>) {
    let Some(window) = panel(app) else {
        return;
    };
    if let Some(origin) = icon.and_then(|rect| place_under_icon(app, rect)) {
        if let Err(error) = window.set_position(origin) {
            eprintln!("breeze: could not place the panel: {error}");
        }
    }
    if let Err(error) = window.show().and_then(|()| window.set_focus()) {
        eprintln!("breeze: could not show the panel: {error}");
    }
}

pub fn hide(app: &AppHandle) {
    if let Some(window) = panel(app) {
        if let Err(error) = window.hide() {
            eprintln!("breeze: could not hide the panel: {error}");
        }
    }
}

fn just_hidden_by_blur(app: &AppHandle) -> bool {
    let focus = app.state::<PanelFocus>();
    let blurred_at = *focus.0.lock().unwrap_or_else(|e| e.into_inner());
    blurred_at.is_some_and(|at| at.elapsed() < REOPEN_GUARD)
}

pub fn toggle(app: &AppHandle, icon: Option<Rect>) {
    let visible = panel(app).is_some_and(|window| window.is_visible().unwrap_or(false));
    if visible {
        hide(app);
    } else if !just_hidden_by_blur(app) {
        show_panel(app, icon);
    }
}

pub fn hide_on_blur(app: &AppHandle) {
    let Some(window) = panel(app) else {
        return;
    };
    let handle = app.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::Focused(false) = event {
            *handle
                .state::<PanelFocus>()
                .0
                .lock()
                .unwrap_or_else(|e| e.into_inner()) = Some(Instant::now());
            hide(&handle);
        }
    });
}

#[tauri::command]
pub fn hide_panel(app: AppHandle) {
    hide(&app);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCREEN: Bounds = Bounds {
        x: 0.0,
        y: 0.0,
        width: 1512.0,
        height: 982.0,
    };

    fn icon_at(x: f64) -> Bounds {
        Bounds {
            x,
            y: 0.0,
            width: 30.0,
            height: 24.0,
        }
    }

    #[test]
    fn the_panel_is_centred_under_the_icon() {
        let (x, y) = origin_under(icon_at(1000.0), SCREEN, PANEL_WIDTH);
        assert_eq!(x, 1015.0 - PANEL_WIDTH / 2.0);
        assert_eq!(y, 24.0 + GAP_BELOW_ICON);
    }

    #[test]
    fn an_icon_near_the_right_edge_keeps_the_panel_on_screen() {
        let (x, _) = origin_under(icon_at(1490.0), SCREEN, PANEL_WIDTH);
        assert_eq!(x, SCREEN.width - PANEL_WIDTH - SCREEN_MARGIN);
    }

    // Retina @2x à gauche (1512 pt), écran externe @1x à droite (1920 pt).
    const RETINA: Screen = Screen {
        bounds: SCREEN,
        scale: 2.0,
    };
    const EXTERNAL: Screen = Screen {
        bounds: Bounds {
            x: 1512.0,
            y: 0.0,
            width: 1920.0,
            height: 1080.0,
        },
        scale: 1.0,
    };

    // Le même rect physique : icône à 2000 pt sur l'externe, ou à 1000 pt sur le Retina.
    const AMBIGUOUS_ICON: Bounds = Bounds {
        x: 2000.0,
        y: 0.0,
        width: 60.0,
        height: 48.0,
    };

    #[test]
    fn with_mixed_scales_the_cursor_picks_the_external_screen() {
        let (screen, icon) =
            screen_for_icon(AMBIGUOUS_ICON, &[RETINA, EXTERNAL], Some((2020.0, 10.0))).unwrap();
        assert_eq!(screen, EXTERNAL);
        assert_eq!(icon.x, 2000.0);
    }

    #[test]
    fn with_mixed_scales_the_cursor_picks_the_retina_screen() {
        let (screen, icon) =
            screen_for_icon(AMBIGUOUS_ICON, &[RETINA, EXTERNAL], Some((1010.0, 10.0))).unwrap();
        assert_eq!(screen, RETINA);
        assert_eq!(icon.x, 1000.0);
        assert_eq!(icon.width, 30.0);
    }

    #[test]
    fn an_icon_only_one_screen_can_hold_needs_no_cursor() {
        let icon = Bounds {
            x: 3200.0,
            ..AMBIGUOUS_ICON
        };
        let (screen, _) = screen_for_icon(icon, &[RETINA, EXTERNAL], None).unwrap();
        assert_eq!(screen, EXTERNAL);
    }

    #[test]
    fn a_secondary_screen_to_the_left_keeps_its_own_bounds() {
        let left = Bounds {
            x: -1920.0,
            ..SCREEN
        };
        let (x, _) = origin_under(icon_at(-1915.0), left, PANEL_WIDTH);
        assert_eq!(x, -1920.0 + SCREEN_MARGIN);
    }
}
