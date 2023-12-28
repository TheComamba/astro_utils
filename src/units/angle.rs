use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use crate::{Float, PI, TWO_PI};

static RADIANS_PER_DEGREE: Float = PI / 180.0;
static DEGREES_PER_RADIAN: Float = 1.0 / RADIANS_PER_DEGREE;
static ARCSECS_PER_RADIAN: Float = 3600.0 * DEGREES_PER_RADIAN;
static RADIAN_PER_ARCSEC: Float = 1.0 / ARCSECS_PER_RADIAN;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Angle {
    radian: Float,
}

pub enum Normalization {
    ZeroToTwoPi,
    MinusPiToPi,
    MinusPiHalfToPiHalf,
}

impl Angle {
    pub const fn from_radians(radian: Float) -> Angle {
        Angle { radian }
    }

    pub fn from_degrees(degree: Float) -> Angle {
        Angle {
            radian: degree * RADIANS_PER_DEGREE,
        }
    }

    pub fn from_arcsecs(arcsec: Float) -> Angle {
        Angle {
            radian: arcsec * RADIAN_PER_ARCSEC,
        }
    }

    pub fn as_radians(&self) -> Float {
        self.radian
    }

    pub fn as_degrees(&self) -> Float {
        self.radian * DEGREES_PER_RADIAN
    }

    pub fn as_arcsecs(&self) -> Float {
        self.radian * ARCSECS_PER_RADIAN
    }

    pub fn eq_within(&self, other: Angle, relative_accuracy: Float) -> bool {
        let abs_accuracy = relative_accuracy * TWO_PI;
        let diff = self.radian - other.radian;
        let diff = diff % TWO_PI;
        let diff = if diff > PI {
            diff - TWO_PI
        } else if diff < -PI {
            diff + TWO_PI
        } else {
            diff
        };
        diff.abs() <= abs_accuracy
    }

    pub fn normalize(&mut self, normalization: Normalization) {
        match normalization {
            Normalization::ZeroToTwoPi => {
                self.radian = self.radian % TWO_PI;
                if self.radian < 0.0 {
                    self.radian += TWO_PI;
                }
            }
            Normalization::MinusPiToPi => {
                self.radian = self.radian % TWO_PI;
                if self.radian > PI {
                    self.radian -= TWO_PI;
                } else if self.radian < -PI {
                    self.radian += TWO_PI;
                }
            }
            Normalization::MinusPiHalfToPiHalf => {
                self.radian = self.radian % PI;
                if self.radian > PI / 2.0 {
                    self.radian -= PI;
                } else if self.radian < -PI / 2.0 {
                    self.radian += PI;
                }
            }
        }
    }

    pub fn sin(&self) -> Float {
        self.radian.sin()
    }

    pub fn cos(&self) -> Float {
        self.radian.cos()
    }

    pub fn tan(&self) -> Float {
        self.radian.tan()
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, other: Angle) -> Angle {
        Angle {
            radian: self.radian + other.radian,
        }
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

impl Mul<Float> for Angle {
    type Output = Angle;

    fn mul(self, f: Float) -> Angle {
        Angle {
            radian: self.radian * f,
        }
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

impl Div<Float> for Angle {
    type Output = Angle;

    fn div(self, f: Float) -> Angle {
        Angle {
            radian: self.radian / f,
        }
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.2} deg", self.as_degrees())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Float;

    const TEST_ACCURACY: Float = 1e-5;

    #[test]
    fn test_radians() {
        let angle = Angle::from_radians(1.0);
        assert!((angle.as_radians() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_degrees() {
        let expected = Angle::from_radians(RADIANS_PER_DEGREE);
        let angle = Angle::from_degrees(1.0);
        assert!(angle.eq_within(expected, TEST_ACCURACY));
        assert!((angle.as_degrees() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_arcsecs() {
        let expected = Angle::from_radians(RADIAN_PER_ARCSEC);
        let angle = Angle::from_arcsecs(1.0);
        assert!(angle.eq_within(expected, TEST_ACCURACY));
        assert!((angle.as_arcsecs() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn quarter_circle_in_various_units() {
        let radians = Angle::from_radians(PI / 2.0);
        let degrees = Angle::from_degrees(90.0);
        let arcsecs = Angle::from_arcsecs(90.0 * 3600.0);
        assert!(radians.eq_within(degrees, TEST_ACCURACY));
        assert!(radians.eq_within(arcsecs, TEST_ACCURACY));
    }

    #[test]
    fn half_circle_in_various_units() {
        let radians = Angle::from_radians(PI);
        let degrees = Angle::from_degrees(180.0);
        let arcsecs = Angle::from_arcsecs(180.0 * 3600.0);
        assert!(radians.eq_within(degrees, TEST_ACCURACY));
        assert!(radians.eq_within(arcsecs, TEST_ACCURACY));
    }

    #[test]
    fn test_add() {
        let angle1 = Angle::from_radians(1.0);
        let angle2 = Angle::from_radians(2.0);
        let expected = Angle::from_radians(3.0);
        assert!(angle1 + angle2 == expected);
    }

    #[test]
    fn test_sub() {
        let angle1 = Angle::from_radians(1.0);
        let angle2 = Angle::from_radians(2.0);
        let expected = Angle::from_radians(-1.0);
        assert!(angle1 - angle2 == expected);
    }

    #[test]
    fn test_normalization_range() {
        for i in -100..100 {
            let radians = TWO_PI / 5.0 * i as Float;
            let mut angle = Angle::from_radians(radians);
            angle.normalize(Normalization::ZeroToTwoPi);
            println!(
                "ZeroToTwoPi: {} deg got normalised to {}",
                radians * DEGREES_PER_RADIAN,
                angle
            );
            assert!(angle.as_radians() >= 0.0);
            assert!(angle.as_radians() < TWO_PI);

            let mut angle = Angle::from_radians(radians);
            angle.normalize(Normalization::MinusPiToPi);
            println!(
                "MinusPiToPi: {} deg got normalised to {}",
                radians * DEGREES_PER_RADIAN,
                angle
            );
            assert!(angle.as_radians() >= -PI);
            assert!(angle.as_radians() < PI);

            let mut angle = Angle::from_radians(radians);
            angle.normalize(Normalization::MinusPiHalfToPiHalf);
            println!(
                "MinusPiHalfToPiHalf: {} deg got normalised to {}",
                radians * DEGREES_PER_RADIAN,
                angle
            );
            assert!(angle.as_radians() >= -PI / 2.0);
            assert!(angle.as_radians() < PI / 2.0);
        }
    }

    #[test]
    fn test_normalizing_quarter_pi() {
        let mut angle = Angle::from_radians(PI / 4.0);
        let expected = Angle::from_radians(PI / 4.0);
        angle.normalize(Normalization::ZeroToTwoPi);
        println!("ZeroToTwoPi: expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, TEST_ACCURACY));

        angle.normalize(Normalization::MinusPiToPi);
        println!("MinusPiToPi: expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, TEST_ACCURACY));

        angle.normalize(Normalization::MinusPiHalfToPiHalf);
        println!(
            "MinusPiHalfToPiHalf: expected: {}, actual: {}",
            expected, angle
        );
        assert!(angle.eq_within(expected, TEST_ACCURACY));
    }

    #[test]
    fn test_normalizing_three_quarters_pi() {
        let mut angle = Angle::from_radians(3.0 * PI / 4.0);
        let expected = Angle::from_radians(3.0 * PI / 4.0);
        angle.normalize(Normalization::ZeroToTwoPi);
        println!("ZeroToTwoPi: expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, TEST_ACCURACY));

        angle.normalize(Normalization::MinusPiToPi);
        println!("MinusPiToPi: expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, TEST_ACCURACY));

        let expected = Angle::from_radians(-PI / 4.0);
        angle.normalize(Normalization::MinusPiHalfToPiHalf);
        println!(
            "MinusPiHalfToPiHalf: expected: {}, actual: {}",
            expected, angle
        );
        assert!(angle.eq_within(expected, TEST_ACCURACY));
    }

    #[test]
    fn test_normalizing_minus_quarter_pi() {
        let mut angle = Angle::from_radians(-PI / 4.0);
        let expected = Angle::from_radians(7.0 * PI / 4.0);
        angle.normalize(Normalization::ZeroToTwoPi);
        println!("ZeroToTwoPi: expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, TEST_ACCURACY));

        let expected = Angle::from_radians(-PI / 4.0);
        angle.normalize(Normalization::MinusPiToPi);
        println!("MinusPiToPi: expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, TEST_ACCURACY));

        angle.normalize(Normalization::MinusPiHalfToPiHalf);
        println!(
            "MinusPiHalfToPiHalf: expected: {}, actual: {}",
            expected, angle
        );
        assert!(angle.eq_within(expected, TEST_ACCURACY));
    }

    #[test]
    fn test_display() {
        let angle = Angle::from_degrees(100.0);
        assert_eq!(format!("{}", angle), "100.00 deg");
    }

    #[test]
    fn test_eq_within() {
        let starts = vec![-10.0, 0.0, 3.0, 7.0];
        let small_diffs = vec![-TEST_ACCURACY / 100.0, 0.0, TEST_ACCURACY / 100.0];
        let large_diffs = vec![-TWO_PI, 0.0, TWO_PI, 100.0 * TWO_PI];
        for start in starts {
            for small_diff in &small_diffs {
                for large_diff in &large_diffs {
                    let rad1 = start;
                    let rad2 = start + small_diff + large_diff;
                    let angle1 = Angle::from_radians(rad1);
                    let angle2 = Angle::from_radians(rad2);
                    println!("Expecting {} == {}", angle1, angle2);
                    assert!(angle1.eq_within(angle2, TEST_ACCURACY));
                }
            }
        }
    }
}
