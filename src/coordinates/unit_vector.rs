use super::{cartesian::CartesianCoordinates, ecliptic::EclipticCoordinates};
use crate::{units::length::Length, Float};
use std::fmt::Display;

pub const X_DIRECTION: UnitVector = UnitVector {
    x: 1.,
    y: 0.,
    z: 0.,
};
pub const Y_DIRECTION: UnitVector = UnitVector {
    x: 0.,
    y: 1.,
    z: 0.,
};
pub const Z_DIRECTION: UnitVector = UnitVector {
    x: 0.,
    y: 0.,
    z: 1.,
};

pub struct UnitVector {
    x: Float,
    y: Float,
    z: Float,
}

impl UnitVector {
    pub(crate) fn from_cartesian(coords: &CartesianCoordinates) -> Self {
        let length = coords.length();
        UnitVector {
            x: coords.x() / length,
            y: coords.y() / length,
            z: coords.z() / length,
        }
    }

    pub(crate) fn from_ecliptic(ecliptic: &EclipticCoordinates) -> Self {
        todo!()
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
}

impl Display for UnitVector {
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
                    let unit_vector = UnitVector::from_ecliptic(&ecliptic);

                    assert!((unit_vector.x() - expected_x).abs() < TEST_ACCURACY);
                    assert!((unit_vector.y() - expected_y).abs() < TEST_ACCURACY);
                    assert!((unit_vector.z() - expected_z).abs() < TEST_ACCURACY);
                }
            }
        }
    }
}
