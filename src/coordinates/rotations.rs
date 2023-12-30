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
