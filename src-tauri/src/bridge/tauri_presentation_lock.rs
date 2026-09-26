use super::native_presentation::{self, Pid};
use breeze_ports::PresentationLockPort;
use tauri::AppHandle;

// Verrou de présentation macOS. Comme les surfaces, chaque opération est expédiée au thread
// principal sans l'attendre : l'appel a lieu sous le verrou du scheduler, l'exécution plus
// tard, hors verrou. L'ordre d'expédition est l'ordre d'exécution : le verrou passe derrière
// la création des surfaces, sa levée derrière leur retrait.
pub struct TauriPresentationLock {
    app: AppHandle,
    // L'application à qui rendre la main à la sortie de la pause.
    previous: Option<Pid>,
}

impl TauriPresentationLock {
    pub fn new(app: AppHandle) -> Self {
        TauriPresentationLock {
            app,
            previous: None,
        }
    }

    fn on_main_thread(&self, what: &'static str, task: impl FnOnce() + Send + 'static) {
        if let Err(error) = self.app.run_on_main_thread(task) {
            eprintln!("breeze: presentation {what} not dispatched: {error}");
        }
    }
}

impl PresentationLockPort for TauriPresentationLock {
    fn lock(&mut self) {
        self.previous = native_presentation::frontmost_other_app();
        self.on_main_thread("lock", native_presentation::lock);
    }

    fn hold(&mut self) {
        self.on_main_thread("hold", native_presentation::hold);
    }

    fn release(&mut self) {
        let previous = self.previous.take();
        self.on_main_thread("release", move || native_presentation::release(previous));
    }
}
