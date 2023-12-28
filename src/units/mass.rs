use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use crate::Float;

pub(crate) static KILOGRAMS_PER_EARTH_MASS: Float = 5.972e24;
static EARTH_MASSES_PER_KILOGRAM: Float = 1.0 / KILOGRAMS_PER_EARTH_MASS;
pub(crate) static KILOGRAMS_PER_JUPITER_MASS: Float = 1.898e27;
static JUPITER_MASSES_PER_KILOGRAM: Float = 1.0 / KILOGRAMS_PER_JUPITER_MASS;
pub(crate) static KILOGRAMS_PER_SOLAR_MASS: Float = 1.989e30;
static SOLAR_MASSES_PER_KILOGRAM: Float = 1.0 / KILOGRAMS_PER_SOLAR_MASS;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Mass {
    kilograms: Float,
}

impl Mass {
    pub const fn from_kilograms(kilograms: Float) -> Mass {
        Mass { kilograms }
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

    pub fn as_earth_masses(&self) -> Float {
        self.kilograms * EARTH_MASSES_PER_KILOGRAM
    }

    pub fn as_jupiter_masses(&self) -> Float {
        self.kilograms * JUPITER_MASSES_PER_KILOGRAM
    }

    pub fn as_solar_masses(&self) -> Float {
        self.kilograms * SOLAR_MASSES_PER_KILOGRAM
    }

    pub fn eq_within(&self, other: Mass, relative_accuracy: Float) -> bool {
        let max = if self.kilograms > other.kilograms {
            self.kilograms.abs()
        } else {
            other.kilograms.abs()
        };
        (self.kilograms - other.kilograms).abs() <= relative_accuracy * max
    }
}

impl Display for Mass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.kilograms > 0.99 * KILOGRAMS_PER_SOLAR_MASS {
            write!(f, "{:.2} solar masses", self.as_solar_masses())
        } else if self.kilograms > 0.99 * KILOGRAMS_PER_JUPITER_MASS {
            write!(f, "{:.2} Jupiter masses", self.as_jupiter_masses())
        } else if self.kilograms > 0.99 * KILOGRAMS_PER_EARTH_MASS {
            write!(f, "{:.2} Earth masses", self.as_earth_masses())
        } else {
            write!(f, "{:.2} kg", self.kilograms)
        }
    }
}

impl Add for Mass {
    type Output = Mass;

    fn add(self, other: Mass) -> Mass {
        Mass {
            kilograms: self.kilograms + other.kilograms,
        }
    }
}

impl Sub for Mass {
    type Output = Mass;

    fn sub(self, other: Mass) -> Mass {
        Mass {
            kilograms: self.kilograms - other.kilograms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ACCURACY: Float = 1e-5;

    #[test]
    fn test_kilogram() {
        let mass = Mass::from_kilograms(1.0);
        assert!((mass.as_kilograms() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_earth_masses() {
        let expected = Mass::from_kilograms(KILOGRAMS_PER_EARTH_MASS);
        let mass = Mass::from_earth_masses(1.0);
        assert!(mass.eq_within(expected, TEST_ACCURACY));
        assert!((mass.as_earth_masses() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_jupiter_masses() {
        let expected = Mass::from_kilograms(KILOGRAMS_PER_JUPITER_MASS);
        let mass = Mass::from_jupiter_masses(1.0);
        assert!(mass.eq_within(expected, TEST_ACCURACY));
        assert!((mass.as_jupiter_masses() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_solar_masses() {
        let expected = Mass::from_kilograms(KILOGRAMS_PER_SOLAR_MASS);
        let mass = Mass::from_solar_masses(1.0);
        assert!(mass.eq_within(expected, TEST_ACCURACY));
        assert!((mass.as_solar_masses() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_addition() {
        let mass1 = Mass::from_kilograms(1.0);
        let mass2 = Mass::from_kilograms(2.0);
        let expected = Mass::from_kilograms(3.0);
        assert!((mass1 + mass2).eq_within(expected, TEST_ACCURACY));
    }

    #[test]
    fn test_subtraction() {
        let mass1 = Mass::from_kilograms(1.0);
        let mass2 = Mass::from_kilograms(2.0);
        let expected = Mass::from_kilograms(-1.0);
        assert!((mass1 - mass2).eq_within(expected, TEST_ACCURACY));
    }

    #[test]
    fn test_display() {
        let mass = Mass::from_kilograms(1.23);
        assert_eq!(mass.to_string(), "1.23 kg");
        let mass = Mass::from_earth_masses(1.23);
        assert_eq!(mass.to_string(), "1.23 Earth masses");
        let mass = Mass::from_jupiter_masses(1.23);
        assert_eq!(mass.to_string(), "1.23 Jupiter masses");
        let mass = Mass::from_solar_masses(1.23);
        assert_eq!(mass.to_string(), "1.23 solar masses");
    }

    #[test]
    fn test_display_thresholds() {
        let mass = Mass::from_kilograms(1.0);
        assert_eq!(mass.to_string(), "1.00 kg");
        let mass = Mass::from_earth_masses(1.0);
        assert_eq!(mass.to_string(), "1.00 Earth masses");
        let mass = Mass::from_jupiter_masses(1.0);
        assert_eq!(mass.to_string(), "1.00 Jupiter masses");
        let mass = Mass::from_solar_masses(1.0);
        assert_eq!(mass.to_string(), "1.00 solar masses");
    }
}
