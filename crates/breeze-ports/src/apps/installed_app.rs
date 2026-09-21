use breeze_domain::AppId;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InstalledApp {
    pub id: AppId,
    pub name: String,
    // PNG déjà rendu par l'adaptateur ; None = l'UI retombe sur la lettre.
    pub icon_png: Option<Vec<u8>>,
}
