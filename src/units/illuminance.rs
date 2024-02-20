use crate::astro_display::AstroDisplay;
use simple_si_units::electromagnetic::Illuminance;

pub const IRRADIANCE_ZERO: Illuminance<f64> = Illuminance { lux: 0. };
pub const APARENT_VISIBLE_MAGNITUDE_ZERO: Illuminance<f64> = Illuminance { lux: 2.6e-6 };
pub const IRRADIANCE_OF_BOLOMETRIC_ZERO: Illuminance<f64> = Illuminance { lux: 2.518e-8 }; // W/m^2, not lux

pub const fn from_lux(lux: f64) -> Illuminance<f64> {
    Illuminance { lux }
}

pub fn apparent_magnitude_to_illuminance(apparent_magnitude: f64) -> Illuminance<f64> {
    let exponent = apparent_magnitude / -2.5;
    APARENT_VISIBLE_MAGNITUDE_ZERO * 10_f64.powf(exponent)
}

pub fn illuminance_to_apparent_magnitude(illuminance: &Illuminance<f64>) -> f64 {
    -2.5 * (illuminance / &APARENT_VISIBLE_MAGNITUDE_ZERO).log10()
}

impl AstroDisplay for Illuminance<f64> {
    fn astro_display(&self) -> String {
        let apparent_magnitude = illuminance_to_apparent_magnitude(self);
        format!("{:.2} app. mag.", apparent_magnitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        tests::{eq, eq_within},
        units::luminous_intensity::{luminous_intensity_to_illuminance, SOLAR_LUMINOUS_INTENSITY},
    };
    use simple_si_units::base::Distance;

    const REAL_DATA_TEST_ACCURACY: f64 = 0.05;

    #[test]
    fn apparent_magnitude_roundtrip() {
        for apparent_magnitude in -10..10 {
            let input = apparent_magnitude as f64;
            let illuminance = apparent_magnitude_to_illuminance(input);
            let output = illuminance_to_apparent_magnitude(&illuminance);
            assert!(eq(input, output));
        }
    }

    #[test]
    fn apparent_magnitude_difference_of_1_corresponds_to_factor_of_2_512() {
        let expected = 100_f64.powf(1. / 5.);
        for i in -10..10 {
            let illuminance = apparent_magnitude_to_illuminance(i as f64);
            let illuminance_plus_1 = apparent_magnitude_to_illuminance((i + 1) as f64);
            let ratio = illuminance / illuminance_plus_1;
            println!("i: {}, ratio: {}", i, ratio);
            assert!(eq(ratio, expected));
        }
    }

    #[test]
    fn test_sunlight() {
        let luminous_intensity = SOLAR_LUMINOUS_INTENSITY;
        let distance = Distance::from_au(1.);
        let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &distance);
        let apparent_magnitude = illuminance_to_apparent_magnitude(&illuminance);
        let expected_app_mag = -26.74;
        assert!(eq_within(apparent_magnitude, expected_app_mag, 0.05));
    }

    #[test]
    fn test_lux_of_sunlight() {
        let apparent_magnitude = -26.72;
        let illuminance = apparent_magnitude_to_illuminance(apparent_magnitude);

        let expected_lux = 107_527.;
        assert!(eq_within(
            illuminance.lux,
            expected_lux,
            5. * REAL_DATA_TEST_ACCURACY * expected_lux
        ));
    }

    #[test]
    fn test_sirius() {
        let luminous_intensity = 22. * SOLAR_LUMINOUS_INTENSITY;
        let distance = Distance::from_lyr(8.6);
        let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &distance);
        let apparent_magnitude = illuminance_to_apparent_magnitude(&illuminance);
        let expected_app_mag = -1.46;
        assert!(eq_within(apparent_magnitude, expected_app_mag, 0.05));
    }
}
