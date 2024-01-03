use crate::{
    coordinates::cartesian::CartesianCoordinates,
    units::{
        angle::Angle, illuminance::Illuminance, length::Length,
        luminosity::Luminosity,
    },
    Float, PI,
};

/*
 * illuminated_area = area of circle = PI * planet_radius^2;
 * luminating_area = half area of sphere = 4 * PI * planet_radius^2 / 2;
 */
const LUMINATING_AREA_PER_ILLUMINATED_AREA: Float = 0.5;

fn solid_angle(radius: &Length, distance: &Length, reflection_angle: &Angle) -> Angle {
    let apparent_radius = radius / distance;
    let apparent_area = PI * apparent_radius * apparent_radius * reflection_angle.cos();
    let solid_angle = apparent_area / (4. * PI);
    Angle::from_radians(solid_angle)
}

pub fn planet_brightness(
    star_luminosity: Luminosity,
    star_position: &CartesianCoordinates,
    planet_position: &CartesianCoordinates,
    observer_position: &CartesianCoordinates,
    planet_radius: Length,
    planet_albedo: Float,
) -> Illuminance {
    let planet_to_star = star_position - planet_position;
    let planet_to_observer = observer_position - planet_position;
    let reflection_angle = planet_to_star.angle_to(&planet_to_observer);
    let planet_illuminance = star_luminosity.to_illuminance(&planet_to_star.length());
    let planet_luminance =
        (planet_illuminance * LUMINATING_AREA_PER_ILLUMINATED_AREA * planet_albedo).to_luminance();
    let solid_angle_at_obsverver = solid_angle(
        &planet_radius,
        &planet_to_observer.length(),
        &reflection_angle,
    );
    planet_luminance.to_illuminance(&solid_angle_at_obsverver)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        coordinates::cartesian::CartesianCoordinates, data::planets::*, units::length::Length,
    };

    const REAL_DATA_TEST_ACCURACY: f32 = 0.05;

    #[test]
    fn jupiter_at_opposition() {
        let sun_luminosity = Luminosity::from_solar_luminosities(1.);
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position =
            CartesianCoordinates::new(JUPITER_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let observer_position =
            CartesianCoordinates::new(EARTH_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let expected = -2.94;
        let actual = planet_brightness(
            sun_luminosity,
            &star_position,
            &planet_position,
            &observer_position,
            JUPITER_RADIUS,
            JUPITER_BOND_ALBEDO,
        )
        .as_apparent_magnitude();
        println!("expected: {}, actual: {}", expected, actual);
        assert!((actual - expected).abs() < REAL_DATA_TEST_ACCURACY);
    }

    #[test]
    fn venus_at_greatest_elongation() {
        let sun_luminosity = Luminosity::from_solar_luminosities(1.);
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position =
            CartesianCoordinates::new(Length::ZERO, VENUS_SEMI_MAJOR_AXIS, Length::ZERO);
        let observer_position =
            CartesianCoordinates::new(EARTH_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let expected = -4.92;
        let actual = planet_brightness(
            sun_luminosity,
            &star_position,
            &planet_position,
            &observer_position,
            VENUS_RADIUS,
            VENUS_BOND_ALBEDO,
        )
        .as_apparent_magnitude();
        println!("expected: {}, actual: {}", expected, actual);
        assert!((actual - expected).abs() < REAL_DATA_TEST_ACCURACY);
    }

    #[test]
    fn venus_at_occultation() {
        let sun_luminosity = Luminosity::from_solar_luminosities(1.);
        let star_position = CartesianCoordinates::ORIGIN;
        let planet_position =
            CartesianCoordinates::new(VENUS_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let observer_position =
            CartesianCoordinates::new(EARTH_SEMI_MAJOR_AXIS, Length::ZERO, Length::ZERO);
        let expected = 0.;
        let actual = planet_brightness(
            sun_luminosity,
            &star_position,
            &planet_position,
            &observer_position,
            VENUS_RADIUS,
            VENUS_BOND_ALBEDO,
        )
        .get_lux();
        println!("expected: {}, actual: {}", expected, actual);
        assert!((actual - expected).abs() < REAL_DATA_TEST_ACCURACY);
    }
}
