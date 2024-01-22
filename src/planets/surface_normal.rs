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

/*
 * Rotates object_direction as it would seem to an observer looking in the direction of observer_normal.
 * The rotation_reference is used to determine the direction of the rotation.
 * Its projection onto the new XY plane points along the new X axis.
 */
pub fn direction_relative_to_surface_normal(
    object_direction: &Direction,
    observer_normal: &Direction,
    rotation_reference: &Direction,
) -> Direction {
    let object_direction = object_direction.passive_rotation_to_new_z_axis(observer_normal);
    let rotation_reference = rotation_reference.passive_rotation_to_new_z_axis(observer_normal);
    let new_x = Direction::new(rotation_reference.x(), rotation_reference.y(), 0.);
    let new_x = match new_x {
        Ok(new_x) => new_x,
        Err(_) => return object_direction,
    };
    let mut rotation_angle = new_x.angle_to(&Direction::X);
    if new_x.y() > 0. {
        rotation_angle = -rotation_angle;
    }
    object_direction.rotated(rotation_angle, &Direction::Z)
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

        let expected = -&Direction::X;
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
        let time_since_epoch = Time::from_years(0.5);

        let expected = -&Direction::X;
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

        let expected = -&Direction::Y;
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

        let expected = -&Direction::Z;
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

        let expected = -&Direction::Z;
        let actual =
            surface_normal_at_time(observer, angle_at_epoch, time_since_epoch, siderial_day);
        println!("expected: {},\n actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    #[test]
    fn objects_along_the_new_x_axis_have_no_y_component() {
        let ordinates = vec![0., 1., -1., 12.];
        let angles = vec![
            Angle::ZERO,
            Angle::from_radians(1.),
            Angle::from_radians(-1.),
            Angle::from_radians(12.),
        ];
        for x1 in ordinates.clone().iter() {
            for y1 in ordinates.clone().iter() {
                for z1 in ordinates.clone().iter() {
                    for x2 in ordinates.clone().iter() {
                        for y2 in ordinates.clone().iter() {
                            for z2 in ordinates.clone().iter() {
                                for angle in angles.iter() {
                                    let observer_normal = Direction::new(*x1, *y1, *z1);
                                    let rotation_reference = Direction::new(*x2, *y2, *z2);
                                    if observer_normal.is_err() || rotation_reference.is_err() {
                                        continue;
                                    }
                                    let observer_normal = observer_normal.unwrap();
                                    let rotation_reference = rotation_reference.unwrap();
                                    let ortho = observer_normal.cross_product(&rotation_reference);
                                    if ortho.is_err() {
                                        continue;
                                    }
                                    let ortho = ortho.unwrap();
                                    let object_direction =
                                        rotation_reference.rotated(*angle, &ortho);

                                    let dir_in_new_system = direction_relative_to_surface_normal(
                                        &object_direction,
                                        &observer_normal,
                                        &rotation_reference,
                                    );

                                    println!("\nobject_direction: {}", object_direction);
                                    println!("observer_normal: {}", observer_normal);
                                    println!("rotation_reference: {}", rotation_reference);
                                    println!("dir_in_new_system: {}", dir_in_new_system);
                                    println!("y component: {}", dir_in_new_system.y());
                                    assert!(dir_in_new_system.y().abs() < 10. * TEST_ACCURACY);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
