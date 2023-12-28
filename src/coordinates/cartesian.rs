use crate::{
    units::{angle::Angle, length::Length},
    Float,
};
use std::ops::{Add, Div, Mul, Neg, Sub};

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

    pub(super) fn rotated(&self, angle: Angle, axis: UnitVector) -> CartesianCoordinates {
        todo!();
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

#[cfg(test)]
mod tests {
    use super::CartesianCoordinates;
    use crate::{
        coordinates::unit_vector::{self},
        units::{angle::Angle, length::Length},
        Float, PI, PI_HALF,
    };

    const TEST_ACCURACY: Float = 1e-5;

    const X_DIRECTION: CartesianCoordinates = CartesianCoordinates {
        x: Length::from_meters(1.0),
        y: Length::from_meters(0.0),
        z: Length::from_meters(0.0),
    };

    const Y_DIRECTION: CartesianCoordinates = CartesianCoordinates {
        x: Length::from_meters(0.0),
        y: Length::from_meters(1.0),
        z: Length::from_meters(0.0),
    };

    const Z_DIRECTION: CartesianCoordinates = CartesianCoordinates {
        x: Length::from_meters(0.0),
        y: Length::from_meters(0.0),
        z: Length::from_meters(1.0),
    };

    const QUARTER_TURN: Angle = Angle::from_radians(PI / 4.0);
    const HALF_TURN: Angle = Angle::from_radians(PI_HALF);
    const THREE_QUARTER_TURN: Angle = Angle::from_radians(3.0 * PI / 4.0);
    const FULL_TURN: Angle = Angle::from_radians(PI);

    #[test]
    fn test_length() {
        use super::CartesianCoordinates;
        use crate::units::length::Length;

        let coordinates = CartesianCoordinates {
            x: Length::from_meters(3.0),
            y: Length::from_meters(4.0),
            z: Length::from_meters(5.0),
        };

        assert!(coordinates
            .length()
            .eq_within(Length::from_meters(7.0710678118654755), TEST_ACCURACY));
    }

    #[test]
    fn test_rotating_x_around_z() {
        let start = X_DIRECTION;

        let rotated = start.rotated(QUARTER_TURN, unit_vector::Z_DIRECTION);
        let expected = Y_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(HALF_TURN, unit_vector::Z_DIRECTION);
        let expected = -X_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, unit_vector::Z_DIRECTION);
        let expected = -Y_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(FULL_TURN, unit_vector::Z_DIRECTION);
        let expected = X_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn test_rotating_y_around_z() {
        let start = Y_DIRECTION;

        let rotated = start.rotated(QUARTER_TURN, unit_vector::Z_DIRECTION);
        let expected = -X_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(HALF_TURN, unit_vector::Z_DIRECTION);
        let expected = -Y_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, unit_vector::Z_DIRECTION);
        let expected = X_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(FULL_TURN, unit_vector::Z_DIRECTION);
        let expected = Y_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn test_rotating_z_around_z() {
        let start = Z_DIRECTION;

        let rotated = start.rotated(QUARTER_TURN, unit_vector::Z_DIRECTION);
        let expected = Z_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(HALF_TURN, unit_vector::Z_DIRECTION);
        let expected = Z_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, unit_vector::Z_DIRECTION);
        let expected = Z_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(FULL_TURN, unit_vector::Z_DIRECTION);
        let expected = Z_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn test_rotating_x_around_x() {
        let start = X_DIRECTION;

        let rotated = start.rotated(QUARTER_TURN, unit_vector::X_DIRECTION);
        let expected = X_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(HALF_TURN, unit_vector::X_DIRECTION);
        let expected = X_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, unit_vector::X_DIRECTION);
        let expected = X_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(FULL_TURN, unit_vector::X_DIRECTION);
        let expected = X_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn test_rotating_y_around_x() {
        let start = Y_DIRECTION;

        let rotated = start.rotated(QUARTER_TURN, unit_vector::X_DIRECTION);
        let expected = Z_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(HALF_TURN, unit_vector::X_DIRECTION);
        let expected = -Y_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, unit_vector::X_DIRECTION);
        let expected = -Z_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(FULL_TURN, unit_vector::X_DIRECTION);
        let expected = Y_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn test_rotating_z_around_x() {
        let start = Z_DIRECTION;

        let rotated = start.rotated(QUARTER_TURN, unit_vector::X_DIRECTION);
        let expected = -Y_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(HALF_TURN, unit_vector::X_DIRECTION);
        let expected = -Z_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, unit_vector::X_DIRECTION);
        let expected = Y_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(FULL_TURN, unit_vector::X_DIRECTION);
        let expected = Z_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn test_rotating_x_around_y() {
        let start = X_DIRECTION;

        let rotated = start.rotated(QUARTER_TURN, unit_vector::Y_DIRECTION);
        let expected = -Z_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(HALF_TURN, unit_vector::Y_DIRECTION);
        let expected = -X_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, unit_vector::Y_DIRECTION);
        let expected = Z_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(FULL_TURN, unit_vector::Y_DIRECTION);
        let expected = X_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn test_rotating_y_around_y() {
        let start = Y_DIRECTION;

        let rotated = start.rotated(QUARTER_TURN, unit_vector::Y_DIRECTION);
        let expected = Y_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(HALF_TURN, unit_vector::Y_DIRECTION);
        let expected = Y_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, unit_vector::Y_DIRECTION);
        let expected = Y_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(FULL_TURN, unit_vector::Y_DIRECTION);
        let expected = Y_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn test_rotating_z_around_y() {
        let start = Z_DIRECTION;

        let rotated = start.rotated(QUARTER_TURN, unit_vector::Y_DIRECTION);
        let expected = X_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(HALF_TURN, unit_vector::Y_DIRECTION);
        let expected = -Z_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, unit_vector::Y_DIRECTION);
        let expected = -X_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));

        let rotated = start.rotated(FULL_TURN, unit_vector::Y_DIRECTION);
        let expected = Z_DIRECTION;
        assert!(rotated.eq_within(&expected, TEST_ACCURACY));
    }

    //TODO: test rotating around arbitrary axes
}
