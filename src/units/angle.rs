use std::f64::consts::PI;

use uom::si::{
    angle::{degree, radian},
    f64::Angle,
};

use crate::astro_display::AstroDisplay;

#[inline(always)]
pub fn angle_zero() -> Angle {
    Angle::new::<radian>(0.)
}

#[inline(always)]
pub(crate) fn full_circ() -> Angle {
    Angle::new::<radian>(2. * PI)
}

#[inline(always)]
pub(crate) fn quarter_circ() -> Angle {
    Angle::new::<radian>(2. * PI / 4.)
}

#[inline(always)]
pub(crate) fn half_circ() -> Angle {
    Angle::new::<radian>(2. * PI / 2.)
}
#[cfg(test)]
#[inline(always)]
pub(crate) fn three_quarter_circ() -> Angle {
    Angle::new::<radian>(2. * PI * 3. / 4.)
}

#[inline(always)]
pub fn one_second_angle() -> Angle {
    Angle::new::<radian>(2. * PI / (24. * 60. * 60.))
}

#[inline(always)]
pub fn angle_from_second_angle(second_angle: f64) -> Angle {
    second_angle * one_second_angle()
}

#[inline(always)]
pub fn angle_to_second_angle(angle: Angle) -> f64 {
    (angle / one_second_angle()).into()
}

/*
* Normalize the angle to a range of −π to +π radians, -180° to 180°.
*/
pub fn normalized_angle(mut angle: Angle) -> Angle {
    angle %= full_circ();
    if angle > half_circ() {
        angle -= full_circ();
    } else if angle < -half_circ() {
        angle += full_circ();
    }
    angle
}

#[cfg(test)]
pub(crate) fn angle_eq_within(actual: Angle, expected: Angle, accuracy: Angle) -> bool {
    let diff = normalized_angle(actual - expected);
    diff.abs() < accuracy
}

#[cfg(test)]
pub(crate) fn angle_eq(actual: Angle, expected: Angle) -> bool {
    use uom::si::angle::radian;

    use crate::tests::TEST_ACCURACY;

    angle_eq_within(actual, expected, Angle::new::<radian>(TEST_ACCURACY))
}

impl AstroDisplay for Angle {
    fn astro_display(&self) -> String {
        format!("{:.2} °", self.get::<degree>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::eq;

    #[test]
    fn second_angle_roundtrip() {
        for i in -10..10 {
            let input = i as f64;
            let angle = angle_from_second_angle(input);
            let output = angle_to_second_angle(angle);
            assert!(eq(input, output));
        }
    }
}
