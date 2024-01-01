use std::fmt::Display;

use super::{direction::Direction, spherical::SphericalCoordinates};
use crate::units::angle::Angle;

pub struct EquatorialCoordinates {
    spherical: SphericalCoordinates,
    rotation_axis: Direction,
}

impl EquatorialCoordinates {
    pub const fn new(spherical: SphericalCoordinates, rotation_axis: Direction) -> Self {
        Self {
            spherical,
            rotation_axis,
        }
    }

    pub fn get_longitude(&self) -> Angle {
        self.spherical.get_longitude()
    }

    pub fn get_latitude(&self) -> Angle {
        self.spherical.get_latitude()
    }

    pub fn set_longitude(&mut self, longitude: Angle) {
        self.spherical.set_longitude(longitude);
    }

    pub fn set_latitude(&mut self, latitude: Angle) {
        self.spherical.set_latitude(latitude);
    }

    pub fn get_spherical(&self) -> &SphericalCoordinates {
        &self.spherical
    }

    pub fn get_rotation_axis(&self) -> &Direction {
        &self.rotation_axis
    }

    pub(crate) fn to_direction(&self) -> Direction {
        let dir = Direction::from_spherical(&self.spherical);
        dir.active_rotation_to_new_z_axis(&self.rotation_axis)
    }
}

impl Display for EquatorialCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} relative to equatorial plane with north pole at {}",
            self.spherical, self.rotation_axis
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        coordinates::{
            direction::Direction,
            earth_equatorial::{
                EarthEquatorialCoordinates, EARTH_NORTH_POLE_IN_ECLIPTIC_COORDINATES,
            },
            equatorial::EquatorialCoordinates,
            spherical::SphericalCoordinates,
        },
        tests::TEST_ACCURACY,
        units::angle::Angle,
        Float,
    };

    #[test]
    fn north_pole_points_along_axis() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        for x in ordinates.clone() {
            for y in ordinates.clone() {
                for z in ordinates.clone() {
                    if (x * x + y * y + z * z).abs() < 1e-5 {
                        continue;
                    }
                    let axis = Direction::new(x, y, z);
                    let coordinates =
                        EquatorialCoordinates::new(SphericalCoordinates::Z_DIRECTION, axis.clone());
                    let expected = axis;
                    let actual = coordinates.to_direction();
                    println!("expected: {},\n actual: {}", expected, actual);
                    assert!(actual.eq_within(&expected, TEST_ACCURACY));
                }
            }
        }
    }

    #[test]
    fn south_pole_points_along_negative_axis() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        for x in ordinates.clone() {
            for y in ordinates.clone() {
                for z in ordinates.clone() {
                    if (x * x + y * y + z * z).abs() < 1e-5 {
                        continue;
                    }
                    let axis = Direction::new(x, y, z);
                    let coordinates = super::EquatorialCoordinates::new(
                        -SphericalCoordinates::Z_DIRECTION,
                        axis.clone(),
                    );
                    let expected = -axis;
                    let actual = coordinates.to_direction();
                    println!("expected: {},\n actual: {}", expected, actual);
                    assert!(actual.eq_within(&expected, TEST_ACCURACY));
                }
            }
        }
    }

    #[test]
    fn x_axis_lies_in_horizontal_plane() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        for x in ordinates.clone() {
            for y in ordinates.clone() {
                for z in ordinates.clone() {
                    if (x * x + y * y + z * z).abs() < 1e-5 {
                        continue;
                    }
                    let axis = Direction::new(x, y, z);
                    let coordinates =
                        super::EquatorialCoordinates::new(SphericalCoordinates::X_DIRECTION, axis);
                    let direction = coordinates.to_direction();
                    assert!(direction.z().abs() < TEST_ACCURACY);
                }
            }
        }
    }

    #[test]
    fn minus_x_axis_lies_in_horizontal_plane() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        for x in ordinates.clone() {
            for y in ordinates.clone() {
                for z in ordinates.clone() {
                    if (x * x + y * y + z * z).abs() < 1e-5 {
                        continue;
                    }
                    let axis = Direction::new(x, y, z);
                    let coordinates =
                        super::EquatorialCoordinates::new(-SphericalCoordinates::X_DIRECTION, axis);
                    let direction = coordinates.to_direction();
                    assert!(direction.z().abs() < TEST_ACCURACY);
                }
            }
        }
    }

    #[test]
    fn behaves_like_earth_equatorial() {
        let ordinates: Vec<Float> = vec![-1., 0., 1., 10.];
        let earth_north =
            Direction::from_spherical(&EARTH_NORTH_POLE_IN_ECLIPTIC_COORDINATES.get_spherical());

        for long in ordinates.clone() {
            for lat in ordinates.clone() {
                let long = Angle::from_radians(long);
                let lat = Angle::from_radians(lat);
                let spherical = SphericalCoordinates::new(long, lat);

                let equatorial_coordinates =
                    super::EquatorialCoordinates::new(spherical, earth_north.clone());
                let earth_equatorial_coordinates = EarthEquatorialCoordinates::new(long, lat);

                let expected = earth_equatorial_coordinates.to_direction();
                let actual = equatorial_coordinates.to_direction();
                println!("expected: {},\n actual: {}", expected, actual);
                assert!(actual.eq_within(&expected, TEST_ACCURACY));
            }
        }
    }
}
