use super::native_window;
use breeze_ports::{Rect, WindowId};
use tauri::window::{Effect, EffectState, EffectsBuilder};
use tauri::{
    AppHandle, LogicalPosition, LogicalSize, Manager, WebviewUrl, WebviewWindow,
    WebviewWindowBuilder,
};

// Rayon des coins d'une fenêtre macOS : le voile en épouse la forme.
const CORNER_RADIUS: f64 = 12.0;

// Voile d'une seule fenêtre bloquée. Les cadres de CGWindowList et les positions logiques de
// Tauri sur macOS partagent le même repère : des points, origine en haut à gauche de l'écran
// principal, y vers le bas, écrans secondaires en coordonnées globales (tao convertit avec la
// hauteur de `CGDisplay::main()`, exactement l'origine de Core Graphics). Le cadre se pose
// donc tel quel, en logique, sans facteur d'échelle.
//
// Niveau de fenêtre NORMAL, jamais « toujours au premier plan » : le voile est rangé juste
// au-dessus de sa fenêtre cible, si bien qu'une application épargnée placée devant (ou amenée
// par-dessus) reste devant lui et utilisable (§8.3, §10.6). Il appartient à l'espace de sa
// fenêtre, pas à tous les bureaux. À appeler sur le thread principal.
pub fn build(app: &AppHandle, label: &str, target: WindowId, frame: Rect) -> tauri::Result<()> {
    let url = WebviewUrl::App("overlay.html?kind=window".into());
    let (position, size) = logical(frame);
    let window = WebviewWindowBuilder::new(app, label, url)
        .title("Breeze — pause")
        .decorations(false)
        .shadow(false)
        .resizable(false)
        .minimizable(false)
        .maximizable(false)
        .closable(false)
        .fullscreen(false)
        .always_on_top(false)
        .visible_on_all_workspaces(false)
        .skip_taskbar(true)
        .transparent(true)
        // Le voile ne prend jamais le focus : l'app épargnée que l'utilisateur utilise
        // pendant la pause le garde.
        .focused(false)
        .focusable(false)
        .effects(
            EffectsBuilder::new()
                .effect(Effect::Popover)
                .state(EffectState::Active)
                .radius(CORNER_RADIUS)
                .build(),
        )
        .position(position.x, position.y)
        .inner_size(size.width, size.height)
        .build()?;
    native_window::join_full_screen_spaces(&window)?;
    native_window::order_above(&window, target)
}

// Suit la fenêtre voilée sans recréer la surface. À appeler sur le thread principal.
pub fn reframe(app: &AppHandle, label: &str, frame: Rect) -> tauri::Result<()> {
    let Some(window) = app.get_webview_window(label) else {
        return Ok(());
    };
    apply_frame(&window, frame)
}

// Ramène le voile juste au-dessus de sa fenêtre cible : l'utilisateur a pu cliquer l'app
// bloquée et la faire passer devant. Sans effet sur le focus. Thread principal.
pub fn keep_above(app: &AppHandle, label: &str, target: WindowId) -> tauri::Result<()> {
    let Some(window) = app.get_webview_window(label) else {
        return Ok(());
    };
    native_window::order_above(&window, target)
}

// Ce qui se déplace et se redimensionne : la fenêtre du voile, ou un double en test.
trait Placeable {
    fn resize(&self, size: LogicalSize<f64>) -> tauri::Result<()>;
    fn move_to(&self, position: LogicalPosition<f64>) -> tauri::Result<()>;
}

impl Placeable for WebviewWindow {
    fn resize(&self, size: LogicalSize<f64>) -> tauri::Result<()> {
        self.set_size(size)
    }

    fn move_to(&self, position: LogicalPosition<f64>) -> tauri::Result<()> {
        self.set_position(position)
    }
}

// La taille AVANT la position : AppKit redimensionne en gardant le coin bas-gauche fixe,
// donc un redimensionnement après coup ferait glisser le bord haut du voile de l'écart de
// hauteur ; la position posée en dernier fixe le coin haut-gauche là où il doit être.
fn apply_frame(target: &impl Placeable, frame: Rect) -> tauri::Result<()> {
    let (position, size) = logical(frame);
    target.resize(size)?;
    target.move_to(position)
}

fn logical(frame: Rect) -> (LogicalPosition<f64>, LogicalSize<f64>) {
    (
        LogicalPosition::new(f64::from(frame.x), f64::from(frame.y)),
        LogicalSize::new(f64::from(frame.width), f64::from(frame.height)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default)]
    struct RecordingWindow {
        steps: RefCell<Vec<&'static str>>,
    }

    impl Placeable for RecordingWindow {
        fn resize(&self, _size: LogicalSize<f64>) -> tauri::Result<()> {
            self.steps.borrow_mut().push("size");
            Ok(())
        }

        fn move_to(&self, _position: LogicalPosition<f64>) -> tauri::Result<()> {
            self.steps.borrow_mut().push("position");
            Ok(())
        }
    }

    #[test]
    fn a_frame_on_a_secondary_screen_keeps_its_global_points() {
        let frame = Rect {
            x: -1280,
            y: -200,
            width: 640,
            height: 480,
        };
        let (position, size) = logical(frame);
        assert_eq!((position.x, position.y), (-1280.0, -200.0));
        assert_eq!((size.width, size.height), (640.0, 480.0));
    }

    #[test]
    fn a_reframe_resizes_before_it_moves() {
        let window = RecordingWindow::default();
        let frame = Rect {
            x: 10,
            y: 20,
            width: 300,
            height: 200,
        };
        apply_frame(&window, frame).unwrap();
        assert_eq!(*window.steps.borrow(), vec!["size", "position"]);
    }
}
