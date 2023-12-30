use super::{
    cartesian::CartesianCoordinates, ecliptic::EclipticCoordinates, rotations::rotated_tuple,
};
use crate::{
    units::{angle::Angle, length::Length},
    Float,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub const X: Direction = Direction {
    x: 1.,
    y: 0.,
    z: 0.,
};
pub const Y: Direction = Direction {
    x: 0.,
    y: 1.,
    z: 0.,
};
pub const Z: Direction = Direction {
    x: 0.,
    y: 0.,
    z: 1.,
};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Direction {
    x: Float,
    y: Float,
    z: Float,
}

impl Direction {
    pub fn new(x: Float, y: Float, z: Float) -> Direction {
        let length = (x * x + y * y + z * z).sqrt();
        Direction {
            x: x / length,
            y: y / length,
            z: z / length,
        }
    }

    pub fn from_cartesian(coords: &CartesianCoordinates) -> Self {
        let length = coords.length();
        Direction {
            x: coords.x() / length,
            y: coords.y() / length,
            z: coords.z() / length,
        }
    }

    pub(crate) fn from_ecliptic(ecliptic: &EclipticCoordinates) -> Self {
        let x = ecliptic.longitude().cos() * ecliptic.latitude().cos();
        let y = ecliptic.longitude().sin() * ecliptic.latitude().cos();
        let z = ecliptic.latitude().sin();
        Direction { x, y, z }
    }

    pub(crate) fn to_cartesian(&self, length: Length) -> CartesianCoordinates {
        CartesianCoordinates::new(self.x * length, self.y * length, self.z * length)
    }

    pub(crate) fn x(&self) -> Float {
        self.x
    }

    pub(crate) fn y(&self) -> Float {
        self.y
    }

    pub(crate) fn z(&self) -> Float {
        self.z
    }

    pub fn rotated(&self, angle: Angle, axis: &Direction) -> Direction {
        let (x, y, z) = rotated_tuple((self.x, self.y, self.z), angle, axis);
        Direction { x, y, z }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::TEST_ACCURACY;

    #[test]
    fn from_ecliptic() {
        let values = vec![-1., 1., 10.];
        for x in values.iter() {
            for y in values.iter() {
                for z in values.iter() {
                    let x = Length::from_meters(*x);
                    let y = Length::from_meters(*y);
                    let z = Length::from_meters(*z);
                    let cartesian = CartesianCoordinates::new(x, y, z);
                    let length = cartesian.length();
                    let expected_x = x / length;
                    let expected_y = y / length;
                    let expected_z = z / length;

                    let ecliptic = EclipticCoordinates::from_cartesian(&cartesian);
                    let direction = Direction::from_ecliptic(&ecliptic);

                    assert!((direction.x() - expected_x).abs() < TEST_ACCURACY);
                    assert!((direction.y() - expected_y).abs() < TEST_ACCURACY);
                    assert!((direction.z() - expected_z).abs() < TEST_ACCURACY);
                }
            }
        }
    }
}
