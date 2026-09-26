use crate::settings::weekday::Weekday;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct NextStart {
    pub days_ahead: u8,
    pub weekday: Weekday,
    pub minute_of_day: u16,
}
