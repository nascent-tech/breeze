use breeze_domain::constants::{BREAKER_DISARM, BREAKER_WINDOW};
use breeze_domain::{Breaker, WallClock};

fn at(seconds: u64) -> WallClock {
    WallClock::from_unix_secs(seconds)
}

#[test]
fn three_crashes_within_the_window_arm_the_breaker() {
    let mut breaker = Breaker::new();
    breaker.record_crash(at(0));
    breaker.record_crash(at(60));
    assert!(!breaker.is_armed());
    breaker.record_crash(at(120));
    assert!(breaker.is_armed());
}

#[test]
fn crashes_a_full_window_apart_never_arm() {
    let window = BREAKER_WINDOW.as_secs();
    let mut breaker = Breaker::new();
    breaker.record_crash(at(0));
    breaker.record_crash(at(window));
    breaker.record_crash(at(window + window));
    assert!(!breaker.is_armed());
}

#[test]
fn the_breaker_disarms_a_day_after_the_last_crash() {
    let day = BREAKER_DISARM.as_secs();
    let mut breaker = Breaker::new();
    breaker.record_crash(at(0));
    breaker.record_crash(at(60));
    breaker.record_crash(at(120));
    assert!(breaker.is_armed());
    breaker.tick(at(120 + day));
    assert!(!breaker.is_armed());
}

#[test]
fn a_fresh_crash_pushes_the_disarm_out() {
    let day = BREAKER_DISARM.as_secs();
    let mut breaker = Breaker::new();
    breaker.record_crash(at(0));
    breaker.record_crash(at(60));
    breaker.record_crash(at(120));
    let nearly_a_day = 120 + day - 60;
    breaker.record_crash(at(nearly_a_day));
    assert!(breaker.is_armed());
    breaker.tick(at(nearly_a_day + day - 60));
    assert!(breaker.is_armed());
}

#[test]
fn a_user_reset_disarms_the_breaker() {
    let mut breaker = Breaker::new();
    breaker.record_crash(at(0));
    breaker.record_crash(at(60));
    breaker.record_crash(at(120));
    assert!(breaker.is_armed());
    breaker.reset();
    assert!(!breaker.is_armed());
}

#[test]
fn restore_round_trips_the_state() {
    let mut breaker = Breaker::new();
    breaker.record_crash(at(0));
    breaker.record_crash(at(60));
    breaker.record_crash(at(120));
    let restored = Breaker::restore(breaker.crashes().to_vec(), breaker.is_armed());
    assert_eq!(restored, breaker);
}
