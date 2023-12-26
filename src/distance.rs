use crate::Float;
use std::{
    fmt::Display,
    ops::{Add, Sub},
};

pub static METERS_PER_KILOMETER: Float = 1000.0;
pub static METERS_PER_EARTH_RADIUS: Float = 6_371_000.0;
pub static METERS_PER_JUPITER_RADIUS: Float = 69_911_000.0;
pub static METERS_PER_SUN_RADIUS: Float = 695_700_000.0;
pub static METERS_PER_ASTRONOMICAL_UNIT: Float = 149_597_870_700.0;
pub static METERS_PER_LIGHT_YEAR: Float = 9_460_730_472_580_800.0;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Distance {
    meters: Float,
}

impl Distance {
    pub fn from_meters(meters: Float) -> Distance {
        Distance { meters }
    }

    pub fn from_kilometers(kilometers: Float) -> Distance {
        Distance {
            meters: kilometers * METERS_PER_KILOMETER,
        }
    }

    pub fn from_earth_radii(earth_radii: Float) -> Distance {
        Distance {
            meters: earth_radii * METERS_PER_EARTH_RADIUS,
        }
    }

    pub fn from_jupiter_radii(jupiter_radii: Float) -> Distance {
        Distance {
            meters: jupiter_radii * METERS_PER_JUPITER_RADIUS,
        }
    }

    pub fn from_sun_radii(sun_radii: Float) -> Distance {
        Distance {
            meters: sun_radii * METERS_PER_SUN_RADIUS,
        }
    }

    pub fn from_astronomical_units(astronomical_units: Float) -> Distance {
        Distance {
            meters: astronomical_units * METERS_PER_ASTRONOMICAL_UNIT,
        }
    }

    pub fn from_light_years(light_years: Float) -> Distance {
        Distance {
            meters: light_years * METERS_PER_LIGHT_YEAR,
        }
    }

    pub fn as_meters(&self) -> Float {
        self.meters
    }

    pub fn as_kilometers(&self) -> Float {
        self.meters / METERS_PER_KILOMETER
    }

    pub fn as_earth_radii(&self) -> Float {
        self.meters / METERS_PER_EARTH_RADIUS
    }

    pub fn as_jupiter_radii(&self) -> Float {
        self.meters / METERS_PER_JUPITER_RADIUS
    }

    pub fn as_sun_radii(&self) -> Float {
        self.meters / METERS_PER_SUN_RADIUS
    }

    pub fn as_astronomical_units(&self) -> Float {
        self.meters / METERS_PER_ASTRONOMICAL_UNIT
    }

    pub fn as_light_years(&self) -> Float {
        self.meters / METERS_PER_LIGHT_YEAR
    }

    pub fn eq_within(&self, other: Distance, relative_accuracy: Float) -> bool {
        let max = if self.meters > other.meters {
            self.meters.abs()
        } else {
            other.meters.abs()
        };
        (self.meters - other.meters).abs() <= relative_accuracy * max
    }
}

impl Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.meters >= METERS_PER_LIGHT_YEAR {
            write!(f, "{} light years", self.as_light_years())
        } else if self.meters >= METERS_PER_ASTRONOMICAL_UNIT {
            write!(f, "{} AU", self.as_astronomical_units())
        } else if self.meters >= METERS_PER_SUN_RADIUS {
            write!(f, "{} Sun radii", self.as_sun_radii())
        } else if self.meters >= METERS_PER_JUPITER_RADIUS {
            write!(f, "{} Jupiter radii", self.as_jupiter_radii())
        } else if self.meters >= METERS_PER_EARTH_RADIUS {
            write!(f, "{} Earth radii", self.as_earth_radii())
        } else if self.meters >= METERS_PER_KILOMETER {
            write!(f, "{} km", self.as_kilometers())
        } else {
            write!(f, "{} m", self.as_meters())
        }
    }
}

impl Add for Distance {
    type Output = Distance;

    fn add(self, other: Distance) -> Distance {
        Distance {
            meters: self.meters + other.meters,
        }
    }
}

impl Sub for Distance {
    type Output = Distance;

    fn sub(self, other: Distance) -> Distance {
        Distance {
            meters: self.meters - other.meters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{distance::METERS_PER_KILOMETER, Float};

    const TEST_ACCURACY: Float = 1e-5;

    #[test]
    fn test_meters() {
        let distance = Distance::from_meters(1.0);
        assert!((distance.as_meters() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_kilometer() {
        let expected = Distance::from_meters(METERS_PER_KILOMETER);
        let distance = Distance::from_kilometers(1.0);
        assert!(distance.eq_within(expected, TEST_ACCURACY));
        assert!((distance.as_kilometers() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_earth_radii() {
        let expected = Distance::from_meters(METERS_PER_EARTH_RADIUS);
        let distance = Distance::from_earth_radii(1.0);
        assert!(distance.eq_within(expected, TEST_ACCURACY));
        assert!((distance.as_earth_radii() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_jupiter_radii() {
        let expected = Distance::from_meters(METERS_PER_JUPITER_RADIUS);
        let distance = Distance::from_jupiter_radii(1.0);
        assert!(distance.eq_within(expected, TEST_ACCURACY));
        assert!((distance.as_jupiter_radii() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_sun_radii() {
        let expected = Distance::from_meters(METERS_PER_SUN_RADIUS);
        let distance = Distance::from_sun_radii(1.0);
        assert!(distance.eq_within(expected, TEST_ACCURACY));
        assert!((distance.as_sun_radii() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_astronomical_units() {
        let expected = Distance::from_meters(METERS_PER_ASTRONOMICAL_UNIT);
        let distance = Distance::from_astronomical_units(1.0);
        assert!(distance.eq_within(expected, TEST_ACCURACY));
        assert!((distance.as_astronomical_units() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_light_years() {
        let expected = Distance::from_meters(METERS_PER_LIGHT_YEAR);
        let distance = Distance::from_light_years(1.0);
        assert!(distance.eq_within(expected, TEST_ACCURACY));
        assert!((distance.as_light_years() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_add() {
        let expected = Distance::from_meters(2.0 * METERS_PER_KILOMETER);
        let distance = Distance::from_kilometers(1.0);
        assert!(distance.add(distance).eq_within(expected, TEST_ACCURACY));
    }

    #[test]
    fn test_sub() {
        let expected = Distance::from_meters(0.0);
        let distance = Distance::from_kilometers(1.0);
        assert!(distance.sub(distance).eq_within(expected, TEST_ACCURACY));
    }

    #[test]
    fn test_display() {
        use crate::distance::Distance;
        let km = Distance::from_kilometers(1.23);
        assert_eq!(format!("{}", km), "1.23 km");
        let earth_radii = Distance::from_earth_radii(1.23);
        assert_eq!(format!("{}", earth_radii), "1.23 Earth radii");
        let jupiter_radii = Distance::from_jupiter_radii(1.23);
        assert_eq!(format!("{}", jupiter_radii), "1.23 Jupiter radii");
        let sun_radii = Distance::from_sun_radii(1.23);
        assert_eq!(format!("{}", sun_radii), "1.23 Sun radii");
        let astronomical_units = Distance::from_astronomical_units(1.23);
        assert_eq!(format!("{}", astronomical_units), "1.23 AU");
        let light_years = Distance::from_light_years(1.23);
        assert_eq!(format!("{}", light_years), "1.23 light years");
    }
}
