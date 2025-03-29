use std::f64::consts::PI;

use uom::si::{
    angle::{degree, radian},
    f64::Angle,
};

use crate::astro_display::AstroDisplay;

pub fn ANGLE_ZERO() -> Angle {
    Angle::new::<radian>(0.)
}
pub(crate) fn FULL_CIRC() -> Angle {
    Angle::new::<radian>(2. * PI)
}
pub(crate) fn QUARTER_CIRC() -> Angle {
    Angle::new::<radian>(2. * PI / 4.)
}
pub(crate) fn HALF_CIRC() -> Angle {
    Angle::new::<radian>(2. * PI / 2.)
}
#[cfg(test)]
pub(crate) fn THREE_QUARTER_CIRC() -> Angle {
    Angle::new::<radian>(2. * PI * 3. / 4.)
}

pub const ARCSEC: Angle = Angle {
    rad: 2. * PI / (360. * 60. * 60.),
};
pub const SECOND_ANGLE: Angle = Angle {
    rad: 2. * PI / (24. * 60. * 60.),
};

pub fn angle_from_arcsecs(arcsec: f64) -> Angle {
    arcsec * ARCSEC
}

pub fn angle_to_arcsecs(angle: &Angle) -> f64 {
    angle / &ARCSEC
}

pub fn angle_from_second_angle(second_angle: f64) -> Angle {
    second_angle * SECOND_ANGLE
}

pub fn angle_to_second_angle(angle: &Angle) -> f64 {
    angle / &SECOND_ANGLE
}

/*
* Normalize the angle to a range of −π to +π radians, -180° to 180°.
*/
pub fn normalized_angle(mut angle: Angle) -> Angle {
    angle %= FULL_CIRC();
    if angle > HALF_CIRC() {
        angle -= FULL_CIRC();
    } else if angle < -HALF_CIRC() {
        angle += FULL_CIRC();
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
    fn arcsec_roundtrip() {
        for i in -10..10 {
            let input = i as f64;
            let angle = angle_from_arcsecs(input);
            let output = angle_to_arcsecs(&angle);
            assert!(eq(input, output));
        }
    }

    #[test]
    fn second_angle_roundtrip() {
        for i in -10..10 {
            let input = i as f64;
            let angle = angle_from_second_angle(input);
            let output = angle_to_second_angle(&angle);
            assert!(eq(input, output));
        }
    }
}
