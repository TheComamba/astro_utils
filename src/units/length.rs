use crate::Float;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

const METERS_PER_NANOMETERS: Float = 1e-9;
const METERS_PER_CENTIMETERS: Float = 0.01;
const METERS_PER_KILOMETERS: Float = 1000.;
const METERS_PER_EARTH_RADII: Float = 6_371_000.;
const METERS_PER_JUPITER_RADIUS: Float = 69_911_000.;
const METERS_PER_SUN_RADII: Float = 695_700_000.;
const METERS_PER_AU: Float = 149_597_870_700.;
const METERS_PER_LIGHT_YEAR: Float = 9_460_730_472_580_800.;
const METERS_PER_PARSEC: Float = 30_856_775_814_671_900.;
pub(crate) const AU_PER_NANOMETERS: Float = AU_PER_METER * METERS_PER_NANOMETERS;
pub(crate) const AU_PER_CENTIMETERS: Float = AU_PER_METER * METERS_PER_CENTIMETERS;
pub(crate) const AU_PER_METER: Float = 1. / METERS_PER_AU;
pub(crate) const AU_PER_KILOMETERS: Float = AU_PER_METER * METERS_PER_KILOMETERS;
pub(crate) const AU_PER_EARTH_RADII: Float = AU_PER_METER * METERS_PER_EARTH_RADII;
pub(crate) const AU_PER_JUPITER_RADII: Float = AU_PER_METER * METERS_PER_JUPITER_RADIUS;
pub(crate) const AU_PER_SUN_RADII: Float = AU_PER_METER * METERS_PER_SUN_RADII;
pub(crate) const AU_PER_LIGHT_YEARS: Float = AU_PER_METER * METERS_PER_LIGHT_YEAR;
pub(crate) const AU_PER_PARSECS: Float = AU_PER_METER * METERS_PER_PARSEC;
const NANOMETERS_PER_AU: Float = 1. / AU_PER_NANOMETERS;
const CENTIMETERS_PER_AU: Float = 1. / AU_PER_CENTIMETERS;
const KILOMETERS_PER_AU: Float = 1. / AU_PER_KILOMETERS;
const EARTH_RADII_PER_AU: Float = 1. / AU_PER_EARTH_RADII;
const JUPITER_RADII_PER_AU: Float = 1. / AU_PER_JUPITER_RADII;
const SUN_RADII_PER_AU: Float = 1. / AU_PER_SUN_RADII;
const LIGHT_YEARS_PER_AU: Float = 1. / AU_PER_LIGHT_YEARS;
const PARSECS_PER_AU: Float = 1. / AU_PER_PARSECS;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Length {
    pub(crate) au: Float,
}

impl Length {
    pub const ZERO: Length = Length { au: 0. };

    pub fn from_nanometers(nanometers: Float) -> Length {
        Length {
            au: nanometers * AU_PER_NANOMETERS,
        }
    }

    pub fn from_centimeters(centimeters: Float) -> Length {
        Length {
            au: centimeters * AU_PER_CENTIMETERS,
        }
    }

    pub fn from_meters(meters: Float) -> Length {
        Length {
            au: meters * AU_PER_METER,
        }
    }

    pub fn from_kilometers(kilometers: Float) -> Length {
        Length {
            au: kilometers * AU_PER_KILOMETERS,
        }
    }

    pub fn from_earth_radii(earth_radii: Float) -> Length {
        Length {
            au: earth_radii * AU_PER_EARTH_RADII,
        }
    }

    pub fn from_jupiter_radii(jupiter_radii: Float) -> Length {
        Length {
            au: jupiter_radii * AU_PER_JUPITER_RADII,
        }
    }

    pub fn from_sun_radii(sun_radii: Float) -> Length {
        Length {
            au: sun_radii * AU_PER_SUN_RADII,
        }
    }

    pub const fn from_astronomical_units(astronomical_units: Float) -> Length {
        Length {
            au: astronomical_units,
        }
    }

    pub fn from_light_years(light_years: Float) -> Length {
        Length {
            au: light_years * AU_PER_LIGHT_YEARS,
        }
    }

    pub fn from_parsecs(parsecs: Float) -> Length {
        Length {
            au: parsecs * AU_PER_PARSECS,
        }
    }

    pub fn as_nanometers(&self) -> Float {
        self.au * NANOMETERS_PER_AU
    }

    pub fn as_centimeters(&self) -> Float {
        self.au * CENTIMETERS_PER_AU
    }

    pub fn as_meters(&self) -> Float {
        self.au * METERS_PER_AU
    }

    pub fn as_kilometers(&self) -> Float {
        self.au * KILOMETERS_PER_AU
    }

    pub fn as_earth_radii(&self) -> Float {
        self.au * EARTH_RADII_PER_AU
    }

    pub fn as_jupiter_radii(&self) -> Float {
        self.au * JUPITER_RADII_PER_AU
    }

    pub fn as_sun_radii(&self) -> Float {
        self.au * SUN_RADII_PER_AU
    }

    pub fn as_astronomical_units(&self) -> Float {
        self.au
    }

    pub fn as_light_years(&self) -> Float {
        self.au * LIGHT_YEARS_PER_AU
    }

    pub fn as_parsecs(&self) -> Float {
        self.au * PARSECS_PER_AU
    }

    #[cfg(test)]
    pub(crate) fn eq_within(&self, other: &Length, accuracy: Length) -> bool {
        let diff = self.au - other.au;
        diff.abs() <= accuracy.au
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.au.abs() > 0.0099 * AU_PER_LIGHT_YEARS {
            write!(f, "{:.2} ly", self.as_light_years())
        } else if self.au.abs() > 0.0099 {
            write!(f, "{:.2} AU", self.as_astronomical_units())
        } else if self.au.abs() > 0.099 * AU_PER_SUN_RADII {
            write!(f, "{:.2} R☉", self.as_sun_radii())
        } else if self.au.abs() > 0.99 * AU_PER_EARTH_RADII {
            write!(f, "{:.2} R🜨", self.as_earth_radii())
        } else if self.au.abs() > 0.99 * AU_PER_KILOMETERS {
            write!(f, "{:.2} km", self.as_kilometers())
        } else if self.au.abs() > 1001. * AU_PER_NANOMETERS {
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
    fn test_nanometers() {
        let expected = Length::from_meters(METERS_PER_NANOMETERS);
        let length = Length::from_nanometers(1.);
        assert!(length.eq_within(&expected, TEST_LENGTH_ACCURACY));
        assert!((length.as_nanometers() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_centimeters() {
        let expected = Length::from_meters(METERS_PER_CENTIMETERS);
        let length = Length::from_centimeters(1.);
        assert!(length.eq_within(&expected, TEST_LENGTH_ACCURACY));
        assert!((length.as_centimeters() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_meters() {
        let expected = Length::from_meters(1.);
        let length = Length::from_meters(1.);
        assert!(length.eq_within(&expected, TEST_LENGTH_ACCURACY));
        assert!((length.as_meters() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_kilometer() {
        let expected = Length::from_meters(METERS_PER_KILOMETERS);
        let length = Length::from_kilometers(1.);
        assert!(length.eq_within(&expected, TEST_LENGTH_ACCURACY));
        assert!((length.as_kilometers() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_earth_radii() {
        let expected = Length::from_meters(METERS_PER_EARTH_RADII);
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
        let expected = Length::from_meters(METERS_PER_SUN_RADII);
        let length = Length::from_sun_radii(1.);
        assert!(length.eq_within(&expected, TEST_LENGTH_ACCURACY));
        assert!((length.as_sun_radii() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_astronomical_units() {
        let expected = Length::from_meters(METERS_PER_AU);
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
        let expected = Length::from_meters(2. * METERS_PER_KILOMETERS);
        let length = Length::from_kilometers(1.);
        assert!((length + length).eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_sub() {
        let expected = Length::ZERO;
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
    fn test_negative_display() {
        let nm = Length::from_nanometers(-1.23);
        assert_eq!(format!("{}", nm), "-1.23 nm");
        let m = Length::from_meters(-1.23);
        assert_eq!(format!("{}", m), "-1.23 m");
        let km = Length::from_kilometers(-1.23);
        assert_eq!(format!("{}", km), "-1.23 km");
        let earth_radii = Length::from_earth_radii(-1.23);
        assert_eq!(format!("{}", earth_radii), "-1.23 R🜨");
        let sun_radii = Length::from_sun_radii(-1.23);
        assert_eq!(format!("{}", sun_radii), "-1.23 R☉");
        let astronomical_units = Length::from_astronomical_units(-1.23);
        assert_eq!(format!("{}", astronomical_units), "-1.23 AU");
        let light_years = Length::from_light_years(-1.23);
        assert_eq!(format!("{}", light_years), "-1.23 ly");
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
