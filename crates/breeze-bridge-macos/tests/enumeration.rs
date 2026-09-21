#![cfg(target_os = "macos")]

use breeze_bridge_macos::MacInstalledApps;
use breeze_ports::InstalledAppsPort;

#[test]
fn it_finds_real_applications_on_this_mac() {
    let apps = MacInstalledApps::new().installed_apps().unwrap();
    assert!(
        !apps.is_empty(),
        "expected at least one app under /Applications or /System/Applications"
    );
}

#[test]
fn every_app_carries_a_valid_bundle_id_and_a_name() {
    let apps = MacInstalledApps::new().installed_apps().unwrap();
    for app in &apps {
        assert!(!app.id.as_str().is_empty());
        assert!(!app.name.trim().is_empty());
    }
}

#[test]
fn at_least_one_app_exposes_a_real_png_icon() {
    let apps = MacInstalledApps::new().installed_apps().unwrap();
    let with_icon = apps.iter().filter(|a| a.icon_png.is_some()).count();
    assert!(
        with_icon > 0,
        "no icon extracted — the sips/plist pipeline is broken"
    );
    for app in apps.iter().filter_map(|a| a.icon_png.as_ref()) {
        assert_eq!(&app[..4], &[0x89, 0x50, 0x4E, 0x47], "expected PNG magic");
    }
}

#[test]
fn a_second_call_is_served_from_cache_and_matches() {
    let port = MacInstalledApps::new();
    let first = port.installed_apps().unwrap();
    let second = port.installed_apps().unwrap();
    assert_eq!(first, second);
}
