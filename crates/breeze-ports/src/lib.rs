#![forbid(unsafe_code)]

pub mod apps;
pub mod clock;
pub mod display;
pub mod geometry;
pub mod overlay;
pub mod persistence;
pub mod session;

pub use apps::{
    AccessibilityPermissionPort, AppEnumerationError, InstalledApp, InstalledAppsPort,
    PermissionStatus,
};
pub use clock::ClockPort;
pub use display::{Display, DisplayEnumerationPort, DisplayId};
pub use geometry::Rect;
pub use overlay::{OverlayCapability, OverlaySurfacesPort, SurfaceId, SurfaceKind};
pub use persistence::{PersistedState, PersistenceError, PersistencePort};
pub use session::{SessionSignals, SessionSignalsPort};
