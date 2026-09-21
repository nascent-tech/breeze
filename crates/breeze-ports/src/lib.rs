#![forbid(unsafe_code)]

pub mod clock;
pub mod display;
pub mod geometry;
pub mod overlay;
pub mod persistence;

pub use clock::ClockPort;
pub use display::{Display, DisplayEnumerationPort, DisplayId};
pub use geometry::Rect;
pub use overlay::{OverlayCapability, OverlaySurfacesPort, SurfaceId, SurfaceKind};
pub use persistence::{PersistedState, PersistencePort};
