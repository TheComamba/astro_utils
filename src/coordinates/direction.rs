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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Direction {
    pub(super) x: Float,
    pub(super) y: Float,
    pub(super) z: Float,
}

impl Direction {
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

    pub fn new(x: Float, y: Float, z: Float) -> Self {
        let length = (x * x + y * y + z * z).sqrt();
        if length < NORMALIZATION_THRESHOLD {
            Self::Z //return default axis
        } else {
            Direction {
                x: x / length,
                y: y / length,
                z: z / length,
            }
        }
    }

    pub fn to_cartesian(&self, length: Length) -> CartesianCoordinates {
        CartesianCoordinates::new(self.x * length, self.y * length, self.z * length)
    }

    pub fn to_spherical(&self) -> SphericalCoordinates {
        SphericalCoordinates::cartesian_to_spherical((self.x, self.y, self.z))
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

    #[cfg(test)]
    pub(crate) fn eq_within(&self, other: &Direction, accuracy: Float) -> bool {
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
            return Angle::ZERO;
        } else if cosine_argument < -1. {
            return Angle::from_radians(PI);
        }
        Angle::from_radians(cosine_argument.acos())
    }

    fn some_orthogonal_vector(&self) -> Direction {
        if self.x().abs() > NORMALIZATION_THRESHOLD {
            self.cross_product(&Self::Y)
        } else if self.y().abs() > NORMALIZATION_THRESHOLD {
            self.cross_product(&Self::Z)
        } else if self.z().abs() > NORMALIZATION_THRESHOLD {
            self.cross_product(&Self::X)
        } else {
            //vector is (0,0,0)
            Self::Z
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

    /*
     * This method is for example used to convert from equatorial coordinates to ecliptic coordinates.
     * It operates the following way:
     * 1. The vector is rotated around the X axis by the angle between new and old Z axis.
     * 2. The vector is rotated around old Z by the angle between new and old X, projected on the old X-Y plane.
     * The result is the coordinates. The new X-axis still lies in the old X-Y plane.
     */
    pub fn active_rotation_to_new_z_axis(&self, new_z: &Direction) -> Direction {
        let angle_to_old_z = new_z.angle_to(&Self::Z);

        let axis_projected_onto_xy_plane = Direction::new(new_z.x(), new_z.y(), 0.);
        let mut polar_rotation_angle = axis_projected_onto_xy_plane.angle_to(&Self::Y);
        if axis_projected_onto_xy_plane.x() < 0. {
            polar_rotation_angle = -polar_rotation_angle;
        }

        let mut dir = self.rotated(-angle_to_old_z, &Self::X);
        dir = dir.rotated(-polar_rotation_angle, &Self::Z);
        dir
    }

    pub fn passive_rotation_to_new_z_axis(&self, new_z: &Direction) -> Direction {
        let axis_projected_onto_xy_plane = Direction::new(new_z.x(), new_z.y(), 0.);
        let mut polar_rotation_angle = axis_projected_onto_xy_plane.angle_to(&Self::Y);
        if axis_projected_onto_xy_plane.x() < 0. {
            polar_rotation_angle = -polar_rotation_angle;
        }

        let angle_to_old_z = new_z.angle_to(&Self::Z);

        let mut dir = self.rotated(polar_rotation_angle, &Self::Z);
        dir = dir.rotated(angle_to_old_z, &Self::X);
        dir
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
                    let direction = cartesian.to_spherical().to_direction();

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

        let angle = Direction::X.angle_to(&(-Direction::X));
        println!("angle: {}", angle);
        assert!(angle.eq_within(EXPECTED, ROTATION_ANGLE_ACCURACY));

        let angle = Direction::Y.angle_to(&(-Direction::Y));
        println!("angle: {}", angle);
        assert!(angle.eq_within(EXPECTED, ROTATION_ANGLE_ACCURACY));

        let angle = Direction::Z.angle_to(&(-Direction::Z));
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

        let angle = Direction::X.angle_to(&Direction::Y);
        println!("expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, ROTATION_ANGLE_ACCURACY));

        let angle = Direction::X.angle_to(&Direction::Z);
        println!("expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, ROTATION_ANGLE_ACCURACY));

        let angle = Direction::Y.angle_to(&Direction::Z);
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
        const EXPECTED: Angle = Angle::ZERO;

        let angle = Direction::X.angle_to(&Direction::X);
        println!("expected: {}, actual: {}", EXPECTED, angle);
        assert!(angle.eq_within(EXPECTED, ROTATION_ANGLE_ACCURACY));

        let angle = Direction::Y.angle_to(&Direction::Y);
        println!("expected: {}, actual: {}", EXPECTED, angle);
        assert!(angle.eq_within(EXPECTED, ROTATION_ANGLE_ACCURACY));

        let angle = Direction::Z.angle_to(&Direction::Z);
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

    #[test]
    fn test_revertability_of_rotation() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        for x1 in ordinates.clone() {
            for y1 in ordinates.clone() {
                for z1 in ordinates.clone() {
                    for x2 in ordinates.clone() {
                        for y2 in ordinates.clone() {
                            for z2 in ordinates.clone() {
                                if x1.abs() < TEST_ACCURACY
                                    && y1.abs() < TEST_ACCURACY
                                    && z1.abs() < TEST_ACCURACY
                                {
                                    continue;
                                }
                                if x2.abs() < TEST_ACCURACY
                                    && y2.abs() < TEST_ACCURACY
                                    && z2.abs() < TEST_ACCURACY
                                {
                                    continue;
                                }

                                let original_dir = Direction::new(x1, y1, z1);
                                let new_z_axis = Direction::new(x2, y2, z2);

                                let after_active_rotation =
                                    original_dir.active_rotation_to_new_z_axis(&new_z_axis);
                                let transformed_back = after_active_rotation
                                    .passive_rotation_to_new_z_axis(&new_z_axis);

                                println!("new_z_axis: {}", new_z_axis);
                                println!("original_dir: {}", original_dir);
                                println!("after_active_rotation: {}", after_active_rotation);
                                println!("transformed_back: {}", transformed_back);
                                assert!(original_dir.eq_within(&transformed_back, TEST_ACCURACY));
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_north_after_active_rotation() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        for x in ordinates.clone() {
            for y in ordinates.clone() {
                for z in ordinates.clone() {
                    if x.abs() < TEST_ACCURACY && y.abs() < TEST_ACCURACY && z.abs() < TEST_ACCURACY
                    {
                        continue;
                    }

                    let new_z_axis = Direction::new(x, y, z);

                    let expected = new_z_axis.clone();
                    let actual = Direction::Z.active_rotation_to_new_z_axis(&new_z_axis);

                    println!("new_z_axis: {}", new_z_axis);
                    println!("expected: {}", expected);
                    println!("actual: {}", actual);
                    assert!(expected.eq_within(&actual, TEST_ACCURACY));
                }
            }
        }
    }

    #[test]
    fn test_north_after_passive_rotation() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        for x in ordinates.clone() {
            for y in ordinates.clone() {
                for z in ordinates.clone() {
                    if x.abs() < TEST_ACCURACY && y.abs() < TEST_ACCURACY && z.abs() < TEST_ACCURACY
                    {
                        continue;
                    }

                    let new_z_axis = Direction::new(x, y, z);

                    let expected = Direction::Z;
                    let actual = new_z_axis.passive_rotation_to_new_z_axis(&new_z_axis);

                    println!("new_z_axis: {}", new_z_axis);
                    println!("expected: {}", expected);
                    println!("actual: {}", actual);
                    assert!(expected.eq_within(&actual, TEST_ACCURACY));
                }
            }
        }
    }

    #[test]
    fn direction_from_large_vector_is_ok() {
        let x = Length::from_light_years(2000.);
        let y = Length::from_light_years(1e-10);
        let z = Length::from_light_years(-2000.);
        let cartesian = CartesianCoordinates::new(x, y, z);
        let expected = Direction::new(1., 0., -1.);
        let actual = cartesian.to_direction();
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }
}
