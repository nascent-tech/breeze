#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OverlayCapability {
    Layered,
    BestEffort,
    PlainFullscreen,
}
