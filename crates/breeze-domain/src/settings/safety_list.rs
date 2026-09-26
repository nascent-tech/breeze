use crate::settings::app_id::AppId;
use std::collections::BTreeSet;

// Liste de sécurité de l'OS courant (§10.6) : fournie par l'adaptateur de la plateforme,
// jamais éditable. Une app qui y figure vaut toujours Épargnée, jamais Ignorée.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SafetyList(BTreeSet<AppId>);

impl SafetyList {
    pub fn new(ids: impl IntoIterator<Item = AppId>) -> Self {
        SafetyList(ids.into_iter().collect())
    }

    pub fn contains(&self, id: &AppId) -> bool {
        self.0.contains(id)
    }

    pub fn ids(&self) -> impl Iterator<Item = &AppId> {
        self.0.iter()
    }
}
