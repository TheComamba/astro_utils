use crate::{
    coordinates::{
        cartesian::CartesianCoordinates,
        direction::{Direction, X, Z},
    },
    units::angle::Angle,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrbitOrientation {
    inclination: Angle, // The angle between the orbital plane and the reference plane
    longitude_of_ascending_node: Angle, // The angle between the reference plane and the ascending node
    argument_of_periapsis: Angle,       // The angle between the ascending node and the periapsis
}

impl OrbitOrientation {
    pub const fn new(
        inclination: Angle,
        longitude_of_ascending_node: Angle,
        argument_of_periapsis: Angle,
    ) -> OrbitOrientation {
        OrbitOrientation {
            inclination,
            longitude_of_ascending_node,
            argument_of_periapsis,
        }
    }

    pub fn inclination(&self) -> Angle {
        self.inclination
    }

    pub fn longitude_of_ascending_node(&self) -> Angle {
        self.longitude_of_ascending_node
    }

    pub fn argument_of_periapsis(&self) -> Angle {
        self.argument_of_periapsis
    }

    pub(crate) fn apply_to(&self, position_in_plane: CartesianCoordinates) -> CartesianCoordinates {
        let position = position_in_plane.rotated(self.inclination, &X);
        let position = position.rotated(self.longitude_of_ascending_node, &Z);
        let position = position.rotated(self.argument_of_periapsis, &self.normal());
        position
    }

    pub(crate) fn normal(&self) -> Direction {
        let ecliptic_normal = Z;
        let orbit_normal = ecliptic_normal.rotated(self.inclination, &X);
        let orbit_normal = orbit_normal.rotated(self.longitude_of_ascending_node, &Z);
        orbit_normal
    }
}
