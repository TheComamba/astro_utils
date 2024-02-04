use simple_si_units::{base::Distance, electromagnetic::Illuminance};
use std::f64::consts::PI;

pub const ILLUMINANCE_ZERO: Illuminance<f64> = Illuminance { lux: 0. };

pub const fn from_lux(lux: f64) -> Illuminance<f64> {
    Illuminance { lux }
}

pub fn illuminance_from_apparent_magnitude(apparent_magnitude: f64) -> Illuminance<f64> {
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
    Luminosity::from_absolute_magnitude(absolute_magnitude)
}

pub fn to_luminance_flat_surface(&self) -> Luminance {
    let nit = self.lux / PI;
    Luminance::from_nit(nit)
}

#[cfg(test)]
mod tests {

    use crate::tests::eq;

    const REAL_DATA_TEST_ACCURACY: f64 = 0.05;

    #[test]
    fn test_apparent_magnitudes() {
        for magnitude in -10..10 {
            let input = magnitude as f64;
            let illuminance = Illuminance::from_apparent_magnitude(input);
            let output = illuminance.to_apparent_magnitude();
            println!("input: {}, output: {}", input, output);
            assert!(eq(input, output));
        }
    }

    #[test]
    fn test_sunlight() {
        let luminosity = Luminosity::from_solar_luminosities(1.);
        let distance = Distance::from_astronomical_units(1.);
        let illuminance = luminosity.to_illuminance(&distance);
        let actual = illuminance.to_lux();
        let expected = 107_527.;
        println!("expected: {}, actual: {}", expected, actual);
        assert!((actual - expected).abs() < REAL_DATA_TEST_ACCURACY * expected);
    }
}
