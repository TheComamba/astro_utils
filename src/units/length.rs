use crate::Float;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

static METERS_PER_KILOMETER: Float = 1000.0;
static KILOMETERS_PER_METER: Float = 1.0 / METERS_PER_KILOMETER;
pub(crate) static METERS_PER_EARTH_RADIUS: Float = 6_371_000.0;
static EARTH_RADII_PER_METER: Float = 1.0 / METERS_PER_EARTH_RADIUS;
pub(crate) static METERS_PER_JUPITER_RADIUS: Float = 69_911_000.0;
static JUPITER_RADII_PER_METER: Float = 1.0 / METERS_PER_JUPITER_RADIUS;
pub(crate) static METERS_PER_SUN_RADIUS: Float = 695_700_000.0;
static SUN_RADII_PER_METER: Float = 1.0 / METERS_PER_SUN_RADIUS;
static METERS_PER_ASTRONOMICAL_UNIT: Float = 149_597_870_700.0;
static ASTRONOMICAL_UNITS_PER_METER: Float = 1.0 / METERS_PER_ASTRONOMICAL_UNIT;
static METERS_PER_LIGHT_YEAR: Float = 9_460_730_472_580_800.0;
static LIGHT_YEARS_PER_METER: Float = 1.0 / METERS_PER_LIGHT_YEAR;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Length {
    meters: Float,
}

impl Length {
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

    pub fn eq_within(&self, other: Length, relative_accuracy: Float) -> bool {
        let max = if self.meters > other.meters {
            self.meters.abs()
        } else {
            other.meters.abs()
        };
        (self.meters - other.meters).abs() <= relative_accuracy * max
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.meters > 0.99 * METERS_PER_LIGHT_YEAR {
            write!(f, "{:.2} light years", self.as_light_years())
        } else if self.meters > 0.99 * METERS_PER_ASTRONOMICAL_UNIT {
            write!(f, "{:.2} AU", self.as_astronomical_units())
        } else if self.meters > 0.99 * METERS_PER_SUN_RADIUS {
            write!(f, "{:.2} Sun radii", self.as_sun_radii())
        } else if self.meters > 0.99 * METERS_PER_JUPITER_RADIUS {
            write!(f, "{:.2} Jupiter radii", self.as_jupiter_radii())
        } else if self.meters > 0.99 * METERS_PER_EARTH_RADIUS {
            write!(f, "{:.2} Earth radii", self.as_earth_radii())
        } else if self.meters > 0.99 * METERS_PER_KILOMETER {
            write!(f, "{:.2} km", self.as_kilometers())
        } else {
            write!(f, "{:.2} m", self.as_meters())
        }
    }
}

impl Add for Length {
    type Output = Length;

    fn add(self, other: Length) -> Length {
        Length {
            meters: self.meters + other.meters,
        }
    }
}

impl Sub for Length {
    type Output = Length;

    fn sub(self, other: Length) -> Length {
        Length {
            meters: self.meters - other.meters,
        }
    }
}

impl Mul<Float> for Length {
    type Output = Length;

    fn mul(self, f: Float) -> Length {
        Length {
            meters: self.meters * f,
        }
    }
}

impl Mul<Length> for Float {
    type Output = Length;

    fn mul(self, length: Length) -> Length {
        Length {
            meters: self * length.meters,
        }
    }
}

impl Div<Float> for Length {
    type Output = Length;

    fn div(self, f: Float) -> Length {
        Length {
            meters: self.meters / f,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ACCURACY: Float = 1e-5;

    #[test]
    fn test_meters() {
        let length = Length::from_meters(1.0);
        assert!((length.as_meters() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_kilometer() {
        let expected = Length::from_meters(METERS_PER_KILOMETER);
        let length = Length::from_kilometers(1.0);
        assert!(length.eq_within(expected, TEST_ACCURACY));
        assert!((length.as_kilometers() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_earth_radii() {
        let expected = Length::from_meters(METERS_PER_EARTH_RADIUS);
        let length = Length::from_earth_radii(1.0);
        assert!(length.eq_within(expected, TEST_ACCURACY));
        assert!((length.as_earth_radii() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_jupiter_radii() {
        let expected = Length::from_meters(METERS_PER_JUPITER_RADIUS);
        let length = Length::from_jupiter_radii(1.0);
        assert!(length.eq_within(expected, TEST_ACCURACY));
        assert!((length.as_jupiter_radii() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_sun_radii() {
        let expected = Length::from_meters(METERS_PER_SUN_RADIUS);
        let length = Length::from_sun_radii(1.0);
        assert!(length.eq_within(expected, TEST_ACCURACY));
        assert!((length.as_sun_radii() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_astronomical_units() {
        let expected = Length::from_meters(METERS_PER_ASTRONOMICAL_UNIT);
        let length = Length::from_astronomical_units(1.0);
        assert!(length.eq_within(expected, TEST_ACCURACY));
        assert!((length.as_astronomical_units() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_light_years() {
        let expected = Length::from_meters(METERS_PER_LIGHT_YEAR);
        let length = Length::from_light_years(1.0);
        assert!(length.eq_within(expected, TEST_ACCURACY));
        assert!((length.as_light_years() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_add() {
        let expected = Length::from_meters(2.0 * METERS_PER_KILOMETER);
        let length = Length::from_kilometers(1.0);
        assert!(length.add(length).eq_within(expected, TEST_ACCURACY));
    }

    #[test]
    fn test_sub() {
        let expected = Length::from_meters(0.0);
        let length = Length::from_kilometers(1.0);
        assert!(length.sub(length).eq_within(expected, TEST_ACCURACY));
    }

    #[test]
    fn test_display() {
        let km = Length::from_kilometers(1.23);
        assert_eq!(format!("{}", km), "1.23 km");
        let earth_radii = Length::from_earth_radii(1.23);
        assert_eq!(format!("{}", earth_radii), "1.23 Earth radii");
        let jupiter_radii = Length::from_jupiter_radii(1.23);
        assert_eq!(format!("{}", jupiter_radii), "1.23 Jupiter radii");
        let sun_radii = Length::from_sun_radii(1.23);
        assert_eq!(format!("{}", sun_radii), "1.23 Sun radii");
        let astronomical_units = Length::from_astronomical_units(1.23);
        assert_eq!(format!("{}", astronomical_units), "1.23 AU");
        let light_years = Length::from_light_years(1.23);
        assert_eq!(format!("{}", light_years), "1.23 light years");
    }

    #[test]
    fn test_display_thresholds() {
        let km = Length::from_kilometers(1.0);
        assert_eq!(format!("{}", km), "1.00 km");
        let earth_radii = Length::from_earth_radii(1.0);
        assert_eq!(format!("{}", earth_radii), "1.00 Earth radii");
        let jupiter_radii = Length::from_jupiter_radii(1.0);
        assert_eq!(format!("{}", jupiter_radii), "1.00 Jupiter radii");
        let sun_radii = Length::from_sun_radii(1.0);
        assert_eq!(format!("{}", sun_radii), "1.00 Sun radii");
        let astronomical_units = Length::from_astronomical_units(1.0);
        assert_eq!(format!("{}", astronomical_units), "1.00 AU");
        let light_years = Length::from_light_years(1.0);
        assert_eq!(format!("{}", light_years), "1.00 light years");
    }
}
