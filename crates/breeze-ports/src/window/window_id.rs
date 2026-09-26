// Numéro de fenêtre attribué par le système, stable tant que la fenêtre existe.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct WindowId(pub u32);
