use crate::{
    coordinates::{direction::Direction, equatorial::EquatorialCoordinates},
    units::{angle::Angle, time::Time},
    TWO_PI,
};

pub fn surface_normal_at_time(
    mut observer: EquatorialCoordinates,
    time_since_epoch: Time,
    siderial_day: Time,
) -> Direction {
    let time_of_day = time_since_epoch % siderial_day;
    let rotation = Angle::from_radians(time_of_day / siderial_day * TWO_PI);
    observer.add_longitude(rotation);
    observer.to_direction()
}
