use crate::{units::length::Length, Float};
use std::ops::{Add, Div, Mul, Sub};

use super::unit_vector::UnitVector;

pub const ORIGIN: CartesianCoordinates = CartesianCoordinates {
    x: Length::from_meters(0.0),
    y: Length::from_meters(0.0),
    z: Length::from_meters(0.0),
};

pub struct CartesianCoordinates {
    x: Length,
    y: Length,
    z: Length,
}

impl CartesianCoordinates {
    pub fn eq_within(&self, other: &CartesianCoordinates, accuracy: Float) -> bool {
        self.x.eq_within(other.x, accuracy)
            && self.y.eq_within(other.y, accuracy)
            && self.z.eq_within(other.z, accuracy)
    }

    pub fn length(&self) -> Length {
        let x = self.x.as_meters();
        let y = self.y.as_meters();
        let z = self.z.as_meters();
        Length::from_meters((x * x + y * y + z * z).sqrt())
    }

    pub fn distance(&self, other: &CartesianCoordinates) -> Length {
        let diff = self - other;
        diff.length()
    }

    pub(super) fn x(&self) -> Length {
        self.x
    }

    pub(super) fn y(&self) -> Length {
        self.y
    }

    pub(super) fn z(&self) -> Length {
        self.z
    }
}

impl Add for &CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn add(self, other: &CartesianCoordinates) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for &CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn sub(self, other: &CartesianCoordinates) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Float> for CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn mul(self, f: Float) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

impl Div<Float> for CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn div(self, f: Float) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_length() {
        use super::CartesianCoordinates;
        use crate::units::length::Length;

        let coordinates = CartesianCoordinates {
            x: Length::from_meters(3.0),
            y: Length::from_meters(4.0),
            z: Length::from_meters(5.0),
        };

        assert_eq!(
            coordinates.length(),
            Length::from_meters(7.0710678118654755)
        );
    }
}
