use crate::{
    coordinates::{cartesian::CartesianCoordinates, direction::Direction},
    planets::kepler_orbit::{
        eccentric_anomaly, mean_anomaly, orbital_period, position_relative_to_central_body,
        true_anomaly,
    },
    stars::star_data::StarData,
    Float,
};
use serde::{Deserialize, Serialize};
use simple_si_units::{
    base::{Distance, Mass, Time},
    geometry::Angle,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitParameters {
    pub(crate) semi_major_axis: Distance<Float>,
    pub(crate) eccentricity: Float,
    pub(crate) inclination: Angle<Float>, // The angle between the orbital plane and the reference plane
    pub(crate) longitude_of_ascending_node: Angle<Float>, // The angle between the reference plane and the ascending node
    pub(crate) argument_of_periapsis: Angle<Float>, // The angle between the ascending node and the periapsis
}

impl OrbitParameters {
    pub fn new(
        semi_major_axis: Distance<Float>,
        eccentricity: Float,
        inclination: Angle<Float>, // The angle between the orbital plane and the reference plane
        longitude_of_ascending_node: Angle<Float>, // The angle between the reference plane and the ascending node
        argument_of_periapsis: Angle<Float>, // The angle between the ascending node and the periapsis
    ) -> Self {
        OrbitParameters {
            semi_major_axis,
            eccentricity,
            inclination,
            longitude_of_ascending_node,
            argument_of_periapsis,
        }
    }

    pub fn get_semi_major_axis(&self) -> Distance<Float> {
        self.semi_major_axis
    }

    pub fn get_eccentricity(&self) -> Float {
        self.eccentricity
    }

    pub fn get_inclination(&self) -> Angle<Float> {
        self.inclination
    }

    pub fn get_longitude_of_ascending_node(&self) -> Angle<Float> {
        self.longitude_of_ascending_node
    }

    pub fn get_argument_of_periapsis(&self) -> Angle<Float> {
        self.argument_of_periapsis
    }

    pub fn calculate_position(
        &self,
        body_mass: Mass<Float>,
        central_body: &StarData,
        time: Time<Float>,
    ) -> CartesianCoordinates {
        let orbital_period = orbital_period(
            self.semi_major_axis,
            body_mass,
            central_body.get_mass().unwrap(),
        );
        let mean_anomaly = mean_anomaly(orbital_period, time);
        let eccentric_anomaly = eccentric_anomaly(mean_anomaly, self.eccentricity);
        let true_anomaly = true_anomaly(eccentric_anomaly, self.eccentricity);
        let position = position_relative_to_central_body(
            self.semi_major_axis,
            self.eccentricity,
            true_anomaly,
            &self,
        );
        position
    }

    pub(crate) fn apply_orientation_to(
        &self,
        position_in_plane: CartesianCoordinates,
    ) -> CartesianCoordinates {
        let position = position_in_plane.rotated(self.inclination, &Direction::X);
        let position = position.rotated(self.longitude_of_ascending_node, &Direction::Z);
        let position = position.rotated(self.argument_of_periapsis, &self.normal());
        position
    }

    pub(crate) fn normal(&self) -> Direction {
        let ecliptic_normal = Direction::Z;
        let orbit_normal = ecliptic_normal.rotated(self.inclination, &Direction::X);
        let orbit_normal = orbit_normal.rotated(self.longitude_of_ascending_node, &Direction::Z);
        orbit_normal
    }
}
