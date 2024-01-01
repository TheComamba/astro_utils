use super::time::Time;
use crate::Float;
use forward_ref_generic::{forward_ref_binop, forward_ref_op_assign, forward_ref_unop};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

impl Add for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        Time {
            seconds: self.seconds + other.seconds,
        }
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, other: Time) {
        self.seconds += other.seconds;
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
        Time {
            seconds: self.seconds - other.seconds,
        }
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, other: Time) {
        self.seconds -= other.seconds;
    }
}

impl Mul<Float> for Time {
    type Output = Time;

    fn mul(self, f: Float) -> Time {
        Time {
            seconds: self.seconds * f,
        }
    }
}

impl MulAssign<Float> for Time {
    fn mul_assign(&mut self, f: Float) {
        self.seconds *= f;
    }
}

impl Mul<Time> for Float {
    type Output = Time;

    fn mul(self, time: Time) -> Time {
        Time {
            seconds: time.seconds * self,
        }
    }
}

impl MulAssign<Time> for Float {
    fn mul_assign(&mut self, time: Time) {
        *self *= time.seconds;
    }
}

impl Div<Float> for Time {
    type Output = Time;

    fn div(self, f: Float) -> Time {
        Time {
            seconds: self.seconds / f,
        }
    }
}

impl DivAssign<Float> for Time {
    fn div_assign(&mut self, f: Float) {
        self.seconds /= f;
    }
}

impl Div<Time> for Time {
    type Output = Float;

    fn div(self, time: Time) -> Float {
        self.seconds / time.seconds
    }
}

impl Neg for Time {
    type Output = Time;

    fn neg(self) -> Time {
        Time {
            seconds: -self.seconds,
        }
    }
}

impl Rem for Time {
    type Output = Time;

    fn rem(self, other: Time) -> Time {
        Time {
            seconds: self.seconds % other.seconds,
        }
    }
}

impl RemAssign for Time {
    fn rem_assign(&mut self, other: Time) {
        self.seconds %= other.seconds;
    }
}

forward_ref_binop!(impl Add, add for Time, Time);
forward_ref_binop!(impl Sub, sub for Time, Time);
forward_ref_binop!(impl Mul, mul for Time, Float);
forward_ref_binop!(impl Mul, mul for Float, Time);
forward_ref_binop!(impl Div, div for Time, Float);
forward_ref_binop!(impl Rem, rem for Time, Time);
forward_ref_op_assign!(impl AddAssign, add_assign for Time, Time);
forward_ref_op_assign!(impl SubAssign, sub_assign for Time, Time);
forward_ref_op_assign!(impl MulAssign, mul_assign for Time, Float);
forward_ref_op_assign!(impl DivAssign, div_assign for Time, Float);
forward_ref_op_assign!(impl RemAssign, rem_assign for Time, Time);
forward_ref_unop!(impl Neg, neg for Time);
