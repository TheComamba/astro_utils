use crate::{
    coordinates::{
        direction::Direction, ecliptic::EclipticCoordinates, equatorial::EquatorialCoordinates,
        spherical::SphericalCoordinates,
    },
    units::{angle::Angle, time::Time},
    TWO_PI,
};

pub fn surface_normal_at_time(
    mut observer: EquatorialCoordinates,
    angle_at_epoch: Angle,
    time_since_epoch: Time,
    siderial_day: Time,
) -> Direction {
    let time_of_day = time_since_epoch % siderial_day;
    let rotation = angle_at_epoch + Angle::from_radians(time_of_day / siderial_day * TWO_PI);
    observer.set_longitude(observer.get_longitude() + rotation);
    observer.to_direction()
}

pub fn apparent_celestial_position(
    object: &EclipticCoordinates,
    observer_normal: &Direction,
) -> SphericalCoordinates {
    object
        .get_spherical()
        .passive_rotation_to_new_z_axis(observer_normal)
}
