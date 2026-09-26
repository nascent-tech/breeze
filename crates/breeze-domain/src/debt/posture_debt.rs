use crate::constants::SECONDS_PER_MINUTE;
use core::time::Duration;

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct PostureDebt {
    owed: Duration,
    absorbed: Duration,
}

impl PostureDebt {
    pub fn none() -> Self {
        PostureDebt::default()
    }

    pub fn restore(total: Duration) -> Self {
        PostureDebt {
            owed: total,
            absorbed: Duration::ZERO,
        }
    }

    pub fn owed(self) -> Duration {
        self.owed
    }

    // Allongement prêté à la pause en cours, pas encore remboursé.
    pub fn absorbed(self) -> Duration {
        self.absorbed
    }

    pub fn total(self) -> Duration {
        self.owed.saturating_add(self.absorbed)
    }

    pub fn is_none(self) -> bool {
        self.total().is_zero()
    }

    // La dette affichée (§9.2, « visible en permanence sur l'icône ») : le total, prêt
    // en cours compris — une pause allongée n'a rien remboursé tant qu'elle n'a pas servi.
    // Arrondi au plafond pour ne jamais afficher 0 devant une dette réelle.
    pub fn minutes(self) -> u16 {
        u16::try_from(self.total().as_secs().div_ceil(SECONDS_PER_MINUTE)).unwrap_or(u16::MAX)
    }

    // `unserved` est mesuré sur l'échéance déjà allongée : le prêt (`absorbed`) y est
    // compris, d'où la remise à zéro d'`absorbed` sans double compte.
    pub fn credit(&mut self, unserved: Duration) {
        self.owed = self.owed.saturating_add(unserved);
        self.absorbed = Duration::ZERO;
    }

    pub fn extension_within(self, room: Duration) -> Duration {
        self.owed.min(room)
    }

    pub fn absorb(&mut self, extension: Duration) {
        let taken = extension.min(self.owed);
        self.owed -= taken;
        self.absorbed = taken;
    }

    pub fn settle(&mut self) {
        self.absorbed = Duration::ZERO;
    }

    pub fn freeze(&mut self) {
        self.owed = self.owed.saturating_add(self.absorbed);
        self.absorbed = Duration::ZERO;
    }

    pub fn clear(&mut self) {
        *self = PostureDebt::none();
    }
}
