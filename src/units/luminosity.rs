use super::illuminance::{apparent_magnitude_to_illuminance, illuminance_to_apparent_magnitude};
use simple_si_units::{
    base::{Distance, Luminosity},
    electromagnetic::Illuminance,
};
use std::f64::consts::PI;

pub const LUMINOSITY_ZERO: Luminosity<f64> = Luminosity { cd: 0. };

pub const SOLAR_LUMINOSITY: Luminosity<f64> = Luminosity { cd: 3.828e26 };

pub const fn absolute_magnitude_to_luminosity(absolute_magnitude: f64) -> Luminosity<f64> {
    let ten_pc = Distance::from_parsec(10.);
    let illuminance = apparent_magnitude_to_illuminance(absolute_magnitude);
    illuminance_to_luminosity(&illuminance, &ten_pc)
}

pub fn luminosity_to_absolute_magnitude(luminosity: Luminosity<f64>) -> f64 {
    let ten_pc = Distance::from_parsec(10.);
    let illuminance = luminosity_to_illuminance(&luminosity, &ten_pc);
    illuminance_to_apparent_magnitude(&illuminance)
}

pub fn luminosity_to_illuminance(
    luminosity: &Luminosity<f64>,
    distance: &Distance<f64>,
) -> Illuminance<f64> {
    let lux = luminosity.cd / (4. * PI * distance * distance).m2;
    Illuminance { lux }
}

pub fn illuminance_to_luminosity(
    illuminance: &Illuminance<f64>,
    distance: &Distance<f64>,
) -> Luminosity<f64> {
    let watts = illuminance.lux * 4. * PI * distance.m * distance.m;
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

    #[test]
    fn illuminance_roundtrip() {
        for i in -10..10 {
            let input = i as f64;
            let luminosity = Luminosity::from_cd(input);
            let distance = Distance::from_m(1.);
            let illuminance = luminosity_to_illuminance(&luminosity, &distance);
            let output = illuminance_to_luminosity(&illuminance, &distance);
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
    fn illuminance_of_1_watt_source_at_1_m() {
        let luminosity = Luminosity::from_cd(1.); //should be watts
        let distance = Distance::from_m(1.);
        let illuminance = luminosity_to_illuminance(&luminosity, &distance);
        let actual = illuminance.to_lux();
        let expected = 1.;
        println!("expected: {}, actual: {}", expected, actual);
        assert!(eq(actual, expected));
    }

    #[test]
    fn apparent_and_absolute_magnitude_at_ten_parsecs_are_equal() {
        let ten_pc = Distance::from_parsec(10.);
        for i in -10..10 {
            let input = i as f64;
            let luminosity = absolute_magnitude_to_luminosity(input);
            let illuminance = luminosity_to_illuminance(&luminosity, &ten_pc);
            let apparent_magnitude = illuminance_to_apparent_magnitude(&illuminance);
            let absolute_magnitude = luminosity_to_absolute_magnitude(luminosity);
            println!("input: {}", input);
            println!("apparent magnitude: {}", apparent_magnitude);
            println!("absolute magnitude: {}", absolute_magnitude);
            assert!(eq(apparent_magnitude, absolute_magnitude));
        }
    }

    #[test]
    fn test_the_sun() {
        let sun = absolute_magnitude_to_luminosity(4.83);

        let distance = Distance::from_au(1.);
        let sun_illuminance = luminosity_to_illuminance(&sun, &distance);
        let sun_app_mag = illuminance_to_apparent_magnitude(&sun_illuminance);
        let expected = -26.74;
        println!("expected: {},\nactual: {}", expected, sun_app_mag);
        assert!(eq_within(expected, sun_app_mag, REAL_DATA_TEST_ACCURACY));
    }

    #[test]
    fn test_sirius() {
        let sirius = absolute_magnitude_to_luminosity(1.43);

        let distance = Distance::from_lyr(8.6);
        let sirius_illuminance = luminosity_to_illuminance(&sirius, &distance);
        let sirius_app_mag = illuminance_to_apparent_magnitude(&sirius_illuminance);
        let expected = -1.46;
        println!(
            "Apparnet Magnitude:\nexpected: {},\nactual: {}",
            expected, sirius_app_mag
        );
        assert!(eq_within(expected, sirius_app_mag, REAL_DATA_TEST_ACCURACY));
    }
}
