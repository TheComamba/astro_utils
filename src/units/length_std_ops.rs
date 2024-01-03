use super::length::Length;
use crate::Float;
use forward_ref_generic::{forward_ref_binop, forward_ref_op_assign, forward_ref_unop};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

impl Add for Length {
    type Output = Length;

    fn add(self, other: Length) -> Length {
        Length {
            au: self.au + other.au,
        }
    }
}

impl AddAssign for Length {
    fn add_assign(&mut self, other: Length) {
        self.au += other.au;
    }
}

impl Sub for Length {
    type Output = Length;

    fn sub(self, other: Length) -> Length {
        Length {
            au: self.au - other.au,
        }
    }
}

impl SubAssign for Length {
    fn sub_assign(&mut self, other: Length) {
        self.au -= other.au;
    }
}

impl Mul<Float> for Length {
    type Output = Length;

    fn mul(self, f: Float) -> Length {
        Length { au: self.au * f }
    }
}

impl MulAssign<Float> for Length {
    fn mul_assign(&mut self, f: Float) {
        self.au *= f;
    }
}

impl Mul<Length> for Float {
    type Output = Length;

    fn mul(self, length: Length) -> Length {
        Length {
            au: self * length.au,
        }
    }
}

impl MulAssign<Length> for Float {
    fn mul_assign(&mut self, length: Length) {
        *self *= length.au;
    }
}

impl Div<Float> for Length {
    type Output = Length;

    fn div(self, f: Float) -> Length {
        Length { au: self.au / f }
    }
}

impl DivAssign<Float> for Length {
    fn div_assign(&mut self, f: Float) {
        self.au /= f;
    }
}

impl Div<Length> for Length {
    type Output = Float;

    fn div(self, other: Length) -> Float {
        self.au / other.au
    }
}

impl DivAssign<Length> for Length {
    fn div_assign(&mut self, other: Length) {
        self.au /= other.au;
    }
}

impl Neg for Length {
    type Output = Length;

    fn neg(self) -> Length {
        Length { au: -self.au }
    }
}

impl Rem for Length {
    type Output = Length;

    fn rem(self, other: Length) -> Length {
        Length {
            au: self.au % other.au,
        }
    }
}

impl RemAssign for Length {
    fn rem_assign(&mut self, other: Length) {
        self.au %= other.au;
    }
}

forward_ref_binop!(impl Add, add for Length, Length);
forward_ref_binop!(impl Sub, sub for Length, Length);
forward_ref_binop!(impl Mul, mul for Length, Float);
forward_ref_binop!(impl Mul, mul for Float, Length);
forward_ref_binop!(impl Div, div for Length, Float);
forward_ref_binop!(impl Div, div for Length, Length);
forward_ref_op_assign!(impl AddAssign, add_assign for Length, Length);
forward_ref_op_assign!(impl SubAssign, sub_assign for Length, Length);
forward_ref_op_assign!(impl MulAssign, mul_assign for Length, Float);
forward_ref_op_assign!(impl DivAssign, div_assign for Length, Float);
forward_ref_op_assign!(impl DivAssign, div_assign for Length, Length);
forward_ref_unop!(impl Neg, neg for Length);
