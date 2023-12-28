use crate::{
    units::{angle::Angle, length::Length, mass::Mass, time::Time},
    Float, TWO_PI,
};

/*
 * The orbital period is the time it takes for a given object to make one full orbit around another object.
 * https://en.wikipedia.org/wiki/Orbital_period
 */
pub fn orbital_period(semi_major_axis: Length, mass1: Mass, mass2: Mass) -> Time {
    const G: Float = 6.67430e-11;

    let semi_major_axis_cubed = semi_major_axis.as_meters().powi(3);
    let total_mass = (mass1 + mass2).as_kilograms();
    let orbital_period = TWO_PI * (semi_major_axis_cubed / total_mass / G).sqrt();
    Time::from_seconds(orbital_period)
}

/*
 * The mean anomaly is the angle between the direction of periapsis and the current position of the orbiting body,
 * as seen from the center of the ellipse (the point around which the object orbits).
 * https://en.wikipedia.org/wiki/Mean_anomaly
 *
 * Output is normalised to the range [-π, π].
 */
pub fn mean_anomaly(orbital_period: Time, time: Time) -> Angle {
    let mean_motion = TWO_PI / orbital_period.as_seconds();
    let mean_anomaly = mean_motion * (time.as_seconds() % orbital_period.as_seconds());
    let mut mean_anomaly = Angle::from_radians(mean_anomaly);
    mean_anomaly.normalize();
    mean_anomaly
}

/*
 * The eccentric anomaly is the angle between the direction of periapsis and the current position of the orbiting body,
 * as seen from the center of the ellipse (the point around which the object orbits).
 * https://en.wikipedia.org/wiki/Eccentric_anomaly
 */
pub fn eccentric_anomaly(mean_anomaly: Angle, eccentricity: Float) -> Angle {
    const ACCURACY: Float = 1e-6;
    let mean_anomaly = mean_anomaly.as_radians();
    let mut eccentric_anomaly = mean_anomaly;
    let mut error = 10.0 * ACCURACY;
    while error > ACCURACY {
        let numerator = eccentric_anomaly - eccentricity * eccentric_anomaly.sin() - mean_anomaly;
        let denominator = 1.0 - eccentricity * eccentric_anomaly.cos();
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
pub fn true_anomaly(eccentric_anomaly: Angle, eccentricity: Float) -> Angle {
    let sqrt_arg = (1.0 + eccentricity) / (1.0 - eccentricity);
    let artan_arg = (eccentric_anomaly.as_radians() / 2.0).tan() * sqrt_arg.sqrt();
    let true_anomaly = 2.0 * artan_arg.atan();
    Angle::from_radians(true_anomaly)
}

#[cfg(test)]
mod tests {
    use crate::{
        solar_system_data::{
            EARTH_MASS, EARTH_SEMI_MAJOR_AXIS, JUPITER_MASS, JUPITER_SEMI_MAJOR_AXIS, MOON_MASS,
            MOON_SEMI_MAJOR_AXIS, SUN_MASS,
        },
        tests::TEST_ANGLE_ACCURACY,
    };

    use super::*;

    #[test]
    fn orbital_period_of_earth() {
        let expected_orbital_period = Time::from_days(365.256);
        let orbital_period = orbital_period(EARTH_SEMI_MAJOR_AXIS, EARTH_MASS, SUN_MASS);
        println!("Expected orbital period: {}", expected_orbital_period);
        println!("Calculated orbital period: {}", orbital_period);
        assert!(orbital_period.eq_within(expected_orbital_period, 1e-3 * expected_orbital_period));
    }

    #[test]
    fn orbital_period_of_jupiter() {
        let expected_orbital_period = Time::from_days(4332.59);
        let orbital_period = orbital_period(JUPITER_SEMI_MAJOR_AXIS, JUPITER_MASS, SUN_MASS);
        println!("Expected orbital period: {}", expected_orbital_period);
        println!("Calculated orbital period: {}", orbital_period);
        assert!(orbital_period.eq_within(expected_orbital_period, 1e-3 * expected_orbital_period));
    }

    #[test]
    fn orbital_period_of_moon() {
        let expected_orbital_period = Time::from_days(27.321);
        let orbital_period = orbital_period(MOON_SEMI_MAJOR_AXIS, MOON_MASS, EARTH_MASS);
        println!("Expected orbital period: {}", expected_orbital_period);
        println!("Calculated orbital period: {}", orbital_period);
        assert!(orbital_period.eq_within(expected_orbital_period, 9e-3 * expected_orbital_period));
    }

    #[test]
    fn mean_anomaly_a_quarter_circle() {
        let expected_mean_anomaly = Angle::from_radians(TWO_PI / 4.0);
        let mean_anomaly = mean_anomaly(Time::from_years(4.0), Time::from_years(1.0));
        println!("Expected mean anomaly: {}", expected_mean_anomaly);
        println!("Calculated mean anomaly: {}", mean_anomaly);
        assert!(mean_anomaly.eq_within(expected_mean_anomaly, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn mean_anomaly_a_half_circle() {
        let expected_mean_anomaly = Angle::from_radians(TWO_PI / 2.0);
        let mean_anomaly = mean_anomaly(Time::from_years(4.0), Time::from_years(2.0));
        println!("Expected mean anomaly: {}", expected_mean_anomaly);
        println!("Calculated mean anomaly: {}", mean_anomaly);
        assert!(mean_anomaly.eq_within(expected_mean_anomaly, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn mean_anomaly_three_quarters_circle() {
        let expected_mean_anomaly = Angle::from_radians(TWO_PI * 3.0 / 4.0);
        let mean_anomaly = mean_anomaly(Time::from_years(4.0), Time::from_years(-1.0));
        println!("Expected mean anomaly: {}", expected_mean_anomaly);
        println!("Calculated mean anomaly: {}", mean_anomaly);
        assert!(mean_anomaly.eq_within(expected_mean_anomaly, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn mean_anomaly_is_stable_after_loads_of_revolutions() {
        const LOCAL_TEST_ANGLE_ACCURACY: Angle = Angle::from_radians(5e-3 * TWO_PI);

        let expected_mean_anomaly = Angle::from_radians(TWO_PI / 4.0);
        let passed_time = Time::from_years(1e5 + 0.25);
        let mean_anomaly = mean_anomaly(Time::from_years(1.0), passed_time);
        println!("Expected mean anomaly: {}", expected_mean_anomaly);
        println!("Calculated mean anomaly: {}", mean_anomaly);
        assert!(mean_anomaly.eq_within(expected_mean_anomaly, LOCAL_TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn eccentric_anomaly_from_quarter_circle_mean_anomaly_and_zero_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_radians(TWO_PI / 4.0);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(TWO_PI / 4.0), 0.0);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eccentric_anomaly.eq_within(expected_eccentric_anomaly, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn eccentric_anomaly_from_half_circle_mean_anomaly_and_zero_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_radians(TWO_PI / 2.0);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(TWO_PI / 2.0), 0.0);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eccentric_anomaly.eq_within(expected_eccentric_anomaly, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn eccentric_anomaly_from_three_quarters_circle_mean_anomaly_and_zero_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_radians(TWO_PI * 3.0 / 4.0);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(TWO_PI * 3.0 / 4.0), 0.0);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eccentric_anomaly.eq_within(expected_eccentric_anomaly, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn eccentric_anomaly_from_negative_quarter_circle_mean_anomaly_and_zero_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_radians(-TWO_PI / 4.0);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(-TWO_PI / 4.0), 0.0);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eccentric_anomaly.eq_within(expected_eccentric_anomaly, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn eccentric_anomaly_from_quarter_circle_mean_anomaly_and_half_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_degrees(115.79362093315422);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(TWO_PI / 4.0), 0.5);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eccentric_anomaly.eq_within(expected_eccentric_anomaly, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn eccentric_anomaly_from_half_circle_mean_anomaly_and_half_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_radians(TWO_PI / 2.0);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(TWO_PI / 2.0), 0.5);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eccentric_anomaly.eq_within(expected_eccentric_anomaly, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn eccentric_anomaly_from_three_quarters_circle_mean_anomaly_and_half_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_degrees(-115.79362093315422);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(TWO_PI * 3.0 / 4.0), 0.5);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eccentric_anomaly.eq_within(expected_eccentric_anomaly, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn eccentric_anomaly_from_negative_quarter_circle_mean_anomaly_and_half_eccentricity() {
        let expected_eccentric_anomaly = Angle::from_degrees(-115.79362093315422);
        let eccentric_anomaly = eccentric_anomaly(Angle::from_radians(-TWO_PI / 4.0), 0.5);
        println!("Expected eccentric anomaly: {}", expected_eccentric_anomaly);
        println!("Calculated eccentric anomaly: {}", eccentric_anomaly);
        assert!(eccentric_anomaly.eq_within(expected_eccentric_anomaly, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn true_anomaly_from_quarter_circle_eccentric_anomaly_and_zero_eccentricity() {
        let expected_true_anomaly = Angle::from_radians(TWO_PI / 4.0);
        let true_anomaly = true_anomaly(Angle::from_radians(TWO_PI / 4.0), 0.0);
        println!("Expected true anomaly: {}", expected_true_anomaly);
        println!("Calculated true anomaly: {}", true_anomaly);
        assert!(true_anomaly.eq_within(expected_true_anomaly, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn true_anomaly_from_half_circle_eccentric_anomaly_and_zero_eccentricity() {
        let expected_true_anomaly = Angle::from_radians(TWO_PI / 2.0);
        let true_anomaly = true_anomaly(Angle::from_radians(TWO_PI / 2.0), 0.0);
        println!("Expected true anomaly: {}", expected_true_anomaly);
        println!("Calculated true anomaly: {}", true_anomaly);
        assert!(true_anomaly.eq_within(expected_true_anomaly, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn true_anomaly_from_three_quarters_circle_eccentric_anomaly_and_zero_eccentricity() {
        let expected_true_anomaly = Angle::from_radians(TWO_PI * 3.0 / 4.0);
        let true_anomaly = true_anomaly(Angle::from_radians(TWO_PI * 3.0 / 4.0), 0.0);
        println!("Expected true anomaly: {}", expected_true_anomaly);
        println!("Calculated true anomaly: {}", true_anomaly);
        assert!(true_anomaly.eq_within(expected_true_anomaly, TEST_ANGLE_ACCURACY));
    }
}
