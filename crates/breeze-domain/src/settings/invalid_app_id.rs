#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InvalidAppId {
    Empty,
    TooLong,
    ForbiddenCharacter,
}
