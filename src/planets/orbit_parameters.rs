use astro_coords::{cartesian::CartesianCoordinates, direction::Direction};
use serde::{Deserialize, Serialize};
use simple_si_units::{
    base::{Distance, Mass, Time},
    geometry::Angle,
};

use crate::planets::kepler_orbit::{
    eccentric_anomaly, mean_anomaly, orbital_period, position_relative_to_central_body,
    true_anomaly,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitParameters {
    pub(crate) semi_major_axis: Distance<f64>,
    pub(crate) eccentricity: f64,
    pub(crate) inclination: Angle<f64>, // The angle between the orbital plane and the reference plane
    pub(crate) longitude_of_ascending_node: Angle<f64>, // The angle between the reference plane and the ascending node
    pub(crate) argument_of_periapsis: Angle<f64>, // The angle between the ascending node and the periapsis
}

impl OrbitParameters {
    pub fn new(
        semi_major_axis: Distance<f64>,
        eccentricity: f64,
        inclination: Angle<f64>, // The angle between the orbital plane and the reference plane
        longitude_of_ascending_node: Angle<f64>, // The angle between the reference plane and the ascending node
        argument_of_periapsis: Angle<f64>, // The angle between the ascending node and the periapsis
    ) -> Self {
        OrbitParameters {
            semi_major_axis,
            eccentricity,
            inclination,
            longitude_of_ascending_node,
            argument_of_periapsis,
        }
    }

    pub fn get_semi_major_axis(&self) -> Distance<f64> {
        self.semi_major_axis
    }

    pub fn get_eccentricity(&self) -> f64 {
        self.eccentricity
    }

    pub fn get_inclination(&self) -> Angle<f64> {
        self.inclination
    }

    pub fn get_longitude_of_ascending_node(&self) -> Angle<f64> {
        self.longitude_of_ascending_node
    }

    pub fn get_argument_of_periapsis(&self) -> Angle<f64> {
        self.argument_of_periapsis
    }

    pub fn calculate_position(
        &self,
        body_mass: Mass<f64>,
        central_body_mass: Mass<f64>,
        time: Time<f64>,
    ) -> CartesianCoordinates {
        let orbital_period = orbital_period(self.semi_major_axis, body_mass, central_body_mass);
        let mean_anomaly = mean_anomaly(orbital_period, time);
        let eccentric_anomaly = eccentric_anomaly(mean_anomaly, self.eccentricity);
        let true_anomaly = true_anomaly(eccentric_anomaly, self.eccentricity);
        position_relative_to_central_body(
            self.semi_major_axis,
            self.eccentricity,
            true_anomaly,
            self,
        )
    }

    pub(crate) fn apply_orientation_to(
        &self,
        position_in_plane: CartesianCoordinates,
    ) -> CartesianCoordinates {
        let position = position_in_plane.rotated(self.inclination, &Direction::X);
        let position = position.rotated(self.longitude_of_ascending_node, &Direction::Z);
        position.rotated(self.argument_of_periapsis, &self.normal())
    }

    pub(crate) fn normal(&self) -> Direction {
        let ecliptic_normal = Direction::Z;
        let orbit_normal = ecliptic_normal.rotated(self.inclination, &Direction::X);
        orbit_normal.rotated(self.longitude_of_ascending_node, &Direction::Z)
    }
}

#[cfg(test)]
mod test {
    use simple_si_units::geometry::Angle;

    use crate::{real_data::planets::*, units::angle::angle_eq_within};

    const TILT_ACCURACY: Angle<f64> = Angle { rad: 2e-3 };

    #[test]
    fn axis_tilt_of_mercury() {
        let orbit_normal = MERCURY.orbit.normal();
        let north = MERCURY.rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = MERCURY.axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(angle_eq_within(actual, expected, TILT_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_venus() {
        let orbit_normal = VENUS.orbit.normal();
        let north = VENUS.rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = VENUS.axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(angle_eq_within(actual, expected, TILT_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_earth() {
        let orbit_normal = EARTH.orbit.normal();
        let north = EARTH.rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = EARTH.axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(angle_eq_within(actual, expected, TILT_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_mars() {
        let orbit_normal = MARS.orbit.normal();
        let north = MARS.rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = MARS.axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(angle_eq_within(actual, expected, TILT_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_ceres() {
        let orbit_normal = CERES.orbit.normal();
        let north = CERES.rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = CERES.axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(angle_eq_within(actual, expected, TILT_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_jupiter() {
        let orbit_normal = JUPITER.orbit.normal();
        let north = JUPITER.rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = JUPITER.axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(angle_eq_within(actual, expected, TILT_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_saturn() {
        let orbit_normal = SATURN.orbit.normal();
        let north = SATURN.rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = SATURN.axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(angle_eq_within(actual, expected, TILT_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_uranus() {
        let orbit_normal = URANUS.orbit.normal();
        let north = URANUS.rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = URANUS.axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(angle_eq_within(actual, expected, TILT_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_neptune() {
        let orbit_normal = NEPTUNE.orbit.normal();
        let north = NEPTUNE.rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = NEPTUNE.axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(angle_eq_within(actual, expected, TILT_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_pluto() {
        let orbit_normal = PLUTO.orbit.normal();
        let north = PLUTO.rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = PLUTO.axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(angle_eq_within(actual, expected, TILT_ACCURACY));
    }
}
