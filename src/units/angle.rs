use simple_si_units::geometry::Angle;
use std::f64::consts::PI;

pub const ANGLE_ZERO: Angle<f64> = Angle { rad: 0. };
pub(crate) const FULL_CIRC: Angle<f64> = Angle { rad: 2. * PI };
pub(crate) const QUARTER_CIRC: Angle<f64> = Angle { rad: 2. * PI / 4. };
pub(crate) const HALF_CIRC: Angle<f64> = Angle { rad: 2. * PI / 2. };
pub(crate) const THREE_QUARTER_CIRC: Angle<f64> = Angle {
    rad: 2. * PI * 3. / 4.,
};
pub(crate) const ONE_THIRD_CIRC: Angle<f64> = Angle { rad: 2. * PI / 3. };
pub(crate) const TWO_THIRDS_CIRC: Angle<f64> = Angle {
    rad: 2. * PI * 2. / 3.,
};

pub(crate) const RADIANS_PER_DEGREE: f64 = PI / 180.;
pub(crate) const DEGREES_PER_RADIAN: f64 = 1. / RADIANS_PER_DEGREE;
pub(crate) const ARCSECS_PER_RADIAN: f64 = 3600. * DEGREES_PER_RADIAN;
pub(crate) const RADIAN_PER_ARCSEC: f64 = 1. / ARCSECS_PER_RADIAN;
pub(crate) const SECOND_ANGLE_PER_RADIAN: f64 = 24. * 60. * 60. / 2. / PI;
pub(crate) const RADIANS_PER_SECOND_ANGLE: f64 = 1. / SECOND_ANGLE_PER_RADIAN;

pub fn angle_from_arcsecs(arcsec: f64) -> Angle<f64> {
    Angle {
        rad: arcsec * RADIAN_PER_ARCSEC,
    }
}

pub fn angle_to_arcsecs(angle: &Angle<f64>) -> f64 {
    angle.rad * ARCSECS_PER_RADIAN
}

pub fn angle_from_second_angle(second_angle: f64) -> Angle<f64> {
    Angle {
        rad: second_angle * RADIANS_PER_SECOND_ANGLE,
    }
}

pub fn angle_to_second_angle(angle: &Angle<f64>) -> f64 {
    angle.rad * SECOND_ANGLE_PER_RADIAN
}

/*
* Normalize the angle to a range of −π to +π radians, -180° to 180°.
*/
pub fn normalized_angle(mut angle: Angle<f64>) -> Angle<f64> {
    angle.rad = angle.rad % FULL_CIRC.rad;
    if angle.rad > PI {
        angle.rad -= FULL_CIRC.rad;
    } else if angle.rad < -PI {
        angle.rad += FULL_CIRC.rad;
    }
    angle
}
