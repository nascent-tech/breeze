use breeze_domain::Instant;
use breeze_ports::{SessionSignals, SessionSignalsPort};
use core::time::Duration;
use objc2_core_graphics::{CGEventSource, CGEventSourceStateID, CGEventType};

// kCGAnyInputEventType : toute saisie (clavier, souris, trackpad, tablette).
const ANY_INPUT: CGEventType = CGEventType(u32::MAX);
// `now - idle` tremble de quelques millisecondes d'un relevé à l'autre ; sans cette
// marge, le tremblement passerait pour une saisie fraîche et dégèlerait le décompte.
const JITTER: Duration = Duration::from_secs(1);

// Temps d'inactivité réel lu dans l'état HID de la session. Aucune permission requise
// (ni Accessibilité, ni surveillance des saisies) : seulement « depuis quand ».
pub struct MacSessionSignals {
    last_reported: Instant,
}

impl Default for MacSessionSignals {
    fn default() -> Self {
        Self::new()
    }
}

impl MacSessionSignals {
    pub fn new() -> Self {
        MacSessionSignals {
            last_reported: Instant::EPOCH,
        }
    }

    pub fn idle(&self) -> Duration {
        let seconds = CGEventSource::seconds_since_last_event_type(
            CGEventSourceStateID::HIDSystemState,
            ANY_INPUT,
        );
        Duration::try_from_secs_f64(seconds).unwrap_or(Duration::ZERO)
    }
}

fn settle(previous: Instant, candidate: Instant) -> Instant {
    if candidate > previous.plus(JITTER) {
        candidate
    } else {
        previous
    }
}

impl SessionSignalsPort for MacSessionSignals {
    fn poll(&mut self, now: Instant) -> SessionSignals {
        self.last_reported = settle(self.last_reported, now.minus(self.idle()));
        SessionSignals {
            last_input: self.last_reported,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const START: Instant = Instant::EPOCH;

    #[test]
    fn a_few_milliseconds_of_jitter_are_not_a_fresh_input() {
        let previous = START.plus(Duration::from_secs(60));
        let jittered = previous.plus(Duration::from_millis(4));
        assert_eq!(settle(previous, jittered), previous);
    }

    #[test]
    fn a_clearly_later_input_is_reported() {
        let previous = START.plus(Duration::from_secs(60));
        let later = previous.plus(Duration::from_secs(5));
        assert_eq!(settle(previous, later), later);
    }

    #[test]
    fn the_reported_input_never_goes_back() {
        let previous = START.plus(Duration::from_secs(60));
        assert_eq!(settle(previous, START), previous);
    }
}
