use crate::{units::angle::Angle, Float, PI};
use std::fmt::{Display, Formatter};
use std::ops::Neg;

use super::cartesian::CartesianCoordinates;

pub const X_DIRECTION: EclipticCoordinates = EclipticCoordinates {
    longitude: Angle::from_radians(0.),
    latitude: Angle::from_radians(0.),
};

pub const Y_DIRECTION: EclipticCoordinates = EclipticCoordinates {
    longitude: Angle::from_radians(PI / 2.),
    latitude: Angle::from_radians(0.),
};

pub const Z_DIRECTION: EclipticCoordinates = EclipticCoordinates {
    longitude: Angle::from_radians(0.),
    latitude: Angle::from_radians(PI / 2.),
};

const PI_HALF: Float = PI / 2.;

/*  The "absolute" reference we use for polar coordiantes is heliocentric ecliptic coordinates:
 * Longitude denotes the angle between the vernal equinox and the body, measured in the ecliptic plane.
 * Latitude denotes the angle between the ecliptic plane and the body.
 * https://en.wikipedia.org/wiki/Ecliptic_coordinate_system
 */
#[derive(Debug, Clone, Copy)]
pub struct EclipticCoordinates {
    longitude: Angle,
    latitude: Angle,
}

impl EclipticCoordinates {
    pub(crate) fn from_cartesian(cart: &CartesianCoordinates) -> EclipticCoordinates {
        let x = cart.x().as_meters();
        let y = cart.y().as_meters();
        let z = cart.z().as_meters();
        let longitude = Angle::from_radians(y.atan2(x));
        let latitude = Angle::from_radians(z.atan2((x * x + y * y).sqrt()));
        EclipticCoordinates {
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

    pub fn eq_within(&self, other: &EclipticCoordinates, accuracy: Angle) -> bool {
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
}

impl Neg for EclipticCoordinates {
    type Output = EclipticCoordinates;

    fn neg(self) -> EclipticCoordinates {
        let mut longitude = self.longitude + Angle::from_radians(PI);
        longitude.normalize();
        let latitude = -self.latitude;
        EclipticCoordinates {
            longitude,
            latitude,
        }
    }
}

impl Display for EclipticCoordinates {
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
        let expected = EclipticCoordinates {
            longitude: Angle::from_degrees(45.),
            latitude: Angle::from_degrees(90. - 54.7356),
        };
        let actual = EclipticCoordinates::from_cartesian(&cartesian);
        println!("x, expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let cartesian = CartesianCoordinates::new(
            Length::from_meters(1.),
            Length::from_meters(1.),
            Length::from_meters(-1.),
        );
        let expected = EclipticCoordinates {
            longitude: Angle::from_degrees(45.),
            latitude: Angle::from_degrees(-90. + 54.7356),
        };
        let actual = EclipticCoordinates::from_cartesian(&cartesian);
        println!("x, expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let cartesian = CartesianCoordinates::new(
            Length::from_meters(1.),
            Length::from_meters(-1.),
            Length::from_meters(1.),
        );
        let expected = EclipticCoordinates {
            longitude: Angle::from_degrees(135.),
            latitude: Angle::from_degrees(90. - 54.7356),
        };
        let actual = EclipticCoordinates::from_cartesian(&cartesian);
        println!("x, expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let cartesian = CartesianCoordinates::new(
            Length::from_meters(1.),
            Length::from_meters(-1.),
            Length::from_meters(-1.),
        );
        let expected = EclipticCoordinates {
            longitude: Angle::from_degrees(135.),
            latitude: Angle::from_degrees(-90. + 54.7356),
        };
        let actual = EclipticCoordinates::from_cartesian(&cartesian);
        println!("x, expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let cartesian = CartesianCoordinates::new(
            Length::from_meters(-1.),
            Length::from_meters(1.),
            Length::from_meters(1.),
        );
        let expected = EclipticCoordinates {
            longitude: Angle::from_degrees(-45.),
            latitude: Angle::from_degrees(90. - 54.7356),
        };
        let actual = EclipticCoordinates::from_cartesian(&cartesian);
        println!("x, expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let cartesian = CartesianCoordinates::new(
            Length::from_meters(-1.),
            Length::from_meters(1.),
            Length::from_meters(-1.),
        );
        let expected = EclipticCoordinates {
            longitude: Angle::from_degrees(-45.),
            latitude: Angle::from_degrees(-90. + 54.7356),
        };
        let actual = EclipticCoordinates::from_cartesian(&cartesian);
        println!("x, expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let cartesian = CartesianCoordinates::new(
            Length::from_meters(-1.),
            Length::from_meters(-1.),
            Length::from_meters(1.),
        );
        let expected = EclipticCoordinates {
            longitude: Angle::from_degrees(-135.),
            latitude: Angle::from_degrees(90. - 54.7356),
        };
        let actual = EclipticCoordinates::from_cartesian(&cartesian);
        println!("x, expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let cartesian = CartesianCoordinates::new(
            Length::from_meters(-1.),
            Length::from_meters(-1.),
            Length::from_meters(-1.),
        );
        let expected = EclipticCoordinates {
            longitude: Angle::from_degrees(-135.),
            latitude: Angle::from_degrees(-90. + 54.7356),
        };
        let actual = EclipticCoordinates::from_cartesian(&cartesian);
        println!("x, expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn test_unary_minus() {
        let x = X_DIRECTION;
        let y = Y_DIRECTION;
        let z = Z_DIRECTION;
        let xyz = EclipticCoordinates {
            longitude: Angle::from_radians(PI / 4.),
            latitude: Angle::from_radians(PI / 4.),
        };
        let expected_minus_x = EclipticCoordinates {
            longitude: Angle::from_radians(PI),
            latitude: Angle::from_radians(0.),
        };
        let expected_minus_y = EclipticCoordinates {
            longitude: Angle::from_radians(-PI_HALF),
            latitude: Angle::from_radians(0.),
        };
        let expected_minus_z = EclipticCoordinates {
            longitude: Angle::from_radians(0.),
            latitude: Angle::from_radians(-PI_HALF),
        };
        let expected_minus_xyz = EclipticCoordinates {
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
                    let coords1 = EclipticCoordinates {
                        longitude: longitude2,
                        latitude: latitude1,
                    };
                    let coords2 = EclipticCoordinates {
                        longitude: longitude1,
                        latitude: latitude2,
                    };
                    let coords3 = EclipticCoordinates {
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
                    let mut coords1 = EclipticCoordinates {
                        longitude: longitude1,
                        latitude: latitude1,
                    };
                    let mut coords2 = EclipticCoordinates {
                        longitude: longitude1,
                        latitude: latitude2,
                    };
                    let mut coords3 = EclipticCoordinates {
                        longitude: longitude2,
                        latitude: latitude1,
                    };
                    let mut coords4 = EclipticCoordinates {
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
        let mut coord = EclipticCoordinates {
            longitude: Angle::from_radians(0.),
            latitude: Angle::from_radians(3. / 4. * PI),
        };
        let expected = EclipticCoordinates {
            longitude: Angle::from_radians(PI),
            latitude: Angle::from_radians(PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(coord.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let mut coord = EclipticCoordinates {
            longitude: Angle::from_radians(0.),
            latitude: Angle::from_radians(-3. / 4. * PI),
        };
        let expected = EclipticCoordinates {
            longitude: Angle::from_radians(PI),
            latitude: Angle::from_radians(-PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(coord.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let mut coord = EclipticCoordinates {
            longitude: Angle::from_radians(PI),
            latitude: Angle::from_radians(3. / 4. * PI),
        };
        let expected = EclipticCoordinates {
            longitude: Angle::from_radians(0.),
            latitude: Angle::from_radians(PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(coord.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let mut coord = EclipticCoordinates {
            longitude: Angle::from_radians(PI),
            latitude: Angle::from_radians(-3. / 4. * PI),
        };
        let expected = EclipticCoordinates {
            longitude: Angle::from_radians(0.),
            latitude: Angle::from_radians(-PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(coord.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let mut coord = EclipticCoordinates {
            longitude: Angle::from_radians(PI / 2.),
            latitude: Angle::from_radians(3. / 4. * PI),
        };
        let expected = EclipticCoordinates {
            longitude: Angle::from_radians(3. / 2. * PI),
            latitude: Angle::from_radians(PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(coord.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let mut coord = EclipticCoordinates {
            longitude: Angle::from_radians(PI / 2.),
            latitude: Angle::from_radians(-3. / 4. * PI),
        };
        let expected = EclipticCoordinates {
            longitude: Angle::from_radians(3. / 2. * PI),
            latitude: Angle::from_radians(-PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(coord.eq_within(&expected, TEST_ANGLE_ACCURACY));

        let mut coord = EclipticCoordinates {
            longitude: Angle::from_radians(3. / 2. * PI),
            latitude: Angle::from_radians(3. / 4. * PI),
        };
        let expected = EclipticCoordinates {
            longitude: Angle::from_radians(PI / 2.),
            latitude: Angle::from_radians(PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(coord.eq_within(&expected, TEST_ANGLE_ACCURACY));
    }
}
