use breeze_domain::Instant;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SessionSignals {
    pub last_input: Instant,
}
