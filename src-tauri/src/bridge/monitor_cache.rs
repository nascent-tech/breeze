use breeze_ports::{Display, DisplayId, Rect};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Monitor};

const FNV_OFFSET: u32 = 0x811C_9DC5;
const FNV_PRIME: u32 = 0x0100_0193;

pub fn display_id_of(monitor: &Monitor) -> DisplayId {
    hash_display(
        monitor.name().map(String::as_str),
        monitor.position().x,
        monitor.position().y,
    )
}

fn hash_display(name: Option<&str>, x: i32, y: i32) -> DisplayId {
    let mut hash = FNV_OFFSET;
    let mut feed = |bytes: &[u8]| {
        for byte in bytes {
            hash ^= u32::from(*byte);
            hash = hash.wrapping_mul(FNV_PRIME);
        }
    };
    feed(name.map(str::as_bytes).unwrap_or_default());
    feed(&x.to_le_bytes());
    feed(&y.to_le_bytes());
    DisplayId(hash)
}

pub fn display_of(monitor: &Monitor) -> Display {
    Display {
        id: display_id_of(monitor),
        bounds: Rect {
            x: monitor.position().x,
            y: monitor.position().y,
            width: monitor.size().width,
            height: monitor.size().height,
        },
    }
}

#[derive(Clone, Default)]
pub struct MonitorCache(Arc<Mutex<Vec<Display>>>);

impl MonitorCache {
    pub fn snapshot(&self) -> Vec<Display> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }

    // À appeler depuis le thread ticker, HORS du verrou scheduler : non bloquant
    // (le rafraîchissement lit les moniteurs sur le main thread, plus tard).
    pub fn refresh_from_main_thread(&self, app: &AppHandle) {
        let cache = self.clone();
        let handle = app.clone();
        let dispatched = app.run_on_main_thread(move || {
            let displays = handle
                .available_monitors()
                .map(|monitors| monitors.iter().map(display_of).collect())
                .unwrap_or_default();
            *cache
                .0
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner) = displays;
        });
        if let Err(error) = dispatched {
            eprintln!("breeze: monitor refresh not dispatched: {error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_same_screen_hashes_to_the_same_id() {
        assert_eq!(
            hash_display(Some("DELL"), 0, 0),
            hash_display(Some("DELL"), 0, 0)
        );
    }

    #[test]
    fn two_identical_screens_at_different_positions_differ() {
        assert_ne!(
            hash_display(Some("DELL"), 0, 0),
            hash_display(Some("DELL"), 1920, 0)
        );
    }

    #[test]
    fn a_nameless_screen_still_gets_an_id_bound_to_its_position() {
        assert_ne!(hash_display(None, 0, 0), hash_display(None, 0, 1080));
    }
}
