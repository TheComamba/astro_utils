use crate::units::angle::Angle;

use super::{direction::Direction, spherical::SphericalCoordinates};

pub struct EquatorialCoordinates {
    spherical: SphericalCoordinates,
    axis: Direction,
}

impl EquatorialCoordinates {
    pub const fn new(spherical: SphericalCoordinates, axis: Direction) -> Self {
        Self { spherical, axis }
    }

    pub(crate) fn add_longitude(&mut self, longitude: Angle) {
        self.spherical
            .set_longitude(longitude + self.spherical.get_longitude());
        self.spherical.normalize();
    }

    pub(crate) fn to_direction(&self) -> Direction {
        self.spherical
            .direction_after_passive_rotation_to_new_z_axis(&self.axis)
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
            spherical::{SphericalCoordinates, X_DIRECTION, Z_DIRECTION},
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
                    let coordinates = EquatorialCoordinates::new(Z_DIRECTION, axis);
                    let expected = axis;
                    let actual = coordinates.to_direction();
                    println!("expected: {},\n actual: {}", expected, actual);
                    assert!(actual.eq_within(&axis, TEST_ACCURACY));
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
                    let coordinates = super::EquatorialCoordinates::new(-Z_DIRECTION, axis);
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
                    let coordinates = super::EquatorialCoordinates::new(X_DIRECTION, axis);
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
                    let coordinates = super::EquatorialCoordinates::new(-X_DIRECTION, axis);
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
                    super::EquatorialCoordinates::new(spherical, earth_north);
                let earth_equatorial_coordinates = EarthEquatorialCoordinates::new(long, lat);

                let expected = earth_equatorial_coordinates.to_direction();
                let actual = equatorial_coordinates.to_direction();
                println!("expected: {},\n actual: {}", expected, actual);
                assert!(actual.eq_within(&expected, TEST_ACCURACY));
            }
        }
    }
}
