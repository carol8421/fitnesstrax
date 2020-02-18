use std::convert::TryFrom;
use std::fmt;

use crate::errors::Error;

#[derive(Debug, PartialEq)]
pub struct HoursMinutes<T>(T);

impl<T> HoursMinutes<T> {
    pub fn new(val: T) -> HoursMinutes<T> {
        HoursMinutes(val)
    }
    pub fn unwrap(&self) -> &T {
        &self.0
    }
}

impl fmt::Display for HoursMinutes<&chrono::NaiveTime> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0.format("%H:%M"))
    }
}

impl TryFrom<&str> for HoursMinutes<chrono::NaiveTime> {
    type Error = Error;

    fn try_from(inp: &str) -> Result<Self, Self::Error> {
        chrono::NaiveTime::parse_from_str(inp, "%H:%M")
            .map(|t| HoursMinutes(t))
            .map_err(|_| Error::ParseTimeError)
    }
}

impl fmt::Display for HoursMinutes<chrono::NaiveTime> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", HoursMinutes(&self.0))
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveTime;
    use std::convert::TryFrom;

    use super::HoursMinutes;
    use crate::errors::Error;

    #[test]
    fn it_displays_hours_and_minutes() {
        let t = HoursMinutes::new(NaiveTime::from_hms(20, 15, 0));
        assert_eq!(format!("{}", t), "20:15");
    }

    #[test]
    fn it_parses_hours_and_minutes() {
        let t = HoursMinutes::try_from("20:15").unwrap();
        assert_eq!(t, HoursMinutes::new(NaiveTime::from_hms(20, 15, 0)));
    }
}
