use dimensioned::si;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::errors::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnitSystem {
    SI,
    USA,
}

impl From<&UnitSystem> for String {
    fn from(units: &UnitSystem) -> String {
        match units {
            UnitSystem::SI => String::from("SI"),
            UnitSystem::USA => String::from("USA"),
        }
    }
}

impl TryFrom<&str> for UnitSystem {
    type Error = Error;

    fn try_from(s: &str) -> Result<UnitSystem, Error> {
        match s {
            "SI" => Ok(UnitSystem::SI),
            "USA" => Ok(UnitSystem::USA),
            _ => Err(Error::ParseUnitsError),
        }
    }
}

impl UnitSystem {
    pub fn render_mass(&self, inp: si::Kilogram<f64>) -> String {
        match self {
            UnitSystem::SI => format!("{}", Kilograms::new(inp)),
            UnitSystem::USA => format!("{}", Pounds::new(inp)),
        }
    }

    pub fn parse_mass(&self, inp: &str) -> Result<si::Kilogram<f64>, Error> {
        match self {
            UnitSystem::SI => Kilograms::try_from(inp).map(|v| v.extract()),
            UnitSystem::USA => Pounds::try_from(inp).map(|v| v.extract()),
        }
    }

    pub fn render_distance(&self, inp: si::Meter<f64>) -> String {
        match self {
            UnitSystem::SI => format!("{}", Kilometers::new(inp)),
            UnitSystem::USA => format!("{}", Miles::new(inp)),
        }
    }

    pub fn parse_distance(&self, inp: &str) -> Result<si::Meter<f64>, Error> {
        match self {
            UnitSystem::SI => Kilometers::try_from(inp).map(|v| v.extract()),
            UnitSystem::USA => Miles::try_from(inp).map(|v| v.extract()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Kilograms {
    val: si::Kilogram<f64>,
    with_units: bool,
}

impl Kilograms {
    pub fn new(val: si::Kilogram<f64>) -> Kilograms {
        Kilograms {
            val,
            with_units: false,
        }
    }

    /*
    pub fn new_with_units(val: si::Kilogram<f64>) -> Kilograms {
        Kilograms {
            val,
            with_units: true,
        }
    }
    */

    pub fn extract(self) -> si::Kilogram<f64> {
        self.val
    }
}

impl fmt::Display for Kilograms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.with_units {
            write!(f, "{:.2} kg", self.val.value_unsafe)
        } else {
            write!(f, "{:.2}", self.val.value_unsafe)
        }
    }
}

impl TryFrom<&str> for Kilograms {
    type Error = Error;

    fn try_from(inp: &str) -> Result<Self, Self::Error> {
        inp.parse::<f64>()
            .map(|v| Kilograms::new(v * si::KG))
            .map_err(|_| Error::ParseDistanceError)
    }
}

impl FromStr for Kilograms {
    type Err = Error;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        Self::try_from(inp)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pounds {
    val: si::Kilogram<f64>,
    with_units: bool,
}

impl Pounds {
    pub fn new(val: si::Kilogram<f64>) -> Pounds {
        Pounds {
            val,
            with_units: false,
        }
    }

    /*
    pub fn new_with_units(val: si::Kilogram<f64>) -> Pounds {
        Pounds {
            val,
            with_units: true,
        }
    }
    */

    pub fn extract(self) -> si::Kilogram<f64> {
        self.val
    }
}

impl fmt::Display for Pounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.with_units {
            write!(f, "{:.2} lbs", (self.val / si::LB).value_unsafe)
        } else {
            write!(f, "{:.2}", (self.val / si::LB).value_unsafe)
        }
    }
}

impl TryFrom<&str> for Pounds {
    type Error = Error;

    fn try_from(inp: &str) -> Result<Self, Self::Error> {
        inp.parse::<f64>()
            .map(|v| Pounds::new(v * si::LB))
            .map_err(|_| Error::ParseDistanceError)
    }
}

impl FromStr for Pounds {
    type Err = Error;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        Self::try_from(inp)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Kilometers {
    val: si::Meter<f64>,
    with_units: bool,
}

impl Kilometers {
    pub fn new(val: si::Meter<f64>) -> Kilometers {
        Kilometers {
            val,
            with_units: false,
        }
    }

    /*
    pub fn new_with_units(val: si::Meter<f64>) -> Kilometers {
        Kilometers {
            val,
            with_units: true,
        }
    }
    */

    pub fn extract(self) -> si::Meter<f64> {
        self.val
    }
}

impl fmt::Display for Kilometers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.with_units {
            write!(f, "{:.2} km", self.val.value_unsafe / 1000.0)
        } else {
            write!(f, "{:.2}", self.val.value_unsafe / 1000.0)
        }
    }
}

impl TryFrom<&str> for Kilometers {
    type Error = Error;

    fn try_from(inp: &str) -> Result<Self, Self::Error> {
        inp.parse::<f64>()
            .map(|v| Kilometers::new(v * 1000.0 * si::M))
            .map_err(|_| Error::ParseDistanceError)
    }
}

impl FromStr for Kilometers {
    type Err = Error;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        Self::try_from(inp)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Miles {
    val: si::Meter<f64>,
    with_units: bool,
}

impl Miles {
    pub fn new(val: si::Meter<f64>) -> Miles {
        Miles {
            val,
            with_units: false,
        }
    }

    /*
    pub fn new_with_units(val: si::Meter<f64>) -> Miles {
        Miles {
            val,
            with_units: true,
        }
    }
    */

    pub fn extract(self) -> si::Meter<f64> {
        self.val
    }
}

impl fmt::Display for Miles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.with_units {
            write!(f, "{:.2} mi", (self.val / si::MI).value_unsafe)
        } else {
            write!(f, "{:.2}", (self.val / si::MI).value_unsafe)
        }
    }
}

impl TryFrom<&str> for Miles {
    type Error = Error;

    fn try_from(inp: &str) -> Result<Self, Self::Error> {
        inp.parse::<f64>()
            .map(|v| Miles::new(v * si::MI))
            .map_err(|_| Error::ParseDistanceError)
    }
}

impl FromStr for Miles {
    type Err = Error;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        Self::try_from(inp)
    }
}

#[cfg(test)]
mod test {
    use dimensioned::si;
    use std::convert::TryFrom;

    use super::{Kilograms, Kilometers, Miles, Pounds};
    use crate::errors::Error;
    use crate::i18n::UnitSystem;

    #[test]
    fn it_displays_mass_in_kg() {
        let t = Kilograms::new(20. * si::KG);
        assert_eq!(format!("{}", t), "20.00");
    }

    /*
    #[test]
    fn it_displays_mass_in_kg_with_units() {
        let t = Kilograms::new_with_units(20. * si::KG);
        assert_eq!(format!("{}", t), "20.00 kg");
    }
    */

    #[test]
    fn it_parses_kilograms() {
        let t: Kilograms = "20.00".parse().expect("successful parse");
        assert_eq!(t, Kilograms::new(20. * si::KG));
    }

    #[test]
    fn it_displays_mass_in_lbs() {
        let t = Pounds::new(20. * si::KG);
        assert_eq!(format!("{}", t), "44.09");
    }

    /*
    #[test]
    fn it_displays_mass_in_lbs_with_units() {
        let t = Pounds::new_with_units(20. * si::KG);
        assert_eq!(format!("{}", t), "44.09 lbs");
    }
    */

    #[test]
    fn it_parses_pounds() {
        let t: Pounds = "20.00".parse().expect("successful parse");
        assert_eq!(t, Pounds::new(20. * si::LB));
    }

    #[test]
    fn it_displays_distance_in_km() {
        let t = Kilometers::new(20000. * si::M);
        assert_eq!(format!("{}", t), "20.00");
    }

    /*
    #[test]
    fn it_displays_distance_in_km_with_units() {
        let t = Kilometers::new_with_units(20000. * si::M);
        assert_eq!(format!("{}", t), "20.00 km");
    }
    */

    #[test]
    fn it_parses_kilometers() {
        let t: Kilometers = "20.00".parse().expect("successful parse");
        assert_eq!(t, Kilometers::new(20000. * si::M));
    }

    #[test]
    fn it_displays_distance_in_miles() {
        let t = Miles::new(20000. * si::M);
        assert_eq!(format!("{}", t), "12.43");
    }

    /*
    #[test]
    fn it_displays_distance_in_miles_with_units() {
        let t = Miles::new_with_units(20000. * si::M);
        assert_eq!(format!("{}", t), "12.43 mi");
    }
    */

    #[test]
    fn it_parses_miles() {
        let t: Miles = "20.00".parse().expect("successful parse");
        assert_eq!(t, Miles::new(20. * si::MI));
    }
}
