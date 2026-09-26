#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AppStatus {
    Blocked,
    Spared,
    Ignored,
}

impl AppStatus {
    // Ce que le statut impose, du plus fort au plus faible : Bloquée (voilée, compte
    // comme travail) > Épargnée (libre, compte comme travail) > Ignorée (libre, gèle).
    fn constraint(self) -> u8 {
        match self {
            AppStatus::Blocked => 2,
            AppStatus::Spared => 1,
            AppStatus::Ignored => 0,
        }
    }

    // Passer de `from` à `self` relâche-t-il la contrainte (§10.3) ?
    pub fn weakens(self, from: AppStatus) -> bool {
        self.constraint() < from.constraint()
    }
}
