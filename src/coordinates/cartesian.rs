use super::{direction::Direction, rotations::rotated_tuple};
use crate::{
    units::{angle::Angle, length::Length},
    Float,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CartesianCoordinates {
    x: Length,
    y: Length,
    z: Length,
}

impl CartesianCoordinates {
    pub const ORIGIN: CartesianCoordinates = CartesianCoordinates {
        x: Length::from_astronomical_units(0.),
        y: Length::from_astronomical_units(0.),
        z: Length::from_astronomical_units(0.),
    };

    pub const fn new(x: Length, y: Length, z: Length) -> CartesianCoordinates {
        CartesianCoordinates { x, y, z }
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn eq_within(&self, other: &CartesianCoordinates, accuracy: Length) -> bool {
        self.x.eq_within(&other.x, accuracy)
            && self.y.eq_within(&other.y, accuracy)
            && self.z.eq_within(&other.z, accuracy)
    }

    pub fn length(&self) -> Length {
        let x = self.x.au;
        let y = self.y.au;
        let z = self.z.au;
        Length::from_astronomical_units((x * x + y * y + z * z).sqrt())
    }

    pub fn distance(&self, other: &CartesianCoordinates) -> Length {
        let diff = self - other;
        diff.length()
    }

    pub fn x(&self) -> Length {
        self.x
    }

    pub fn y(&self) -> Length {
        self.y
    }

    pub fn z(&self) -> Length {
        self.z
    }

    pub fn rotated(&self, angle: Angle, axis: &Direction) -> CartesianCoordinates {
        let (x, y, z) = rotated_tuple((self.x, self.y, self.z), angle, axis);
        CartesianCoordinates { x, y, z }
    }

    pub fn angle_to(&self, other: &CartesianCoordinates) -> Angle {
        self.to_direction().angle_to(&other.to_direction())
    }

    pub fn to_direction(&self) -> Direction {
        let length = self.length();
        Direction {
            x: self.x() / length,
            y: self.y() / length,
            z: self.z() / length,
        }
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

impl Add for CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn add(self, other: CartesianCoordinates) -> CartesianCoordinates {
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

impl Sub for CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn sub(self, other: CartesianCoordinates) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Float> for &CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn mul(self, f: Float) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
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

impl Div<Float> for &CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn div(self, f: Float) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f,
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

impl Neg for CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn neg(self) -> CartesianCoordinates {
        CartesianCoordinates {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn neg(self) -> CartesianCoordinates {
        CartesianCoordinates {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Display for CartesianCoordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::CartesianCoordinates;
    use crate::{tests::TEST_LENGTH_ACCURACY, units::length::Length};

    #[test]
    fn test_length() {
        let coordinates = CartesianCoordinates {
            x: Length::from_meters(3.),
            y: Length::from_meters(4.),
            z: Length::from_meters(5.),
        };

        assert!(coordinates.length().eq_within(
            &Length::from_meters(7.0710678118654755),
            TEST_LENGTH_ACCURACY
        ));
    }
}
