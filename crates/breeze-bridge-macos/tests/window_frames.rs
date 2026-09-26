#![cfg(target_os = "macos")]

use breeze_bridge_macos::{MacForegroundApp, MacInstalledApps, MacSafetyList, MacWindowFrames};
use breeze_ports::{ForegroundAppPort, InstalledAppsPort, SafetyListPort, WindowFramesPort};
use std::time::{Duration, Instant};

const MIN_SIDE: u32 = 40;

// Vérification réelle, sans rien toucher à l'écran : l'énumération tourne sans panique et
// rend des cadres plausibles. Aucun titre n'est lu (le port n'en transporte pas).
#[test]
fn the_visible_windows_are_enumerated_without_permission() {
    let windows = MacWindowFrames::new()
        .visible_windows()
        .expect("CGWindowListCopyWindowInfo answers without any permission");
    let identified = windows.iter().filter(|w| w.owner.is_some()).count();
    eprintln!(
        "visible windows: {} ({identified} with a bundle id)",
        windows.len()
    );
    for window in &windows {
        assert!(window.frame.width >= MIN_SIDE && window.frame.height >= MIN_SIDE);
    }
}

// Le ticker relit les cadres toutes les 250 ms : la lecture doit rester de l'ordre de la
// milliseconde, cache d'identités compris.
#[test]
fn a_reading_stays_well_under_a_tick() {
    const READINGS: u32 = 20;
    const TICK: Duration = Duration::from_millis(250);
    let mut port = MacWindowFrames::new();
    let started = Instant::now();
    for _ in 0..READINGS {
        port.visible_windows().unwrap();
    }
    let per_reading = started.elapsed() / READINGS;
    eprintln!("one reading of the window frames: {per_reading:?}");
    assert!(per_reading < TICK / 10);
}

#[test]
fn the_foreground_app_is_read_without_panicking() {
    let front = MacForegroundApp::new().foreground_app();
    eprintln!("foreground app: {:?}", front.as_ref().map(|id| id.as_str()));
}

#[test]
fn installed_safety_listed_apps_appear_in_the_catalogue() {
    let safety = MacSafetyList.safety_list();
    let apps = MacInstalledApps::new().installed_apps().unwrap();
    let listed = apps.iter().filter(|app| safety.contains(&app.id)).count();
    eprintln!("safety-listed apps in the catalogue: {listed}");
    assert!(
        apps.iter().any(|app| app.id.as_str() == "com.apple.finder"),
        "the Finder lives in CoreServices and must still be listed"
    );
}
