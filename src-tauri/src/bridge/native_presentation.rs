// Verrou de présentation d'une pause Hardcore, côté AppKit (§8.5, §5.3) : barre de menus et
// Dock masqués, ⌘Tab et « Masquer Breeze » désactivés, Breeze au premier plan avec la
// surface de l'écran principal pour fenêtre clé (elle reçoit Échap). Forcer à quitter et la
// fermeture de session ne sont JAMAIS désactivés. macOS seulement ; sauf `frontmost_other_app`,
// tout s'appelle sur le thread principal et, hors de lui, ne fait rien (journalisé).

use super::native_window::shielding_level;
use super::presentation_options::{hardcore_options, is_legal};
use objc2::rc::Retained;
use objc2::MainThreadMarker;
use objc2_app_kit::{
    NSApplication, NSApplicationActivationOptions, NSApplicationPresentationOptions,
    NSRunningApplication, NSScreen, NSWindow, NSWorkspace,
};

pub type Pid = i32;

// La fenêtre « Forcer à quitter » (⌥⌘Échap) et le dialogue d'extinction appartiennent à
// loginwindow : Breeze ne lui reprend jamais la main, forcer à quitter reste possible (§8.5).
const SESSION_UI: &str = "com.apple.loginwindow";

// L'application au premier plan, si ce n'est pas Breeze : celle à qui rendre la main à la
// sortie. Lue sur le fil du ticker au moment où le verrou est demandé : la création des
// surfaces vient d'être expédiée au thread principal et n'a pas eu le temps d'activer Breeze
// (fenêtre, webview, puis aller-retour avec le serveur de fenêtres). NSWorkspace se lit de
// tout thread, comme pour la détection du premier plan.
pub fn frontmost_other_app() -> Option<Pid> {
    let pid = NSWorkspace::sharedWorkspace()
        .frontmostApplication()?
        .processIdentifier();
    let own = Pid::try_from(std::process::id()).ok()?;
    (pid != own).then_some(pid)
}

fn main_thread(what: &str) -> Option<MainThreadMarker> {
    let marker = MainThreadMarker::new();
    if marker.is_none() {
        eprintln!("breeze: presentation {what} called off the main thread");
    }
    marker
}

pub fn lock() {
    let Some(mtm) = main_thread("lock") else {
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    apply(&app, hardcore_options());
    bring_forward(&app, mtm);
}

// À chaque tick : rien tant que Breeze est actif avec une surface Hardcore pour fenêtre clé
// (celle de n'importe quel écran : chacune porte le geste), ni quand la session a la main.
// Sinon — une notification, Spotlight, un clic l'ont prise — Breeze est réactivé et ses
// surfaces ramenées devant.
pub fn hold() {
    let Some(mtm) = main_thread("hold") else {
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    if app.presentationOptions() != hardcore_options() {
        apply(&app, hardcore_options());
    }
    let key_is_shield = app.keyWindow().is_some_and(|window| is_shield(&window));
    if (app.isActive() && key_is_shield) || session_ui_in_front() {
        return;
    }
    bring_forward(&app, mtm);
}

fn session_ui_in_front() -> bool {
    NSWorkspace::sharedWorkspace()
        .frontmostApplication()
        .and_then(|front| front.bundleIdentifier())
        .is_some_and(|bundle| bundle.to_string() == SESSION_UI)
}

// Options rendues d'abord, puis la main rendue à l'application d'avant la pause si elle
// tourne encore et si Breeze l'a toujours : sinon l'utilisateur est déjà ailleurs.
pub fn release(previous: Option<Pid>) {
    let Some(mtm) = main_thread("release") else {
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    apply(&app, NSApplicationPresentationOptions::Default);
    let Some(previous) =
        previous.and_then(NSRunningApplication::runningApplicationWithProcessIdentifier)
    else {
        return;
    };
    if app.isActive() && !previous.isTerminated() {
        previous.activateWithOptions(NSApplicationActivationOptions::empty());
    }
}

// Au dernier instant du processus : la présentation par défaut, sans attendre que macOS la
// rende lui-même à la mort de Breeze.
pub fn restore_default() {
    let Some(mtm) = main_thread("restore") else {
        return;
    };
    apply(
        &NSApplication::sharedApplication(mtm),
        NSApplicationPresentationOptions::Default,
    );
}

fn apply(app: &NSApplication, options: NSApplicationPresentationOptions) {
    if !is_legal(options) {
        eprintln!("breeze: presentation options {options:?} refused by AppKit rules");
        return;
    }
    app.setPresentationOptions(options);
}

// Depuis macOS 14 l'activation est coopérative : ce `activateIgnoringOtherApps:` est ignoré
// sans geste de l'utilisateur (vérifié sur macOS 27), et c'est le premier clic sur la surface
// qui donne réellement la main à Breeze. Il reste tenté : il agit sous macOS 13, où `activate`
// n'existe pas. Les surfaces, elles, sont toujours ramenées devant.
fn bring_forward(app: &NSApplication, mtm: MainThreadMarker) {
    #[allow(deprecated)]
    app.activateIgnoringOtherApps(true);
    let shields = shields(app);
    for shield in &shields {
        shield.orderFrontRegardless();
    }
    if let Some(key) = on_primary_screen(&shields, mtm).or_else(|| shields.first()) {
        key.makeKeyAndOrderFront(None);
    }
}

// Les surfaces Hardcore, reconnues à leur niveau : aucune autre fenêtre de Breeze n'y monte.
fn is_shield(window: &NSWindow) -> bool {
    window.isVisible() && window.level() == shielding_level()
}

fn shields(app: &NSApplication) -> Vec<Retained<NSWindow>> {
    app.windows()
        .to_vec()
        .into_iter()
        .filter(|window| is_shield(window))
        .collect()
}

// L'écran principal est le premier de la liste (celui de la barre de menus).
fn on_primary_screen(
    shields: &[Retained<NSWindow>],
    mtm: MainThreadMarker,
) -> Option<&Retained<NSWindow>> {
    let primary = NSScreen::screens(mtm).firstObject()?.frame();
    shields.iter().find(|shield| {
        shield
            .screen()
            .is_some_and(|screen| screen.frame() == primary)
    })
}
