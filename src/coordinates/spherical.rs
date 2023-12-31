use super::{
    cartesian::CartesianCoordinates,
    direction::{Direction, X, Y, Z},
};
use crate::{units::angle::Angle, Float, PI};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    ops::Neg,
};

pub const X_DIRECTION: SphericalCoordinates = SphericalCoordinates {
    longitude: Angle::from_radians(0.),
    latitude: Angle::from_radians(0.),
};

pub const Y_DIRECTION: SphericalCoordinates = SphericalCoordinates {
    longitude: Angle::from_radians(PI / 2.),
    latitude: Angle::from_radians(0.),
};

pub const Z_DIRECTION: SphericalCoordinates = SphericalCoordinates {
    longitude: Angle::from_radians(0.),
    latitude: Angle::from_radians(PI / 2.),
};

const PI_HALF: Float = PI / 2.;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct SphericalCoordinates {
    longitude: Angle,
    latitude: Angle,
}

impl SphericalCoordinates {
    pub const fn new(longitude: Angle, latitude: Angle) -> Self {
        Self {
            longitude,
            latitude,
        }
    }
    pub fn normalize(&mut self) {
        self.longitude.normalize();
        self.latitude.normalize();
        if self.latitude.as_radians() > PI_HALF {
            self.longitude = self.longitude + Angle::from_radians(PI);
            self.longitude.normalize();
            self.latitude = Angle::from_radians(PI) - self.latitude;
        } else if self.latitude.as_radians() < -PI_HALF {
            self.longitude = self.longitude + Angle::from_radians(PI);
            self.longitude.normalize();
            self.latitude = Angle::from_radians(-PI) - self.latitude;
        }
    }

    pub fn eq_within(&self, other: &Self, accuracy: Angle) -> bool {
        const NORTHPOLE_LATITUDE: Angle = Angle::from_radians(PI_HALF);
        const SOUTHPOLE_LATITUDE: Angle = Angle::from_radians(-PI_HALF);
        let mut clone = self.clone();
        let mut other_clone = other.clone();
        clone.normalize();
        other_clone.normalize();
        let latitudes_equal = clone.latitude.eq_within(other_clone.latitude, accuracy);
        let is_pole = clone.latitude.eq_within(NORTHPOLE_LATITUDE, accuracy)
            || clone.latitude.eq_within(SOUTHPOLE_LATITUDE, accuracy);
        let longitudes_equal = if is_pole {
            true
        } else {
            clone.longitude.eq_within(other_clone.longitude, accuracy)
        };
        latitudes_equal && longitudes_equal
    }

    pub fn get_longitude(&self) -> Angle {
        self.longitude
    }

    pub fn get_latitude(&self) -> Angle {
        self.latitude
    }

    pub fn set_longitude(&mut self, longitude: Angle) {
        self.longitude = longitude;
    }

    pub fn set_latitude(&mut self, latitude: Angle) {
        self.latitude = latitude;
    }

    pub fn from_cartesian(cart: &CartesianCoordinates) -> Self {
        Self::cartesian_to_spherical((
            cart.x().as_meters(),
            cart.y().as_meters(),
            cart.z().as_meters(),
        ))
    }

    pub fn from_direction(dir: &Direction) -> Self {
        Self::cartesian_to_spherical((dir.x(), dir.y(), dir.z()))
    }

    fn cartesian_to_spherical(cart: (Float, Float, Float)) -> Self {
        let (x, y, z) = cart;
        let longitude = Angle::from_radians(y.atan2(x));
        let latitude = Angle::from_radians(z.atan2((x * x + y * y).sqrt()));
        Self {
            longitude,
            latitude,
        }
    }

    /*
     * This method is for example used to convert from equatorial coordinates to ecliptic coordinates.
     * It operates the following way:
     * 1. The spherical coordinates are converted to a direction vector.
     * 2. The vector is rotated around the X axis by the angle between new and old Z axis.
     * 3. The vector is rotated around old Z by the angle between new and old X, projected on the old X-Y plane.
     * The result is the coordinates. The new X-axis still lies in the old X-Y plane.
     */
    pub fn direction_after_active_rotation_to_new_z_axis(&self, new_z: &Direction) -> Direction {
        let angle_to_old_z = new_z.angle_to(&Z);

        let axis_projected_onto_xy_plane = Direction::new(new_z.x(), new_z.y(), 0.);
        let mut polar_rotation_angle = axis_projected_onto_xy_plane.angle_to(&Y);
        if axis_projected_onto_xy_plane.x() < 0. {
            polar_rotation_angle = -polar_rotation_angle;
        }

        let mut dir = Direction::from_spherical(&self);
        dir = dir.rotated(-angle_to_old_z, &X);
        dir = dir.rotated(-polar_rotation_angle, &Z);
        dir
    }

    pub fn active_rotation_to_new_z_axis(&self, new_z: &Direction) -> Self {
        Self::from_direction(&self.direction_after_active_rotation_to_new_z_axis(new_z))
    }

    pub fn direction_after_passive_rotation_to_new_z_axis(&self, new_z: &Direction) -> Direction {
        let axis_projected_onto_xy_plane = Direction::new(new_z.x(), new_z.y(), 0.);
        let mut polar_rotation_angle = axis_projected_onto_xy_plane.angle_to(&Y);
        if axis_projected_onto_xy_plane.x() < 0. {
            polar_rotation_angle = -polar_rotation_angle;
        }

        let angle_to_old_z = new_z.angle_to(&Z);

        let mut dir = Direction::from_spherical(&self);
        dir = dir.rotated(polar_rotation_angle, &Z);
        dir = dir.rotated(angle_to_old_z, &X);
        dir
    }

    pub fn passive_rotation_to_new_z_axis(&self, new_z: &Direction) -> Self {
        Self::from_direction(&self.direction_after_passive_rotation_to_new_z_axis(new_z))
    }
}

impl Neg for &SphericalCoordinates {
    type Output = SphericalCoordinates;

    fn neg(self) -> SphericalCoordinates {
        let mut longitude = self.longitude + Angle::from_radians(PI);
        longitude.normalize();
        let latitude = -self.latitude;
        SphericalCoordinates {
            longitude,
            latitude,
        }
    }
}

impl Neg for SphericalCoordinates {
    type Output = SphericalCoordinates;

    fn neg(self) -> SphericalCoordinates {
        -&self
    }
}

impl Display for SphericalCoordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} long, {} lat)", self.longitude, self.latitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tests::TEST_ANGLE_ACCURACY, units::length::Length, TWO_PI};

    #[test]
    fn test_from_cartesian() {
        let cartesian = CartesianCoordinates::new(
            Length::from_meters(1.),
            Length::from_meters(1.),
            Length::from_meters(1.),
        );
        let expected: SphericalCoordinates = SphericalCoordinates {
            longitude: Angle::from_degrees(45.),
            latitude: Angle::from_degrees(90. - 54.7356),
        };
        let actual = SphericalCoordinates::from_cartesian(&cartesian);
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let cartesian = CartesianCoordinates::new(
            Length::from_meters(1.),
            Length::from_meters(1.),
            Length::from_meters(-1.),
        );
        let expected = SphericalCoordinates {
            longitude: Angle::from_degrees(45.),
            latitude: Angle::from_degrees(-90. + 54.7356),
        };
        let actual = SphericalCoordinates::from_cartesian(&cartesian);
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let cartesian = CartesianCoordinates::new(
            Length::from_meters(1.),
            Length::from_meters(-1.),
            Length::from_meters(1.),
        );
        let expected = SphericalCoordinates {
            longitude: Angle::from_degrees(-45.),
            latitude: Angle::from_degrees(90. - 54.7356),
        };
        let actual = SphericalCoordinates::from_cartesian(&cartesian);
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let cartesian = CartesianCoordinates::new(
            Length::from_meters(1.),
            Length::from_meters(-1.),
            Length::from_meters(-1.),
        );
        let expected = SphericalCoordinates {
            longitude: Angle::from_degrees(-45.),
            latitude: Angle::from_degrees(-90. + 54.7356),
        };
        let actual = SphericalCoordinates::from_cartesian(&cartesian);
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let cartesian = CartesianCoordinates::new(
            Length::from_meters(-1.),
            Length::from_meters(1.),
            Length::from_meters(1.),
        );
        let expected = SphericalCoordinates {
            longitude: Angle::from_degrees(135.),
            latitude: Angle::from_degrees(90. - 54.7356),
        };
        let actual = SphericalCoordinates::from_cartesian(&cartesian);
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let cartesian = CartesianCoordinates::new(
            Length::from_meters(-1.),
            Length::from_meters(1.),
            Length::from_meters(-1.),
        );
        let expected = SphericalCoordinates {
            longitude: Angle::from_degrees(135.),
            latitude: Angle::from_degrees(-90. + 54.7356),
        };
        let actual = SphericalCoordinates::from_cartesian(&cartesian);
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let cartesian = CartesianCoordinates::new(
            Length::from_meters(-1.),
            Length::from_meters(-1.),
            Length::from_meters(1.),
        );
        let expected = SphericalCoordinates {
            longitude: Angle::from_degrees(-135.),
            latitude: Angle::from_degrees(90. - 54.7356),
        };
        let actual = SphericalCoordinates::from_cartesian(&cartesian);
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let cartesian = CartesianCoordinates::new(
            Length::from_meters(-1.),
            Length::from_meters(-1.),
            Length::from_meters(-1.),
        );
        let expected = SphericalCoordinates {
            longitude: Angle::from_degrees(-135.),
            latitude: Angle::from_degrees(-90. + 54.7356),
        };
        let actual = SphericalCoordinates::from_cartesian(&cartesian);
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn test_unary_minus() {
        let x = X_DIRECTION;
        let y = Y_DIRECTION;
        let z = Z_DIRECTION;
        let xyz = SphericalCoordinates {
            longitude: Angle::from_radians(PI / 4.),
            latitude: Angle::from_radians(PI / 4.),
        };
        let expected_minus_x = SphericalCoordinates {
            longitude: Angle::from_radians(PI),
            latitude: Angle::from_radians(0.),
        };
        let expected_minus_y = SphericalCoordinates {
            longitude: Angle::from_radians(-PI_HALF),
            latitude: Angle::from_radians(0.),
        };
        let expected_minus_z = SphericalCoordinates {
            longitude: Angle::from_radians(0.),
            latitude: Angle::from_radians(-PI_HALF),
        };
        let expected_minus_xyz = SphericalCoordinates {
            longitude: Angle::from_radians(-PI * 3. / 4.),
            latitude: Angle::from_radians(-PI / 4.),
        };

        let minus_x = -x;
        println!("-x, expected: {}, actual: {}", expected_minus_x, minus_x);
        assert!(minus_x.eq_within(&expected_minus_x, TEST_ANGLE_ACCURACY));

        let minus_y = -y;
        println!("-y, expected: {}, actual: {}", expected_minus_y, minus_y);
        assert!(minus_y.eq_within(&expected_minus_y, TEST_ANGLE_ACCURACY));

        let minus_z = -z;
        println!("-z, expected: {}, actual: {}", expected_minus_z, minus_z);
        assert!(minus_z.eq_within(&expected_minus_z, TEST_ANGLE_ACCURACY));

        let minus_xyz = -xyz;
        println!(
            "-xyz, expected: {}, actual: {}",
            expected_minus_xyz, minus_xyz
        );
        assert!(minus_xyz.eq_within(&expected_minus_xyz, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn test_eq_within() {
        let local_test_angle_accuracy: Angle = 10. * TEST_ANGLE_ACCURACY;

        let small_offsets = vec![
            -local_test_angle_accuracy / 100.,
            Angle::from_radians(0.),
            local_test_angle_accuracy / 100.,
        ];
        let large_offsets = vec![-TWO_PI, 0., TWO_PI, 100. * TWO_PI];
        let directions = vec![
            X_DIRECTION,
            Y_DIRECTION,
            Z_DIRECTION,
            -X_DIRECTION,
            -Y_DIRECTION,
            -Z_DIRECTION,
        ];
        for direction in directions.clone() {
            for small_offset in small_offsets.clone() {
                for large_offset in large_offsets.clone() {
                    let longitude1 = direction.longitude;
                    let latitude1 = direction.latitude;
                    let longitude2 = longitude1 + Angle::from_radians(large_offset) + small_offset;
                    let latitude2 = latitude1 + Angle::from_radians(large_offset) + small_offset;
                    let coords1 = SphericalCoordinates {
                        longitude: longitude2,
                        latitude: latitude1,
                    };
                    let coords2 = SphericalCoordinates {
                        longitude: longitude1,
                        latitude: latitude2,
                    };
                    let coords3 = SphericalCoordinates {
                        longitude: longitude2,
                        latitude: latitude2,
                    };
                    println!("Expecting {} == {}", direction, coords1);
                    assert![direction.eq_within(&coords1, local_test_angle_accuracy)];
                    println!("Expecting {} == {}", direction, coords2);
                    assert![direction.eq_within(&coords2, local_test_angle_accuracy)];
                    println!("Expecting {} == {}", direction, coords3);
                    assert![direction.eq_within(&coords3, local_test_angle_accuracy)];
                }
            }
        }
    }

    #[test]
    fn test_normalization_two_pi_offsets() {
        let longitudes = vec![
            -0.75 * PI,
            -0.5 * PI,
            -0.25 * PI,
            0.,
            0.25 * PI,
            0.5 * PI,
            0.75 * PI,
            PI,
            1.25 * PI,
            1.5 * PI,
            1.75 * PI,
            2. * PI,
        ];
        let latitudes = vec![-0.25 * PI, 0., 0.25 * PI];
        let offsets = vec![-TWO_PI, 0., TWO_PI, 100. * TWO_PI];
        for longitude in longitudes.clone() {
            for latitude in latitudes.clone() {
                for offset in offsets.clone() {
                    let longitude1 = Angle::from_radians(longitude);
                    let latitude1 = Angle::from_radians(latitude);
                    let longitude2 = Angle::from_radians(longitude + offset);
                    let latitude2 = Angle::from_radians(latitude + offset);
                    let mut coords1 = SphericalCoordinates {
                        longitude: longitude1,
                        latitude: latitude1,
                    };
                    let mut coords2 = SphericalCoordinates {
                        longitude: longitude1,
                        latitude: latitude2,
                    };
                    let mut coords3 = SphericalCoordinates {
                        longitude: longitude2,
                        latitude: latitude1,
                    };
                    let mut coords4 = SphericalCoordinates {
                        longitude: longitude2,
                        latitude: latitude2,
                    };
                    coords1.normalize();
                    coords2.normalize();
                    coords3.normalize();
                    coords4.normalize();
                    assert!(coords1.eq_within(&coords2, 10. * TEST_ANGLE_ACCURACY));
                    assert!(coords1.eq_within(&coords3, 10. * TEST_ANGLE_ACCURACY));
                    assert!(coords1.eq_within(&coords4, 10. * TEST_ANGLE_ACCURACY));
                }
            }
        }
    }

    #[test]
    fn test_normalization_crossing_poles() {
        let mut coord = SphericalCoordinates {
            longitude: Angle::from_radians(0.),
            latitude: Angle::from_radians(3. / 4. * PI),
        };
        let expected = SphericalCoordinates {
            longitude: Angle::from_radians(PI),
            latitude: Angle::from_radians(PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(coord.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let mut coord = SphericalCoordinates {
            longitude: Angle::from_radians(0.),
            latitude: Angle::from_radians(-3. / 4. * PI),
        };
        let expected = SphericalCoordinates {
            longitude: Angle::from_radians(PI),
            latitude: Angle::from_radians(-PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(coord.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let mut coord = SphericalCoordinates {
            longitude: Angle::from_radians(PI),
            latitude: Angle::from_radians(3. / 4. * PI),
        };
        let expected = SphericalCoordinates {
            longitude: Angle::from_radians(0.),
            latitude: Angle::from_radians(PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(coord.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let mut coord = SphericalCoordinates {
            longitude: Angle::from_radians(PI),
            latitude: Angle::from_radians(-3. / 4. * PI),
        };
        let expected = SphericalCoordinates {
            longitude: Angle::from_radians(0.),
            latitude: Angle::from_radians(-PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(coord.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let mut coord = SphericalCoordinates {
            longitude: Angle::from_radians(PI / 2.),
            latitude: Angle::from_radians(3. / 4. * PI),
        };
        let expected = SphericalCoordinates {
            longitude: Angle::from_radians(3. / 2. * PI),
            latitude: Angle::from_radians(PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(coord.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let mut coord = SphericalCoordinates {
            longitude: Angle::from_radians(PI / 2.),
            latitude: Angle::from_radians(-3. / 4. * PI),
        };
        let expected = SphericalCoordinates {
            longitude: Angle::from_radians(3. / 2. * PI),
            latitude: Angle::from_radians(-PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(coord.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let mut coord = SphericalCoordinates {
            longitude: Angle::from_radians(3. / 2. * PI),
            latitude: Angle::from_radians(3. / 4. * PI),
        };
        let expected = SphericalCoordinates {
            longitude: Angle::from_radians(PI / 2.),
            latitude: Angle::from_radians(PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(coord.eq_within(&expected, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn test_revertability_of_rotation() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        for long1 in ordinates.clone() {
            for lat1 in ordinates.clone() {
                for long2 in ordinates.clone() {
                    for lat2 in ordinates.clone() {
                        let mut original_point = SphericalCoordinates {
                            longitude: Angle::from_radians(long1),
                            latitude: Angle::from_radians(lat1),
                        };
                        original_point.normalize();
                        let new_z_axis = Direction::from_spherical(&SphericalCoordinates {
                            longitude: Angle::from_radians(long2),
                            latitude: Angle::from_radians(lat2),
                        });

                        let mut point_after_active_rotation =
                            original_point.active_rotation_to_new_z_axis(&new_z_axis);
                        point_after_active_rotation.normalize();
                        let mut point_transformed_back_to_original =
                            point_after_active_rotation.passive_rotation_to_new_z_axis(&new_z_axis);
                        point_transformed_back_to_original.normalize();

                        println!("new_z_axis: {}", new_z_axis);
                        println!("original_point: {}", original_point);
                        println!("point_in_new_system: {}", point_after_active_rotation);
                        println!(
                            "point_transformed_back_to_original: {}",
                            point_transformed_back_to_original
                        );
                        assert!(original_point
                            .eq_within(&point_transformed_back_to_original, TEST_ANGLE_ACCURACY));
                    }
                }
            }
        }
    }

    #[test]
    fn test_north_after_active_rotation() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        for long in ordinates.clone() {
            for lat in ordinates.clone() {
                let mut new_z_axis = SphericalCoordinates {
                    longitude: Angle::from_radians(long),
                    latitude: Angle::from_radians(lat),
                };
                new_z_axis.normalize();
                let new_z_axis_direction = Direction::from_spherical(&new_z_axis);

                let expected = new_z_axis;
                let mut actual = Z_DIRECTION.active_rotation_to_new_z_axis(&new_z_axis_direction);
                actual.normalize();

                println!("new_z_axis: {}", new_z_axis);
                println!("expected: {}", expected);
                println!("actual: {}", actual);
                assert!(expected.eq_within(&actual, TEST_ANGLE_ACCURACY));
            }
        }
    }

    #[test]
    fn test_north_after_passive_rotation() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        for long in ordinates.clone() {
            for lat in ordinates.clone() {
                let mut new_z_axis = SphericalCoordinates {
                    longitude: Angle::from_radians(long),
                    latitude: Angle::from_radians(lat),
                };
                new_z_axis.normalize();
                let new_z_axis_direction = Direction::from_spherical(&new_z_axis);

                let expected = Z_DIRECTION;
                let mut actual = new_z_axis.passive_rotation_to_new_z_axis(&new_z_axis_direction);
                actual.normalize();

                println!("new_z_axis: {}", new_z_axis);
                println!("expected: {}", expected);
                println!("actual: {}", actual);
                assert!(expected.eq_within(&actual, TEST_ANGLE_ACCURACY));
            }
        }
    }
}
