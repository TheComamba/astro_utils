use super::luminosity::absolute_magnitude_to_luminosity;
use simple_si_units::{
    base::{Distance, Luminosity},
    electromagnetic::Illuminance,
};

pub const ILLUMINANCE_ZERO: Illuminance<f64> = Illuminance { lux: 0. };

pub const fn from_lux(lux: f64) -> Illuminance<f64> {
    Illuminance { lux }
}

pub fn apparent_magnitude_to_illuminance(apparent_magnitude: f64) -> Illuminance<f64> {
    let exponent = (-14.18 - apparent_magnitude) / 2.5;
    let lux = (10. as f64).powf(exponent);
    Illuminance { lux }
}

pub fn illuminance_to_apparent_magnitude(illuminance: &Illuminance<f64>) -> f64 {
    -14.18 - 2.5 * illuminance.lux.log10()
}

pub fn illuminance_to_luminosity(
    illuminance: &Illuminance<f64>,
    distance: &Distance<f64>,
) -> Luminosity<f64> {
    let absolute_magnitude =
        illuminance_to_apparent_magnitude(illuminance) - 5. * distance.to_parsec().log10() + 5.;
    absolute_magnitude_to_luminosity(absolute_magnitude)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        tests::eq,
        units::luminosity::{luminosity_to_illuminance, WATTS_PER_SOLAR_LUMINOSITY},
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
        let luminosity = Luminosity::from_cd(WATTS_PER_SOLAR_LUMINOSITY);
        let distance = Distance::from_au(1.);
        let illuminance = luminosity_to_illuminance(&luminosity, &distance);
        let actual = illuminance.to_lux();
        let expected = 107_527.;
        println!("expected: {}, actual: {}", expected, actual);
        assert!((actual - expected).abs() < REAL_DATA_TEST_ACCURACY * expected);
    }
}
