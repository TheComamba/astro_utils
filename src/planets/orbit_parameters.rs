use astro_coords::{cartesian::Cartesian, direction::Direction, traits::*};
use serde::{Deserialize, Serialize};
use uom::si::f64::{Angle, Length, Mass, Time};

use crate::planets::kepler_orbit::{
    eccentric_anomaly, mean_anomaly, orbital_period, position_relative_to_central_body,
    true_anomaly,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitParameters {
    pub(crate) semi_major_axis: Length,
    pub(crate) eccentricity: f64,
    pub(crate) inclination: Angle, // The angle between the orbital plane and the reference plane
    pub(crate) longitude_of_ascending_node: Angle, // The angle between the reference plane and the ascending node
    pub(crate) argument_of_periapsis: Angle, // The angle between the ascending node and the periapsis
}

impl OrbitParameters {
    pub fn new(
        semi_major_axis: Length,
        eccentricity: f64,
        inclination: Angle, // The angle between the orbital plane and the reference plane
        longitude_of_ascending_node: Angle, // The angle between the reference plane and the ascending node
        argument_of_periapsis: Angle, // The angle between the ascending node and the periapsis
    ) -> Self {
        OrbitParameters {
            semi_major_axis,
            eccentricity,
            inclination,
            longitude_of_ascending_node,
            argument_of_periapsis,
        }
    }

    pub fn get_semi_major_axis(&self) -> Length {
        self.semi_major_axis
    }

    pub fn get_eccentricity(&self) -> f64 {
        self.eccentricity
    }

    pub fn get_inclination(&self) -> Angle {
        self.inclination
    }

    pub fn get_longitude_of_ascending_node(&self) -> Angle {
        self.longitude_of_ascending_node
    }

    pub fn get_argument_of_periapsis(&self) -> Angle {
        self.argument_of_periapsis
    }

    pub fn calculate_position(
        &self,
        body_mass: Mass,
        central_body_mass: Mass,
        time: Time,
    ) -> Cartesian {
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

    pub(crate) fn apply_orientation_to(&self, position_in_plane: Cartesian) -> Cartesian {
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

    use uom::si::{angle::radian, f64::Angle};

    use crate::{astro_display::AstroDisplay, real_data::planets::*};

    fn tilt_accuracy() -> Angle {
        Angle::new::<radian>(2e-3)
    }

    #[test]
    fn axis_tilt_of_mercury() {
        let orbit_normal = mercury().orbit.normal();
        let north = mercury().rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = mercury().axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        assert!(angle_eq_within(actual, expected, tilt_accuracy()));
    }

    #[test]
    fn axis_tilt_of_venus() {
        let orbit_normal = venus().orbit.normal();
        let north = venus().rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = venus().axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        assert!(angle_eq_within(actual, expected, tilt_accuracy()));
    }

    #[test]
    fn axis_tilt_of_earth() {
        let orbit_normal = earth().orbit.normal();
        let north = earth().rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = earth().axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        assert!(angle_eq_within(actual, expected, tilt_accuracy()));
    }

    #[test]
    fn axis_tilt_of_mars() {
        let orbit_normal = mars().orbit.normal();
        let north = mars().rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = mars().axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        assert!(angle_eq_within(actual, expected, tilt_accuracy()));
    }

    #[test]
    fn axis_tilt_of_ceres() {
        let orbit_normal = ceres().orbit.normal();
        let north = ceres().rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = ceres().axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        assert!(angle_eq_within(actual, expected, tilt_accuracy()));
    }

    #[test]
    fn axis_tilt_of_jupiter() {
        let orbit_normal = jupiter().orbit.normal();
        let north = jupiter().rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = jupiter().axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        assert!(angle_eq_within(actual, expected, tilt_accuracy()));
    }

    #[test]
    fn axis_tilt_of_saturn() {
        let orbit_normal = saturn().orbit.normal();
        let north = saturn().rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = saturn().axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        assert!(angle_eq_within(actual, expected, tilt_accuracy()));
    }

    #[test]
    fn axis_tilt_of_uranus() {
        let orbit_normal = uranus().orbit.normal();
        let north = uranus().rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = uranus().axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        assert!(angle_eq_within(actual, expected, tilt_accuracy()));
    }

    #[test]
    fn axis_tilt_of_neptune() {
        let orbit_normal = neptune().orbit.normal();
        let north = neptune().rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = neptune().axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        assert!(angle_eq_within(actual, expected, tilt_accuracy()));
    }

    #[test]
    fn axis_tilt_of_pluto() {
        let orbit_normal = pluto().orbit.normal();
        let north = pluto().rotation_axis.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = pluto().axis_tilt;
        let actual = orbit_normal.angle_to(&north);
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        assert!(angle_eq_within(actual, expected, tilt_accuracy()));
    }
}
