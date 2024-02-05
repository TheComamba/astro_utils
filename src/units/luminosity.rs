use super::irradiance::{apparent_magnitude_to_irradiance, irradiance_to_apparent_magnitude};
use simple_si_units::{
    base::{Distance, Luminosity},
    electromagnetic::Illuminance,
};
use std::f64::consts::PI;

pub const LUMINOSITY_ZERO: Luminosity<f64> = Luminosity { cd: 0. };
pub const SOLAR_LUMINOSITY: Luminosity<f64> = Luminosity {
    /*Watts*/ cd: 3.828e26,
};
pub const SOLAR_LUMINOUS_INTENSITY: Luminosity<f64> = Luminosity { cd: 2.98e27 };

pub fn absolute_magnitude_to_luminosity(absolute_magnitude: f64) -> Luminosity<f64> {
    let ten_pc = Distance::from_parsec(10.);
    let irradiance = apparent_magnitude_to_irradiance(absolute_magnitude);
    irradiance_to_luminosity(&irradiance, &ten_pc)
}

pub fn luminosity_to_absolute_magnitude(luminosity: Luminosity<f64>) -> f64 {
    let ten_pc = Distance::from_parsec(10.);
    let irradiance = luminosity_to_irradiance(&luminosity, &ten_pc);
    irradiance_to_apparent_magnitude(&irradiance)
}

pub fn luminosity_to_irradiance(
    luminosity: &Luminosity<f64>,
    distance: &Distance<f64>,
) -> Illuminance<f64> {
    let lux = luminosity.cd / (4. * PI * distance * distance).m2;
    Illuminance { lux }
}

pub fn irradiance_to_luminosity(
    irradiance: &Illuminance<f64>,
    distance: &Distance<f64>,
) -> Luminosity<f64> {
    let watts = irradiance.lux * 4. * PI * distance.m * distance.m;
    Luminosity { cd: watts }
}

pub fn display_luminosity(luminosity: &Luminosity<f64>) -> String {
    format!("{} W", luminosity.cd) //should be watts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{eq, eq_within};

    const REAL_DATA_TEST_ACCURACY: f64 = 0.05;
    const ILLUMINANCE_AT_UNIT_DISTANCE: f64 = 1. / (4. * PI);

    #[test]
    fn irradiance_roundtrip() {
        for i in -10..10 {
            let input = i as f64;
            let luminosity = Luminosity::from_cd(input);
            let distance = Distance::from_m(1.);
            let irradiance = luminosity_to_irradiance(&luminosity, &distance);
            let output = irradiance_to_luminosity(&irradiance, &distance);
            println!("input: {}, output: {}", input, output.cd);
            assert!(eq(input, output.cd));
        }
    }

    #[test]
    fn absolute_magnitude_roundtrip() {
        for i in -10..10 {
            let input = i as f64;
            let luminosity = absolute_magnitude_to_luminosity(input);
            let output = luminosity_to_absolute_magnitude(luminosity);
            println!("input: {}, output: {}", input, output);
            assert!(eq(input, output));
        }
    }

    #[test]
    fn irradiance_of_1_watt_source_at_1_m() {
        let luminosity = Luminosity::from_cd(1.); //should be watts
        let distance = Distance::from_m(1.);
        let irradiance = luminosity_to_irradiance(&luminosity, &distance);
        let actual = irradiance.to_lux();
        let expected = ILLUMINANCE_AT_UNIT_DISTANCE;
        println!("expected: {}, actual: {}", expected, actual);
        assert!(eq(actual, expected));
    }

    #[test]
    fn irradiance_is_proportional_to_luminosity() {
        for i in 1..10 {
            let cd = i as f64;
            let luminosity = Luminosity::from_cd(cd);
            let distance = Distance::from_m(1.);
            let irradiance = luminosity_to_irradiance(&luminosity, &distance);
            let expected = cd * ILLUMINANCE_AT_UNIT_DISTANCE;
            let actual = irradiance.to_lux();
            println!("expected: {}, actual: {}", expected, actual);
            assert!(eq(actual, expected));
        }
    }

    #[test]
    fn irradiance_is_inversely_proportional_to_distance_squared() {
        for d in 1..10 {
            let distance = Distance::from_m(d as f64);
            let luminosity = Luminosity::from_cd(1.);
            let irradiance = luminosity_to_irradiance(&luminosity, &distance);
            let expected = ILLUMINANCE_AT_UNIT_DISTANCE / (d * d) as f64;
            let actual = irradiance.to_lux();
            println!("expected: {}, actual: {}", expected, actual);
            assert!(eq(actual, expected));
        }
    }

    #[test]
    fn apparent_and_absolute_magnitude_at_ten_parsecs_are_equal() {
        let ten_pc = Distance::from_parsec(10.);
        for i in -10..10 {
            let input = i as f64;
            let luminosity = absolute_magnitude_to_luminosity(input);
            let irradiance = luminosity_to_irradiance(&luminosity, &ten_pc);
            let apparent_magnitude = irradiance_to_apparent_magnitude(&irradiance);
            let absolute_magnitude = luminosity_to_absolute_magnitude(luminosity);
            println!("input: {}", input);
            println!("apparent magnitude: {}", apparent_magnitude);
            println!("absolute magnitude: {}", absolute_magnitude);
            assert!(eq(apparent_magnitude, absolute_magnitude));
        }
    }

    #[test]
    fn test_the_sun() {
        let sun_abs_mag = luminosity_to_absolute_magnitude(SOLAR_LUMINOSITY);
        let expected = 4.83;
        println!("expected: {},\nactual: {}", expected, sun_abs_mag);
        assert!(eq_within(expected, sun_abs_mag, REAL_DATA_TEST_ACCURACY));
    }

    #[test]
    fn test_sirius() {
        let sun_abs_mag = luminosity_to_absolute_magnitude(25.4 * SOLAR_LUMINOSITY);
        let expected = 1.43;
        println!("expected: {},\nactual: {}", expected, sun_abs_mag);
        assert!(eq_within(expected, sun_abs_mag, REAL_DATA_TEST_ACCURACY));
    }
}
