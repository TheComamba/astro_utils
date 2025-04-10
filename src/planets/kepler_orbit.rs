use std::f64::consts::PI;

use astro_coords::{cartesian::Cartesian, spherical::Spherical};
use uom::si::{
    angle::radian,
    f64::{Angle, Length, Mass, Time},
    specific_volume::cubic_meter_per_kilogram,
    time::second,
};

use crate::units::angle::{full_circ, normalized_angle};

use super::orbit_parameters::OrbitParameters;

pub(crate) const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;

/*
 * The orbital period is the time it takes for a given object to make one full orbit around another object.
 * https://en.wikipedia.org/wiki/Orbital_period
 */
pub fn orbital_period(semi_major_axis: Length, mass1: Mass, mass2: Mass) -> Time {
    let semi_major_axis_cubed = semi_major_axis * semi_major_axis * semi_major_axis;
    let total_mass = mass1 + mass2;
    let orbital_period = 2.
        * PI
        * ((semi_major_axis_cubed / total_mass).get::<cubic_meter_per_kilogram>()
            / GRAVITATIONAL_CONSTANT)
            .sqrt();
    Time::new::<second>(orbital_period)
}

/*
 * The mean anomaly is the angle between the direction of periapsis and the current position of the orbiting body,
 * as seen from the center of the ellipse (the point around which the object orbits).
 * https://en.wikipedia.org/wiki/Mean_anomaly
 *
 * Output is normalised to the range [-π, π].
 */
pub fn mean_anomaly(orbital_period: Time, time: Time) -> Angle {
    let mean_motion = full_circ() / orbital_period;
    let time_in_orbit = time % orbital_period;
    let mean_anomaly = mean_motion * time_in_orbit;
    normalized_angle(mean_anomaly.into())
}

/*
 * The eccentric anomaly is the angle between the direction of periapsis and the current position of the orbiting body,
 * as seen from the center of the ellipse (the point around which the object orbits).
 * https://en.wikipedia.org/wiki/Eccentric_anomaly
 */
pub fn eccentric_anomaly(mean_anomaly: Angle, eccentricity: f64) -> Angle {
    const ACCURACY: f64 = 1e-6;
    let mean_anomaly = mean_anomaly.get::<radian>();
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

    Angle::new::<radian>(eccentric_anomaly)
}

/*
 * The true anomaly is the angle between the direction of periapsis and the current position of the orbiting body,
 * as seen from the main focus of the ellipse (the point around which the object orbits).
 * https://en.wikipedia.org/wiki/True_anomaly
 */
pub fn true_anomaly(eccentric_anomaly: Angle, eccentricity: f64) -> Angle {
    let sqrt_arg = (1. + eccentricity) / (1. - eccentricity);
    let artan_arg = (eccentric_anomaly.get::<radian>() / 2.).tan() * sqrt_arg.sqrt();
    let true_anomaly = 2. * artan_arg.atan();
    Angle::new::<radian>(true_anomaly)
}

/*
 * The distance from the focus is the distance between the orbiting body and the main focus of the ellipse
 * (the point around which the object orbits).
 * https://en.wikipedia.org/wiki/Ellipse#Length_from_focus
 */
fn distance_from_focus(semi_major_axis: Length, true_anomaly: Angle, eccentricity: f64) -> Length {
    let numerator = 1. - eccentricity * eccentricity;
    let denominator = 1. + eccentricity * true_anomaly.get::<radian>().cos();
    semi_major_axis * numerator / denominator
}

/*
 * The position relative to the central body is the position of the orbiting body relative to the central body.
 * https://en.wikipedia.org/wiki/Orbital_elements#Position_relative_to_the_central_body
 */
pub fn position_relative_to_central_body(
    semi_major_axis: Length,
    eccentricity: f64,
    true_anomaly: Angle,
    orientation: &OrbitParameters,
) -> Cartesian {
    let ecliptic_from_focus = Spherical::new(true_anomaly, Angle::new::<radian>(0.));
    let direction = ecliptic_from_focus.to_direction();
    let distance_from_focus = distance_from_focus(semi_major_axis, true_anomaly, eccentricity);
    let position = direction.to_cartesian(distance_from_focus);
    orientation.apply_orientation_to(position)
}

#[cfg(test)]
mod tests {
    use uom::si::{
        angle::degree,
        length::{astronomical_unit, meter},
        time::{day, year},
    };

    use crate::{
        astro_display::AstroDisplay,
        real_data::planets::*,
        tests::{eq, eq_within},
        units::{angle::*, mass::solar_mass},
    };

    use super::*;

    #[test]
    fn orbital_period_of_earth() {
        let expected_orbital_period = Time::new::<day>(365.256);
        let earth_semi_major_axis = EARTH.orbit.get_semi_major_axis();
        let orbital_period = orbital_period(
            earth_semi_major_axis,
            EARTH.mass,
            Mass::new::<solar_mass>(1.),
        );
        println!(
            "Expected orbital period: {}",
            expected_orbital_period.astro_display()
        );
        println!(
            "Calculated orbital period: {}",
            orbital_period.astro_display()
        );
        assert!(eq_within(
            orbital_period.get::<year>(),
            expected_orbital_period.get::<year>(),
            1e-3
        ));
    }

    #[test]
    fn orbital_period_of_jupiter() {
        let expected_orbital_period = Time::new::<day>(4332.59);
        let jupiter_semi_major_axis = JUPITER.orbit.get_semi_major_axis();
        let orbital_period = orbital_period(
            jupiter_semi_major_axis,
            JUPITER.mass,
            Mass::new::<solar_mass>(1.),
        );
        println!(
            "Expected orbital period: {}",
            expected_orbital_period.astro_display()
        );
        println!(
            "Calculated orbital period: {}",
            orbital_period.astro_display()
        );
        assert!(eq_within(
            orbital_period.get::<year>(),
            expected_orbital_period.get::<year>(),
            1e-2
        ));
    }

    #[test]
    fn orbital_period_of_moon() {
        let expected_orbital_period = Time::new::<day>(27.321);
        let moon_semi_major_axis = luna.orbit.get_semi_major_axis();
        let orbital_period = orbital_period(moon_semi_major_axis, luna.mass, EARTH.mass);
        println!(
            "Expected orbital period: {}",
            expected_orbital_period.astro_display()
        );
        println!(
            "Calculated orbital period: {}",
            orbital_period.astro_display()
        );
        assert!(eq_within(
            orbital_period.get::<year>(),
            expected_orbital_period.get::<year>(),
            1e-3
        ));
    }

    #[test]
    fn mean_anomaly_a_quarter_circle() {
        let expected_mean_anomaly = quarter_circ();
        let mean_anomaly = mean_anomaly(Time::new::<year>(4.), Time::new::<year>(1.));
        println!(
            "Expected mean anomaly: {}",
            expected_mean_anomaly.astro_display()
        );
        println!("Calculated mean anomaly: {}", mean_anomaly.astro_display());
        assert!(angle_eq(mean_anomaly, expected_mean_anomaly));
    }

    #[test]
    fn mean_anomaly_a_half_circle() {
        let expected_mean_anomaly = half_circ();
        let mean_anomaly = mean_anomaly(Time::new::<year>(4.), Time::new::<year>(2.));
        println!(
            "Expected mean anomaly: {}",
            expected_mean_anomaly.astro_display()
        );
        println!("Calculated mean anomaly: {}", mean_anomaly.astro_display());
        assert!(angle_eq(mean_anomaly, expected_mean_anomaly));
    }

    #[test]
    fn mean_anomaly_three_quarters_circle() {
        let expected_mean_anomaly = three_quarter_circ();
        let mean_anomaly = mean_anomaly(Time::new::<year>(4.), Time::new::<year>(-1.));
        println!(
            "Expected mean anomaly: {}",
            expected_mean_anomaly.astro_display()
        );
        println!("Calculated mean anomaly: {}", mean_anomaly.astro_display());
        assert!(angle_eq(mean_anomaly, expected_mean_anomaly));
    }

    #[test]
    fn mean_anomaly_is_stable_after_loads_of_revolutions() {
        let local_test_angle_accuracy: Angle = 5e-3 * full_circ();

        let expected_mean_anomaly = quarter_circ();
        let passed_time = Time::new::<year>(1e5 + 0.25);
        let mean_anomaly = mean_anomaly(Time::new::<year>(1.), passed_time);
        println!(
            "Expected mean anomaly: {}",
            expected_mean_anomaly.astro_display()
        );
        println!("Calculated mean anomaly: {}", mean_anomaly.astro_display());
        assert!(angle_eq_within(
            mean_anomaly,
            expected_mean_anomaly,
            local_test_angle_accuracy
        ));
    }

    #[test]
    fn eccentric_anomaly_from_quarter_circle_mean_anomaly_and_zero_eccentricity() {
        let expected_eccentric_anomaly = quarter_circ();
        let eccentric_anomaly = eccentric_anomaly(quarter_circ(), 0.);
        println!(
            "Expected eccentric anomaly: {}",
            expected_eccentric_anomaly.astro_display()
        );
        println!(
            "Calculated eccentric anomaly: {}",
            eccentric_anomaly.astro_display()
        );
        assert!(angle_eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn eccentric_anomaly_from_half_circle_mean_anomaly_and_zero_eccentricity() {
        let expected_eccentric_anomaly = half_circ();
        let eccentric_anomaly = eccentric_anomaly(half_circ(), 0.);
        println!(
            "Expected eccentric anomaly: {}",
            expected_eccentric_anomaly.astro_display()
        );
        println!(
            "Calculated eccentric anomaly: {}",
            eccentric_anomaly.astro_display()
        );
        assert!(angle_eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn eccentric_anomaly_from_three_quarters_circle_mean_anomaly_and_zero_eccentricity() {
        let expected_eccentric_anomaly = three_quarter_circ();
        let eccentric_anomaly = eccentric_anomaly(three_quarter_circ(), 0.);
        println!(
            "Expected eccentric anomaly: {}",
            expected_eccentric_anomaly.astro_display()
        );
        println!(
            "Calculated eccentric anomaly: {}",
            eccentric_anomaly.astro_display()
        );
        assert!(angle_eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn eccentric_anomaly_from_negative_quarter_circle_mean_anomaly_and_zero_eccentricity() {
        let expected_eccentric_anomaly = -quarter_circ();
        let eccentric_anomaly = eccentric_anomaly(-quarter_circ(), 0.);
        println!(
            "Expected eccentric anomaly: {}",
            expected_eccentric_anomaly.astro_display()
        );
        println!(
            "Calculated eccentric anomaly: {}",
            eccentric_anomaly.astro_display()
        );
        assert!(angle_eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn eccentric_anomaly_from_quarter_circle_mean_anomaly_and_half_eccentricity() {
        let expected_eccentric_anomaly = Angle::new::<degree>(115.79362093315422);
        let eccentric_anomaly = eccentric_anomaly(quarter_circ(), 0.5);
        println!(
            "Expected eccentric anomaly: {}",
            expected_eccentric_anomaly.astro_display()
        );
        println!(
            "Calculated eccentric anomaly: {}",
            eccentric_anomaly.astro_display()
        );
        assert!(angle_eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn eccentric_anomaly_from_half_circle_mean_anomaly_and_half_eccentricity() {
        let expected_eccentric_anomaly = half_circ();
        let eccentric_anomaly = eccentric_anomaly(half_circ(), 0.5);
        println!(
            "Expected eccentric anomaly: {}",
            expected_eccentric_anomaly.astro_display()
        );
        println!(
            "Calculated eccentric anomaly: {}",
            eccentric_anomaly.astro_display()
        );
        assert!(angle_eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn eccentric_anomaly_from_three_quarters_circle_mean_anomaly_and_half_eccentricity() {
        let expected_eccentric_anomaly = Angle::new::<degree>(-115.79362093315422);
        let eccentric_anomaly = eccentric_anomaly(three_quarter_circ(), 0.5);
        println!(
            "Expected eccentric anomaly: {}",
            expected_eccentric_anomaly.astro_display()
        );
        println!(
            "Calculated eccentric anomaly: {}",
            eccentric_anomaly.astro_display()
        );
        assert!(angle_eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn eccentric_anomaly_from_negative_quarter_circle_mean_anomaly_and_half_eccentricity() {
        let expected_eccentric_anomaly = Angle::new::<degree>(-115.79362093315422);
        let eccentric_anomaly = eccentric_anomaly(-quarter_circ(), 0.5);
        println!(
            "Expected eccentric anomaly: {}",
            expected_eccentric_anomaly.astro_display()
        );
        println!(
            "Calculated eccentric anomaly: {}",
            eccentric_anomaly.astro_display()
        );
        assert!(angle_eq(eccentric_anomaly, expected_eccentric_anomaly));
    }

    #[test]
    fn true_anomaly_from_quarter_circle_eccentric_anomaly_and_zero_eccentricity() {
        let expected_true_anomaly = quarter_circ();
        let true_anomaly = true_anomaly(quarter_circ(), 0.);
        println!(
            "Expected true anomaly: {}",
            expected_true_anomaly.astro_display()
        );
        println!("Calculated true anomaly: {}", true_anomaly.astro_display());
        assert!(angle_eq(true_anomaly, expected_true_anomaly));
    }

    #[test]
    fn true_anomaly_from_half_circle_eccentric_anomaly_and_zero_eccentricity() {
        let expected_true_anomaly = half_circ();
        let true_anomaly = true_anomaly(half_circ(), 0.);
        println!(
            "Expected true anomaly: {}",
            expected_true_anomaly.astro_display()
        );
        println!("Calculated true anomaly: {}", true_anomaly.astro_display());
        assert!(angle_eq(true_anomaly, expected_true_anomaly));
    }

    #[test]
    fn true_anomaly_from_three_quarters_circle_eccentric_anomaly_and_zero_eccentricity() {
        let expected_true_anomaly = three_quarter_circ();
        let true_anomaly = true_anomaly(three_quarter_circ(), 0.);
        println!(
            "Expected true anomaly: {}",
            expected_true_anomaly.astro_display()
        );
        println!("Calculated true anomaly: {}", true_anomaly.astro_display());
        assert!(angle_eq(true_anomaly, expected_true_anomaly));
    }

    #[test]
    fn distance_from_focus_for_eccentric_ellipse() {
        let semi_major_axis = Length::new::<meter>(2.);
        let semi_minor_axis = Length::new::<meter>(1.);
        let eccentricity = (1. - (semi_minor_axis / semi_major_axis).value.powi(2)).sqrt();
        let linear_eccentricity = semi_major_axis * eccentricity;
        let focal_point = Cartesian::new(
            linear_eccentricity,
            Length::new::<astronomical_unit>(0.),
            Length::new::<astronomical_unit>(0.),
        );

        let eccentric_anom = angle_zero();
        let true_anom = true_anomaly(eccentric_anom, eccentricity);
        let point = Cartesian::new(
            semi_major_axis,
            Length::new::<astronomical_unit>(0.),
            Length::new::<astronomical_unit>(0.),
        );
        let expected = focal_point.distance(&point);
        let actual = distance_from_focus(semi_major_axis, true_anom, eccentricity);
        println!("Expected distance from focus: {}", expected.astro_display());
        println!("Calculated distance from focus: {}", actual.astro_display());
        assert!(eq(actual.value, expected.value));

        let eccentric_anom = quarter_circ();
        let true_anom = true_anomaly(eccentric_anom, eccentricity);
        let point = Cartesian::new(
            Length::new::<astronomical_unit>(0.),
            semi_minor_axis,
            Length::new::<astronomical_unit>(0.),
        );
        let expected = focal_point.distance(&point);
        let actual = distance_from_focus(semi_major_axis, true_anom, eccentricity);
        println!("Expected distance from focus: {}", expected.astro_display());
        println!("Calculated distance from focus: {}", actual.astro_display());
        assert!(eq(actual.value, expected.value));

        let eccentric_anom = half_circ();
        let true_anom = true_anomaly(eccentric_anom, eccentricity);
        let point = Cartesian::new(
            -semi_major_axis,
            Length::new::<astronomical_unit>(0.),
            Length::new::<astronomical_unit>(0.),
        );
        let expected = focal_point.distance(&point);
        let actual = distance_from_focus(semi_major_axis, true_anom, eccentricity);
        println!("Expected distance from focus: {}", expected.astro_display());
        println!("Calculated distance from focus: {}", actual.astro_display());
        assert!(eq(actual.value, expected.value));

        let eccentric_anom = three_quarter_circ();
        let true_anom = true_anomaly(eccentric_anom, eccentricity);
        let point = Cartesian::new(
            Length::new::<astronomical_unit>(0.),
            -semi_minor_axis,
            Length::new::<astronomical_unit>(0.),
        );
        let expected = focal_point.distance(&point);
        let actual = distance_from_focus(semi_major_axis, true_anom, eccentricity);
        println!("Expected distance from focus: {}", expected.astro_display());
        println!("Calculated distance from focus: {}", actual.astro_display());
        assert!(eq(actual.value, expected.value));
    }
}
