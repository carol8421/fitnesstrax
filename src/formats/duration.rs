use dimensioned::si;

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::errors::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Duration(si::Second<f64>);

impl Duration {
    pub fn new(val: si::Second<f64>) -> Duration {
        Duration(val)
    }

    pub fn extract(self) -> si::Second<f64> {
        self.0
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let seconds = self.0.value_unsafe.rem_euclid(60.0) as i8;
        let minutes = (self.0.value_unsafe / 60.0).rem_euclid(60.0) as i8;
        let hours = (self.0.value_unsafe / 3600.0) as i32;

        if hours == 0 {
            if minutes == 0 {
                write!(f, "{}", seconds)
            } else {
                write!(f, "{}:{:02}", minutes, seconds)
            }
        } else {
            write!(f, "{}:{:02}:{:02}", hours, minutes, seconds)
        }
    }
}

impl TryFrom<&str> for Duration {
    type Error = Error;

    fn try_from(inp: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = inp.split(":").collect();
        match parts.len() {
            3 => {
                let seconds = parts[2]
                    .parse::<f64>()
                    .map_err(|_| Error::ParseDurationError)?;
                let minutes = parts[1]
                    .parse::<f64>()
                    .map_err(|_| Error::ParseDurationError)?;
                let hours = parts[0]
                    .parse::<f64>()
                    .map_err(|_| Error::ParseDurationError)?;
                Ok((seconds + minutes * 60.0 + hours * 3600.0) * si::S)
            }
            2 => {
                let seconds = parts[1]
                    .parse::<f64>()
                    .map_err(|_| Error::ParseDurationError)?;
                let minutes = parts[0]
                    .parse::<f64>()
                    .map_err(|_| Error::ParseDurationError)?;
                Ok((seconds + minutes * 60.0) * si::S)
            }
            1 => parts[0]
                .parse::<f64>()
                .map(|f| f * si::S)
                .map_err(|_| Error::ParseDurationError),
            _ => Err(Error::ParseDurationError),
        }
        .map(|d| Duration(d))
    }
}

impl FromStr for Duration {
    type Err = Error;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        Self::try_from(inp)
    }
}

#[cfg(test)]
mod test {
    use dimensioned::si;
    use std::convert::TryFrom;

    use super::Duration;
    use crate::errors::Error;

    #[test]
    fn it_displays_s() {
        let t = Duration::new(15. * si::S);
        assert_eq!(format!("{}", t), "15");
    }

    #[test]
    fn it_displays_ms() {
        let t = Duration::new(315. * si::S);
        assert_eq!(format!("{}", t), "5:15");
    }

    #[test]
    fn it_displays_hms() {
        let t = Duration::new(3915. * si::S);
        assert_eq!(format!("{}", t), "1:05:15");
    }

    #[test]
    fn it_parses_s_with_try_from() {
        let t = Duration::try_from("15").expect("successful parse");
        assert_eq!(t, Duration::new(15. * si::S));
    }

    #[test]
    fn it_parses_ms_with_try_from() {
        let t = Duration::try_from("5:15").expect("successful parse");
        assert_eq!(t, Duration::new(315. * si::S));
    }

    #[test]
    fn it_parses_hms_with_try_from() {
        let t = Duration::try_from("1:05:15").expect("successful parse");
        assert_eq!(t, Duration::new(3915. * si::S));
    }

    #[test]
    fn it_parses_hms_with_from_str() {
        let t = "1:05:15".parse::<Duration>().expect("successful parse");
        assert_eq!(t, Duration::new(3915. * si::S));
    }
}
