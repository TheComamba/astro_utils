use crate::Float;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

const METERS_PER_NANOMETER: Float = 1e-9;
const NANOMETERS_PER_METER: Float = 1. / METERS_PER_NANOMETER;
const METERS_PER_KILOMETER: Float = 1000.;
const KILOMETERS_PER_METER: Float = 1. / METERS_PER_KILOMETER;
pub(crate) const METERS_PER_EARTH_RADIUS: Float = 6_371_000.;
const EARTH_RADII_PER_METER: Float = 1. / METERS_PER_EARTH_RADIUS;
pub(crate) const METERS_PER_JUPITER_RADIUS: Float = 69_911_000.;
const JUPITER_RADII_PER_METER: Float = 1. / METERS_PER_JUPITER_RADIUS;
pub(crate) const METERS_PER_SUN_RADIUS: Float = 695_700_000.;
const SUN_RADII_PER_METER: Float = 1. / METERS_PER_SUN_RADIUS;
const METERS_PER_ASTRONOMICAL_UNIT: Float = 149_597_870_700.;
const ASTRONOMICAL_UNITS_PER_METER: Float = 1. / METERS_PER_ASTRONOMICAL_UNIT;
pub(crate) const METERS_PER_LIGHT_YEAR: Float = 9_460_730_472_580_800.;
const LIGHT_YEARS_PER_METER: Float = 1. / METERS_PER_LIGHT_YEAR;
pub(crate) const METERS_PER_PARSEC: Float = 30_856_775_814_671_900.;
const PARSECS_PER_METER: Float = 1. / METERS_PER_PARSEC;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Length {
    pub(super) meters: Float,
}

impl Length {
    pub const ZERO: Length = Length { meters: 0. };

    pub fn from_nanometers(nanometers: Float) -> Length {
        Length {
            meters: nanometers * METERS_PER_NANOMETER,
        }
    }

    pub const fn from_meters(meters: Float) -> Length {
        Length { meters }
    }

    pub fn from_kilometers(kilometers: Float) -> Length {
        Length {
            meters: kilometers * METERS_PER_KILOMETER,
        }
    }

    pub fn from_earth_radii(earth_radii: Float) -> Length {
        Length {
            meters: earth_radii * METERS_PER_EARTH_RADIUS,
        }
    }

    pub fn from_jupiter_radii(jupiter_radii: Float) -> Length {
        Length {
            meters: jupiter_radii * METERS_PER_JUPITER_RADIUS,
        }
    }

    pub fn from_sun_radii(sun_radii: Float) -> Length {
        Length {
            meters: sun_radii * METERS_PER_SUN_RADIUS,
        }
    }

    pub fn from_astronomical_units(astronomical_units: Float) -> Length {
        Length {
            meters: astronomical_units * METERS_PER_ASTRONOMICAL_UNIT,
        }
    }

    pub fn from_light_years(light_years: Float) -> Length {
        Length {
            meters: light_years * METERS_PER_LIGHT_YEAR,
        }
    }

    pub fn from_parsecs(parsecs: Float) -> Length {
        Length {
            meters: parsecs * METERS_PER_PARSEC,
        }
    }

    pub fn as_nanometers(&self) -> Float {
        self.meters * NANOMETERS_PER_METER
    }

    pub const fn as_meters(&self) -> Float {
        self.meters
    }

    pub fn as_kilometers(&self) -> Float {
        self.meters * KILOMETERS_PER_METER
    }

    pub fn as_earth_radii(&self) -> Float {
        self.meters * EARTH_RADII_PER_METER
    }

    pub fn as_jupiter_radii(&self) -> Float {
        self.meters * JUPITER_RADII_PER_METER
    }

    pub fn as_sun_radii(&self) -> Float {
        self.meters * SUN_RADII_PER_METER
    }

    pub fn as_astronomical_units(&self) -> Float {
        self.meters * ASTRONOMICAL_UNITS_PER_METER
    }

    pub fn as_light_years(&self) -> Float {
        self.meters * LIGHT_YEARS_PER_METER
    }

    pub fn as_parsecs(&self) -> Float {
        self.meters * PARSECS_PER_METER
    }

    pub fn eq_within(&self, other: &Length, accuracy: Length) -> bool {
        let diff = self.meters - other.meters;
        diff.abs() <= accuracy.meters
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.meters > 0.0099 * METERS_PER_LIGHT_YEAR {
            write!(f, "{:.2} ly", self.as_light_years())
        } else if self.meters > 0.0099 * METERS_PER_ASTRONOMICAL_UNIT {
            write!(f, "{:.2} AU", self.as_astronomical_units())
        } else if self.meters > 0.099 * METERS_PER_SUN_RADIUS {
            write!(f, "{:.2} R☉", self.as_sun_radii())
        } else if self.meters > 0.99 * METERS_PER_EARTH_RADIUS {
            write!(f, "{:.2} R🜨", self.as_earth_radii())
        } else if self.meters > 0.99 * METERS_PER_KILOMETER {
            write!(f, "{:.2} km", self.as_kilometers())
        } else if self.meters > 1001. * METERS_PER_NANOMETER {
            write!(f, "{:.2} m", self.as_meters())
        } else {
            write!(f, "{:.2} nm", self.as_nanometers())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{TEST_ACCURACY, TEST_LENGTH_ACCURACY};

    use super::*;

    #[test]
    fn test_meters() {
        let length = Length::from_meters(1.);
        assert!((length.as_meters() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_kilometer() {
        let expected = Length::from_meters(METERS_PER_KILOMETER);
        let length = Length::from_kilometers(1.);
        assert!(length.eq_within(&expected, TEST_LENGTH_ACCURACY));
        assert!((length.as_kilometers() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_earth_radii() {
        let expected = Length::from_meters(METERS_PER_EARTH_RADIUS);
        let length = Length::from_earth_radii(1.);
        assert!(length.eq_within(&expected, TEST_LENGTH_ACCURACY));
        assert!((length.as_earth_radii() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_jupiter_radii() {
        let expected = Length::from_meters(METERS_PER_JUPITER_RADIUS);
        let length = Length::from_jupiter_radii(1.);
        assert!(length.eq_within(&expected, TEST_LENGTH_ACCURACY));
        assert!((length.as_jupiter_radii() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_sun_radii() {
        let expected = Length::from_meters(METERS_PER_SUN_RADIUS);
        let length = Length::from_sun_radii(1.);
        assert!(length.eq_within(&expected, TEST_LENGTH_ACCURACY));
        assert!((length.as_sun_radii() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_astronomical_units() {
        let expected = Length::from_meters(METERS_PER_ASTRONOMICAL_UNIT);
        let length = Length::from_astronomical_units(1.);
        assert!(length.eq_within(&expected, TEST_LENGTH_ACCURACY));
        assert!((length.as_astronomical_units() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_light_years() {
        let expected = Length::from_meters(METERS_PER_LIGHT_YEAR);
        let length = Length::from_light_years(1.);
        assert!(length.eq_within(&expected, TEST_LENGTH_ACCURACY));
        assert!((length.as_light_years() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_parsecs() {
        let expected = Length::from_meters(METERS_PER_PARSEC);
        let length = Length::from_parsecs(1.);
        assert!(length.eq_within(&expected, TEST_LENGTH_ACCURACY));
        assert!((length.as_parsecs() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_add() {
        let expected = Length::from_meters(2. * METERS_PER_KILOMETER);
        let length = Length::from_kilometers(1.);
        assert!((length + length).eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_sub() {
        let expected = Length::from_meters(0.);
        let length = Length::from_kilometers(1.);
        assert!((length - length).eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_display() {
        let nm = Length::from_nanometers(1.23);
        assert_eq!(format!("{}", nm), "1.23 nm");
        let m = Length::from_meters(1.23);
        assert_eq!(format!("{}", m), "1.23 m");
        let km = Length::from_kilometers(1.23);
        assert_eq!(format!("{}", km), "1.23 km");
        let earth_radii = Length::from_earth_radii(1.23);
        assert_eq!(format!("{}", earth_radii), "1.23 R🜨");
        let sun_radii = Length::from_sun_radii(1.23);
        assert_eq!(format!("{}", sun_radii), "1.23 R☉");
        let astronomical_units = Length::from_astronomical_units(1.23);
        assert_eq!(format!("{}", astronomical_units), "1.23 AU");
        let light_years = Length::from_light_years(1.23);
        assert_eq!(format!("{}", light_years), "1.23 ly");
    }

    #[test]
    fn test_display_thresholds() {
        let nm = Length::from_nanometers(1000.);
        assert_eq!(format!("{}", nm), "1000.00 nm");
        let m = Length::from_meters(1.);
        assert_eq!(format!("{}", m), "1.00 m");
        let km = Length::from_kilometers(1.);
        assert_eq!(format!("{}", km), "1.00 km");
        let earth_radii = Length::from_earth_radii(1.);
        assert_eq!(format!("{}", earth_radii), "1.00 R🜨");
        let sun_radii = Length::from_sun_radii(0.1);
        assert_eq!(format!("{}", sun_radii), "0.10 R☉");
        let astronomical_units = Length::from_astronomical_units(0.01);
        assert_eq!(format!("{}", astronomical_units), "0.01 AU");
        let light_years = Length::from_light_years(0.01);
        assert_eq!(format!("{}", light_years), "0.01 ly");
    }
}
