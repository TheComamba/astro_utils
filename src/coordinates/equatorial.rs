use crate::{
    coordinates::direction::{X, Y, Z},
    units::angle::Angle,
};

use super::{direction::Direction, spherical::SphericalCoordinates};

pub struct EquatorialCoordinates {
    coords: SphericalCoordinates,
    axis: Direction,
}

impl EquatorialCoordinates {
    pub const fn new(coords: SphericalCoordinates, axis: Direction) -> Self {
        Self { coords, axis }
    }

    pub(crate) fn add_longitude(&mut self, longitude: Angle) {
        self.coords
            .set_longitude(longitude + self.coords.get_longitude());
        self.coords.normalize();
    }

    pub(crate) fn to_direction(&self) -> Direction {
        let axis_projected_onto_xy_plane = Direction::new(self.axis.x(), self.axis.y(), 0.);
        let mut polar_rotation_angle = axis_projected_onto_xy_plane.angle_to(&Y);
        if axis_projected_onto_xy_plane.x() < 0. {
            polar_rotation_angle = -polar_rotation_angle;
        }
        let axis_tilt_to_ecliptic = self.axis.angle_to(&Z);

        let dir = Direction::from_spherical(&self.coords);
        let dir = dir.rotated(-axis_tilt_to_ecliptic, &X);
        dir.rotated(-polar_rotation_angle, &Z)
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
        Float, PI,
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
