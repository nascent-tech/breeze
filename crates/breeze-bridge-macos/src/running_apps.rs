use breeze_domain::AppId;
use objc2_app_kit::NSRunningApplication;

pub type Pid = i32;

// Identité (bundle id) d'une application lancée. `None` quand le système n'en donne pas
// (processus sans Info.plist) ou qu'elle n'est pas un AppId valide : l'application reste
// alors inconnue, donc Bloquée (§8.3).
pub fn app_of(running: &NSRunningApplication) -> Option<AppId> {
    let bundle = running.bundleIdentifier()?;
    AppId::parse(&bundle.to_string()).ok()
}

pub fn app_of_pid(pid: Pid) -> Option<AppId> {
    let running = NSRunningApplication::runningApplicationWithProcessIdentifier(pid)?;
    app_of(&running)
}
