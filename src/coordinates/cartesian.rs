use super::{
    direction::Direction, ecliptic::EclipticCoordinates, rotations::rotated_tuple,
    spherical::SphericalCoordinates,
};
use crate::{error::AstroUtilError, units::distance::DISTANCE_ZERO};
use serde::{Deserialize, Serialize};
use simple_si_units::{base::Distance, geometry::Angle};
use std::{
    fmt::{Display, Formatter},
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CartesianCoordinates {
    x: Distance<f64>,
    y: Distance<f64>,
    z: Distance<f64>,
}

impl CartesianCoordinates {
    pub const ORIGIN: CartesianCoordinates = CartesianCoordinates {
        x: DISTANCE_ZERO,
        y: DISTANCE_ZERO,
        z: DISTANCE_ZERO,
    };

    pub const fn new(x: Distance<f64>, y: Distance<f64>, z: Distance<f64>) -> CartesianCoordinates {
        CartesianCoordinates { x, y, z }
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn eq_within(&self, other: &CartesianCoordinates, accuracy: Distance<f64>) -> bool {
        self.x.eq_within(&other.x, accuracy)
            && self.y.eq_within(&other.y, accuracy)
            && self.z.eq_within(&other.z, accuracy)
    }

    pub fn length(&self) -> Distance<f64> {
        let x = self.x.au;
        let y = self.y.au;
        let z = self.z.au;
        Distance::from_astronomical_units((x * x + y * y + z * z).sqrt())
    }

    pub fn distance(&self, other: &CartesianCoordinates) -> Distance<f64> {
        let diff = self - other;
        diff.length()
    }

    pub fn x(&self) -> Distance<f64> {
        self.x
    }

    pub fn y(&self) -> Distance<f64> {
        self.y
    }

    pub fn z(&self) -> Distance<f64> {
        self.z
    }

    pub fn rotated(&self, angle: Angle<f64>, axis: &Direction) -> CartesianCoordinates {
        let (x, y, z) = rotated_tuple((self.x, self.y, self.z), angle, axis);
        CartesianCoordinates { x, y, z }
    }

    pub fn angle_to(&self, other: &CartesianCoordinates) -> Result<Angle<f64>, AstroUtilError> {
        Ok(self.to_direction()?.angle_to(&other.to_direction()?))
    }

    pub fn to_direction(&self) -> Result<Direction, AstroUtilError> {
        Direction::new(self.x.au, self.y.au, self.z.au)
    }

    pub fn to_ecliptic(&self) -> EclipticCoordinates {
        EclipticCoordinates {
            spherical: self.to_spherical(),
        }
    }

    pub fn to_spherical(&self) -> SphericalCoordinates {
        SphericalCoordinates::cartesian_to_spherical((self.x.au, self.y.au, self.z.au))
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

impl Mul<f64> for &CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn mul(self, f: f64) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

impl Mul<f64> for CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn mul(self, f: f64) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

impl Div<f64> for &CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn div(self, f: f64) -> CartesianCoordinates {
        CartesianCoordinates {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f,
        }
    }
}

impl Div<f64> for CartesianCoordinates {
    type Output = CartesianCoordinates;

    fn div(self, f: f64) -> CartesianCoordinates {
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
    use crate::tests::eq;

    use super::*;

    #[test]
    fn test_length() {
        let coordinates = CartesianCoordinates {
            x: Distance::from_meters(3.),
            y: Distance::from_meters(4.),
            z: Distance::from_meters(5.),
        };

        assert!(eq(coordinates.length().to_m(), 7.0710678118654755));
    }
}
