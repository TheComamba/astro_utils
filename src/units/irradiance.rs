use super::luminosity::absolute_magnitude_to_luminosity;
use simple_si_units::{
    base::{Distance, Luminosity},
    electromagnetic::Illuminance,
};

pub const IRRADIANCE_ZERO: Illuminance<f64> = Illuminance { lux: 0. };

pub const fn from_lux(lux: f64) -> Illuminance<f64> {
    Illuminance { lux }
}

pub fn apparent_magnitude_to_irradiance(apparent_magnitude: f64) -> Illuminance<f64> {
    let exponent = (-14.18 - apparent_magnitude) / 2.5;
    let lux = (10. as f64).powf(exponent);
    Illuminance { lux }
}

pub fn irradiance_to_apparent_magnitude(irradiance: &Illuminance<f64>) -> f64 {
    -14.18 - 2.5 * irradiance.lux.log10()
}

pub fn irradiance_to_luminosity(
    irradiance: &Illuminance<f64>,
    distance: &Distance<f64>,
) -> Luminosity<f64> {
    let absolute_magnitude =
        irradiance_to_apparent_magnitude(irradiance) - 5. * distance.to_parsec().log10() + 5.;
    absolute_magnitude_to_luminosity(absolute_magnitude)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        tests::eq,
        units::luminosity::{luminosity_to_irradiance, SOLAR_LUMINOSITY, SOLAR_LUMINOUS_INTENSITY},
    };

    const REAL_DATA_TEST_ACCURACY: f64 = 0.05;

    #[test]
    fn apparent_magnitude_roundtrip() {
        for apparent_magnitude in -10..10 {
            let input = apparent_magnitude as f64;
            let irradiance = apparent_magnitude_to_irradiance(input);
            let output = irradiance_to_apparent_magnitude(&irradiance);
            println!("input: {}, output: {}", input, output);
            assert!(eq(input, output));
        }
    }

    #[test]
    fn test_sunlight() {
        let luminosity = SOLAR_LUMINOUS_INTENSITY;
        let distance = Distance::from_au(1.);
        let irradiance = luminosity_to_irradiance(&luminosity, &distance);
        let apparent_magnitude = irradiance_to_apparent_magnitude(&irradiance);
        let expected_app_mag = -26.74;
        println!(
            "Apparent Magnitude:\nexpected: {}, actual: {}",
            expected_app_mag, apparent_magnitude
        );
        assert!(eq(apparent_magnitude, expected_app_mag));

        let lux = irradiance.to_lux();
        let expected_lux = 107_527.;
        println!("lux:\nexpected: {}, actual: {}", expected_lux, lux);
        assert!((lux - expected_lux).abs() < REAL_DATA_TEST_ACCURACY * expected_lux);
    }

    #[test]
    fn test_sirius() {
        let luminosity = 25.4 * SOLAR_LUMINOUS_INTENSITY;
        let distance = Distance::from_au(8.6);
        let irradiance = luminosity_to_irradiance(&luminosity, &distance);
        let apparent_magnitude = irradiance_to_apparent_magnitude(&irradiance);
        let expected_app_mag = -1.46;
        println!(
            "Apparent Magnitude:\nexpected: {}, actual: {}",
            expected_app_mag, apparent_magnitude
        );
        assert!(eq(apparent_magnitude, expected_app_mag));
    }
}
