use crate::Float;
use serde::{Deserialize, Serialize};

use super::length::Length;

const WATTS_PER_SOLAR_LUMINOSITY: Float = 3.828e26;
const SOLAR_LUMINOSITY_PER_WATT: Float = 1. / WATTS_PER_SOLAR_LUMINOSITY;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Luminosity {
    pub(super) absolute_magnitude: Float,
}

impl Luminosity {
    pub const fn from_absolute_magnitude(absolute_magnitude: Float) -> Luminosity {
        Luminosity { absolute_magnitude }
    }

    pub fn from_apparent_magnitude(apparent_magnitude: Float, distance: &Length) -> Luminosity {
        todo!()
    }

    pub fn from_solar_luminosities(solar_luminosities: Float) -> Luminosity {
        todo!()
    }

    pub fn from_watts(watts: Float) -> Luminosity {
        todo!()
    }

    pub const fn get_absolute_magnitude(&self) -> Float {
        self.absolute_magnitude
    }

    pub fn as_apparent_magnitude(&self, distance: &Length) -> Float {
        todo!()
    }

    pub fn as_solar_luminosities(&self) -> Float {
        todo!()
    }

    pub fn as_watts(&self) -> Float {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::TEST_ACCURACY;

    use super::*;

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
                println!("input: {}, output: {}", input, output);
                assert!((input - output).abs() < TEST_ACCURACY);
            }
        }
    }

    #[test]
    fn test_solar_luminosities() {
        for i in -3..3 {
            let input = (i as Float).exp();
            let luminosity = Luminosity::from_solar_luminosities(input);
            let output = luminosity.as_solar_luminosities();
            println!("input: {}, output: {}", input, output);
            assert!((input - output).abs() < TEST_ACCURACY);
        }
    }

    #[test]
    fn test_watts() {
        for i in -3..3 {
            let input = (i as Float).exp() * WATTS_PER_SOLAR_LUMINOSITY;
            let luminosity = Luminosity::from_watts(input);
            let output = luminosity.as_watts();
            println!("input: {}, output: {}", input, output);
            assert!((input - output).abs() < TEST_ACCURACY);
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
        assert!((expected_apparent_magnitude - sun_apparent_magnitude).abs() < TEST_ACCURACY);

        let sun_solar_luminosities = sun.as_solar_luminosities();
        let expected_solar_luminosities = 1.;
        println!(
            "Solar Luminosities:\nexpected: {},\nactual: {}",
            expected_solar_luminosities, sun_solar_luminosities
        );
        assert!((expected_solar_luminosities - sun_solar_luminosities).abs() < TEST_ACCURACY);

        let sun_watts = sun.as_watts();
        let expected_watts = WATTS_PER_SOLAR_LUMINOSITY;
        println!(
            "Watts:\nexpected: {},\nactual: {}",
            expected_watts, sun_watts
        );
        assert!((expected_watts - sun_watts).abs() < TEST_ACCURACY * WATTS_PER_SOLAR_LUMINOSITY);
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
        assert!((expected_apparent_magnitude - sirius_apparent_magnitude).abs() < TEST_ACCURACY);

        let sirius_solar_luminosities = sirius.as_solar_luminosities();
        let expected_solar_luminosities = 25.4;
        println!(
            "Solar Luminosities:\nexpected: {},\nactual: {}",
            expected_solar_luminosities, sirius_solar_luminosities
        );
        assert!((expected_solar_luminosities - sirius_solar_luminosities).abs() < TEST_ACCURACY);

        let sirius_watts = sirius.as_watts();
        let expected_watts = 25.4 * WATTS_PER_SOLAR_LUMINOSITY;
        println!(
            "Watts:\nexpected: {},\nactual: {}",
            expected_watts, sirius_watts
        );
        assert!((expected_watts - sirius_watts).abs() < TEST_ACCURACY * WATTS_PER_SOLAR_LUMINOSITY);
    }
}
