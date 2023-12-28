use crate::{
    units::{angle::Angle, length::Length},
    Float,
};
use std::{
    fmt::{Display, Formatter},
    ops::{Add, Div, Mul, Neg, Sub},
};

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
    pub fn eq_within(&self, other: &CartesianCoordinates, accuracy: Length) -> bool {
        self.x.eq_within(&other.x, accuracy)
            && self.y.eq_within(&other.y, accuracy)
            && self.z.eq_within(&other.z, accuracy)
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

    pub(super) fn rotated(&self, angle: Angle, axis: &UnitVector) -> CartesianCoordinates {
        let cos = angle.cos();
        let sin = angle.sin();

        let r_11 = cos + axis.x() * axis.x() * (1.0 - cos);
        let r_12 = axis.x() * axis.y() * (1.0 - cos) - axis.z() * sin;
        let r_13 = axis.x() * axis.z() * (1.0 - cos) + axis.y() * sin;

        let r_21 = axis.y() * axis.x() * (1.0 - cos) + axis.z() * sin;
        let r_22 = cos + axis.y() * axis.y() * (1.0 - cos);
        let r_23 = axis.y() * axis.z() * (1.0 - cos) - axis.x() * sin;

        let r_31 = axis.z() * axis.x() * (1.0 - cos) - axis.y() * sin;
        let r_32 = axis.z() * axis.y() * (1.0 - cos) + axis.x() * sin;
        let r_33 = cos + axis.z() * axis.z() * (1.0 - cos);

        let x = self.x * r_11 + self.y * r_12 + self.z * r_13;
        let y = self.x * r_21 + self.y * r_22 + self.z * r_23;
        let z = self.x * r_31 + self.y * r_32 + self.z * r_33;

        CartesianCoordinates { x, y, z }
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

impl Display for CartesianCoordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::CartesianCoordinates;
    use crate::{
        coordinates::unit_vector::{UnitVector, X_DIRECTION, Y_DIRECTION, Z_DIRECTION},
        tests::TEST_LENGTH_ACCURACY,
        units::{angle::Angle, length::Length},
        Float, TWO_PI,
    };

    const X_VECTOR: CartesianCoordinates = CartesianCoordinates {
        x: Length::from_meters(1.0),
        y: Length::from_meters(0.0),
        z: Length::from_meters(0.0),
    };

    const Y_VECTOR: CartesianCoordinates = CartesianCoordinates {
        x: Length::from_meters(0.0),
        y: Length::from_meters(1.0),
        z: Length::from_meters(0.0),
    };

    const Z_VECTOR: CartesianCoordinates = CartesianCoordinates {
        x: Length::from_meters(0.0),
        y: Length::from_meters(0.0),
        z: Length::from_meters(1.0),
    };

    const QUARTER_TURN: Angle = Angle::from_radians(TWO_PI / 4.0);
    const HALF_TURN: Angle = Angle::from_radians(TWO_PI / 2.0);
    const THREE_QUARTER_TURN: Angle = Angle::from_radians(3.0 / 4.0 * TWO_PI);
    const FULL_TURN: Angle = Angle::from_radians(TWO_PI);
    const ONE_THIRD_TURN: Angle = Angle::from_radians(TWO_PI / 3.0);
    const TWO_THIRDS_TURN: Angle = Angle::from_radians(2.0 / 3.0 * TWO_PI);

    #[test]
    fn test_length() {
        use super::CartesianCoordinates;
        use crate::units::length::Length;

        let coordinates = CartesianCoordinates {
            x: Length::from_meters(3.0),
            y: Length::from_meters(4.0),
            z: Length::from_meters(5.0),
        };

        assert!(coordinates.length().eq_within(
            &Length::from_meters(7.0710678118654755),
            TEST_LENGTH_ACCURACY
        ));
    }

    #[test]
    fn test_rotating_x_around_z() {
        let start = X_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &Z_DIRECTION);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &Z_DIRECTION);
        let expected = -X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &Z_DIRECTION);
        let expected = -Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &Z_DIRECTION);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_y_around_z() {
        let start = Y_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &Z_DIRECTION);
        let expected = -X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &Z_DIRECTION);
        let expected = -Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &Z_DIRECTION);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &Z_DIRECTION);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_z_around_z() {
        let start = Z_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &Z_DIRECTION);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &Z_DIRECTION);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &Z_DIRECTION);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &Z_DIRECTION);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_x_around_x() {
        let start = X_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &X_DIRECTION);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &X_DIRECTION);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &X_DIRECTION);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &X_DIRECTION);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_y_around_x() {
        let start = Y_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &X_DIRECTION);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &X_DIRECTION);
        let expected = -Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &X_DIRECTION);
        let expected = -Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &X_DIRECTION);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_z_around_x() {
        let start = Z_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &X_DIRECTION);
        let expected = -Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &X_DIRECTION);
        let expected = -Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &X_DIRECTION);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &X_DIRECTION);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_x_around_y() {
        let start = X_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &Y_DIRECTION);
        let expected = -Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &Y_DIRECTION);
        let expected = -X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &Y_DIRECTION);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &Y_DIRECTION);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_y_around_y() {
        let start = Y_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &Y_DIRECTION);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &Y_DIRECTION);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &Y_DIRECTION);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &Y_DIRECTION);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_z_around_y() {
        let start = Z_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &Y_DIRECTION);
        let expected = -X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &Y_DIRECTION);
        let expected = -Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &Y_DIRECTION);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &Y_DIRECTION);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_around_diagonal_axis() {
        let start = X_VECTOR;
        let axis = UnitVector::from_cartesian(&CartesianCoordinates {
            x: Length::from_meters(1.0),
            y: Length::from_meters(1.0),
            z: Length::from_meters(0.0),
        });

        let rotated = X_VECTOR.rotated(ONE_THIRD_TURN, &axis);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = Y_VECTOR.rotated(ONE_THIRD_TURN, &axis);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = Z_VECTOR.rotated(ONE_THIRD_TURN, &axis);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = X_VECTOR.rotated(TWO_THIRDS_TURN, &axis);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = Y_VECTOR.rotated(TWO_THIRDS_TURN, &axis);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = Z_VECTOR.rotated(TWO_THIRDS_TURN, &axis);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = X_VECTOR.rotated(-ONE_THIRD_TURN, &axis);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = Y_VECTOR.rotated(-ONE_THIRD_TURN, &axis);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = Z_VECTOR.rotated(-ONE_THIRD_TURN, &axis);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }
}
