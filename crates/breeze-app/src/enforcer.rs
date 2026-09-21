use breeze_domain::{CycleState, Severity};
use breeze_ports::{
    DisplayEnumerationPort, DisplayId, OverlaySurfacesPort, SurfaceId, SurfaceKind,
};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Enforcer {
    covered: BTreeMap<DisplayId, SurfaceId>,
}

impl Enforcer {
    pub fn reconcile<O, D>(&mut self, state: CycleState, overlay: &mut O, displays: &D)
    where
        O: OverlaySurfacesPort,
        D: DisplayEnumerationPort,
    {
        match kind_for(state) {
            Some(kind) => self.cover_missing(kind, overlay, displays),
            None => self.lift(overlay),
        }
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

    fn lift<O>(&mut self, overlay: &mut O)
    where
        O: OverlaySurfacesPort,
    {
        if self.covered.is_empty() {
            return;
        }
        overlay.dismiss_all();
        self.covered.clear();
    }
}

fn kind_for(state: CycleState) -> Option<SurfaceKind> {
    match state {
        CycleState::BreakActive {
            severity: Severity::Hardcore,
            ..
        } => Some(SurfaceKind::Hardcore),
        CycleState::BreakActive { .. } => Some(SurfaceKind::Veil),
        _ => None,
    }
}
