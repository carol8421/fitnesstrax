use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::errors::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HoursMinutes<T>(T);

impl<T> HoursMinutes<T> {
    pub fn new(val: T) -> HoursMinutes<T> {
        HoursMinutes(val)
    }

    pub fn extract(self) -> T {
        self.0
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

impl FromStr for HoursMinutes<chrono::NaiveTime> {
    type Err = Error;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        Self::try_from(inp)
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
    fn it_parses_hours_and_minutes_with_try_from() {
        let t = HoursMinutes::try_from("20:15").expect("successful parse");
        assert_eq!(t, HoursMinutes::new(NaiveTime::from_hms(20, 15, 0)));
    }

    #[test]
    fn it_parses_hours_and_minutes_with_from_str() {
        let t = "20:15".parse::<HoursMinutes<NaiveTime>>().unwrap();
        assert_eq!(t, HoursMinutes::new(NaiveTime::from_hms(20, 15, 0)));
    }
}
