#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    pub fn bit(self) -> u8 {
        match self {
            Weekday::Monday => 1 << 0,
            Weekday::Tuesday => 1 << 1,
            Weekday::Wednesday => 1 << 2,
            Weekday::Thursday => 1 << 3,
            Weekday::Friday => 1 << 4,
            Weekday::Saturday => 1 << 5,
            Weekday::Sunday => 1 << 6,
        }
    }
}
