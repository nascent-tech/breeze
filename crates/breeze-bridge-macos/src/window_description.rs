use breeze_ports::Rect;
use objc2_core_foundation::{CFDictionary, CFNumber, CFRetained, CFString, CFType};

use crate::cf_containers::entries_of;

// En deçà, un voile ne couvrirait qu'une poignée de pixels (contrat, décision 7).
const MIN_SIDE: f64 = 40.0;

pub type Description = CFDictionary<CFType, CFType>;

// Lecture d'un dictionnaire de description rendu par CGWindowListCopyWindowInfo. Les clés
// valent leur propre nom, ce qui évite de lire les constantes externes de Core Graphics
// (accès non sûr). Le titre (`kCGWindowName`) n'est jamais demandé.
pub struct WindowDescription {
    number: CFRetained<CFString>,
    layer: CFRetained<CFString>,
    bounds: CFRetained<CFString>,
    owner_pid: CFRetained<CFString>,
    alpha: CFRetained<CFString>,
    x: CFRetained<CFString>,
    y: CFRetained<CFString>,
    width: CFRetained<CFString>,
    height: CFRetained<CFString>,
}

impl Default for WindowDescription {
    fn default() -> Self {
        WindowDescription {
            number: CFString::from_static_str("kCGWindowNumber"),
            layer: CFString::from_static_str("kCGWindowLayer"),
            bounds: CFString::from_static_str("kCGWindowBounds"),
            owner_pid: CFString::from_static_str("kCGWindowOwnerPID"),
            alpha: CFString::from_static_str("kCGWindowAlpha"),
            x: CFString::from_static_str("X"),
            y: CFString::from_static_str("Y"),
            width: CFString::from_static_str("Width"),
            height: CFString::from_static_str("Height"),
        }
    }
}

impl WindowDescription {
    pub fn number(&self, entry: &Description) -> Option<i64> {
        integer(entry, &self.number)
    }

    pub fn layer(&self, entry: &Description) -> Option<i64> {
        integer(entry, &self.layer)
    }

    pub fn owner_pid(&self, entry: &Description) -> Option<i64> {
        integer(entry, &self.owner_pid)
    }

    // Une fenêtre entièrement transparente ne montre rien : la voiler dessinerait un voile
    // sur du vide.
    pub fn is_transparent(&self, entry: &Description) -> bool {
        real(entry, &self.alpha).is_some_and(|alpha| alpha <= 0.0)
    }

    pub fn frame(&self, entry: &Description) -> Option<Rect> {
        let rect = value(entry, &self.bounds)?
            .downcast::<CFDictionary>()
            .ok()?;
        let rect = entries_of(rect);
        let origin = (real(&rect, &self.x)?, real(&rect, &self.y)?);
        let size = (real(&rect, &self.width)?, real(&rect, &self.height)?);
        to_rect(origin, size)
    }
}

fn value(entry: &Description, key: &CFString) -> Option<CFRetained<CFType>> {
    let key: &CFType = key;
    entry.get(key)
}

fn number(entry: &Description, key: &CFString) -> Option<CFRetained<CFNumber>> {
    value(entry, key)?.downcast::<CFNumber>().ok()
}

fn integer(entry: &Description, key: &CFString) -> Option<i64> {
    number(entry, key)?.as_i64()
}

fn real(entry: &Description, key: &CFString) -> Option<f64> {
    number(entry, key)?.as_f64()
}

// Points Core Graphics (repère global, origine en haut à gauche de l'écran principal)
// arrondis au point entier ; `None` pour une fenêtre trop petite pour être voilée.
fn to_rect((x, y): (f64, f64), (width, height): (f64, f64)) -> Option<Rect> {
    if !(width >= MIN_SIDE && height >= MIN_SIDE) {
        return None;
    }
    Some(Rect {
        x: to_i32(x)?,
        y: to_i32(y)?,
        width: u32::try_from(to_i32(width)?).ok()?,
        height: u32::try_from(to_i32(height)?).ok()?,
    })
}

fn to_i32(value: f64) -> Option<i32> {
    let rounded = value.round();
    let fits =
        rounded.is_finite() && rounded >= f64::from(i32::MIN) && rounded <= f64::from(i32::MAX);
    #[allow(clippy::cast_possible_truncation)]
    fits.then_some(rounded as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ORIGIN: (f64, f64) = (0.0, 0.0);
    const WIDE: f64 = 300.0;

    #[test]
    fn a_window_smaller_than_forty_points_is_never_veiled() {
        assert_eq!(to_rect(ORIGIN, (MIN_SIDE - 1.0, WIDE)), None);
        assert_eq!(to_rect(ORIGIN, (WIDE, MIN_SIDE - 0.6)), None);
    }

    #[test]
    fn a_window_of_exactly_forty_points_is_kept() {
        assert!(to_rect(ORIGIN, (MIN_SIDE, MIN_SIDE)).is_some());
    }

    #[test]
    fn a_frame_on_a_screen_left_of_the_main_one_keeps_its_negative_origin() {
        assert_eq!(
            to_rect((-1440.4, -120.2), (800.0, 600.0)),
            Some(Rect {
                x: -1440,
                y: -120,
                width: 800,
                height: 600
            })
        );
    }

    #[test]
    fn a_non_finite_frame_is_dropped() {
        assert_eq!(to_rect((f64::NAN, 0.0), (WIDE, WIDE)), None);
        assert_eq!(to_rect(ORIGIN, (f64::INFINITY, WIDE)), None);
    }
}
