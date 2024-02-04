use super::orbit_parameters::OrbitParameters;
use crate::{
    coordinates::{cartesian::CartesianCoordinates, spherical::SphericalCoordinates},
    units::angle::{ANGLE_ZERO, TWO_PI},
};
use simple_si_units::{
    base::{Distance, Mass, Time},
    geometry::Angle,
};

/*
 * The orbital period is the time it takes for a given object to make one full orbit around another object.
 * https://en.wikipedia.org/wiki/Orbital_period
 */
pub fn orbital_period(
    semi_major_axis: Distance<f64>,
    mass1: Mass<f64>,
    mass2: Mass<f64>,
) -> Time<f64> {
    const G: f64 = 6.67430e-11;

    let semi_major_axis_cubed = semi_major_axis * semi_major_axis * semi_major_axis;
    let total_mass = mass1 + mass2;
    let orbital_period = TWO_PI * ((semi_major_axis_cubed / total_mass).m3_per_kg / G).sqrt();
    Time::from_seconds(orbital_period)
}

/*
 * The mean anomaly is the angle between the direction of periapsis and the current position of the orbiting body,
 * as seen from the center of the ellipse (the point around which the object orbits).
 * https://en.wikipedia.org/wiki/Mean_anomaly
 *
 * Output is normalised to the range [-π, π].
 */
pub fn mean_anomaly(orbital_period: Time<f64>, time: Time<f64>) -> Angle<f64> {
    let mean_motion = TWO_PI / orbital_period;
    let mean_anomaly = mean_motion * (time % orbital_period);
    mean_anomaly.normalize();
    mean_anomaly
}

/*
 * The eccentric anomaly is the angle between the direction of periapsis and the current position of the orbiting body,
 * as seen from the center of the ellipse (the point around which the object orbits).
 * https://en.wikipedia.org/wiki/Eccentric_anomaly
 */
pub fn eccentric_anomaly(mean_anomaly: Angle<f64>, eccentricity: f64) -> Angle<f64> {
    const ACCURACY: f64 = 1e-6;
    let mean_anomaly = mean_anomaly.to_radians();
    let mut eccentric_anomaly = mean_anomaly;
    let mut error = 10. * ACCURACY;
    while error > ACCURACY {
        let numerator = eccentric_anomaly - eccentricity * eccentric_anomaly.sin() - mean_anomaly;
        let denominator = 1. - eccentricity * eccentric_anomaly.cos();
        let next_correction = numerator / denominator;
        let next_eccentric_anomaly = eccentric_anomaly - next_correction;
        error = next_correction.abs();
        eccentric_anomaly = next_eccentric_anomaly;
    }

    Angle::from_radians(eccentric_anomaly)
}

/*
 * The true anomaly is the angle between the direction of periapsis and the current position of the orbiting body,
 * as seen from the main focus of the ellipse (the point around which the object orbits).
 * https://en.wikipedia.org/wiki/True_anomaly
 */
pub fn true_anomaly(eccentric_anomaly: Angle<f64>, eccentricity: f64) -> Angle<f64> {
    let sqrt_arg = (1. + eccentricity) / (1. - eccentricity);
    let artan_arg = (eccentric_anomaly.to_radians() / 2.).tan() * sqrt_arg.sqrt();
    let true_anomaly = 2. * artan_arg.atan();
    Angle::from_radians(true_anomaly)
}

/*
 * The distance from the focus is the distance between the orbiting body and the main focus of the ellipse
 * (the point around which the object orbits).
 * https://en.wikipedia.org/wiki/Ellipse#Distance_from_focus
 */
fn distance_from_focus(
    semi_major_axis: Distance<f64>,
    true_anomaly: Angle<f64>,
    eccentricity: f64,
) -> Distance<f64> {
    let numerator = 1. - eccentricity * eccentricity;
    let denominator = 1. + eccentricity * true_anomaly.cos();
    semi_major_axis * numerator / denominator
}

/*
 * The position relative to the central body is the position of the orbiting body relative to the central body.
 * https://en.wikipedia.org/wiki/Orbital_elements#Position_relative_to_the_central_body
 */
pub fn position_relative_to_central_body(
    semi_major_axis: Distance<f64>,
    eccentricity: f64,
    true_anomaly: Angle<f64>,
    orientation: &OrbitParameters,
) -> CartesianCoordinates {
    let ecliptic_from_focus = SphericalCoordinates::new(true_anomaly, ANGLE_ZERO);
    let direction = ecliptic_from_focus.to_direction();
    let distance_from_focus = distance_from_focus(semi_major_axis, true_anomaly, eccentricity);
    let position = direction.to_cartesian(distance_from_focus);
    orientation.apply_orientation_to(position)
}

#[cfg(test)]
mod tests {
    use crate::{
        coordinates::cartesian::CartesianCoordinates,
        real_data::{planets::*, SUN_MASS},
        tests::{eq, eq_within},
        units::distance::DISTANCE_ZERO,
    };

    use super::*;

    #[test]
    fn orbital_period_of_earth() {
        let expected_orbital_period = Time::from_days(365.256);
        let earth_semi_major_axis = EARTH.orbit.get_semi_major_axis();
        let orbital_period = orbital_period(earth_semi_major_axis, EARTH.mass, SUN_MASS);
        println!("Expected orbital period: {}", expected_orbital_period);
        println!("Calculated orbital period: {}", orbital_period);
        assert!(orbital_period.eq_within(expected_orbital_period, 1e-3 * expected_orbital_period));
    }

    #[test]
    fn orbital_period_of_jupiter() {
        let expected_orbital_period = Time::from_days(4332.59);
        let jupiter_semi_major_axis = JUPITER.orbit.get_semi_major_axis();
        let orbital_period = orbital_period(jupiter_semi_major_axis, JUPITER.mass, SUN_MASS);
        println!("Expected orbital period: {}", expected_orbital_period);
        println!("Calculated orbital period: {}", orbital_period);
        assert!(orbital_period.eq_within(expected_orbital_period, 1e-3 * expected_orbital_period));
    }

    #[test]
    fn orbital_period_of_moon() {
        let expected_orbital_period = Time::from_days(27.321);
        let moon_semi_major_axis = MOON.orbit.get_semi_major_axis();
        let orbital_period = orbital_period(moon_semi_major_axis, MOON.mass, EARTH.mass);
        println!("Expected orbital period: {}", expected_orbital_period);
        println!("Calculated orbital period: {}", orbital_period);
        assert!(orbital_period.eq_within(expected_orbital_period, 9e-3 * expected_orbital_period));
    }

    #[test]
    fn mean_anomaly_a_quarter_circle() {
        let expected_mean_anomaly = Angle::from_radians(TWO_PI / 4.);
        let mean_anomaly = mean_anomaly(Time::from_years(4.), Time::from_years(1.));
        println!("Expected mean anomaly: {}", expected_mean_anomaly);
        println!("Calculated mean anomaly: {}", mean_anomaly);
        assert!(eq(mean_anomaly, expected_mean_anomaly));
    }

    #[test]
    fn mean_anomaly_a_half_circle() {
        let expected_mean_anomaly = Angle::from_radians(TWO_PI / 2.);
        let mean_anomaly = mean_anomaly(Time::from_years(4.), Time::from_years(2.));
        println!("Expected mean anomaly: {}", expected_mean_anomaly);
        println!("Calculated mean anomaly: {}", mean_anomaly);
        assert!(eq(mean_anomaly, expected_mean_anomaly));
    }

    #[test]
    fn mean_anomaly_three_quarters_circle() {
        let expected_mean_anomaly = Angle::from_radians(TWO_PI * 3. / 4.);
        let mean_anomaly = mean_anomaly(Time::from_years(4.), Time::from_years(-1.));
        println!("Expected mean anomaly: {}", expected_mean_anomaly);
        println!("Calculated mean anomaly: {}", mean_anomaly);
        assert!(eq(mean_anomaly, expected_mean_anomaly));
    }

    #[test]
    fn mean_anomaly_is_stable_after_loads_of_revolutions() {
        const LOCAL_TEST_ANGLE_ACCURACY: Angle<f64> = Angle { rad: 5e-3 * TWO_PI };

        let expected_mean_anomaly = Angle::from_radians(TWO_PI / 4.);
        let passed_time = Time::from_years(1e5 + 0.25);
        let mean_anomaly = mean_anomaly(Time::from_years(1.), passed_time);
        println!("Expected mean anomaly: {}", expected_mean_anomaly);
        println!("Calculated mean anomaly: {}", mean_anomaly);
        assert!(eq_within(
            mean_anomaly,
            expected_mean_anomaly,
            LOCAL_TEST_ANGLE_ACCURACY
        ));
    }

    #[test]
    fn eccentric_anomaly_from_quarter_circle_mean_anomaly_and_zero_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_radians(TWO_PI / 4.);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(TWO_PI / 4.), 0.);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn eccentric_anomaly_from_half_circle_mean_anomaly_and_zero_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_radians(TWO_PI / 2.);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(TWO_PI / 2.), 0.);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn eccentric_anomaly_from_three_quarters_circle_mean_anomaly_and_zero_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_radians(TWO_PI * 3. / 4.);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(TWO_PI * 3. / 4.), 0.);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn eccentric_anomaly_from_negative_quarter_circle_mean_anomaly_and_zero_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_radians(-TWO_PI / 4.);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(-TWO_PI / 4.), 0.);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn eccentric_anomaly_from_quarter_circle_mean_anomaly_and_half_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_degrees(115.79362093315422);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(TWO_PI / 4.), 0.5);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn eccentric_anomaly_from_half_circle_mean_anomaly_and_half_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_radians(TWO_PI / 2.);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(TWO_PI / 2.), 0.5);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn eccentric_anomaly_from_three_quarters_circle_mean_anomaly_and_half_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_degrees(-115.79362093315422);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(TWO_PI * 3. / 4.), 0.5);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn eccentric_anomaly_from_negative_quarter_circle_mean_anomaly_and_half_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_degrees(-115.79362093315422);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(-TWO_PI / 4.), 0.5);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn true_anomaly_from_quarter_circle_eccentric_anomaly_and_zero_eccentricity() {
        let expected_true_anomaly = Angle::from_radians(TWO_PI / 4.);
        let true_anomaly = true_anomaly(Angle::from_radians(TWO_PI / 4.), 0.);
        println!("Expected true anomaly: {}", expected_true_anomaly);
        println!("Calculated true anomaly: {}", true_anomaly);
        assert!(eq(true_anomaly, expected_true_anomaly));
    }

    #[test]
    fn true_anomaly_from_half_circle_eccentric_anomaly_and_zero_eccentricity() {
        let expected_true_anomaly = Angle::from_radians(TWO_PI / 2.);
        let true_anomaly = true_anomaly(Angle::from_radians(TWO_PI / 2.), 0.);
        println!("Expected true anomaly: {}", expected_true_anomaly);
        println!("Calculated true anomaly: {}", true_anomaly);
        assert!(eq(true_anomaly, expected_true_anomaly));
    }

    #[test]
    fn true_anomaly_from_three_quarters_circle_eccentric_anomaly_and_zero_eccentricity() {
        let expected_true_anomaly = Angle::from_radians(TWO_PI * 3. / 4.);
        let true_anomaly = true_anomaly(Angle::from_radians(TWO_PI * 3. / 4.), 0.);
        println!("Expected true anomaly: {}", expected_true_anomaly);
        println!("Calculated true anomaly: {}", true_anomaly);
        assert!(eq(true_anomaly, expected_true_anomaly));
    }

    #[test]
    fn distance_from_focus_for_eccentric_ellipse() {
        let semi_major_axis = Distance::from_meters(2.);
        let semi_minor_axis = Distance::from_meters(1.);
        let eccentricity = (1. - (semi_minor_axis / semi_major_axis).powi(2)).sqrt();
        let linear_eccentricity = semi_major_axis * eccentricity;
        let focal_point =
            CartesianCoordinates::new(linear_eccentricity, DISTANCE_ZERO, DISTANCE_ZERO);

        let eccentric_anom = ANGLE_ZERO;
        let true_anom = true_anomaly(eccentric_anom, eccentricity);
        let point = CartesianCoordinates::new(semi_major_axis, DISTANCE_ZERO, DISTANCE_ZERO);
        let expected = focal_point.distance(&point);
        let actual = distance_from_focus(semi_major_axis, true_anom, eccentricity);
        println!("Expected distance from focus: {}", expected);
        println!("Calculated distance from focus: {}", actual);
        assert!(eq(actual, &expected));

        let eccentric_anom = Angle::from_radians(TWO_PI / 4.);
        let true_anom = true_anomaly(eccentric_anom, eccentricity);
        let point = CartesianCoordinates::new(DISTANCE_ZERO, semi_minor_axis, DISTANCE_ZERO);
        let expected = focal_point.distance(&point);
        let actual = distance_from_focus(semi_major_axis, true_anom, eccentricity);
        println!("Expected distance from focus: {}", expected);
        println!("Calculated distance from focus: {}", actual);
        assert!(eq(actual, &expected));

        let eccentric_anom = Angle::from_radians(TWO_PI / 2.);
        let true_anom = true_anomaly(eccentric_anom, eccentricity);
        let point = CartesianCoordinates::new(-semi_major_axis, DISTANCE_ZERO, DISTANCE_ZERO);
        let expected = focal_point.distance(&point);
        let actual = distance_from_focus(semi_major_axis, true_anom, eccentricity);
        println!("Expected distance from focus: {}", expected);
        println!("Calculated distance from focus: {}", actual);
        assert!(eq(actual, &expected));

        let eccentric_anom = Angle::from_radians(TWO_PI * 3. / 4.);
        let true_anom = true_anomaly(eccentric_anom, eccentricity);
        let point = CartesianCoordinates::new(DISTANCE_ZERO, -semi_minor_axis, DISTANCE_ZERO);
        let expected = focal_point.distance(&point);
        let actual = distance_from_focus(semi_major_axis, true_anom, eccentricity);
        println!("Expected distance from focus: {}", expected);
        println!("Calculated distance from focus: {}", actual);
        assert!(eq(actual, &expected));
    }
}
