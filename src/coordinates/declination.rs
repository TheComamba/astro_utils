use simple_si_units::geometry::Angle;
use std::fmt::Display;

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

    pub fn to_angle(&self) -> Angle<f64> {
        let sign = match self.sign {
            Sgn::Pos => 1.,
            Sgn::Neg => -1.,
        };
        let degrees = self.degrees as f64;
        let minutes = self.minutes as f64;
        let seconds = self.seconds as f64;

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
                    let angle_abs = ((min as u32) * 60 + sec as u32) as f64;
                    let sign = match sign {
                        Sgn::Pos => 1.,
                        Sgn::Neg => -1.,
                    };
                    let angle = Angle::from_arcsecs(sign * angle_abs);
                    println!("{} sign {} min {} sec", sign, min, sec);
                    println!("expected: \n{} sec", angle.as_arcsecs());
                    println!("actual: \n{} sec", dec.to_angle().as_arcsecs());
                    let diff = (dec.to_angle().as_arcsecs() - angle.as_arcsecs()).abs();
                    println!("diff: \n{} sec", diff);
                    let mut accuracy = 1e-5 * angle_abs;
                    if accuracy < 1e-5 {
                        accuracy = 1e-5;
                    }
                    println!("accuracy: \n{} sec", accuracy);
                    assert!(diff < accuracy);
                }
            }
        }
    }
}
