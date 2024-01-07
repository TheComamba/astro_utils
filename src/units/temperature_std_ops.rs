use super::temperature::Temperature;
use crate::Float;
use forward_ref_generic::{forward_ref_binop, forward_ref_op_assign, forward_ref_unop};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Add for Temperature {
    type Output = Temperature;

    fn add(self, other: Temperature) -> Temperature {
        Temperature {
            kelvin: self.kelvin + other.kelvin,
        }
    }
}

impl AddAssign for Temperature {
    fn add_assign(&mut self, other: Temperature) {
        self.kelvin += other.kelvin;
    }
}

impl Sub for Temperature {
    type Output = Temperature;

    fn sub(self, other: Temperature) -> Temperature {
        Temperature {
            kelvin: self.kelvin - other.kelvin,
        }
    }
}

impl SubAssign for Temperature {
    fn sub_assign(&mut self, other: Temperature) {
        self.kelvin -= other.kelvin;
    }
}

impl Mul<Float> for Temperature {
    type Output = Temperature;

    fn mul(self, f: Float) -> Temperature {
        Temperature {
            kelvin: self.kelvin * f,
        }
    }
}

impl MulAssign<Float> for Temperature {
    fn mul_assign(&mut self, f: Float) {
        self.kelvin *= f;
    }
}

impl Mul<Temperature> for Float {
    type Output = Temperature;

    fn mul(self, temperature: Temperature) -> Temperature {
        Temperature {
            kelvin: self * temperature.kelvin,
        }
    }
}

impl MulAssign<Temperature> for Float {
    fn mul_assign(&mut self, temperature: Temperature) {
        *self *= temperature.kelvin;
    }
}

impl Div<Float> for Temperature {
    type Output = Temperature;

    fn div(self, f: Float) -> Temperature {
        Temperature {
            kelvin: self.kelvin / f,
        }
    }
}

impl DivAssign<Float> for Temperature {
    fn div_assign(&mut self, f: Float) {
        self.kelvin /= f;
    }
}

impl Div<Temperature> for Temperature {
    type Output = Float;

    fn div(self, temperature: Temperature) -> Float {
        self.kelvin / temperature.kelvin
    }
}

impl Neg for Temperature {
    type Output = Temperature;

    fn neg(self) -> Temperature {
        Temperature {
            kelvin: -self.kelvin,
        }
    }
}

forward_ref_binop!(impl Add, add for Temperature, Temperature);
forward_ref_binop!(impl Sub, sub for Temperature, Temperature);
forward_ref_binop!(impl Mul, mul for Temperature, Float);
forward_ref_binop!(impl Mul, mul for Float, Temperature);
forward_ref_binop!(impl Div, div for Temperature, Float);
forward_ref_binop!(impl Div, div for Temperature, Temperature);
forward_ref_op_assign!(impl AddAssign, add_assign for Temperature, Temperature);
forward_ref_op_assign!(impl SubAssign, sub_assign for Temperature, Temperature);
forward_ref_op_assign!(impl MulAssign, mul_assign for Temperature, Float);
forward_ref_op_assign!(impl DivAssign, div_assign for Temperature, Float);
forward_ref_unop!(impl Neg, neg for Temperature);
