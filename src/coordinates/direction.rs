use super::{
    cartesian::CartesianCoordinates, rotations::rotated_tuple, spherical::SphericalCoordinates,
};
use crate::{
    units::{angle::Angle, length::Length},
    Float, PI,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Neg};

pub(super) const NORMALIZATION_THRESHOLD: Float = 1e-10;

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
        if length < NORMALIZATION_THRESHOLD {
            Z //return default axis
        } else {
            Direction {
                x: x / length,
                y: y / length,
                z: z / length,
            }
        }
    }

    pub fn from_cartesian(cartesian: &CartesianCoordinates) -> Self {
        let length = cartesian.length();
        Direction {
            x: cartesian.x() / length,
            y: cartesian.y() / length,
            z: cartesian.z() / length,
        }
    }

    pub fn from_spherical(spherical: &SphericalCoordinates) -> Self {
        let x = spherical.get_longitude().cos() * spherical.get_latitude().cos();
        let y = spherical.get_longitude().sin() * spherical.get_latitude().cos();
        let z = spherical.get_latitude().sin();
        Direction { x, y, z }
    }

    pub fn to_cartesian(&self, length: Length) -> CartesianCoordinates {
        CartesianCoordinates::new(self.x * length, self.y * length, self.z * length)
    }

    pub fn x(&self) -> Float {
        self.x
    }

    pub fn y(&self) -> Float {
        self.y
    }

    pub fn z(&self) -> Float {
        self.z
    }

    pub fn rotated(&self, angle: Angle, axis: &Direction) -> Direction {
        let (x, y, z) = rotated_tuple((self.x, self.y, self.z), angle, axis);
        Direction { x, y, z }
    }

    pub fn eq_within(&self, other: &Direction, accuracy: Float) -> bool {
        (self.x - other.x).abs() < accuracy
            && (self.y - other.y).abs() < accuracy
            && (self.z - other.z).abs() < accuracy
    }

    pub fn angle_to(&self, other: &Direction) -> Angle {
        let (ax, ay, az) = (self.x(), self.y(), self.z());
        let (bx, by, bz) = (other.x(), other.y(), other.z());

        let cosine_argument = ax * bx + ay * by + az * bz; //Directions have unit length
        if cosine_argument > 1. {
            //Saving acos from being called with an argument > 1 due to numerical instability
            return Angle::from_radians(0.);
        } else if cosine_argument < -1. {
            return Angle::from_radians(PI);
        }
        Angle::from_radians(cosine_argument.acos())
    }

    fn some_orthogonal_vector(&self) -> Direction {
        if self.x().abs() > NORMALIZATION_THRESHOLD {
            self.cross_product(&Y)
        } else if self.y().abs() > NORMALIZATION_THRESHOLD {
            self.cross_product(&Z)
        } else if self.z().abs() > NORMALIZATION_THRESHOLD {
            self.cross_product(&X)
        } else {
            //vector is (0,0,0)
            Z
        }
    }

    pub(super) fn cross_product(&self, other: &Direction) -> Direction {
        let (ax, ay, az) = (self.x, self.y, self.z);
        let (bx, by, bz) = (other.x(), other.y(), other.z());

        let cx = ay * bz - az * by;
        let cy = az * bx - ax * bz;
        let cz = ax * by - ay * bx;

        let cross_product_length = (cx * cx + cy * cy + cz * cz).sqrt();
        if cross_product_length < NORMALIZATION_THRESHOLD {
            self.some_orthogonal_vector()
        } else {
            Direction::new(cx, cy, cz)
        }
    }

    #[cfg(test)]
    fn dot_product(&self, other: &Direction) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        Direction {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
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

    const ROTATION_ANGLE_ACCURACY: Angle = Angle::from_radians(1e-3); //Accos is a bit unstable

    #[test]
    fn from_spherical() {
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

                    let spherical = SphericalCoordinates::from_cartesian(&cartesian);
                    let direction = Direction::from_spherical(&spherical);

                    assert!((direction.x() - expected_x).abs() < TEST_ACCURACY);
                    assert!((direction.y() - expected_y).abs() < TEST_ACCURACY);
                    assert!((direction.z() - expected_z).abs() < TEST_ACCURACY);
                }
            }
        }
    }

    #[test]
    fn angle_between_is_half_turn() {
        const EXPECTED: Angle = Angle::from_radians(PI);

        let angle = X.angle_to(&(-X));
        println!("angle: {}", angle);
        assert!(angle.eq_within(EXPECTED, ROTATION_ANGLE_ACCURACY));

        let angle = Y.angle_to(&(-Y));
        println!("angle: {}", angle);
        assert!(angle.eq_within(EXPECTED, ROTATION_ANGLE_ACCURACY));

        let angle = Z.angle_to(&(-Z));
        println!("angle: {}", angle);
        assert!(angle.eq_within(EXPECTED, ROTATION_ANGLE_ACCURACY));

        let angle1 = Direction::new(1., 1., 0.);
        let angle2 = Direction::new(-1., -1., 0.);
        let angle = angle1.angle_to(&angle2);
        println!("angle: {}", angle);
        assert!(angle.eq_within(EXPECTED, ROTATION_ANGLE_ACCURACY));

        let angle1 = Direction::new(1., 0., 1.);
        let angle2 = Direction::new(-1., 0., -1.);
        let angle = angle1.angle_to(&angle2);
        println!("angle: {}", angle);
        assert!(angle.eq_within(EXPECTED, ROTATION_ANGLE_ACCURACY));

        let angle1 = Direction::new(0., 1., 1.);
        let angle2 = Direction::new(0., -1., -1.);
        let angle = angle1.angle_to(&angle2);
        println!("angle: {}", angle);
        assert!(angle.eq_within(EXPECTED, ROTATION_ANGLE_ACCURACY));
    }

    #[test]
    fn angle_between_is_quarter_turn() {
        let expected = Angle::from_radians(PI / 2.);

        let angle = X.angle_to(&Y);
        println!("expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, ROTATION_ANGLE_ACCURACY));

        let angle = X.angle_to(&Z);
        println!("expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, ROTATION_ANGLE_ACCURACY));

        let angle = Y.angle_to(&Z);
        println!("expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, ROTATION_ANGLE_ACCURACY));

        let angle1 = Direction::new(1., 1., 0.);
        let angle2 = Direction::new(1., -1., 0.);
        let angle = angle1.angle_to(&angle2);
        println!("expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, ROTATION_ANGLE_ACCURACY));

        let angle1 = Direction::new(1., 0., 1.);
        let angle2 = Direction::new(1., 0., -1.);
        let angle = angle1.angle_to(&angle2);
        println!("expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, ROTATION_ANGLE_ACCURACY));
    }

    #[test]
    fn angle_between_is_zero() {
        const EXPECTED: Angle = Angle::from_radians(0.);

        let angle = X.angle_to(&X);
        println!("expected: {}, actual: {}", EXPECTED, angle);
        assert!(angle.eq_within(EXPECTED, ROTATION_ANGLE_ACCURACY));

        let angle = Y.angle_to(&Y);
        println!("expected: {}, actual: {}", EXPECTED, angle);
        assert!(angle.eq_within(EXPECTED, ROTATION_ANGLE_ACCURACY));

        let angle = Z.angle_to(&Z);
        println!("expected: {}, actual: {}", EXPECTED, angle);
        assert!(angle.eq_within(EXPECTED, ROTATION_ANGLE_ACCURACY));

        let angle1 = Direction::new(1., 1., 0.);
        let angle2 = Direction::new(1., 1., 0.);
        let angle = angle1.angle_to(&angle2);
        println!("expected: {}, actual: {}", EXPECTED, angle);
        assert!(angle.eq_within(EXPECTED, ROTATION_ANGLE_ACCURACY));
    }

    #[test]
    fn test_cross_product() {
        let angle1 = Direction::new(1., 0., 0.);
        let angle2 = Direction::new(0., 1., 0.);
        let expected = Direction::new(0., 0., 1.);
        let actual = angle1.cross_product(&angle2);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));

        let angle1 = Direction::new(1., 0., 0.);
        let angle2 = Direction::new(0., 0., 1.);
        let expected = Direction::new(0., -1., 0.);
        let actual = angle1.cross_product(&angle2);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));

        let angle1 = Direction::new(0., 1., 0.);
        let angle2 = Direction::new(0., 0., 1.);
        let expected = Direction::new(1., 0., 0.);
        let actual = angle1.cross_product(&angle2);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));

        let angle1 = Direction::new(0., 1., 0.);
        let angle2 = Direction::new(0., 0., -1.);
        let expected = Direction::new(-1., 0., 0.);
        let actual = angle1.cross_product(&angle2);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));

        let angle1 = Direction::new(0., 0., 1.);
        let angle2 = Direction::new(0., 1., 0.);
        let expected = Direction::new(-1., 0., 0.);
        let actual = angle1.cross_product(&angle2);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));

        let angle1 = Direction::new(0., 0., 1.);
        let angle2 = Direction::new(1., 0., 0.);
        let expected = Direction::new(0., 1., 0.);
        let actual = angle1.cross_product(&angle2);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn cross_product_is_always_orthogonal() {
        let problematic = Direction::new(0., 0., 0.);
        let ordinates = vec![-1., 0., 1., 10.];
        for x in ordinates.clone().iter() {
            for y in ordinates.clone().iter() {
                for z in ordinates.clone().iter() {
                    for u in ordinates.clone().iter() {
                        for v in ordinates.clone().iter() {
                            for w in ordinates.clone().iter() {
                                let a = Direction::new(*x, *y, *z);
                                let b = Direction::new(*u, *v, *w);
                                println!("a: {}, b: {}", a, b);
                                if a.eq_within(&problematic, TEST_ACCURACY)
                                    || b.eq_within(&problematic, TEST_ACCURACY)
                                {
                                    continue;
                                }
                                let cross = a.cross_product(&b);
                                let overlap_with_a = cross.dot_product(&a);
                                let overlap_with_b = cross.dot_product(&b);
                                println!(
                                    "cross: {}, overlap_with_a: {}, overlap_with_b: {}",
                                    cross, overlap_with_a, overlap_with_b
                                );
                                assert!(overlap_with_a.abs() < TEST_ACCURACY);
                                assert!(overlap_with_b.abs() < TEST_ACCURACY);
                            }
                        }
                    }
                }
            }
        }
    }
}
