use super::{
    declination::{Declination, Sgn},
    direction::Direction,
    right_ascension::RightAscension,
};
use crate::{units::angle::Angle, Float, PI};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    ops::Neg,
};

const PI_HALF: Float = PI / 2.;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct SphericalCoordinates {
    longitude: Angle,
    latitude: Angle,
}

impl SphericalCoordinates {
    pub const X_DIRECTION: SphericalCoordinates = SphericalCoordinates {
        longitude: Angle::ZERO,
        latitude: Angle::ZERO,
    };

    pub const Y_DIRECTION: SphericalCoordinates = SphericalCoordinates {
        longitude: Angle::from_radians(PI / 2.),
        latitude: Angle::ZERO,
    };

    pub const Z_DIRECTION: SphericalCoordinates = SphericalCoordinates {
        longitude: Angle::ZERO,
        latitude: Angle::from_radians(PI / 2.),
    };

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

    #[cfg(test)]
    pub(crate) fn eq_within(&self, other: &Self, accuracy: Angle) -> bool {
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

    pub(super) fn cartesian_to_spherical(cart: (Float, Float, Float)) -> Self {
        let (x, y, z) = cart;
        let longitude = Angle::from_radians(y.atan2(x));
        let latitude = Angle::from_radians(z.atan2((x * x + y * y).sqrt()));
        Self {
            longitude,
            latitude,
        }
    }

    pub fn to_direction(&self) -> Direction {
        let x = self.get_longitude().cos() * self.get_latitude().cos();
        let y = self.get_longitude().sin() * self.get_latitude().cos();
        let z = self.get_latitude().sin();
        Direction { x, y, z }
    }

    pub fn to_ra_and_dec(&self) -> (RightAscension, Declination) {
        let mut ra_remainder = self.longitude.as_degrees();
        let ra_hours = (ra_remainder / 15.).floor() as i8;
        ra_remainder -= ra_hours as Float * 15.;
        let ra_minutes = (ra_remainder / 15. * 60.).floor() as i8;
        ra_remainder -= ra_minutes as Float / 60. * 15.;
        let ra_seconds = (ra_remainder / 15. * 3600.).floor() as i8;
        let ra = RightAscension::new(ra_hours, ra_minutes, ra_seconds);

        let mut dec_remainder = self.latitude.as_degrees();
        let sign = if dec_remainder < 0. {
            dec_remainder = dec_remainder.abs();
            Sgn::Neg
        } else {
            Sgn::Pos
        };
        let dec_degrees = dec_remainder.floor() as u8;
        dec_remainder -= dec_degrees as Float;
        let dec_minutes = (dec_remainder * 60.).floor() as u8;
        dec_remainder -= dec_minutes as Float / 60.;
        let dec_seconds = (dec_remainder * 3600.).floor() as u8;
        let dec = Declination::new(sign, dec_degrees, dec_minutes, dec_seconds);

        (ra, dec)
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
    use crate::{
        coordinates::cartesian::CartesianCoordinates, tests::TEST_ANGLE_ACCURACY,
        units::length::Length, TWO_PI,
    };

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
        let actual = cartesian.to_spherical();
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
        let actual = cartesian.to_spherical();
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
        let actual = cartesian.to_spherical();
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
        let actual = cartesian.to_spherical();
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
        let actual = cartesian.to_spherical();
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
        let actual = cartesian.to_spherical();
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
        let actual = cartesian.to_spherical();
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
        let actual = cartesian.to_spherical();
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn test_unary_minus() {
        let x = SphericalCoordinates::X_DIRECTION;
        let y = SphericalCoordinates::Y_DIRECTION;
        let z = SphericalCoordinates::Z_DIRECTION;
        let xyz = SphericalCoordinates {
            longitude: Angle::from_radians(PI / 4.),
            latitude: Angle::from_radians(PI / 4.),
        };
        let expected_minus_x = SphericalCoordinates {
            longitude: Angle::from_radians(PI),
            latitude: Angle::ZERO,
        };
        let expected_minus_y = SphericalCoordinates {
            longitude: Angle::from_radians(-PI_HALF),
            latitude: Angle::ZERO,
        };
        let expected_minus_z = SphericalCoordinates {
            longitude: Angle::ZERO,
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
            Angle::ZERO,
            local_test_angle_accuracy / 100.,
        ];
        let large_offsets = vec![-TWO_PI, 0., TWO_PI, 100. * TWO_PI];
        let directions = vec![
            SphericalCoordinates::X_DIRECTION,
            SphericalCoordinates::Y_DIRECTION,
            SphericalCoordinates::Z_DIRECTION,
            -SphericalCoordinates::X_DIRECTION,
            -SphericalCoordinates::Y_DIRECTION,
            -SphericalCoordinates::Z_DIRECTION,
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
            longitude: Angle::ZERO,
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
            longitude: Angle::ZERO,
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
            longitude: Angle::ZERO,
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
            longitude: Angle::ZERO,
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
    fn roundtrips_to_ra_and_dec() {
        const STEP: usize = 100;
        for i in 0..STEP {
            for j in 0..STEP {
                let ra_angle = Angle::from_radians(2. * PI * i as Float / STEP as Float);
                let dec_angle = Angle::from_radians(PI * j as Float / STEP as Float - PI_HALF);
                let spherical = SphericalCoordinates {
                    longitude: ra_angle,
                    latitude: dec_angle,
                };
                let (ra, dec) = spherical.to_ra_and_dec();
                let spherical2 = SphericalCoordinates::new(ra.to_angle(), dec.to_angle());
                println!("spherical: {}, spherical2: {}", spherical, spherical2);
                assert!(spherical.eq_within(&spherical2, 10. * TEST_ANGLE_ACCURACY));
            }
        }
    }
}
