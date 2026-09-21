#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum InterruptionDoor {
    Quit,
    TrayMenu,
    HardcoreExitGesture,
    SuspensionOverrun,
    Crash,
}

impl InterruptionDoor {
    // Une chute est un défaut du logiciel, pas un choix : elle gèle la dette au lieu
    // de la créditer (§9.2, décision 15). Les trois autres portes créditent.
    pub fn charges_debt(self) -> bool {
        !matches!(self, InterruptionDoor::Crash)
    }
}
