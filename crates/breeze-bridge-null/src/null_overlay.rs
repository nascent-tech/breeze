use breeze_ports::{DisplayId, OverlayCapability, OverlaySurfacesPort, SurfaceId, SurfaceKind};

pub struct NullOverlay;

impl OverlaySurfacesPort for NullOverlay {
    fn cover_display(&mut self, _display: DisplayId, _kind: SurfaceKind) -> SurfaceId {
        SurfaceId(0)
    }

    fn dismiss_all(&mut self) {}

    fn capability(&self) -> OverlayCapability {
        OverlayCapability::PlainFullscreen
    }
}
