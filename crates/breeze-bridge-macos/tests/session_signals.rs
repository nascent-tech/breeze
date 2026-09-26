#![cfg(target_os = "macos")]

use breeze_bridge_macos::MacSessionSignals;
use breeze_domain::Instant;
use breeze_ports::SessionSignalsPort;
use core::time::Duration;

const A_YEAR: Duration = Duration::from_secs(365 * 24 * 3600);

#[test]
fn the_idle_time_read_from_the_session_is_plausible() {
    let idle = MacSessionSignals::new().idle();
    eprintln!("session idle time: {idle:?}");
    assert!(idle < A_YEAR, "implausible idle time: {idle:?}");
}

#[test]
fn the_last_input_lies_in_the_recent_past() {
    let now = Instant::EPOCH.plus(A_YEAR);
    let last_input = MacSessionSignals::new().poll(now).last_input;
    assert!(last_input <= now);
    assert!(now.elapsed_since(last_input) < A_YEAR);
}
