use crate::display::DisplayId;
use crate::geometry::Rect;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Display {
    pub id: DisplayId,
    pub bounds: Rect,
}
