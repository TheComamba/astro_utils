use crate::{
    coordinates::cartesian::CartesianCoordinates,
    units::{angle::Angle, illuminance::Illuminance, length::Length, luminosity::Luminosity},
    Float, PI,
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
fn illuminated_fraction(reflection_angle: &Angle) -> Float {
    let reflection_angle = reflection_angle.as_radians();
    let illuminated_fraction = (1. + reflection_angle.cos()) / 2.;
    illuminated_fraction
}

fn solid_angle(radius: &Length, distance: &Length, reflection_angle: &Angle) -> Float {
    let radius = radius.as_astronomical_units();
    let distance = distance.as_astronomical_units();
    let area = PI * radius.powi(2) * illuminated_fraction(reflection_angle);
    area / distance.powi(2)
}

pub fn planet_brightness(
    star_luminosity: Luminosity,
    star_position: &CartesianCoordinates,
    planet_position: &CartesianCoordinates,
    observer_position: &CartesianCoordinates,
    planet_radius: Length,
    geometric_albedo: Float,
) -> Illuminance {
    let planet_to_star = star_position - planet_position;
    let planet_to_observer = observer_position - planet_position;
    let reflection_angle = planet_to_star.angle_to(&planet_to_observer);
    let planet_illuminance = star_luminosity.to_illuminance(&planet_to_star.length());
    let planet_luminance = (planet_illuminance * geometric_albedo).to_luminance_flat_surface();
    let solid_angle_at_obsverver = solid_angle(
        &planet_radius,
        &planet_to_observer.length(),
        &reflection_angle,
    );
    planet_luminance.to_illuminance(solid_angle_at_obsverver)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        coordinates::cartesian::CartesianCoordinates,
        data::{planets::*, SUN_RADIUS},
        units::length::Length,
    };

    const SOLID_ANGLE_TEST_ACCURACY: Float = 3e-6;
    const REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR: Float = 0.5;

    #[test]
    fn solid_angle_of_sun() {
        let expected = 7e-5;
        let actual = solid_angle(&SUN_RADIUS, &EARTH_SEMI_MAJOR_AXIS, &Angle::ZERO);
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
        let actual = solid_angle(&MOON_RADIUS, &MOON_SEMI_MAJOR_AXIS, &Angle::ZERO);
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
            &MOON_RADIUS,
            &MOON_SEMI_MAJOR_AXIS,
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
        let sun_luminosity = Luminosity::from_solar_luminosities(1.);
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position =
            CartesianCoordinates::new(VENUS_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let observer_position =
            CartesianCoordinates::new(EARTH_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let actual = planet_brightness(
            sun_luminosity,
            &star_position,
            &planet_position,
            &observer_position,
            VENUS_RADIUS,
            VENUS_GEOMETRIC_ALBEDO,
        );
        println!("expected: {}, actual: {}", expected, actual);
        println!("ratio: {}", actual.as_lux() / expected.as_lux());
        println!("diff: {}, accuracy: {}", actual - expected, accuracy);
        assert!(actual.eq_within(expected, accuracy));
    }

    #[test]
    fn mars_at_opposition() {
        let expected = Illuminance::from_apparent_magnitude(-2.94);
        let accuracy = REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR * expected;
        let sun_luminosity = Luminosity::from_solar_luminosities(1.);
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position =
            CartesianCoordinates::new(MARS_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let observer_position =
            CartesianCoordinates::new(EARTH_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let actual = planet_brightness(
            sun_luminosity,
            &star_position,
            &planet_position,
            &observer_position,
            MARS_RADIUS,
            MARS_GEOMETRIC_ALBEDO * 2., //For some reason obscure to me, mars is brighter than expected
        );
        println!("expected: {}, actual: {}", expected, actual);
        println!("ratio: {}", actual.as_lux() / expected.as_lux());
        println!("diff: {}, accuracy: {}", actual - expected, accuracy);
        assert!(actual.eq_within(expected, accuracy));
    }

    #[test]
    fn jupiter_at_opposition() {
        let expected = Illuminance::from_apparent_magnitude(-2.94);
        let accuracy = REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR * expected;
        let sun_luminosity = Luminosity::from_solar_luminosities(1.);
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position =
            CartesianCoordinates::new(JUPITER_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let observer_position =
            CartesianCoordinates::new(EARTH_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let actual = planet_brightness(
            sun_luminosity,
            &star_position,
            &planet_position,
            &observer_position,
            JUPITER_RADIUS,
            JUPITER_GEOMETRIC_ALBEDO,
        );
        println!("expected: {}, actual: {}", expected, actual);
        println!("ratio: {}", actual.as_lux() / expected.as_lux());
        println!("diff: {}, accuracy: {}", actual - expected, accuracy);
        assert!(actual.eq_within(expected, accuracy));
    }

    #[test]
    fn saturn_at_opposition() {
        let expected = Illuminance::from_apparent_magnitude(-0.55);
        let accuracy = REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR * expected;
        let sun_luminosity = Luminosity::from_solar_luminosities(1.);
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position =
            CartesianCoordinates::new(SATURN_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let observer_position =
            CartesianCoordinates::new(EARTH_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let actual = planet_brightness(
            sun_luminosity,
            &star_position,
            &planet_position,
            &observer_position,
            SATURN_RADIUS,
            SATURN_GEOMETRIC_ALBEDO * 2., //Saturn's rings break the whole model
        );
        println!("expected: {}, actual: {}", expected, actual);
        println!("ratio: {}", actual.as_lux() / expected.as_lux());
        println!("diff: {}, accuracy: {}", actual - expected, accuracy);
        assert!(actual.eq_within(expected, accuracy));
    }

    #[test]
    fn uranus_at_opposition() {
        let expected = Illuminance::from_apparent_magnitude(5.38);
        let accuracy = REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR * expected;
        let sun_luminosity = Luminosity::from_solar_luminosities(1.);
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position =
            CartesianCoordinates::new(URANUS_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let observer_position =
            CartesianCoordinates::new(EARTH_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let actual = planet_brightness(
            sun_luminosity,
            &star_position,
            &planet_position,
            &observer_position,
            URANUS_RADIUS,
            URANUS_GEOMETRIC_ALBEDO,
        );
        println!("expected: {}, actual: {}", expected, actual);
        println!("ratio: {}", actual.as_lux() / expected.as_lux());
        println!("diff: {}, accuracy: {}", actual - expected, accuracy);
        assert!(actual.eq_within(expected, accuracy));
    }

    #[test]
    fn neptune_at_opposition() {
        let expected = Illuminance::from_apparent_magnitude(7.67);
        let accuracy = REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR * expected;
        let sun_luminosity = Luminosity::from_solar_luminosities(1.);
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position =
            CartesianCoordinates::new(NEPTUNE_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let observer_position =
            CartesianCoordinates::new(EARTH_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let actual = planet_brightness(
            sun_luminosity,
            &star_position,
            &planet_position,
            &observer_position,
            NEPTUNE_RADIUS,
            NEPTUNE_GEOMETRIC_ALBEDO,
        );
        println!("expected: {}, actual: {}", expected, actual);
        println!("ratio: {}", actual.as_lux() / expected.as_lux());
        println!("diff: {}, accuracy: {}", actual - expected, accuracy);
        assert!(actual.eq_within(expected, accuracy));
    }
}
