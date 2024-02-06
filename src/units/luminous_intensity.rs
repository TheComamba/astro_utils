use super::illuminance::{apparent_magnitude_to_illuminance, illuminance_to_apparent_magnitude};
use simple_si_units::{
    base::{Distance, Luminosity},
    electromagnetic::Illuminance,
    geometry::SolidAngle,
};
use std::f64::consts::PI;

pub const LUMINOSITY_ZERO: Luminosity<f64> = Luminosity { cd: 0. };
pub const SOLAR_LUMINOUS_INTENSITY: Luminosity<f64> = Luminosity { cd: 2.98e27 };
//pub const SOLAR_LUMINOSITY: Luminosity<f64> = Luminosity { cd: 3.828e26 }; //Watts, not cd

pub fn absolute_magnitude_to_luminous_intensity(absolute_magnitude: f64) -> Luminosity<f64> {
    let ten_pc = Distance::from_parsec(10.);
    let illuminance = apparent_magnitude_to_illuminance(absolute_magnitude);
    illuminance_to_luminous_intensity(&illuminance, &ten_pc)
}

pub fn luminous_intensity_to_absolute_magnitude(luminous_intensity: Luminosity<f64>) -> f64 {
    let ten_pc = Distance::from_parsec(10.);
    let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &ten_pc);
    illuminance_to_apparent_magnitude(&illuminance)
}

pub fn luminous_intensity_to_illuminance(
    luminous_intensity: &Luminosity<f64>,
    distance: &Distance<f64>,
) -> Illuminance<f64> {
    luminous_intensity * SolidAngle::from_steradians(1.) / (distance * distance)
}

pub fn illuminance_to_luminous_intensity(
    illuminance: &Illuminance<f64>,
    distance: &Distance<f64>,
) -> Luminosity<f64> {
    illuminance * (distance * distance) / SolidAngle::from_steradians(4. * PI)
}

pub fn display_luminous_intensity(luminous_intensity: &Luminosity<f64>) -> String {
    format!("{:.2} cd", luminous_intensity.cd) //should be watts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{eq, eq_within};

    const REAL_DATA_TEST_ACCURACY: f64 = 0.05;
    const ILLUMINANCE_AT_UNIT_DISTANCE: f64 = 1.;

    #[test]
    fn illuminance_roundtrip() {
        for i in -10..10 {
            let input = i as f64;
            let luminous_intensity = Luminosity::from_cd(input);
            let distance = Distance::from_m(1.);
            let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &distance);
            let output = illuminance_to_luminous_intensity(&illuminance, &distance);
            assert!(eq(input, output.cd));
        }
    }

    #[test]
    fn absolute_magnitude_roundtrip() {
        for i in -10..10 {
            let input = i as f64;
            let luminous_intensity = absolute_magnitude_to_luminous_intensity(input);
            let output = luminous_intensity_to_absolute_magnitude(luminous_intensity);
            assert!(eq(input, output));
        }
    }

    #[test]
    fn illuminance_of_1_cd_source_at_1_m() {
        let luminous_intensity = Luminosity::from_cd(1.);
        let distance = Distance::from_m(1.);
        let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &distance);
        let actual = illuminance.to_lux();
        let expected = ILLUMINANCE_AT_UNIT_DISTANCE;
        assert!(eq(actual, expected));
    }

    #[test]
    fn illuminance_is_proportional_to_luminous_intensity() {
        for i in 1..10 {
            let cd = i as f64;
            let luminous_intensity = Luminosity::from_cd(cd);
            let distance = Distance::from_m(1.);
            let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &distance);
            let expected = cd * ILLUMINANCE_AT_UNIT_DISTANCE;
            let actual = illuminance.to_lux();
            assert!(eq(actual, expected));
        }
    }

    #[test]
    fn illuminance_is_inversely_proportional_to_distance_squared() {
        for d in 1..10 {
            let distance = Distance::from_m(d as f64);
            let luminous_intensity = Luminosity::from_cd(1.);
            let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &distance);
            let expected = ILLUMINANCE_AT_UNIT_DISTANCE / (d * d) as f64;
            let actual = illuminance.to_lux();
            assert!(eq(actual, expected));
        }
    }

    #[test]
    fn apparent_and_absolute_magnitude_at_ten_parsecs_are_equal() {
        let ten_pc = Distance::from_parsec(10.);
        for i in -10..10 {
            let input = i as f64;
            let luminous_intensity = absolute_magnitude_to_luminous_intensity(input);
            let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &ten_pc);
            let apparent_magnitude = illuminance_to_apparent_magnitude(&illuminance);
            let absolute_magnitude = luminous_intensity_to_absolute_magnitude(luminous_intensity);
            assert!(eq(apparent_magnitude, absolute_magnitude));
        }
    }

    #[test]
    fn test_the_sun() {
        let sun_abs_mag = luminous_intensity_to_absolute_magnitude(SOLAR_LUMINOUS_INTENSITY);
        let expected = 4.83;
        assert!(eq_within(sun_abs_mag, expected, REAL_DATA_TEST_ACCURACY));
    }

    #[test]
    fn test_sirius() {
        let sun_abs_mag = luminous_intensity_to_absolute_magnitude(25.4 * SOLAR_LUMINOUS_INTENSITY);
        let expected = 1.43;
        assert!(eq_within(sun_abs_mag, expected, REAL_DATA_TEST_ACCURACY));
    }
}
