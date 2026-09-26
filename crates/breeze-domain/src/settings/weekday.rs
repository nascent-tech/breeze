const DAYS_PER_WEEK: u8 = 7;

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
    const WEEK: [Weekday; DAYS_PER_WEEK as usize] = [
        Weekday::Monday,
        Weekday::Tuesday,
        Weekday::Wednesday,
        Weekday::Thursday,
        Weekday::Friday,
        Weekday::Saturday,
        Weekday::Sunday,
    ];

    pub fn bit(self) -> u8 {
        1 << self.days_from_monday()
    }

    pub fn days_from_monday(self) -> u8 {
        match self {
            Weekday::Monday => 0,
            Weekday::Tuesday => 1,
            Weekday::Wednesday => 2,
            Weekday::Thursday => 3,
            Weekday::Friday => 4,
            Weekday::Saturday => 5,
            Weekday::Sunday => 6,
        }
    }

    pub fn from_days_from_monday(days: u8) -> Weekday {
        Self::WEEK[usize::from(days % DAYS_PER_WEEK)]
    }

    pub fn plus_days(self, days: u8) -> Weekday {
        Self::from_days_from_monday(self.days_from_monday() + days % DAYS_PER_WEEK)
    }

    pub fn previous(self) -> Weekday {
        self.plus_days(DAYS_PER_WEEK - 1)
    }
}
