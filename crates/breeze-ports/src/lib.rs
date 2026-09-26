#![forbid(unsafe_code)]

pub mod apps;
pub mod clock;
pub mod display;
pub mod geometry;
pub mod overlay;
pub mod persistence;
pub mod presentation;
pub mod session;
pub mod window;

pub use apps::{
    AppEnumerationError, ForegroundAppPort, InstalledApp, InstalledAppsPort, SafetyListPort,
};
pub use clock::ClockPort;
pub use display::{Display, DisplayEnumerationPort, DisplayId};
pub use geometry::Rect;
pub use overlay::{OverlayCapability, OverlaySurfacesPort, SurfaceId, SurfaceKind};
pub use persistence::{LedgerDay, LedgerEntry, PersistedState, PersistenceError, PersistencePort};
pub use presentation::PresentationLockPort;
pub use session::{SessionSignals, SessionSignalsPort};
pub use window::{FramesUnobservable, WindowFrame, WindowFramesPort, WindowId};
