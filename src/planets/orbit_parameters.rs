use crate::{
    coordinates::cartesian::CartesianCoordinates,
    planets::kepler_orbit::{
        eccentric_anomaly, mean_anomaly, orbital_period, position_relative_to_central_body,
        true_anomaly,
    },
    planets::orbit_orientation::OrbitOrientation,
    stars::star::Star,
    units::{angle::Angle, length::Length, mass::Mass, time::Time},
    Float,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitParameters {
    semi_major_axis: Length,
    eccentricity: Float,
    orientation: OrbitOrientation,
}

impl OrbitParameters {
    pub fn new(
        semi_major_axis: Length,
        eccentricity: Float,
        orientation: OrbitOrientation,
    ) -> Self {
        OrbitParameters {
            semi_major_axis,
            eccentricity,
            orientation,
        }
    }

    pub fn get_semi_major_axis(&self) -> Length {
        self.semi_major_axis
    }

    pub fn get_eccentricity(&self) -> Float {
        self.eccentricity
    }

    pub fn get_inclination(&self) -> Angle {
        self.orientation.inclination()
    }

    pub fn get_longitude_of_ascending_node(&self) -> Angle {
        self.orientation.longitude_of_ascending_node()
    }

    pub fn get_argument_of_periapsis(&self) -> Angle {
        self.orientation.argument_of_periapsis()
    }

    pub fn calculate_position(
        &self,
        body_mass: Mass,
        central_body: &Star,
        time: Time,
    ) -> CartesianCoordinates {
        let central_body_position = central_body.calculate_position();

        let orbital_period =
            orbital_period(self.semi_major_axis, body_mass, central_body.get_mass());
        let mean_anomaly = mean_anomaly(orbital_period, time);
        let eccentric_anomaly = eccentric_anomaly(mean_anomaly, self.eccentricity);
        let true_anomaly = true_anomaly(eccentric_anomaly, self.eccentricity);
        let position = position_relative_to_central_body(
            self.semi_major_axis,
            self.eccentricity,
            true_anomaly,
            &self.orientation,
        );
        central_body_position + position
    }
}
