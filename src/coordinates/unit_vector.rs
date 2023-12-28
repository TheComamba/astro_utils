use super::cartesian::CartesianCoordinates;
use crate::Float;

pub const X_DIRECTION: UnitVector = UnitVector {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
pub const Y_DIRECTION: UnitVector = UnitVector {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
pub const Z_DIRECTION: UnitVector = UnitVector {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

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
