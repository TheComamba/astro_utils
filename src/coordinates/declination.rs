use std::fmt::Display;

use crate::{units::angle::Angle, Float};

pub struct Declination {
    pub(super) sign: Sgn,
    pub(super) degrees: u8,
    pub(super) minutes: u8,
    pub(super) seconds: u8,
}

#[derive(Copy, Clone)]
pub enum Sgn {
    Pos,
    Neg,
}

impl Declination {
    pub const fn new(sign: Sgn, degrees: u8, minutes: u8, seconds: u8) -> Self {
        Self {
            sign,
            degrees,
            minutes,
            seconds,
        }
    }

    pub fn to_angle(&self) -> Angle {
        let sign = match self.sign {
            Sgn::Pos => 1.,
            Sgn::Neg => -1.,
        };
        let degrees = self.degrees as Float;
        let minutes = self.minutes as Float;
        let seconds = self.seconds as Float;

        sign * Angle::from_degrees(degrees + minutes / 60. + seconds / 3600.)
    }
}

impl Display for Declination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}°{:02}'{:02}\"",
            self.degrees, self.minutes, self.seconds
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_second() {
        let dec = Declination::new(Sgn::Pos, 0, 0, 1);
        assert!((dec.to_angle().as_arcsecs() - 1.) < 1e-5);
    }

    #[test]
    fn small_angle_roundtrips() {
        const STEPS: u8 = 10;
        for sec in 0..STEPS {
            for min in 0..STEPS {
                for sign in [Sgn::Pos, Sgn::Neg] {
                    let dec = Declination::new(sign, 0, min, sec);
                    let angle_abs = (min * 60 + sec) as Float;
                    let sign = match sign {
                        Sgn::Pos => 1.,
                        Sgn::Neg => -1.,
                    };
                    let angle = Angle::from_arcsecs(sign * angle_abs);
                    println!("{} sign {} min {} sec", sign, min, sec);
                    println!("expected: \n{} sec", angle.as_arcsecs());
                    println!("actual: \n{} sec", dec.to_angle().as_arcsecs());
                    assert!((dec.to_angle().as_arcsecs() - angle.as_arcsecs()).abs() < 1e-5);
                }
            }
        }
    }
}
