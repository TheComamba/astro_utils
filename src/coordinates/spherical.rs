use super::{
    declination::{Declination, Sgn},
    direction::Direction,
    right_ascension::RightAscension,
};
use crate::units::angle::{normalized_angle, ANGLE_ZERO};
use serde::{Deserialize, Serialize};
use simple_si_units::geometry::Angle;
use std::{
    f64::consts::PI,
    fmt::{Display, Formatter},
    ops::Neg,
};

const PI_HALF: f64 = PI / 2.;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct SphericalCoordinates {
    longitude: Angle<f64>,
    latitude: Angle<f64>,
}

impl SphericalCoordinates {
    pub const X_DIRECTION: SphericalCoordinates = SphericalCoordinates {
        longitude: ANGLE_ZERO,
        latitude: ANGLE_ZERO,
    };

    pub const Y_DIRECTION: SphericalCoordinates = SphericalCoordinates {
        longitude: Angle::from_radians(PI / 2.),
        latitude: ANGLE_ZERO,
    };

    pub const Z_DIRECTION: SphericalCoordinates = SphericalCoordinates {
        longitude: ANGLE_ZERO,
        latitude: Angle::from_radians(PI / 2.),
    };

    pub const fn new(longitude: Angle<f64>, latitude: Angle<f64>) -> Self {
        Self {
            longitude,
            latitude,
        }
    }
    pub fn normalize(&mut self) {
        self.longitude = normalized_angle(self.longitude);
        self.latitude = normalized_angle(self.latitude);
        if self.latitude.to_radians() > PI_HALF {
            self.longitude = self.longitude + Angle::from_radians(PI);
            self.longitude = normalized_angle(self.longitude);
            self.latitude = Angle::from_radians(PI) - self.latitude;
        } else if self.latitude.to_radians() < -PI_HALF {
            self.longitude = self.longitude + Angle::from_radians(PI);
            self.longitude = normalized_angle(self.longitude);
            self.latitude = Angle::from_radians(-PI) - self.latitude;
        }
    }

    #[cfg(test)]
    pub(crate) fn eq_within(&self, other: &Self, accuracy: Angle<f64>) -> bool {
        use crate::tests::eq_within;

        const NORTHPOLE_LATITUDE: Angle<f64> = Angle { rad: PI_HALF };
        const SOUTHPOLE_LATITUDE: Angle<f64> = Angle { rad: -PI_HALF };
        let mut clone = self.clone();
        let mut other_clone = other.clone();
        clone.normalize();
        other_clone.normalize();
        let latitudes_equal = eq_within(clone.latitude.rad, other_clone.latitude.rad, accuracy.rad);
        let is_pole = eq_within(clone.latitude.rad, NORTHPOLE_LATITUDE.rad, accuracy.rad)
            || eq_within(clone.latitude.rad, SOUTHPOLE_LATITUDE.rad, accuracy.rad);
        let longitudes_equal = if is_pole {
            true
        } else {
            eq_within(clone.longitude.rad, other_clone.longitude.rad, accuracy.rad)
        };
        latitudes_equal && longitudes_equal
    }

    pub fn get_longitude(&self) -> Angle<f64> {
        self.longitude
    }

    pub fn get_latitude(&self) -> Angle<f64> {
        self.latitude
    }

    pub fn set_longitude(&mut self, longitude: Angle<f64>) {
        self.longitude = longitude;
    }

    pub fn set_latitude(&mut self, latitude: Angle<f64>) {
        self.latitude = latitude;
    }

    pub(super) fn cartesian_to_spherical(cart: (f64, f64, f64)) -> Self {
        let (x, y, z) = cart;
        let longitude = Angle::from_radians(y.atan2(x));
        let latitude = Angle::from_radians(z.atan2((x * x + y * y).sqrt()));
        Self {
            longitude,
            latitude,
        }
    }

    pub fn to_direction(&self) -> Direction {
        let x = self.get_longitude().rad.cos() * self.get_latitude().rad.cos();
        let y = self.get_longitude().rad.sin() * self.get_latitude().rad.cos();
        let z = self.get_latitude().rad.sin();
        Direction { x, y, z }
    }

    pub fn to_ra_and_dec(&self) -> (RightAscension, Declination) {
        let mut ra_remainder = self.longitude.to_degrees();
        let ra_hours = (ra_remainder / 15.).floor() as i8;
        ra_remainder -= ra_hours as f64 * 15.;
        let ra_minutes = (ra_remainder / 15. * 60.).floor() as i8;
        ra_remainder -= ra_minutes as f64 / 60. * 15.;
        let ra_seconds = (ra_remainder / 15. * 3600.).floor() as i8;
        let ra = RightAscension::new(ra_hours, ra_minutes, ra_seconds);

        let mut dec_remainder = self.latitude.to_degrees();
        let sign = if dec_remainder < 0. {
            dec_remainder = dec_remainder.abs();
            Sgn::Neg
        } else {
            Sgn::Pos
        };
        let dec_degrees = dec_remainder.floor() as u8;
        dec_remainder -= dec_degrees as f64;
        let dec_minutes = (dec_remainder * 60.).floor() as u8;
        dec_remainder -= dec_minutes as f64 / 60.;
        let dec_seconds = (dec_remainder * 3600.).floor() as u8;
        let dec = Declination::new(sign, dec_degrees, dec_minutes, dec_seconds);

        (ra, dec)
    }
}

impl Neg for &SphericalCoordinates {
    type Output = SphericalCoordinates;

    fn neg(self) -> SphericalCoordinates {
        let mut longitude = self.longitude + Angle::from_radians(PI);
        longitude = normalized_angle(longitude);
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
        coordinates::cartesian::CartesianCoordinates,
        tests::{eq, eq_within, TEST_ACCURACY},
        units::angle::TWO_PI,
    };
    use simple_si_units::base::Distance;

    #[test]
    fn test_from_cartesian() {
        let cartesian = CartesianCoordinates::new(
            Distance::from_meters(1.),
            Distance::from_meters(1.),
            Distance::from_meters(1.),
        );
        let expected: SphericalCoordinates = SphericalCoordinates {
            longitude: Angle::from_degrees(45.),
            latitude: Angle::from_degrees(90. - 54.7356),
        };
        let actual = cartesian.to_spherical();
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(actual.eq_within(&expected));

        let cartesian = CartesianCoordinates::new(
            Distance::from_meters(1.),
            Distance::from_meters(1.),
            Distance::from_meters(-1.),
        );
        let expected = SphericalCoordinates {
            longitude: Angle::from_degrees(45.),
            latitude: Angle::from_degrees(-90. + 54.7356),
        };
        let actual = cartesian.to_spherical();
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(eq(actual, &expected));

        let cartesian = CartesianCoordinates::new(
            Distance::from_meters(1.),
            Distance::from_meters(-1.),
            Distance::from_meters(1.),
        );
        let expected = SphericalCoordinates {
            longitude: Angle::from_degrees(-45.),
            latitude: Angle::from_degrees(90. - 54.7356),
        };
        let actual = cartesian.to_spherical();
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(eq(actual, &expected));

        let cartesian = CartesianCoordinates::new(
            Distance::from_meters(1.),
            Distance::from_meters(-1.),
            Distance::from_meters(-1.),
        );
        let expected = SphericalCoordinates {
            longitude: Angle::from_degrees(-45.),
            latitude: Angle::from_degrees(-90. + 54.7356),
        };
        let actual = cartesian.to_spherical();
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(eq(actual, &expected));

        let cartesian = CartesianCoordinates::new(
            Distance::from_meters(-1.),
            Distance::from_meters(1.),
            Distance::from_meters(1.),
        );
        let expected = SphericalCoordinates {
            longitude: Angle::from_degrees(135.),
            latitude: Angle::from_degrees(90. - 54.7356),
        };
        let actual = cartesian.to_spherical();
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(eq(actual, &expected));

        let cartesian = CartesianCoordinates::new(
            Distance::from_meters(-1.),
            Distance::from_meters(1.),
            Distance::from_meters(-1.),
        );
        let expected = SphericalCoordinates {
            longitude: Angle::from_degrees(135.),
            latitude: Angle::from_degrees(-90. + 54.7356),
        };
        let actual = cartesian.to_spherical();
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(eq(actual, &expected));

        let cartesian = CartesianCoordinates::new(
            Distance::from_meters(-1.),
            Distance::from_meters(-1.),
            Distance::from_meters(1.),
        );
        let expected = SphericalCoordinates {
            longitude: Angle::from_degrees(-135.),
            latitude: Angle::from_degrees(90. - 54.7356),
        };
        let actual = cartesian.to_spherical();
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(eq(actual, &expected));

        let cartesian = CartesianCoordinates::new(
            Distance::from_meters(-1.),
            Distance::from_meters(-1.),
            Distance::from_meters(-1.),
        );
        let expected = SphericalCoordinates {
            longitude: Angle::from_degrees(-135.),
            latitude: Angle::from_degrees(-90. + 54.7356),
        };
        let actual = cartesian.to_spherical();
        println!("{}, expected: {}, actual: {}", cartesian, expected, actual);
        assert!(eq(actual, &expected));
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
            latitude: ANGLE_ZERO,
        };
        let expected_minus_y = SphericalCoordinates {
            longitude: Angle::from_radians(-PI_HALF),
            latitude: ANGLE_ZERO,
        };
        let expected_minus_z = SphericalCoordinates {
            longitude: ANGLE_ZERO,
            latitude: Angle::from_radians(-PI_HALF),
        };
        let expected_minus_xyz = SphericalCoordinates {
            longitude: Angle::from_radians(-PI * 3. / 4.),
            latitude: Angle::from_radians(-PI / 4.),
        };

        let minus_x = -x;
        println!("-x, expected: {}, actual: {}", expected_minus_x, minus_x);
        assert!(eq(minus_x, &expected_minus_x));

        let minus_y = -y;
        println!("-y, expected: {}, actual: {}", expected_minus_y, minus_y);
        assert!(eq(minus_y, &expected_minus_y));

        let minus_z = -z;
        println!("-z, expected: {}, actual: {}", expected_minus_z, minus_z);
        assert!(eq(minus_z, &expected_minus_z));

        let minus_xyz = -xyz;
        println!(
            "-xyz, expected: {}, actual: {}",
            expected_minus_xyz, minus_xyz
        );
        assert!(eq(minus_xyz, &expected_minus_xyz));
    }

    #[test]
    fn test_eq_within() {
        const LOCAL_TEST_ANGLE_ACCURACY: f64 = 10. * TEST_ACCURACY;

        let small_offsets = vec![
            -LOCAL_TEST_ANGLE_ACCURACY / 100.,
            ANGLE_ZERO,
            LOCAL_TEST_ANGLE_ACCURACY / 100.,
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
                    assert![eq_within(direction, &coords1, LOCAL_TEST_ANGLE_ACCURACY)];
                    println!("Expecting {} == {}", direction, coords2);
                    assert![eq_within(direction, &coords2, LOCAL_TEST_ANGLE_ACCURACY)];
                    println!("Expecting {} == {}", direction, coords3);
                    assert![eq_within(direction, &coords3, LOCAL_TEST_ANGLE_ACCURACY)];
                }
            }
        }
    }

    #[test]
    fn test_normalization_two_pi_offsets() {
        const LOCAL_TEST_ANGLE_ACCURACY: f64 = 10. * TEST_ACCURACY;

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
                    assert!(coords1.eq_within(&coords2, LOCAL_TEST_ANGLE_ACCURACY));
                    assert!(coords1.eq_within(&coords3, LOCAL_TEST_ANGLE_ACCURACY));
                    assert!(coords1.eq_within(&coords4, LOCAL_TEST_ANGLE_ACCURACY));
                }
            }
        }
    }

    #[test]
    fn test_normalization_crossing_poles() {
        let mut coord = SphericalCoordinates {
            longitude: ANGLE_ZERO,
            latitude: Angle::from_radians(3. / 4. * PI),
        };
        let expected = SphericalCoordinates {
            longitude: Angle::from_radians(PI),
            latitude: Angle::from_radians(PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(eq(coord, &expected));

        let mut coord = SphericalCoordinates {
            longitude: ANGLE_ZERO,
            latitude: Angle::from_radians(-3. / 4. * PI),
        };
        let expected = SphericalCoordinates {
            longitude: Angle::from_radians(PI),
            latitude: Angle::from_radians(-PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(eq(coord, &expected));

        let mut coord = SphericalCoordinates {
            longitude: Angle::from_radians(PI),
            latitude: Angle::from_radians(3. / 4. * PI),
        };
        let expected = SphericalCoordinates {
            longitude: ANGLE_ZERO,
            latitude: Angle::from_radians(PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(eq(coord, &expected));

        let mut coord = SphericalCoordinates {
            longitude: Angle::from_radians(PI),
            latitude: Angle::from_radians(-3. / 4. * PI),
        };
        let expected = SphericalCoordinates {
            longitude: ANGLE_ZERO,
            latitude: Angle::from_radians(-PI / 4.),
        };
        coord.normalize();
        println!("expected: {}, actual: {}", expected, coord);
        assert!(eq(coord, &expected));

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
        assert!(eq(coord, &expected));

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
        assert!(eq(coord, &expected));

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
        assert!(coord.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn roundtrips_to_ra_and_dec() {
        const STEP: usize = 100;
        for i in 0..STEP {
            for j in 0..STEP {
                let ra_angle = Angle::from_radians(2. * PI * i as f64 / STEP as f64);
                let dec_angle = Angle::from_radians(PI * j as f64 / STEP as f64 - PI_HALF);
                let spherical = SphericalCoordinates {
                    longitude: ra_angle,
                    latitude: dec_angle,
                };
                let (ra, dec) = spherical.to_ra_and_dec();
                let spherical2 = SphericalCoordinates::new(ra.to_angle(), dec.to_angle());
                println!("spherical: {}, spherical2: {}", spherical, spherical2);
                assert!(spherical.eq_within(&spherical2, 10. * TEST_ACCURACY));
            }
        }
    }
}
