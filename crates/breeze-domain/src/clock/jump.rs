use core::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClockJump {
    Forward(Duration),
    Backward(Duration),
}
