use super::illuminance::Illuminance;
use crate::Float;
use forward_ref_generic::{forward_ref_binop, forward_ref_op_assign};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

impl Add for Illuminance {
    type Output = Illuminance;

    fn add(self, other: Illuminance) -> Illuminance {
        Illuminance {
            lux: self.lux + other.lux,
        }
    }
}

impl AddAssign for Illuminance {
    fn add_assign(&mut self, other: Illuminance) {
        self.lux += other.lux;
    }
}

impl Sub for Illuminance {
    type Output = Illuminance;

    fn sub(self, other: Illuminance) -> Illuminance {
        Illuminance {
            lux: self.lux - other.lux,
        }
    }
}

impl SubAssign for Illuminance {
    fn sub_assign(&mut self, other: Illuminance) {
        self.lux -= other.lux;
    }
}

impl Mul<Float> for Illuminance {
    type Output = Illuminance;

    fn mul(self, f: Float) -> Illuminance {
        Illuminance { lux: self.lux * f }
    }
}

impl MulAssign<Float> for Illuminance {
    fn mul_assign(&mut self, f: Float) {
        self.lux *= f;
    }
}

impl Mul<Illuminance> for Float {
    type Output = Illuminance;

    fn mul(self, illuminance: Illuminance) -> Illuminance {
        Illuminance {
            lux: self * illuminance.lux,
        }
    }
}

impl MulAssign<Illuminance> for Float {
    fn mul_assign(&mut self, illuminance: Illuminance) {
        *self *= illuminance.lux;
    }
}

impl Div<Float> for Illuminance {
    type Output = Illuminance;

    fn div(self, f: Float) -> Illuminance {
        Illuminance { lux: self.lux / f }
    }
}

impl DivAssign<Float> for Illuminance {
    fn div_assign(&mut self, f: Float) {
        self.lux /= f;
    }
}

impl Div<Illuminance> for Illuminance {
    type Output = Float;

    fn div(self, other: Illuminance) -> Float {
        self.lux / other.lux
    }
}

forward_ref_binop!(impl Add, add for Illuminance, Illuminance);
forward_ref_binop!(impl Sub, sub for Illuminance, Illuminance);
forward_ref_binop!(impl Mul, mul for Illuminance, Float);
forward_ref_binop!(impl Mul, mul for Float, Illuminance);
forward_ref_binop!(impl Div, div for Illuminance, Float);
forward_ref_binop!(impl Div, div for Illuminance, Illuminance);
forward_ref_op_assign!(impl AddAssign, add_assign for Illuminance, Illuminance);
forward_ref_op_assign!(impl SubAssign, sub_assign for Illuminance, Illuminance);
forward_ref_op_assign!(impl MulAssign, mul_assign for Illuminance, Float);
forward_ref_op_assign!(impl DivAssign, div_assign for Illuminance, Float);
