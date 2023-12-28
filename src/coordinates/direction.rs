use super::{cartesian::CartesianCoordinates, ecliptic::EclipticCoordinates};
use crate::{
    units::{angle::Angle, length::Length},
    Float,
};
use std::{
    fmt::Display,
    ops::{Add, Mul},
};

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

pub struct Direction {
    x: Float,
    y: Float,
    z: Float,
}

impl Direction {
    pub(crate) fn from_cartesian(coords: &CartesianCoordinates) -> Self {
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

pub(super) fn rotated_tuple<T>(tup: (T, T, T), angle: Angle, axis: &Direction) -> (T, T, T)
where
    T: Mul<Float, Output = T> + Add<Output = T> + Copy,
{
    let cos = angle.cos();
    let sin = angle.sin();

    let (x, y, z) = tup;

    let ux = axis.x();
    let uy = axis.y();
    let uz = axis.z();

    let r_11 = cos + ux * ux * (1. - cos);
    let r_12 = ux * uy * (1. - cos) - uz * sin;
    let r_13 = ux * uz * (1. - cos) + uy * sin;

    let r_21 = uy * ux * (1. - cos) + uz * sin;
    let r_22 = cos + uy * uy * (1. - cos);
    let r_23 = uy * uz * (1. - cos) - ux * sin;

    let r_31 = uz * ux * (1. - cos) - uy * sin;
    let r_32 = uz * uy * (1. - cos) + ux * sin;
    let r_33 = cos + uz * uz * (1. - cos);

    let x_out = x * r_11 + y * r_12 + z * r_13;
    let y_out = x * r_21 + y * r_22 + z * r_23;
    let z_out = x * r_31 + y * r_32 + z * r_33;
    (x_out, y_out, z_out)
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
