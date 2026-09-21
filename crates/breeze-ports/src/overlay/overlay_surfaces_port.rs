use crate::display::DisplayId;
use crate::overlay::{OverlayCapability, SurfaceId, SurfaceKind};

pub trait OverlaySurfacesPort {
    fn cover_display(&mut self, display: DisplayId, kind: SurfaceKind) -> SurfaceId;
    fn dismiss_all(&mut self);
    fn capability(&self) -> OverlayCapability;
}
