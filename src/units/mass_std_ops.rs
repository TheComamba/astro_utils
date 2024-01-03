use super::mass::Mass;
use crate::Float;
use forward_ref_generic::{forward_ref_binop, forward_ref_op_assign};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

impl Add for Mass {
    type Output = Mass;

    fn add(self, other: Mass) -> Mass {
        Mass {
            kilograms: self.kilograms + other.kilograms,
        }
    }
}

impl AddAssign for Mass {
    fn add_assign(&mut self, other: Mass) {
        self.kilograms += other.kilograms;
    }
}

impl Sub for Mass {
    type Output = Mass;

    fn sub(self, other: Mass) -> Mass {
        Mass {
            kilograms: self.kilograms - other.kilograms,
        }
    }
}

impl SubAssign for Mass {
    fn sub_assign(&mut self, other: Mass) {
        self.kilograms -= other.kilograms;
    }
}

impl Mul<Float> for Mass {
    type Output = Mass;

    fn mul(self, f: Float) -> Mass {
        Mass {
            kilograms: self.kilograms * f,
        }
    }
}

impl MulAssign<Float> for Mass {
    fn mul_assign(&mut self, f: Float) {
        self.kilograms *= f;
    }
}

impl Mul<Mass> for Float {
    type Output = Mass;

    fn mul(self, mass: Mass) -> Mass {
        Mass {
            kilograms: self * mass.kilograms,
        }
    }
}

impl MulAssign<Mass> for Float {
    fn mul_assign(&mut self, mass: Mass) {
        *self *= mass.kilograms;
    }
}

impl Div<Float> for Mass {
    type Output = Mass;

    fn div(self, f: Float) -> Mass {
        Mass {
            kilograms: self.kilograms / f,
        }
    }
}

impl DivAssign<Float> for Mass {
    fn div_assign(&mut self, f: Float) {
        self.kilograms /= f;
    }
}

impl Div<Mass> for Mass {
    type Output = Float;

    fn div(self, other: Mass) -> Float {
        self.kilograms / other.kilograms
    }
}

forward_ref_binop!(impl Add, add for Mass, Mass);
forward_ref_binop!(impl Sub, sub for Mass, Mass);
forward_ref_binop!(impl Mul, mul for Mass, Float);
forward_ref_binop!(impl Mul, mul for Float, Mass);
forward_ref_binop!(impl Div, div for Mass, Float);
forward_ref_binop!(impl Div, div for Mass, Mass);
forward_ref_op_assign!(impl AddAssign, add_assign for Mass, Mass);
forward_ref_op_assign!(impl SubAssign, sub_assign for Mass, Mass);
forward_ref_op_assign!(impl MulAssign, mul_assign for Mass, Float);
forward_ref_op_assign!(impl DivAssign, div_assign for Mass, Float);
