use crate::settings::rhythm_error::RhythmError;
use crate::settings::weekday::Weekday;

const EVERY_DAY: u8 = 0b0111_1111;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ActiveDays(u8);

impl ActiveDays {
    pub fn from_mask(mask: u8) -> Result<Self, RhythmError> {
        let kept = mask & EVERY_DAY;
        if kept == 0 {
            return Err(RhythmError::NoActiveDay);
        }
        Ok(ActiveDays(kept))
    }

    pub fn everyday() -> Self {
        ActiveDays(EVERY_DAY)
    }

    pub fn mask(self) -> u8 {
        self.0
    }

    pub fn contains(self, day: Weekday) -> bool {
        self.0 & day.bit() != 0
    }
}
