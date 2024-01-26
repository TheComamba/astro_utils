use std::fmt::Display;

use crate::{units::angle::Angle, Float};

pub struct Declination {
    pub(super) degrees: i8,
    pub(super) minutes: i8,
    pub(super) seconds: i8,
}

impl Declination {
    pub const fn new(degrees: i8, minutes: i8, seconds: i8) -> Self {
        Self {
            degrees,
            minutes,
            seconds,
        }
    }

    pub fn to_angle(&self) -> Angle {
        let degrees = self.degrees.abs() as Float;
        let minutes = self.minutes as Float;
        let seconds = self.seconds as Float;

        Angle::from_degrees(degrees + minutes / 60. + seconds / 3600.)
            * self.degrees.signum() as Float
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
        let dec = Declination::new(0, 0, 1);
        assert!((dec.to_angle().as_arcsecs() - 1.) < 1e-5);
    }

    #[test]
    fn small_angle_roundtrips() {
        const STEPS: i8 = 10;
        for sec in 0..STEPS {
            for min in 0..STEPS {
                for sign in [-1, 1] {
                    let angle = Angle::from_arcsecs((sign * (min * 60 + sec)) as Float);
                    let dec = Declination::new(sign * 0, min, sec);
                    println!("{} sign {} min {} sec", sign, min, sec);
                    println!("expected: \n{} sec", angle.as_arcsecs());
                    println!("actual: \n{} sec", dec.to_angle().as_arcsecs());
                    assert!((dec.to_angle().as_arcsecs() - angle.as_arcsecs()).abs() < 1e-5);
                }
            }
        }
    }
}
