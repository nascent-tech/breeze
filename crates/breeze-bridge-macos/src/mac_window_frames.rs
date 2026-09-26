use crate::cf_containers::{entries_of, objects_of};
use crate::running_apps::{app_of_pid, Pid};
use crate::window_description::{Description, WindowDescription};
use breeze_domain::AppId;
use breeze_ports::{FramesUnobservable, WindowFrame, WindowFramesPort, WindowId};
use objc2_core_foundation::CFDictionary;
use objc2_core_graphics::{kCGNullWindowID, CGWindowListCopyWindowInfo, CGWindowListOption};
use std::collections::{HashMap, HashSet};

// Couche des fenêtres d'application ordinaires ; menus, Dock, barres et overlays (dont
// les voiles de Breeze) vivent au-dessus.
const NORMAL_LAYER: i64 = 0;

// Cadres des fenêtres visibles via CGWindowListCopyWindowInfo : sans aucune permission,
// bornes et PID seulement. Le PID devient une identité d'application par
// NSRunningApplication, gardée en cache (si elle a été trouvée) tant que le processus a des
// fenêtres à l'écran.
pub struct MacWindowFrames {
    own_pid: Pid,
    description: WindowDescription,
    owners: HashMap<Pid, AppId>,
}

impl Default for MacWindowFrames {
    fn default() -> Self {
        MacWindowFrames {
            own_pid: Pid::try_from(std::process::id()).unwrap_or(Pid::MAX),
            description: WindowDescription::default(),
            owners: HashMap::new(),
        }
    }
}

impl MacWindowFrames {
    pub fn new() -> Self {
        MacWindowFrames::default()
    }

    // Les fenêtres de Breeze, d'une autre couche, transparentes ou minuscules sont écartées.
    fn read(&mut self, entry: &Description, seen: &mut HashSet<Pid>) -> Option<WindowFrame> {
        let description = &self.description;
        if description.layer(entry)? != NORMAL_LAYER || description.is_transparent(entry) {
            return None;
        }
        let pid = Pid::try_from(description.owner_pid(entry)?).ok()?;
        if pid == self.own_pid {
            return None;
        }
        let id = WindowId(u32::try_from(description.number(entry)?).ok()?);
        let frame = description.frame(entry)?;
        seen.insert(pid);
        let owner = self.owner_of(pid);
        Some(WindowFrame { id, owner, frame })
    }

    // Seule une identité trouvée est gardée : un processus encore sans bundle id (lancement
    // en cours) est redemandé au poll suivant au lieu de rester « inconnu » tout du long.
    fn owner_of(&mut self, pid: Pid) -> Option<AppId> {
        if let Some(owner) = self.owners.get(&pid) {
            return Some(owner.clone());
        }
        let owner = app_of_pid(pid)?;
        self.owners.insert(pid, owner.clone());
        Some(owner)
    }
}

impl WindowFramesPort for MacWindowFrames {
    fn visible_windows(&mut self) -> Result<Vec<WindowFrame>, FramesUnobservable> {
        let options =
            CGWindowListOption::OptionOnScreenOnly | CGWindowListOption::ExcludeDesktopElements;
        let list =
            CGWindowListCopyWindowInfo(options, kCGNullWindowID).ok_or(FramesUnobservable)?;
        let mut seen = HashSet::new();
        let windows = objects_of(list)
            .iter()
            .filter_map(|object| object.downcast::<CFDictionary>().ok())
            .filter_map(|entry| self.read(&entries_of(entry), &mut seen))
            .collect();
        // Un PID sans fenêtre visible est oublié : un PID réutilisé plus tard par une autre
        // application ne reprend jamais une identité périmée.
        self.owners.retain(|pid, _| seen.contains(pid));
        Ok(windows)
    }
}
