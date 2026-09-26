use breeze_domain::{BreakMode, CycleState, Severity};
use breeze_ports::{
    DisplayEnumerationPort, DisplayId, OverlaySurfacesPort, PresentationLockPort, Rect, SurfaceId,
    SurfaceKind, WindowFrame, WindowId,
};
use std::collections::BTreeMap;

// Ce que l'état du cycle demande de couvrir.
#[derive(Clone, Copy)]
enum Coverage {
    Nothing,
    // Un overlay par écran : Hardcore opaque, ou voile Simple dégradé.
    Screens(SurfaceKind),
    // Un voile par fenêtre bloquée (Mode Simple nominal).
    Windows,
}

#[derive(Clone, Copy)]
struct WindowVeil {
    surface: SurfaceId,
    frame: Rect,
}

// Traduit l'état du cycle en surfaces posées, déplacées ou retirées. Il ne décide rien :
// le mode de la pause et les fenêtres bloquées lui arrivent déjà tranchés.
#[derive(Default)]
pub struct Enforcer {
    covered: BTreeMap<DisplayId, SurfaceId>,
    veiled: BTreeMap<WindowId, WindowVeil>,
    // Le verrou de présentation est posé : une couverture Hardcore est en cours.
    locked: bool,
}

impl Enforcer {
    // `blocked` = fenêtres bloquées visibles ; `None` quand les cadres n'ont pas pu être
    // relevés à ce poll : les voiles déjà posés restent alors tels quels.
    pub fn reconcile<O, L, D>(
        &mut self,
        state: CycleState,
        blocked: Option<&[WindowFrame]>,
        overlay: &mut O,
        presentation: &mut L,
        displays: &D,
    ) where
        O: OverlaySurfacesPort,
        L: PresentationLockPort + ?Sized,
        D: DisplayEnumerationPort,
    {
        let coverage = coverage_for(state);
        match coverage {
            Coverage::Nothing => self.lift(overlay),
            Coverage::Screens(kind) => {
                self.lift_window_veils(overlay);
                self.cover_missing(kind, overlay, displays);
            }
            Coverage::Windows => {
                if let Some(blocked) = blocked {
                    self.follow(blocked, overlay);
                }
            }
        }
        let hardcore = matches!(coverage, Coverage::Screens(SurfaceKind::Hardcore));
        self.keep_presentation(hardcore, presentation);
    }

    // Après les surfaces : le verrou s'expédie derrière leur création, et sa levée derrière
    // leur retrait. Posé à l'entrée, tenu à chaque poll, levé une seule fois à la sortie.
    fn keep_presentation<L>(&mut self, hardcore: bool, presentation: &mut L)
    where
        L: PresentationLockPort + ?Sized,
    {
        match (hardcore, self.locked) {
            (true, false) => presentation.lock(),
            (true, true) => presentation.hold(),
            (false, true) => presentation.release(),
            (false, false) => return,
        }
        self.locked = hardcore;
    }

    fn cover_missing<O, D>(&mut self, kind: SurfaceKind, overlay: &mut O, displays: &D)
    where
        O: OverlaySurfacesPort,
        D: DisplayEnumerationPort,
    {
        for display in displays.displays() {
            if self.covered.contains_key(&display.id) {
                continue;
            }
            let surface = overlay.cover_display(display.id, kind);
            self.covered.insert(display.id, surface);
        }
    }

    // Un voile par fenêtre bloquée : posé pour une fenêtre apparue, déplacé pour une
    // fenêtre qui a bougé, retiré pour une fenêtre fermée ou minimisée. Une surface existante
    // est toujours réutilisée.
    fn follow<O: OverlaySurfacesPort>(&mut self, blocked: &[WindowFrame], overlay: &mut O) {
        let wanted: BTreeMap<WindowId, Rect> = blocked.iter().map(|w| (w.id, w.frame)).collect();
        self.drop_vanished(&wanted, overlay);
        for (id, frame) in wanted {
            self.place(id, frame, overlay);
        }
    }

    fn drop_vanished<O: OverlaySurfacesPort>(
        &mut self,
        wanted: &BTreeMap<WindowId, Rect>,
        overlay: &mut O,
    ) {
        self.veiled.retain(|id, veil| {
            let keep = wanted.contains_key(id);
            if !keep {
                overlay.dismiss(veil.surface);
            }
            keep
        });
    }

    fn place<O: OverlaySurfacesPort>(&mut self, id: WindowId, frame: Rect, overlay: &mut O) {
        let Some(veil) = self.veiled.get_mut(&id) else {
            let surface = overlay.cover_window(id, frame);
            self.veiled.insert(id, WindowVeil { surface, frame });
            return;
        };
        if veil.frame != frame {
            overlay.reframe(veil.surface, frame);
            veil.frame = frame;
        }
    }

    fn lift_window_veils<O: OverlaySurfacesPort>(&mut self, overlay: &mut O) {
        for veil in core::mem::take(&mut self.veiled).into_values() {
            overlay.dismiss(veil.surface);
        }
    }

    fn lift<O: OverlaySurfacesPort>(&mut self, overlay: &mut O) {
        if self.covered.is_empty() && self.veiled.is_empty() {
            return;
        }
        overlay.dismiss_all();
        self.covered.clear();
        self.veiled.clear();
    }
}

fn coverage_for(state: CycleState) -> Coverage {
    match state {
        CycleState::BreakActive {
            severity: Severity::Hardcore,
            ..
        } => Coverage::Screens(SurfaceKind::Hardcore),
        CycleState::BreakActive {
            mode: BreakMode::Degraded(_),
            ..
        } => Coverage::Screens(SurfaceKind::Veil),
        CycleState::BreakActive { .. } => Coverage::Windows,
        _ => Coverage::Nothing,
    }
}
