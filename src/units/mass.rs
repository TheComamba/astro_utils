use crate::Float;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub(crate) const KILOGRAMS_PER_MOON_MASS: Float = 7.342e22;
const MOON_MASSES_PER_KILOGRAM: Float = 1. / KILOGRAMS_PER_MOON_MASS;
pub(crate) const KILOGRAMS_PER_EARTH_MASS: Float = 5.972e24;
const EARTH_MASSES_PER_KILOGRAM: Float = 1. / KILOGRAMS_PER_EARTH_MASS;
pub(crate) const KILOGRAMS_PER_JUPITER_MASS: Float = 1.898e27;
const JUPITER_MASSES_PER_KILOGRAM: Float = 1. / KILOGRAMS_PER_JUPITER_MASS;
pub(crate) const KILOGRAMS_PER_SOLAR_MASS: Float = 1.989e30;
const SOLAR_MASSES_PER_KILOGRAM: Float = 1. / KILOGRAMS_PER_SOLAR_MASS;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Mass {
    pub(super) kilograms: Float,
}

impl Mass {
    pub const ZERO: Mass = Mass { kilograms: 0. };

    pub const fn from_kilograms(kilograms: Float) -> Mass {
        Mass { kilograms }
    }

    pub fn from_moon_masses(moon_masses: Float) -> Mass {
        Mass {
            kilograms: moon_masses * KILOGRAMS_PER_MOON_MASS,
        }
    }

    pub fn from_earth_masses(earth_masses: Float) -> Mass {
        Mass {
            kilograms: earth_masses * KILOGRAMS_PER_EARTH_MASS,
        }
    }

    pub fn from_jupiter_masses(jupiter_masses: Float) -> Mass {
        Mass {
            kilograms: jupiter_masses * KILOGRAMS_PER_JUPITER_MASS,
        }
    }

    pub fn from_solar_masses(solar_masses: Float) -> Mass {
        Mass {
            kilograms: solar_masses * KILOGRAMS_PER_SOLAR_MASS,
        }
    }

    pub const fn as_kilograms(&self) -> Float {
        self.kilograms
    }

    pub fn as_moon_masses(&self) -> Float {
        self.kilograms * MOON_MASSES_PER_KILOGRAM
    }

    pub fn as_earth_masses(&self) -> Float {
        self.kilograms * EARTH_MASSES_PER_KILOGRAM
    }

    pub fn as_jupiter_masses(&self) -> Float {
        self.kilograms * JUPITER_MASSES_PER_KILOGRAM
    }

    pub fn as_solar_masses(&self) -> Float {
        self.kilograms * SOLAR_MASSES_PER_KILOGRAM
    }

    #[cfg(test)]
    pub(crate) fn eq_within(&self, other: Mass, accuracy: Mass) -> bool {
        let diff = self.kilograms - other.kilograms;
        diff.abs() <= accuracy.kilograms
    }
}

impl Display for Mass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.kilograms.abs() > 0.099 * KILOGRAMS_PER_SOLAR_MASS {
            write!(f, "{:.2} M☉", self.as_solar_masses())
        } else if self.kilograms.abs() > 0.0099 * KILOGRAMS_PER_EARTH_MASS {
            write!(f, "{:.2} M🜨", self.as_earth_masses())
        } else if self.kilograms.abs() > 0.99e-5 * KILOGRAMS_PER_EARTH_MASS {
            write!(f, "{:.5} M🜨", self.as_earth_masses())
        } else {
            write!(f, "{:.2} kg", self.kilograms)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{TEST_ACCURACY, TEST_MASS_ACCURACY};

    use super::*;

    #[test]
    fn test_kilogram() {
        let mass = Mass::from_kilograms(1.);
        assert!((mass.as_kilograms() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_moon_masses() {
        let expected = Mass::from_kilograms(KILOGRAMS_PER_MOON_MASS);
        let mass = Mass::from_moon_masses(1.);
        assert!(mass.eq_within(expected, TEST_MASS_ACCURACY));
        assert!((mass.as_moon_masses() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_earth_masses() {
        let expected = Mass::from_kilograms(KILOGRAMS_PER_EARTH_MASS);
        let mass = Mass::from_earth_masses(1.);
        assert!(mass.eq_within(expected, TEST_MASS_ACCURACY));
        assert!((mass.as_earth_masses() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_jupiter_masses() {
        let expected = Mass::from_kilograms(KILOGRAMS_PER_JUPITER_MASS);
        let mass = Mass::from_jupiter_masses(1.);
        assert!(mass.eq_within(expected, TEST_MASS_ACCURACY));
        assert!((mass.as_jupiter_masses() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_solar_masses() {
        let expected = Mass::from_kilograms(KILOGRAMS_PER_SOLAR_MASS);
        let mass = Mass::from_solar_masses(1.);
        assert!(mass.eq_within(expected, TEST_MASS_ACCURACY));
        assert!((mass.as_solar_masses() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_addition() {
        let mass1 = Mass::from_kilograms(1.);
        let mass2 = Mass::from_kilograms(2.);
        let expected = Mass::from_kilograms(3.);
        assert!((mass1 + mass2).eq_within(expected, TEST_MASS_ACCURACY));
    }

    #[test]
    fn test_subtraction() {
        let mass1 = Mass::from_kilograms(1.);
        let mass2 = Mass::from_kilograms(2.);
        let expected = Mass::from_kilograms(-1.);
        assert!((mass1 - mass2).eq_within(expected, TEST_MASS_ACCURACY));
    }

    #[test]
    fn test_display() {
        let mass = Mass::from_kilograms(1.23);
        assert_eq!(mass.to_string(), "1.23 kg");
        let mass = Mass::from_earth_masses(1.23);
        assert_eq!(mass.to_string(), "1.23 M🜨");
        let mass = Mass::from_solar_masses(1.23);
        assert_eq!(mass.to_string(), "1.23 M☉");
    }

    #[test]
    fn test_negative_display() {
        let mass = Mass::from_kilograms(-1.23);
        assert_eq!(mass.to_string(), "-1.23 kg");
        let mass = Mass::from_earth_masses(-1.23);
        assert_eq!(mass.to_string(), "-1.23 M🜨");
        let mass = Mass::from_solar_masses(-1.23);
        assert_eq!(mass.to_string(), "-1.23 M☉");
    }

    #[test]
    fn test_display_thresholds() {
        let mass = Mass::from_kilograms(1.);
        assert_eq!(mass.to_string(), "1.00 kg");
        let mass = Mass::from_earth_masses(1e-5);
        assert_eq!(mass.to_string(), "0.00001 M🜨");
        let mass = Mass::from_earth_masses(0.01);
        assert_eq!(mass.to_string(), "0.01 M🜨");
        let mass = Mass::from_solar_masses(0.1);
        assert_eq!(mass.to_string(), "0.10 M☉");
    }
}
