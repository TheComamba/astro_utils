use super::direction::Direction;
use crate::{
    units::{angle::Angle, length::Length},
    Float,
};
use std::{
    fmt::{Display, Formatter},
    ops::{Add, Div, Mul, Neg, Sub},
};

pub const ORIGIN: CartesianCoordinates = CartesianCoordinates {
    x: Length::from_meters(0.),
    y: Length::from_meters(0.),
    z: Length::from_meters(0.),
};

#[derive(Debug, Copy, Clone)]
pub struct CartesianCoordinates {
    x: Length,
    y: Length,
    z: Length,
}

impl CartesianCoordinates {
    pub const fn new(x: Length, y: Length, z: Length) -> CartesianCoordinates {
        CartesianCoordinates { x, y, z }
    }

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

    pub fn rotated(&self, angle: Angle, axis: &Direction) -> CartesianCoordinates {
        let cos = angle.cos();
        let sin = angle.sin();

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
        coordinates::direction::{Direction, X, Y, Z},
        tests::TEST_LENGTH_ACCURACY,
        units::{angle::Angle, length::Length},
        TWO_PI,
    };

    pub(super) const X_VECTOR: CartesianCoordinates = CartesianCoordinates {
        x: Length::from_meters(1.),
        y: Length::from_meters(0.),
        z: Length::from_meters(0.),
    };

    pub(super) const Y_VECTOR: CartesianCoordinates = CartesianCoordinates {
        x: Length::from_meters(0.),
        y: Length::from_meters(1.),
        z: Length::from_meters(0.),
    };

    pub(super) const Z_VECTOR: CartesianCoordinates = CartesianCoordinates {
        x: Length::from_meters(0.),
        y: Length::from_meters(0.),
        z: Length::from_meters(1.),
    };

    const QUARTER_TURN: Angle = Angle::from_radians(TWO_PI / 4.);
    const HALF_TURN: Angle = Angle::from_radians(TWO_PI / 2.);
    const THREE_QUARTER_TURN: Angle = Angle::from_radians(3. / 4. * TWO_PI);
    const FULL_TURN: Angle = Angle::from_radians(TWO_PI);
    const ONE_THIRD_TURN: Angle = Angle::from_radians(TWO_PI / 3.);
    const TWO_THIRDS_TURN: Angle = Angle::from_radians(2. / 3. * TWO_PI);

    #[test]
    fn test_length() {
        use super::CartesianCoordinates;
        use crate::units::length::Length;

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

    #[test]
    fn test_rotating_x_around_z() {
        let start = X_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &Z);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &Z);
        let expected = -X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &Z);
        let expected = -Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &Z);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_y_around_z() {
        let start = Y_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &Z);
        let expected = -X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &Z);
        let expected = -Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &Z);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &Z);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_z_around_z() {
        let start = Z_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &Z);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &Z);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &Z);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &Z);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_x_around_x() {
        let start = X_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &X);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &X);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &X);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &X);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_y_around_x() {
        let start = Y_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &X);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &X);
        let expected = -Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &X);
        let expected = -Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &X);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_z_around_x() {
        let start = Z_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &X);
        let expected = -Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &X);
        let expected = -Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &X);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &X);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_x_around_y() {
        let start = X_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &Y);
        let expected = -Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &Y);
        let expected = -X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &Y);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &Y);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_y_around_y() {
        let start = Y_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &Y);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &Y);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &Y);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &Y);
        let expected = Y_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_z_around_y() {
        let start = Z_VECTOR;

        let rotated = start.rotated(QUARTER_TURN, &Y);
        let expected = X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(HALF_TURN, &Y);
        let expected = -Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(THREE_QUARTER_TURN, &Y);
        let expected = -X_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));

        let rotated = start.rotated(FULL_TURN, &Y);
        let expected = Z_VECTOR;
        println!("expected {}, actual {}", expected, rotated);
        assert!(rotated.eq_within(&expected, TEST_LENGTH_ACCURACY));
    }

    #[test]
    fn test_rotating_around_diagonal_axis() {
        let axis = Direction::from_cartesian(&CartesianCoordinates {
            x: Length::from_meters(1.),
            y: Length::from_meters(1.),
            z: Length::from_meters(1.),
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
