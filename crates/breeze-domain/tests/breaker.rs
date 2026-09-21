use breeze_domain::constants::{BREAKER_DISARM, BREAKER_WINDOW};
use breeze_domain::{Breaker, Instant};
use core::time::Duration;

const ONE_MINUTE: Duration = Duration::from_secs(60);

#[test]
fn three_crashes_within_the_window_arm_the_breaker() {
    let mut breaker = Breaker::new();
    breaker.record_crash(Instant::EPOCH);
    breaker.record_crash(Instant::EPOCH.plus(ONE_MINUTE));
    assert!(!breaker.is_armed());
    breaker.record_crash(Instant::EPOCH.plus(ONE_MINUTE).plus(ONE_MINUTE));
    assert!(breaker.is_armed());
}

#[test]
fn crashes_spread_beyond_the_window_do_not_arm() {
    let mut breaker = Breaker::new();
    let step = BREAKER_WINDOW.saturating_add(ONE_MINUTE);
    breaker.record_crash(Instant::EPOCH);
    breaker.record_crash(Instant::EPOCH.plus(step));
    breaker.record_crash(Instant::EPOCH.plus(step).plus(step));
    assert!(!breaker.is_armed());
}

#[test]
fn the_breaker_disarms_after_a_day_without_a_crash() {
    let mut breaker = Breaker::new();
    breaker.record_crash(Instant::EPOCH);
    breaker.record_crash(Instant::EPOCH.plus(ONE_MINUTE));
    breaker.record_crash(Instant::EPOCH.plus(ONE_MINUTE).plus(ONE_MINUTE));
    assert!(breaker.is_armed());
    breaker.tick(Instant::EPOCH.plus(BREAKER_DISARM).plus(BREAKER_DISARM));
    assert!(!breaker.is_armed());
}

#[test]
fn a_user_reset_disarms_the_breaker() {
    let mut breaker = Breaker::new();
    breaker.record_crash(Instant::EPOCH);
    breaker.record_crash(Instant::EPOCH.plus(ONE_MINUTE));
    breaker.record_crash(Instant::EPOCH.plus(ONE_MINUTE).plus(ONE_MINUTE));
    assert!(breaker.is_armed());
    breaker.reset();
    assert!(!breaker.is_armed());
}
