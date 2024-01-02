use std::fmt::Display;

use crate::Float;
use serde::{Deserialize, Serialize};

use super::length::{Length, METERS_PER_PARSEC};

const WATTS_PER_SOLAR_LUMINOSITY: Float = 3.828e26;
const SOLAR_LUMINOSITY_PER_WATT: Float = 1. / WATTS_PER_SOLAR_LUMINOSITY;
const TEN_PARSEC: Length = Length::from_meters(10. * METERS_PER_PARSEC);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Luminosity {
    pub(super) absolute_magnitude: Float,
}

impl Luminosity {
    pub const fn from_absolute_magnitude(absolute_magnitude: Float) -> Luminosity {
        Luminosity { absolute_magnitude }
    }

    pub fn from_apparent_magnitude(apparent_magnitude: Float, distance: &Length) -> Luminosity {
        let absolute_magnitude = apparent_magnitude - 5. * distance.as_parsecs().log10() + 5.;
        Luminosity { absolute_magnitude }
    }

    pub fn from_solar_luminosities(solar_luminosities: Float) -> Luminosity {
        let absolute_magnitude = 4.8 - 2.5 * solar_luminosities.log10();
        Luminosity { absolute_magnitude }
    }

    pub fn from_watts(watts: Float) -> Luminosity {
        Self::from_solar_luminosities(watts * SOLAR_LUMINOSITY_PER_WATT)
    }

    pub const fn get_absolute_magnitude(&self) -> Float {
        self.absolute_magnitude
    }

    pub fn as_apparent_magnitude(&self, distance: &Length) -> Float {
        self.absolute_magnitude + 5. * distance.as_parsecs().log10() - 5.
    }

    pub fn as_solar_luminosities(&self) -> Float {
        let exponent = (4.8 - self.absolute_magnitude) / 2.5;
        (10. as Float).powf(exponent)
    }

    pub fn as_watts(&self) -> Float {
        self.as_solar_luminosities() * WATTS_PER_SOLAR_LUMINOSITY
    }
}

impl Display for Luminosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} L☉", self.as_solar_luminosities())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::TEST_ACCURACY;

    const REAL_DATA_TEST_ACCURACY: Float = 0.05;

    #[test]
    fn test_absolute_magnitude() {
        for i in -10..10 {
            let input = i as Float;
            let luminosity = Luminosity::from_absolute_magnitude(input);
            let output = luminosity.get_absolute_magnitude();
            println!("input: {}, output: {}", input, output);
            assert!((input - output).abs() < TEST_ACCURACY);
        }
    }

    #[test]
    fn test_apparent_magnitudes() {
        for lum in -10..10 {
            for dist in 1..10 {
                let input = lum as Float;
                let distance = Length::from_light_years(dist as Float);
                let luminosity = Luminosity::from_apparent_magnitude(input, &distance);
                let output = luminosity.as_apparent_magnitude(&distance);
                println!("distance: {}", distance);
                println!("input: {}, output: {}", input, output);
                assert!((input - output).abs() < TEST_ACCURACY);
            }
        }
    }

    #[test]
    fn test_solar_luminosities() {
        for i in -10..10 {
            let input = (i as Float).exp();
            let luminosity = Luminosity::from_solar_luminosities(input);
            let output = luminosity.as_solar_luminosities();
            println!("input: {}, output: {}", input, output);
            assert!((input - output).abs() < TEST_ACCURACY * input);
        }
    }

    #[test]
    fn test_watts() {
        for i in -10..10 {
            let input = (i as Float).exp() * WATTS_PER_SOLAR_LUMINOSITY;
            let luminosity = Luminosity::from_watts(input);
            let output = luminosity.as_watts();
            println!("input: {}, output: {}", input, output);
            assert!((input - output).abs() < TEST_ACCURACY * input);
        }
    }

    #[test]
    fn apparent_and_absolute_magnitude_at_ten_parsecs_are_equal() {
        for i in -10..10 {
            let input = i as Float;
            let luminosity = Luminosity::from_absolute_magnitude(input);
            let distance = TEN_PARSEC;
            let apparent_magnitude = luminosity.as_apparent_magnitude(&distance);
            let absolute_magnitude = luminosity.get_absolute_magnitude();
            println!("input: {}", input);
            println!("apparent magnitude: {}", apparent_magnitude);
            println!("absolute magnitude: {}", absolute_magnitude);
            assert!((apparent_magnitude - absolute_magnitude).abs() < TEST_ACCURACY);
        }
    }

    #[test]
    fn test_the_sun() {
        let sun = Luminosity::from_absolute_magnitude(4.83);

        let sun_apparent_magnitude =
            sun.as_apparent_magnitude(&Length::from_astronomical_units(1.));
        let expected_apparent_magnitude = -26.74;
        println!(
            "Apparnet Magnitude:\nexpected: {},\nactual: {}",
            expected_apparent_magnitude, sun_apparent_magnitude
        );
        assert!(
            (expected_apparent_magnitude - sun_apparent_magnitude).abs() < REAL_DATA_TEST_ACCURACY
        );

        let sun_solar_luminosities = sun.as_solar_luminosities();
        let expected_solar_luminosities = 1.;
        println!(
            "Solar Luminosities:\nexpected: {},\nactual: {}",
            expected_solar_luminosities, sun_solar_luminosities
        );
        assert!(
            (expected_solar_luminosities - sun_solar_luminosities).abs() < REAL_DATA_TEST_ACCURACY
        );

        let sun_watts = sun.as_watts();
        let expected_watts = WATTS_PER_SOLAR_LUMINOSITY;
        println!(
            "Watts:\nexpected: {},\nactual: {}",
            expected_watts, sun_watts
        );
        assert!(
            (expected_watts - sun_watts).abs()
                < REAL_DATA_TEST_ACCURACY * WATTS_PER_SOLAR_LUMINOSITY
        );
    }

    #[test]
    fn test_sirius() {
        let sirius = Luminosity::from_absolute_magnitude(1.43);

        let sirius_apparent_magnitude =
            sirius.as_apparent_magnitude(&&Length::from_light_years(8.6));
        let expected_apparent_magnitude = -1.46;
        println!(
            "Apparnet Magnitude:\nexpected: {},\nactual: {}",
            expected_apparent_magnitude, sirius_apparent_magnitude
        );
        assert!(
            (expected_apparent_magnitude - sirius_apparent_magnitude).abs()
                < REAL_DATA_TEST_ACCURACY
        );

        let sirius_solar_luminosities = sirius.as_solar_luminosities();
        let expected_solar_luminosities = 22.3;
        println!(
            "Solar Luminosities:\nexpected: {},\nactual: {}",
            expected_solar_luminosities, sirius_solar_luminosities
        );
        assert!(
            (expected_solar_luminosities - sirius_solar_luminosities).abs()
                < REAL_DATA_TEST_ACCURACY
        );

        let sirius_watts = sirius.as_watts();
        let expected_watts = 22.3 * WATTS_PER_SOLAR_LUMINOSITY;
        println!(
            "Watts:\nexpected: {},\nactual: {}",
            expected_watts, sirius_watts
        );
        assert!(
            (expected_watts - sirius_watts).abs()
                < REAL_DATA_TEST_ACCURACY * WATTS_PER_SOLAR_LUMINOSITY
        );
    }
}
