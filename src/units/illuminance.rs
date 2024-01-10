use super::{length::Length, luminance::Luminance, luminosity::Luminosity};
use crate::{Float, PI};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Illuminance {
    pub(crate) lux: Float,
}

impl Illuminance {
    pub const ZERO: Illuminance = Illuminance { lux: 0. };

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
        write!(f, "{:.2} mag", self.as_apparent_magnitude())
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
}
