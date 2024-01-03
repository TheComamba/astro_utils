use super::{length::Length, luminance::Luminance, luminosity::Luminosity};
use crate::{Float, PI};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Illuminance {
    pub(crate) lux: Float,
}

impl Illuminance {
    pub const fn from_lux(lux: Float) -> Illuminance {
        Illuminance { lux }
    }

    pub fn from_apparent_magnitude(apparent_magnitude: Float) -> Illuminance {
        let exponent = (-14.18 - apparent_magnitude) / 2.5;
        let lux = (10. as Float).powf(exponent);
        Illuminance { lux }
    }

    pub const fn as_lux(&self) -> Float {
        self.lux
    }

    pub fn as_apparent_magnitude(&self) -> Float {
        -14.18 - 2.5 * self.lux.log10()
    }

    pub fn to_luminosity(&self, distance: &Length) -> Luminosity {
        let absolute_magnitude =
            self.as_apparent_magnitude() - 5. * distance.as_parsecs().log10() + 5.;
        Luminosity::from_absolute_magnitude(absolute_magnitude)
    }

    pub fn to_luminance_flat_surface(&self) -> Luminance {
        let nit = self.lux / PI;
        Luminance::from_nit(nit)
    }

    #[cfg(test)]
    pub(crate) fn eq_within(&self, other: Illuminance, accuracy: Illuminance) -> bool {
        (self.lux - other.lux).abs() <= accuracy.lux
    }
}

impl Display for Illuminance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.lux.abs() > 990. {
            write!(f, "{:.2} klux", self.lux / 1_000.)
        } else if self.lux.abs() > 0.99 {
            write!(f, "{:.2} lux", self.lux)
        } else if self.lux.abs() > 0.99e-3 {
            write!(f, "{:.2} mlux", self.lux * 1_000.)
        } else if self.lux.abs() > 0.99e-6 {
            write!(f, "{:.2} µlux", self.lux * 1_000_000.)
        } else {
            write!(f, "{:.2} nlux", self.lux * 1_000_000_000.)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::TEST_ACCURACY;

    const REAL_DATA_TEST_ACCURACY: Float = 0.05;

    #[test]
    fn test_apparent_magnitudes() {
        for magnitude in -10..10 {
            let input = magnitude as Float;
            let illuminance = Illuminance::from_apparent_magnitude(input);
            let output = illuminance.as_apparent_magnitude();
            println!("input: {}, output: {}", input, output);
            assert!((input - output).abs() < TEST_ACCURACY);
        }
    }

    #[test]
    fn test_sunlight() {
        let luminosity = Luminosity::from_solar_luminosities(1.);
        let distance = Length::from_astronomical_units(1.);
        let illuminance = luminosity.to_illuminance(&distance);
        let actual = illuminance.as_lux();
        let expected = 107_527.;
        println!("expected: {}, actual: {}", expected, actual);
        assert!((actual - expected).abs() < REAL_DATA_TEST_ACCURACY * expected);
    }

    #[test]
    fn test_display() {
        let nlux = Illuminance::from_lux(1.23e-9);
        assert_eq!(format!("{}", nlux), "1.23 nlux");
        let ulux = Illuminance::from_lux(1.23e-6);
        assert_eq!(format!("{}", ulux), "1.23 µlux");
        let mlux = Illuminance::from_lux(1.23e-3);
        assert_eq!(format!("{}", mlux), "1.23 mlux");
        let lux = Illuminance::from_lux(1.23);
        assert_eq!(format!("{}", lux), "1.23 lux");
        let klux = Illuminance::from_lux(1.23e3);
        assert_eq!(format!("{}", klux), "1.23 klux");
    }

    #[test]
    fn test_negative_display() {
        let nlux = Illuminance::from_lux(-1.23e-9);
        assert_eq!(format!("{}", nlux), "-1.23 nlux");
        let ulux = Illuminance::from_lux(-1.23e-6);
        assert_eq!(format!("{}", ulux), "-1.23 µlux");
        let mlux = Illuminance::from_lux(-1.23e-3);
        assert_eq!(format!("{}", mlux), "-1.23 mlux");
        let lux = Illuminance::from_lux(-1.23);
        assert_eq!(format!("{}", lux), "-1.23 lux");
        let klux = Illuminance::from_lux(-1.23e3);
        assert_eq!(format!("{}", klux), "-1.23 klux");
    }

    #[test]
    fn test_display_thresholds() {
        let nlux = Illuminance::from_lux(1e-9);
        assert_eq!(format!("{}", nlux), "1.00 nlux");
        let ulux = Illuminance::from_lux(1e-6);
        assert_eq!(format!("{}", ulux), "1.00 µlux");
        let mlux = Illuminance::from_lux(1e-3);
        assert_eq!(format!("{}", mlux), "1.00 mlux");
        let lux = Illuminance::from_lux(1.);
        assert_eq!(format!("{}", lux), "1.00 lux");
        let klux = Illuminance::from_lux(1e3);
        assert_eq!(format!("{}", klux), "1.00 klux");
    }
}
