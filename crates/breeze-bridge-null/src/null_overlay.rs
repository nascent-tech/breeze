use breeze_ports::{
    DisplayId, OverlayCapability, OverlaySurfacesPort, Rect, SurfaceId, SurfaceKind, WindowId,
};

pub struct NullOverlay;

impl OverlaySurfacesPort for NullOverlay {
    fn cover_display(&mut self, _display: DisplayId, _kind: SurfaceKind) -> SurfaceId {
        SurfaceId(0)
    }

    fn cover_window(&mut self, _window: WindowId, _frame: Rect) -> SurfaceId {
        SurfaceId(0)
    }

    fn reframe(&mut self, _surface: SurfaceId, _frame: Rect) {}

    fn dismiss(&mut self, _surface: SurfaceId) {}

    fn dismiss_all(&mut self) {}

    fn capability(&self) -> OverlayCapability {
        OverlayCapability::PlainFullscreen
    }
}
