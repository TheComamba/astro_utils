use super::luminous_intensity::absolute_magnitude_to_luminous_intensity;
use simple_si_units::{
    base::{Distance, Luminosity},
    electromagnetic::Illuminance,
};

pub const IRRADIANCE_ZERO: Illuminance<f64> = Illuminance { lux: 0. };
pub const ILLUMINANCE_OF_BOLOMETRIC_ZERO: Illuminance<f64> = Illuminance { lux: 2.128e-6 };
pub const IRRADIANCE_OF_BOLOMETRIC_ZERO: Illuminance<f64> = Illuminance { lux: 2.518e-8 }; // W/m^2, not lux

pub const fn from_lux(lux: f64) -> Illuminance<f64> {
    Illuminance { lux }
}

pub fn apparent_magnitude_to_illuminance(apparent_magnitude: f64) -> Illuminance<f64> {
    let exponent = apparent_magnitude / -2.5;
    ILLUMINANCE_OF_BOLOMETRIC_ZERO * 10_f64.powf(exponent)
}

pub fn illuminance_to_apparent_magnitude(illuminance: &Illuminance<f64>) -> f64 {
    -2.5 * (illuminance / &ILLUMINANCE_OF_BOLOMETRIC_ZERO).log10()
}

pub fn illuminance_to_luminous_intensity(
    illuminance: &Illuminance<f64>,
    distance: &Distance<f64>,
) -> Luminosity<f64> {
    let absolute_magnitude =
        illuminance_to_apparent_magnitude(illuminance) - 5. * distance.to_parsec().log10() + 5.;
    absolute_magnitude_to_luminous_intensity(absolute_magnitude)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        tests::eq,
        units::luminous_intensity::{luminous_intensity_to_illuminance, SOLAR_LUMINOUS_INTENSITY},
    };

    const REAL_DATA_TEST_ACCURACY: f64 = 0.05;

    #[test]
    fn apparent_magnitude_roundtrip() {
        for apparent_magnitude in -10..10 {
            let input = apparent_magnitude as f64;
            let illuminance = apparent_magnitude_to_illuminance(input);
            let output = illuminance_to_apparent_magnitude(&illuminance);
            println!("input: {}, output: {}", input, output);
            assert!(eq(input, output));
        }
    }

    #[test]
    fn test_sunlight() {
        let luminous_intensity = SOLAR_LUMINOUS_INTENSITY;
        let distance = Distance::from_au(1.);
        let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &distance);
        let apparent_magnitude = illuminance_to_apparent_magnitude(&illuminance);
        let expected_app_mag = -26.74;
        println!(
            "Apparent Magnitude:\nexpected: {}, actual: {}",
            expected_app_mag, apparent_magnitude
        );
        assert!(eq(apparent_magnitude, expected_app_mag));

        let lux = illuminance.to_lux();
        let expected_lux = 107_527.;
        println!("lux:\nexpected: {}, actual: {}", expected_lux, lux);
        assert!((lux - expected_lux).abs() < REAL_DATA_TEST_ACCURACY * expected_lux);
    }

    #[test]
    fn test_sirius() {
        let luminous_intensity = 25.4 * SOLAR_LUMINOUS_INTENSITY;
        let distance = Distance::from_au(8.6);
        let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &distance);
        let apparent_magnitude = illuminance_to_apparent_magnitude(&illuminance);
        let expected_app_mag = -1.46;
        println!(
            "Apparent Magnitude:\nexpected: {}, actual: {}",
            expected_app_mag, apparent_magnitude
        );
        assert!(eq(apparent_magnitude, expected_app_mag));
    }
}
