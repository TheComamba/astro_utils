use astro_coords::{direction::Direction, equatorial::Equatorial};
use uom::si::{
    f64::{Angle, Time},
    time::second,
};

use crate::units::angle::FULL_CIRC;

pub fn surface_normal_at_time(
    mut observer: Equatorial,
    angle_at_epoch: Angle,
    time_since_epoch: Time,
    siderial_day: Time,
) -> Direction {
    if siderial_day.get::<second>().abs() > 1. {
        let time_of_siderial_day = time_since_epoch % siderial_day;
        let rotation = angle_at_epoch + (time_of_siderial_day / siderial_day).value * FULL_CIRC();
        observer.spherical.longitude = observer.spherical.longitude + rotation;
    }
    observer.to_direction()
}

#[cfg(test)]
mod tests {
    use astro_coords::spherical::Spherical;
    use uom::si::{angle::degree, time::year};

    use crate::{tests::TEST_ACCURACY, units::angle::ANGLE_ZERO};

    use super::*;

    #[test]
    fn surface_normal_at_time_zero_in_x_direction() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::x_direction(), rotation_axis.clone());
        let siderial_day = Time::new::<year>(1.);

        let angle_at_epoch = ANGLE_ZERO();
        let time_since_epoch = Time::new::<year>(0.);

        let expected = Direction::X;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_time_zero_in_y_direction() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::y_direction(), rotation_axis.clone());
        let siderial_day = Time::new::<year>(1.);

        let angle_at_epoch = ANGLE_ZERO();
        let time_since_epoch = Time::new::<year>(0.);

        let expected = Direction::Y;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_time_zero_in_z_direction() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::z_direction(), rotation_axis.clone());
        let siderial_day = Time::new::<year>(1.);

        let angle_at_epoch = ANGLE_ZERO();
        let time_since_epoch = Time::new::<year>(0.);

        let expected = Direction::Z;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_quarter_rotation_in_x_direction() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::x_direction(), rotation_axis.clone());
        let siderial_day = Time::new::<year>(1.);

        let angle_at_epoch = ANGLE_ZERO();
        let time_since_epoch = Time::new::<year>(0.25);

        let expected = Direction::Y;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_quarter_rotation_in_y_direction() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::y_direction(), rotation_axis.clone());
        let siderial_day = Time::new::<year>(1.);

        let angle_at_epoch = ANGLE_ZERO();
        let time_since_epoch = Time::new::<year>(0.25);

        let expected = -&Direction::X;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_quarter_rotation_in_z_direction() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::z_direction(), rotation_axis.clone());
        let siderial_day = Time::new::<year>(1.);

        let angle_at_epoch = ANGLE_ZERO();
        let time_since_epoch = Time::new::<year>(0.25);

        let expected = Direction::Z;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_half_rotation_in_x_direction() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::x_direction(), rotation_axis.clone());
        let siderial_day = Time::new::<year>(1.);

        let angle_at_epoch = ANGLE_ZERO();
        let time_since_epoch = Time::new::<year>(0.5);

        let expected = -&Direction::X;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_of_body_with_retrograde_rotation() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::x_direction(), rotation_axis.clone());
        let siderial_day = Time::new::<year>(-1.);

        let angle_at_epoch = ANGLE_ZERO();
        let time_since_epoch = Time::new::<year>(0.25);

        let expected = -&Direction::Y;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_at_time_zero_in_x_direction_with_tilted_axis() {
        let rotation_axis = Direction::Y;
        let observer = Equatorial::new(Spherical::x_direction(), rotation_axis.clone());
        let siderial_day = Time::new::<year>(1.);

        let angle_at_epoch = ANGLE_ZERO();
        let time_since_epoch = Time::new::<year>(0.);

        let expected = Direction::X;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_after_quarter_turn_in_x_direction_with_tilted_axis() {
        let rotation_axis = Direction::Y;
        let observer = Equatorial::new(Spherical::x_direction(), rotation_axis.clone());
        let siderial_day = Time::new::<year>(1.);

        let angle_at_epoch = ANGLE_ZERO();
        let time_since_epoch = Time::new::<year>(0.25);

        let expected = -&Direction::Z;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_with_angle_at_epoch() {
        let rotation_axis = Direction::Z;
        let observer = Equatorial::new(Spherical::x_direction(), rotation_axis.clone());
        let siderial_day = Time::new::<year>(1.);

        let angle_at_epoch = Angle::new::<degree>(90.);
        let time_since_epoch = Time::new::<year>(0.);

        let expected = Direction::Y;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn surface_normal_of_non_rotating_body_with_axis_tilt() {
        let rotation_axis = Direction::Y;
        let observer = Equatorial::new(Spherical::y_direction(), rotation_axis.clone());
        let siderial_day = Time::new::<year>(0.);

        let angle_at_epoch = ANGLE_ZERO();
        let time_since_epoch = Time::new::<year>(0.);

        let expected = -&Direction::Z;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }
}
