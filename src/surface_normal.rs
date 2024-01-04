use crate::{
    coordinates::{direction::Direction, equatorial::EquatorialCoordinates},
    units::{angle::Angle, time::Time},
    TWO_PI,
};

pub fn surface_normal_at_time(
    mut observer: EquatorialCoordinates,
    angle_at_epoch: Angle,
    time_since_epoch: Time,
    siderial_day: Time,
) -> Direction {
    if siderial_day.as_seconds().abs() > 1. {
        let time_of_siderial_day = time_since_epoch % siderial_day;
        let rotation =
            angle_at_epoch + Angle::from_radians(time_of_siderial_day / siderial_day * TWO_PI);
        observer.set_longitude(observer.get_longitude() + rotation);
    }
    observer.to_direction()
}

pub fn direction_relative_to_surface_normal(
    object_direction: &Direction,
    observer_normal: &Direction,
) -> Direction {
    object_direction.passive_rotation_to_new_z_axis(observer_normal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        coordinates::spherical::SphericalCoordinates, tests::TEST_ACCURACY, units::angle::Angle,
    };

    #[test]
    fn surface_normal_at_time_zero_in_x_direction() {
        let rotation_axis = Direction::Z;
        let observer =
            EquatorialCoordinates::new(SphericalCoordinates::X_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_years(1.);

        let angle_at_epoch = Angle::ZERO;
        let time_since_epoch = Time::from_years(0.);

        let expected = Direction::X;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_time_zero_in_y_direction() {
        let rotation_axis = Direction::Z;
        let observer =
            EquatorialCoordinates::new(SphericalCoordinates::Y_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_years(1.);

        let angle_at_epoch = Angle::ZERO;
        let time_since_epoch = Time::from_years(0.);

        let expected = Direction::Y;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_time_zero_in_z_direction() {
        let rotation_axis = Direction::Z;
        let observer =
            EquatorialCoordinates::new(SphericalCoordinates::Z_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_years(1.);

        let angle_at_epoch = Angle::ZERO;
        let time_since_epoch = Time::from_years(0.);

        let expected = Direction::Z;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_quarter_rotation_in_x_direction() {
        let rotation_axis = Direction::Z;
        let observer =
            EquatorialCoordinates::new(SphericalCoordinates::X_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_years(1.);

        let angle_at_epoch = Angle::ZERO;
        let time_since_epoch = Time::from_years(0.25);

        let expected = Direction::Y;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_quarter_rotation_in_y_direction() {
        let rotation_axis = Direction::Z;
        let observer =
            EquatorialCoordinates::new(SphericalCoordinates::Y_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_years(1.);

        let angle_at_epoch = Angle::ZERO;
        let time_since_epoch = Time::from_years(0.25);

        let expected = -Direction::X;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_quarter_rotation_in_z_direction() {
        let rotation_axis = Direction::Z;
        let observer =
            EquatorialCoordinates::new(SphericalCoordinates::Z_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_years(1.);

        let angle_at_epoch = Angle::ZERO;
        let time_since_epoch = Time::from_years(0.25);

        let expected = Direction::Z;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_half_rotation_in_x_direction() {
        let rotation_axis = Direction::Z;
        let observer =
            EquatorialCoordinates::new(SphericalCoordinates::X_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_years(1.);

        let angle_at_epoch = Angle::ZERO;
        let time_since_epoch = Time::from_years(0.25);

        let expected = -Direction::X;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_of_body_with_retrograde_rotation() {
        let rotation_axis = Direction::Z;
        let observer =
            EquatorialCoordinates::new(SphericalCoordinates::X_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_years(-1.);

        let angle_at_epoch = Angle::ZERO;
        let time_since_epoch = Time::from_years(0.25);

        let expected = -Direction::Y;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_time_zero_in_x_direction_with_tilted_axis() {
        let rotation_axis = Direction::Y;
        let observer =
            EquatorialCoordinates::new(SphericalCoordinates::X_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_years(1.);

        let angle_at_epoch = Angle::ZERO;
        let time_since_epoch = Time::from_years(0.);

        let expected = Direction::X;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_after_quarter_turn_in_x_direction_with_tilted_axis() {
        let rotation_axis = Direction::Y;
        let observer =
            EquatorialCoordinates::new(SphericalCoordinates::X_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_years(1.);

        let angle_at_epoch = Angle::ZERO;
        let time_since_epoch = Time::from_years(0.25);

        let expected = -Direction::Z;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_with_angle_at_epoch() {
        let rotation_axis = Direction::Z;
        let observer =
            EquatorialCoordinates::new(SphericalCoordinates::X_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::from_years(1.);

        let angle_at_epoch = Angle::from_degrees(90.);
        let time_since_epoch = Time::from_years(0.);

        let expected = Direction::Y;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_of_non_rotating_body_with_axis_tilt() {
        let rotation_axis = Direction::Y;
        let observer =
            EquatorialCoordinates::new(SphericalCoordinates::Y_DIRECTION, rotation_axis.clone());
        let siderial_day = Time::ZERO;

        let angle_at_epoch = Angle::ZERO;
        let time_since_epoch = Time::ZERO;

        let expected = -Direction::Z;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }
}
