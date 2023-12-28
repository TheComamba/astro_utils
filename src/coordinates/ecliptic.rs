use std::ops::Neg;

use crate::TWO_PI;
use crate::{units::angle::Angle, Float, PI};

pub static X_DIRECTION: EclipticCoordinates = EclipticCoordinates {
    longitude: Angle::from_radians(0.0),
    latitude: Angle::from_radians(0.0),
};

pub static Y_DIRECTION: EclipticCoordinates = EclipticCoordinates {
    longitude: Angle::from_radians(PI / 2.0),
    latitude: Angle::from_radians(0.0),
};

pub static Z_DIRECTION: EclipticCoordinates = EclipticCoordinates {
    longitude: Angle::from_radians(0.0),
    latitude: Angle::from_radians(PI / 2.0),
};

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
    pub(crate) fn new(longitude: Angle, latitude: Angle) -> EclipticCoordinates {
        let mut coords = EclipticCoordinates {
            longitude,
            latitude,
        };
        coords.normalize();
        coords
    }

    pub fn normalize(&mut self) {
        todo!()
    }

    pub fn eq_within(&self, other: &EclipticCoordinates, relative_accuracy: Float) -> bool {
        todo!()
    }
}

impl Neg for EclipticCoordinates {
    type Output = EclipticCoordinates;

    fn neg(self) -> EclipticCoordinates {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::PI_HALF;

    use super::*;

    const TEST_ACCURACY: Float = 1e-5;

    #[test]
    fn test_unary_minus() {
        let x = X_DIRECTION;
        let y = Y_DIRECTION;
        let z = Z_DIRECTION;
        let xyz =
            EclipticCoordinates::new(Angle::from_radians(PI / 4.0), Angle::from_radians(PI / 4.0));
        let expected_minus_x =
            EclipticCoordinates::new(Angle::from_radians(PI), Angle::from_radians(0.0));
        let expected_minus_y =
            EclipticCoordinates::new(Angle::from_radians(-PI_HALF), Angle::from_radians(0.0));
        let expected_minus_z =
            EclipticCoordinates::new(Angle::from_radians(0.0), Angle::from_radians(-PI_HALF));
        let expected_minus_xyz = EclipticCoordinates::new(
            Angle::from_radians(-PI / 4.0),
            Angle::from_radians(-PI / 4.0),
        );

        let minus_x = -x;
        println!("minus_x: {:?}", minus_x);
        assert!(minus_x.eq_within(&expected_minus_x, TEST_ACCURACY));

        let minus_y = -y;
        println!("minus_y: {:?}", minus_y);
        assert!(minus_y.eq_within(&expected_minus_y, TEST_ACCURACY));

        let minus_z = -z;
        println!("minus_z: {:?}", minus_z);
        assert!(minus_z.eq_within(&expected_minus_z, TEST_ACCURACY));

        let minus_xyz = -xyz;
        println!("minus_xyz: {:?}", minus_xyz);
        assert!(minus_xyz.eq_within(&expected_minus_xyz, TEST_ACCURACY));
    }

    #[test]
    fn test_eq_within() {
        let small_offsets = vec![-TEST_ACCURACY / 100.0, 0.0, TEST_ACCURACY / 100.0];
        let large_offsets = vec![-TWO_PI, 0.0, TWO_PI, 100.0 * TWO_PI];
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
                    let longitude2 = longitude1 + Angle::from_radians(large_offset + small_offset);
                    let latitude2 = latitude1 + Angle::from_radians(large_offset + small_offset);
                    let coords1 = EclipticCoordinates::new(longitude2, latitude1);
                    let coords2 = EclipticCoordinates::new(longitude1, latitude2);
                    let coords3 = EclipticCoordinates::new(longitude2, latitude2);
                    assert![direction.eq_within(&coords1, TEST_ACCURACY)];
                    assert![direction.eq_within(&coords2, TEST_ACCURACY)];
                    assert![direction.eq_within(&coords3, TEST_ACCURACY)];
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
            0.0,
            0.25 * PI,
            0.5 * PI,
            0.75 * PI,
            PI,
            1.25 * PI,
            1.5 * PI,
            1.75 * PI,
            2.0 * PI,
        ];
        let latitudes = vec![-0.25 * PI, 0.0, 0.25 * PI];
        let offsets = vec![-TWO_PI, 0.0, TWO_PI, 100.0 * TWO_PI];
        for longitude in longitudes.clone() {
            for latitude in latitudes.clone() {
                for offset in offsets.clone() {
                    let longitude1 = Angle::from_radians(longitude);
                    let latitude1 = Angle::from_radians(latitude);
                    let longitude2 = Angle::from_radians(longitude + offset);
                    let latitude2 = Angle::from_radians(latitude + offset);
                    let mut coords1 = EclipticCoordinates::new(longitude1, latitude1);
                    let mut coords2 = EclipticCoordinates::new(longitude1, latitude2);
                    let mut coords3 = EclipticCoordinates::new(longitude2, latitude1);
                    let mut coords4 = EclipticCoordinates::new(longitude2, latitude2);
                    coords1.normalize();
                    coords2.normalize();
                    coords3.normalize();
                    coords4.normalize();
                    assert!(coords1.eq_within(&coords2, TEST_ACCURACY));
                    assert!(coords1.eq_within(&coords3, TEST_ACCURACY));
                    assert!(coords1.eq_within(&coords4, TEST_ACCURACY));
                }
            }
        }
    }
}
