use std::fmt::Display;

use super::{length::Length, luminosity::Luminosity};
use crate::Float;

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

    pub const fn get_lux(&self) -> Float {
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
}

impl Display for Illuminance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} lux", self.lux)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::TEST_ACCURACY;

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
}
