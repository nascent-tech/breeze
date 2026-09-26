use crate::running_apps::app_of;
use breeze_domain::AppId;
use breeze_ports::ForegroundAppPort;
use objc2_app_kit::NSWorkspace;

// L'application au premier plan, lue dans NSWorkspace : identité seule, sans permission,
// jamais le titre d'une fenêtre (§10.6).
#[derive(Default)]
pub struct MacForegroundApp;

impl MacForegroundApp {
    pub fn new() -> Self {
        MacForegroundApp
    }
}

impl ForegroundAppPort for MacForegroundApp {
    fn foreground_app(&mut self) -> Option<AppId> {
        let front = NSWorkspace::sharedWorkspace().frontmostApplication()?;
        app_of(&front)
    }
}
