use crate::session::SessionSignals;
use breeze_domain::Instant;

pub trait SessionSignalsPort {
    fn poll(&mut self, now: Instant) -> SessionSignals;
}
