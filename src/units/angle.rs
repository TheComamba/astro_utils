pub const ANGLE_ZERO: Angle<Float> = Angle { rad: 0. };
pub const RADIANS_PER_DEGREE: Float = PI / 180.;

pub fn angle_from_arcsecs(arcsec: Float) -> Angle<Float> {
    Angle {
        radian: arcsec * RADIAN_PER_ARCSEC,
    }
}

pub fn as_arcsecs(&self) -> Float {
    self.radian * ARCSECS_PER_RADIAN
}

pub fn from_second_angle(second_angle: Float) -> Angle {
    Angle {
        radian: second_angle * RADIANS_PER_SECOND_ANGLE,
    }
}

pub fn as_second_angle(&self) -> Float {
    self.radian * SECOND_ANGLE_PER_RADIAN
}

/*
* Normalize the angle to a range of −π to +π radians, -180° to 180°.
*/
pub fn normalize_angle(angle: Angle<Float>) -> Angle<Float> {
    angle.radian = angle.radian % TWO_PI;
    if angle.radian > PI {
        angle.radian -= TWO_PI;
    } else if angle.radian < -PI {
        angle.radian += TWO_PI;
    }
}
