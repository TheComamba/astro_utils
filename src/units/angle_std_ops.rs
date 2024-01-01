use super::angle::Angle;
use crate::Float;
use forward_ref_generic::{forward_ref_binop, forward_ref_op_assign, forward_ref_unop};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Add for Angle {
    type Output = Angle;

    fn add(self, other: Angle) -> Angle {
        Angle {
            radian: self.radian + other.radian,
        }
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, other: Angle) {
        self.radian += other.radian;
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, other: Angle) -> Angle {
        Angle {
            radian: self.radian - other.radian,
        }
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, other: Angle) {
        self.radian -= other.radian;
    }
}

impl Mul<Float> for Angle {
    type Output = Angle;

    fn mul(self, f: Float) -> Angle {
        Angle {
            radian: self.radian * f,
        }
    }
}

impl MulAssign<Float> for Angle {
    fn mul_assign(&mut self, f: Float) {
        self.radian *= f;
    }
}

impl Mul<Angle> for Float {
    type Output = Angle;

    fn mul(self, angle: Angle) -> Angle {
        Angle {
            radian: self * angle.radian,
        }
    }
}

impl MulAssign<Angle> for Float {
    fn mul_assign(&mut self, angle: Angle) {
        *self *= angle.radian;
    }
}

impl Div<Float> for Angle {
    type Output = Angle;

    fn div(self, f: Float) -> Angle {
        Angle {
            radian: self.radian / f,
        }
    }
}

impl DivAssign<Float> for Angle {
    fn div_assign(&mut self, f: Float) {
        self.radian /= f;
    }
}

impl Neg for Angle {
    type Output = Angle;

    fn neg(self) -> Angle {
        Angle {
            radian: -self.radian,
        }
    }
}

forward_ref_binop!(impl Add, add for Angle, Angle);
forward_ref_binop!(impl Sub, sub for Angle, Angle);
forward_ref_binop!(impl Mul, mul for Angle, Float);
forward_ref_binop!(impl Mul, mul for Float, Angle);
forward_ref_binop!(impl Div, div for Angle, Float);
forward_ref_op_assign!(impl AddAssign, add_assign for Angle, Angle);
forward_ref_op_assign!(impl SubAssign, sub_assign for Angle, Angle);
forward_ref_op_assign!(impl MulAssign, mul_assign for Angle, Float);
forward_ref_op_assign!(impl DivAssign, div_assign for Angle, Float);
forward_ref_unop!(impl Neg, neg for Angle);
