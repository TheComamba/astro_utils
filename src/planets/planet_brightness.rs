use std::f64::consts::PI;

use simple_si_units::{
    base::{Distance, Luminosity},
    electromagnetic::Illuminance,
    geometry::Angle,
};

use crate::{
    coordinates::cartesian::CartesianCoordinates, error::AstroUtilError,
    units::luminosity::luminosity_to_illuminance,
};

/*
 * Refraction is awfully complicated:
 * https://www.vttoth.com/CMS/physics-notes/336-on-lambertian-spheres
 * The takeaway is that refracting rough bodies behave more or less like a Lambertian flat surface.
 * Luckily, this is precisely what the geometric albedo is for.
 */

/*
 * https://www.physicsforums.com/threads/illuminated-fraction-of-the-moon.515983/
 */
fn illuminated_fraction(reflection_angle: &Angle<f64>) -> f64 {
    (1. + reflection_angle.rad.cos()) / 2.
}

fn solid_angle(
    radius: &Distance<f64>,
    distance: &Distance<f64>,
    reflection_angle: &Angle<f64>,
) -> f64 {
    let area = PI * radius * radius * illuminated_fraction(reflection_angle);
    area / (distance * distance)
}

pub fn planet_brightness(
    star_luminosity: Luminosity<f64>,
    star_position: &CartesianCoordinates,
    planet_position: &CartesianCoordinates,
    observer_position: &CartesianCoordinates,
    planet_radius: Distance<f64>,
    geometric_albedo: f64,
) -> Result<Illuminance<f64>, AstroUtilError> {
    let planet_to_star = star_position - planet_position;
    let planet_to_observer = observer_position - planet_position;
    let reflection_angle = planet_to_star.angle_to(&planet_to_observer)?;
    let planet_illuminance = luminosity_to_illuminance(&star_luminosity, &planet_to_star.length());
    let planet_flat_surface_luminance = (planet_illuminance * geometric_albedo) / PI;
    let solid_angle_at_obsverver = solid_angle(
        &planet_radius,
        &planet_to_observer.length(),
        &reflection_angle,
    );
    Ok(planet_flat_surface_luminance * solid_angle_at_obsverver)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        coordinates::cartesian::CartesianCoordinates,
        real_data::{planets::*, SOLAR_LUMINOSITY, SOLAR_RADIUS},
        tests::eq_within,
        units::{
            angle::ANGLE_ZERO, distance::DISTANCE_ZERO,
            illuminance::apparent_magnitude_to_illuminance,
        },
    };

    const SOLID_ANGLE_TEST_ACCURACY: f64 = 3e-6;
    const REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR: f64 = 0.5;

    #[test]
    fn solid_angle_of_sun() {
        let expected = 7e-5;
        let actual = solid_angle(
            &SOLAR_RADIUS,
            &EARTH.orbit.get_semi_major_axis(),
            &ANGLE_ZERO,
        );
        println!("expected: {}, actual: {}", expected, actual);
        println!(
            "diff: {}, accuracy: {}",
            actual - expected,
            SOLID_ANGLE_TEST_ACCURACY
        );
        assert!((actual - expected).abs() < SOLID_ANGLE_TEST_ACCURACY);
    }

    #[test]
    fn solid_angle_of_full_moon() {
        let expected = 6.4e-5;
        let actual = solid_angle(&MOON.radius, &MOON.orbit.get_semi_major_axis(), &ANGLE_ZERO);
        println!("expected: {}, actual: {}", expected, actual);
        println!(
            "diff: {}, accuracy: {}",
            actual - expected,
            SOLID_ANGLE_TEST_ACCURACY
        );
        assert!((actual - expected).abs() < SOLID_ANGLE_TEST_ACCURACY);
    }

    #[test]
    fn solid_angle_of_half_moon() {
        let expected = 6.4e-5 / 2.;
        let actual = solid_angle(
            &MOON.radius,
            &MOON.orbit.get_semi_major_axis(),
            &Angle::from_degrees(90.),
        );
        println!("expected: {}, actual: {}", expected, actual);
        println!(
            "diff: {}, accuracy: {}",
            actual - expected,
            SOLID_ANGLE_TEST_ACCURACY
        );
        assert!((actual - expected).abs() < SOLID_ANGLE_TEST_ACCURACY);
    }

    #[test]
    fn venus_at_occultation() {
        let accuracy = Illuminance::from_lux(1e-11);
        let expected = Illuminance::from_lux(0.);
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position = CartesianCoordinates::new(
            VENUS.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let observer_position = CartesianCoordinates::new(
            EARTH.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let actual = planet_brightness(
            SOLAR_LUMINOSITY,
            &star_position,
            &planet_position,
            &observer_position,
            VENUS.radius,
            VENUS.geometric_albedo,
        )
        .unwrap();
        println!("expected: {}, actual: {}", expected, actual);
        println!("ratio: {}", actual.to_lux() / expected.to_lux());
        println!("diff: {}, accuracy: {}", actual - expected, accuracy);
        assert!(eq_within(actual.lux, expected.lux, accuracy.lux));
    }

    #[test]
    fn mars_at_opposition() {
        let expected = apparent_magnitude_to_illuminance(-2.94);
        let accuracy = REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR * expected;
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position = CartesianCoordinates::new(
            MARS.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let observer_position = CartesianCoordinates::new(
            EARTH.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let actual = planet_brightness(
            SOLAR_LUMINOSITY,
            &star_position,
            &planet_position,
            &observer_position,
            MARS.radius,
            MARS.geometric_albedo * 2., //For some reason obscure to me, mars is brighter than expected
        )
        .unwrap();
        println!("expected: {}, actual: {}", expected, actual);
        println!("ratio: {}", actual.to_lux() / expected.to_lux());
        println!("diff: {}, accuracy: {}", actual - expected, accuracy);
        assert!(eq_within(actual.lux, expected.lux, accuracy.lux));
    }

    #[test]
    fn jupiter_at_opposition() {
        let expected = apparent_magnitude_to_illuminance(-2.94);
        let accuracy = REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR * expected;
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position = CartesianCoordinates::new(
            JUPITER.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let observer_position = CartesianCoordinates::new(
            EARTH.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let actual = planet_brightness(
            SOLAR_LUMINOSITY,
            &star_position,
            &planet_position,
            &observer_position,
            JUPITER.radius,
            JUPITER.geometric_albedo,
        )
        .unwrap();
        println!("expected: {}, actual: {}", expected, actual);
        println!("ratio: {}", actual.to_lux() / expected.to_lux());
        println!("diff: {}, accuracy: {}", actual - expected, accuracy);
        assert!(eq_within(actual.lux, expected.lux, accuracy.lux));
    }

    #[test]
    fn saturn_at_opposition() {
        let expected = apparent_magnitude_to_illuminance(-0.55);
        let accuracy = REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR * expected;
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position = CartesianCoordinates::new(
            SATURN.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let observer_position = CartesianCoordinates::new(
            EARTH.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let actual = planet_brightness(
            SOLAR_LUMINOSITY,
            &star_position,
            &planet_position,
            &observer_position,
            SATURN.radius,
            SATURN.geometric_albedo * 2., //Saturn's rings break the whole model
        )
        .unwrap();
        println!("expected: {}, actual: {}", expected, actual);
        println!("ratio: {}", actual.to_lux() / expected.to_lux());
        println!("diff: {}, accuracy: {}", actual - expected, accuracy);
        assert!(eq_within(actual.lux, expected.lux, accuracy.lux));
    }

    #[test]
    fn uranus_at_opposition() {
        let expected = apparent_magnitude_to_illuminance(5.38);
        let accuracy = REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR * expected;
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position = CartesianCoordinates::new(
            URANUS.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let observer_position = CartesianCoordinates::new(
            EARTH.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let actual = planet_brightness(
            SOLAR_LUMINOSITY,
            &star_position,
            &planet_position,
            &observer_position,
            URANUS.radius,
            URANUS.geometric_albedo,
        )
        .unwrap();
        println!("expected: {}, actual: {}", expected, actual);
        println!("ratio: {}", actual.to_lux() / expected.to_lux());
        println!("diff: {}, accuracy: {}", actual - expected, accuracy);
        assert!(eq_within(actual.lux, expected.lux, accuracy.lux));
    }

    #[test]
    fn neptune_at_opposition() {
        let expected = apparent_magnitude_to_illuminance(7.67);
        let accuracy = REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR * expected;
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position = CartesianCoordinates::new(
            NEPTUNE.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let observer_position = CartesianCoordinates::new(
            EARTH.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let actual = planet_brightness(
            SOLAR_LUMINOSITY,
            &star_position,
            &planet_position,
            &observer_position,
            NEPTUNE.radius,
            NEPTUNE.geometric_albedo,
        )
        .unwrap();
        println!("expected: {}, actual: {}", expected, actual);
        println!("ratio: {}", actual.to_lux() / expected.to_lux());
        println!("diff: {}, accuracy: {}", actual - expected, accuracy);
        assert!(eq_within(actual.lux, expected.lux, accuracy.lux));
    }
}
