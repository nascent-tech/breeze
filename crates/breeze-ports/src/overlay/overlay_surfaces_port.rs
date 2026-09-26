use crate::display::DisplayId;
use crate::geometry::Rect;
use crate::overlay::{OverlayCapability, SurfaceId, SurfaceKind};
use crate::window::WindowId;

pub trait OverlaySurfacesPort {
    fn cover_display(&mut self, display: DisplayId, kind: SurfaceKind) -> SurfaceId;
    // Un voile posé sur une seule fenêtre, au cadre donné (points, repère global).
    fn cover_window(&mut self, window: WindowId, frame: Rect) -> SurfaceId;
    // Suit une fenêtre déplacée ou redimensionnée, sans recréer sa surface.
    fn reframe(&mut self, surface: SurfaceId, frame: Rect);
    fn dismiss(&mut self, surface: SurfaceId);
    fn dismiss_all(&mut self);
    fn capability(&self) -> OverlayCapability;
}
