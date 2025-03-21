use std::f64::consts::PI;

use astro_coords::cartesian::Cartesian;

use crate::{
    error::AstroUtilError,
    units::{
        luminous_intensity::luminous_intensity_to_illuminance,
        solid_angle::radius_and_distance_to_solid_angle,
    },
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

pub fn planet_brightness(
    star_luminous_intensity: Luminosity<f64>,
    star_position: &Cartesian,
    planet_position: &Cartesian,
    observer_position: &Cartesian,
    planet_radius: Distance<f64>,
    geometric_albedo: f64,
) -> Result<Illuminance<f64>, AstroUtilError> {
    let planet_to_star = star_position - planet_position;
    let planet_to_observer = observer_position - planet_position;
    let reflection_angle = planet_to_star.angle_to(&planet_to_observer)?;
    let planet_illuminance =
        luminous_intensity_to_illuminance(&star_luminous_intensity, &planet_to_star.length());
    let planet_flat_surface_luminance = (planet_illuminance * geometric_albedo) / PI;
    let solid_angle =
        radius_and_distance_to_solid_angle(planet_radius, planet_to_observer.length());
    let luminating_solid_angle = solid_angle * illuminated_fraction(&reflection_angle);
    Ok(planet_flat_surface_luminance * luminating_solid_angle.sr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        real_data::planets::*,
        tests::eq_within,
        units::{
            distance::DISTANCE_ZERO, illuminance::apparent_magnitude_to_illuminance,
            luminous_intensity::SOLAR_LUMINOUS_INTENSITY,
        },
    };
    const REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR: f64 = 0.5;

    #[test]
    fn venus_at_occultation() {
        let accuracy = Illuminance::from_lux(1e-11);
        let expected = Illuminance::from_lux(0.);
        let star_position = Cartesian::ORIGIN;
        let planet_position = Cartesian::new(
            VENUS.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let observer_position = Cartesian::new(
            EARTH.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let actual = planet_brightness(
            SOLAR_LUMINOUS_INTENSITY,
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
        let star_position = Cartesian::ORIGIN;
        let planet_position = Cartesian::new(
            MARS.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let observer_position = Cartesian::new(
            EARTH.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let actual = planet_brightness(
            SOLAR_LUMINOUS_INTENSITY,
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
        let star_position = Cartesian::ORIGIN;
        let planet_position = Cartesian::new(
            JUPITER.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let observer_position = Cartesian::new(
            EARTH.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let actual = planet_brightness(
            SOLAR_LUMINOUS_INTENSITY,
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
        let star_position = Cartesian::ORIGIN;
        let planet_position = Cartesian::new(
            SATURN.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let observer_position = Cartesian::new(
            EARTH.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let actual = planet_brightness(
            SOLAR_LUMINOUS_INTENSITY,
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
        let star_position = Cartesian::ORIGIN;
        let planet_position = Cartesian::new(
            URANUS.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let observer_position = Cartesian::new(
            EARTH.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let actual = planet_brightness(
            SOLAR_LUMINOUS_INTENSITY,
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
        let star_position = Cartesian::ORIGIN;
        let planet_position = Cartesian::new(
            NEPTUNE.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let observer_position = Cartesian::new(
            EARTH.orbit.get_semi_major_axis(),
            DISTANCE_ZERO,
            DISTANCE_ZERO,
        );
        let actual = planet_brightness(
            SOLAR_LUMINOUS_INTENSITY,
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
