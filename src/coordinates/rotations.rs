use super::direction::Direction;
use crate::{units::angle::Angle, Float};
use std::ops::{Add, Mul};

pub(super) fn rotated_tuple<T>(tup: (T, T, T), angle: Angle, axis: &Direction) -> (T, T, T)
where
    T: Mul<Float, Output = T> + Add<Output = T> + Copy,
{
    let cos = angle.cos();
    let sin = angle.sin();

    let (x, y, z) = tup;

    let ux = axis.x();
    let uy = axis.y();
    let uz = axis.z();

    let r_11 = cos + ux * ux * (1. - cos);
    let r_12 = ux * uy * (1. - cos) - uz * sin;
    let r_13 = ux * uz * (1. - cos) + uy * sin;

    let r_21 = uy * ux * (1. - cos) + uz * sin;
    let r_22 = cos + uy * uy * (1. - cos);
    let r_23 = uy * uz * (1. - cos) - ux * sin;

    let r_31 = uz * ux * (1. - cos) - uy * sin;
    let r_32 = uz * uy * (1. - cos) + ux * sin;
    let r_33 = cos + uz * uz * (1. - cos);

    let x_out = x * r_11 + y * r_12 + z * r_13;
    let y_out = x * r_21 + y * r_22 + z * r_23;
    let z_out = x * r_31 + y * r_32 + z * r_33;
    (x_out, y_out, z_out)
}

pub fn get_rotation_parameters(start: &Direction, end: &Direction) -> (Angle, Direction) {
    let angle = start.angle_to(end);
    let axis = start.cross_product(end);
    (angle, axis)
}

#[cfg(test)]
mod tests {
    use crate::{
        coordinates::{
            direction::Direction,
            rotations::{get_rotation_parameters, rotated_tuple},
        },
        tests::TEST_ACCURACY,
        units::angle::Angle,
        Float, TWO_PI,
    };

    const X_VECTOR: (Float, Float, Float) = (1., 0., 0.);
    const MINUS_X_VECTOR: (Float, Float, Float) = (-1., 0., 0.);
    const Y_VECTOR: (Float, Float, Float) = (0., 1., 0.);
    const MINUS_Y_VECTOR: (Float, Float, Float) = (0., -1., 0.);
    const Z_VECTOR: (Float, Float, Float) = (0., 0., 1.);
    const MINUS_Z_VECTOR: (Float, Float, Float) = (0., 0., -1.);

    const QUARTER_TURN: Angle = Angle::from_radians(TWO_PI / 4.);
    const HALF_TURN: Angle = Angle::from_radians(TWO_PI / 2.);
    const THREE_QUARTER_TURN: Angle = Angle::from_radians(3. / 4. * TWO_PI);
    const FULL_TURN: Angle = Angle::from_radians(TWO_PI);
    const ONE_THIRD_TURN: Angle = Angle::from_radians(TWO_PI / 3.);
    const TWO_THIRDS_TURN: Angle = Angle::from_radians(2. / 3. * TWO_PI);

    fn kinda_equal(a: (Float, Float, Float), b: (Float, Float, Float)) -> bool {
        let (ax, ay, az) = a;
        let (bx, by, bz) = b;
        (ax - bx).abs() < TEST_ACCURACY
            && (ay - by).abs() < TEST_ACCURACY
            && (az - bz).abs() < TEST_ACCURACY
    }

    fn print_expectations(expected: (Float, Float, Float), actual: (Float, Float, Float)) {
        println!(
            "expected ({}, {}, {}), actual ({}, {}, {})",
            expected.0, expected.1, expected.2, actual.0, actual.1, actual.2
        );
    }

    #[test]
    fn test_rotating_x_around_z() {
        let start = X_VECTOR;

        let rotated = rotated_tuple(start, QUARTER_TURN, &Direction::Z);
        let expected = Y_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, HALF_TURN, &Direction::Z);
        let expected = MINUS_X_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, THREE_QUARTER_TURN, &Direction::Z);
        let expected = MINUS_Y_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, FULL_TURN, &Direction::Z);
        let expected = X_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));
    }

    #[test]
    fn test_rotating_y_around_z() {
        let start = Y_VECTOR;

        let rotated = rotated_tuple(start, QUARTER_TURN, &Direction::Z);
        let expected = MINUS_X_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, HALF_TURN, &Direction::Z);
        let expected = MINUS_Y_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, THREE_QUARTER_TURN, &Direction::Z);
        let expected = X_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, FULL_TURN, &Direction::Z);
        let expected = Y_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));
    }

    #[test]
    fn test_rotating_z_around_z() {
        let start = Z_VECTOR;

        let rotated = rotated_tuple(start, QUARTER_TURN, &Direction::Z);
        let expected = Z_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, HALF_TURN, &Direction::Z);
        let expected = Z_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, THREE_QUARTER_TURN, &Direction::Z);
        let expected = Z_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, FULL_TURN, &Direction::Z);
        let expected = Z_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));
    }

    #[test]
    fn test_rotating_x_around_x() {
        let start = X_VECTOR;

        let rotated = rotated_tuple(start, QUARTER_TURN, &Direction::X);
        let expected = X_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, HALF_TURN, &Direction::X);
        let expected = X_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, THREE_QUARTER_TURN, &Direction::X);
        let expected = X_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, FULL_TURN, &Direction::X);
        let expected = X_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));
    }

    #[test]
    fn test_rotating_y_around_x() {
        let start = Y_VECTOR;

        let rotated = rotated_tuple(start, QUARTER_TURN, &Direction::X);
        let expected = Z_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, HALF_TURN, &Direction::X);
        let expected = MINUS_Y_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, THREE_QUARTER_TURN, &Direction::X);
        let expected = MINUS_Z_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, FULL_TURN, &Direction::X);
        let expected = Y_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));
    }

    #[test]
    fn test_rotating_z_around_x() {
        let start = Z_VECTOR;

        let rotated = rotated_tuple(start, QUARTER_TURN, &Direction::X);
        let expected = MINUS_Y_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, HALF_TURN, &Direction::X);
        let expected = MINUS_Z_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, THREE_QUARTER_TURN, &Direction::X);
        let expected = Y_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, FULL_TURN, &Direction::X);
        let expected = Z_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));
    }

    #[test]
    fn test_rotating_x_around_y() {
        let start = X_VECTOR;

        let rotated = rotated_tuple(start, QUARTER_TURN, &Direction::Y);
        let expected = MINUS_Z_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, HALF_TURN, &Direction::Y);
        let expected = MINUS_X_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, THREE_QUARTER_TURN, &Direction::Y);
        let expected = Z_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, FULL_TURN, &Direction::Y);
        let expected = X_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));
    }

    #[test]
    fn test_rotating_y_around_y() {
        let start = Y_VECTOR;

        let rotated = rotated_tuple(start, QUARTER_TURN, &Direction::Y);
        let expected = Y_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, HALF_TURN, &Direction::Y);
        let expected = Y_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, THREE_QUARTER_TURN, &Direction::Y);
        let expected = Y_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, FULL_TURN, &Direction::Y);
        let expected = Y_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));
    }

    #[test]
    fn test_rotating_z_around_y() {
        let start = Z_VECTOR;

        let rotated = rotated_tuple(start, QUARTER_TURN, &Direction::Y);
        let expected = X_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, HALF_TURN, &Direction::Y);
        let expected = MINUS_Z_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, THREE_QUARTER_TURN, &Direction::Y);
        let expected = MINUS_X_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(start, FULL_TURN, &Direction::Y);
        let expected = Z_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));
    }

    #[test]
    fn test_rotating_around_diagonal_axis() {
        let axis = Direction::new(1., 1., 1.);

        let rotated = rotated_tuple(X_VECTOR, ONE_THIRD_TURN, &axis);
        let expected = Y_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(Y_VECTOR, ONE_THIRD_TURN, &axis);
        let expected = Z_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(Z_VECTOR, ONE_THIRD_TURN, &axis);
        let expected = X_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(X_VECTOR, TWO_THIRDS_TURN, &axis);
        let expected = Z_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(Y_VECTOR, TWO_THIRDS_TURN, &axis);
        let expected = X_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(Z_VECTOR, TWO_THIRDS_TURN, &axis);
        let expected = Y_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(X_VECTOR, -ONE_THIRD_TURN, &axis);
        let expected = Z_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(Y_VECTOR, -ONE_THIRD_TURN, &axis);
        let expected = X_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));

        let rotated = rotated_tuple(Z_VECTOR, -ONE_THIRD_TURN, &axis);
        let expected = Y_VECTOR;
        print_expectations(expected, rotated);
        assert!(kinda_equal(rotated, expected));
    }

    #[test]
    fn get_rotation_parameters_test() {
        const ROTATION_DIRECTION_ACCURACY: Float = 1e-3;
        let problematic = Direction::new(0., 0., 0.);
        let ordinates = vec![-1., 0., 1., 10.];
        for start_x in ordinates.clone().iter() {
            for start_y in ordinates.clone().iter() {
                for start_z in ordinates.clone().iter() {
                    for end_x in ordinates.clone().iter() {
                        for end_y in ordinates.clone().iter() {
                            for end_z in ordinates.clone().iter() {
                                let start = Direction::new(*start_x, *start_y, *start_z);
                                let end = Direction::new(*end_x, *end_y, *end_z);
                                println!("start: {}, end: {}", start, end);
                                if start.eq_within(&problematic, TEST_ACCURACY)
                                    || end.eq_within(&problematic, TEST_ACCURACY)
                                {
                                    continue;
                                }
                                let (angle, axis) = get_rotation_parameters(&start, &end);
                                println!("angle: {}, axis: {}", angle, axis);
                                let rotated = start.rotated(angle, &axis);
                                println!("expected: {}, actual: {}", end, rotated);
                                assert!(rotated.eq_within(&end, ROTATION_DIRECTION_ACCURACY));
                            }
                        }
                    }
                }
            }
        }
    }
}
