use crate::geometry::Rect;
use crate::window::WindowId;
use breeze_domain::AppId;

// Une fenêtre visible d'une autre application : son numéro, l'identité de l'application
// qui la porte (`None` si le système ne la donne pas) et son cadre en points, dans le
// repère global de l'écran principal (origine en haut à gauche). Jamais le titre.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindowFrame {
    pub id: WindowId,
    pub owner: Option<AppId>,
    pub frame: Rect,
}
