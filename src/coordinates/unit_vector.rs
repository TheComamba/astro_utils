use crate::Float;

use super::cartesian::CartesianCoordinates;

pub struct UnitVector {
    x: Float,
    y: Float,
    z: Float,
}

impl UnitVector {
    pub fn from_cartesian(coords: &CartesianCoordinates) -> Self {
        let length = coords.length();
        UnitVector {
            x: coords.x() / length,
            y: coords.y() / length,
            z: coords.z() / length,
        }
    }
}
