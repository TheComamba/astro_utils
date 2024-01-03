use crate::{Float, PI, TWO_PI};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub(crate) const RADIANS_PER_DEGREE: Float = PI / 180.;
const DEGREES_PER_RADIAN: Float = 1. / RADIANS_PER_DEGREE;
const ARCSECS_PER_RADIAN: Float = 3600. * DEGREES_PER_RADIAN;
const RADIAN_PER_ARCSEC: Float = 1. / ARCSECS_PER_RADIAN;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Angle {
    pub(super) radian: Float,
}

impl Angle {
    pub const ZERO: Angle = Angle { radian: 0. };

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

    pub const fn as_radians(&self) -> Float {
        self.radian
    }

    pub fn as_degrees(&self) -> Float {
        self.radian * DEGREES_PER_RADIAN
    }

    pub fn as_arcsecs(&self) -> Float {
        self.radian * ARCSECS_PER_RADIAN
    }

    pub fn eq_within(&self, other: Angle, accuracy: Angle) -> bool {
        let diff = self.radian - other.radian;
        let diff = diff % TWO_PI;
        let diff = if diff > PI {
            diff - TWO_PI
        } else if diff < -PI {
            diff + TWO_PI
        } else {
            diff
        };
        diff.abs() <= accuracy.radian
    }

    /*
     * Normalize the angle to a range of −π to +π radians, -180° to 180°.
     */
    pub fn normalize(&mut self) {
        self.radian = self.radian % TWO_PI;
        if self.radian > PI {
            self.radian -= TWO_PI;
        } else if self.radian < -PI {
            self.radian += TWO_PI;
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

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.2} deg", self.as_degrees())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        tests::{TEST_ACCURACY, TEST_ANGLE_ACCURACY},
        Float,
    };

    #[test]
    fn test_radians() {
        let angle = Angle::from_radians(1.);
        assert!((angle.as_radians() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_degrees() {
        let expected = Angle::from_radians(RADIANS_PER_DEGREE);
        let angle = Angle::from_degrees(1.);
        assert!(angle.eq_within(expected, TEST_ANGLE_ACCURACY));
        assert!((angle.as_degrees() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_arcsecs() {
        let expected = Angle::from_radians(RADIAN_PER_ARCSEC);
        let angle = Angle::from_arcsecs(1.);
        assert!(angle.eq_within(expected, TEST_ANGLE_ACCURACY));
        assert!((angle.as_arcsecs() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn quarter_circle_in_various_units() {
        let radians = Angle::from_radians(PI / 2.);
        let degrees = Angle::from_degrees(90.);
        let arcsecs = Angle::from_arcsecs(90. * 3600.);
        assert!(radians.eq_within(degrees, TEST_ANGLE_ACCURACY));
        assert!(radians.eq_within(arcsecs, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn half_circle_in_various_units() {
        let radians = Angle::from_radians(PI);
        let degrees = Angle::from_degrees(180.);
        let arcsecs = Angle::from_arcsecs(180. * 3600.);
        assert!(radians.eq_within(degrees, TEST_ANGLE_ACCURACY));
        assert!(radians.eq_within(arcsecs, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn test_add() {
        let angle1 = Angle::from_radians(1.);
        let angle2 = Angle::from_radians(2.);
        let expected = Angle::from_radians(3.);
        assert!(angle1 + angle2 == expected);
    }

    #[test]
    fn test_sub() {
        let angle1 = Angle::from_radians(1.);
        let angle2 = Angle::from_radians(2.);
        let expected = Angle::from_radians(-1.);
        assert!(angle1 - angle2 == expected);
    }

    #[test]
    fn test_normalization_range() {
        for i in -100..100 {
            let radians = TWO_PI / 5. * i as Float;
            let mut angle = Angle::from_radians(radians);
            angle.normalize();
            println!(
                "{} deg got normalised to {}",
                radians * DEGREES_PER_RADIAN,
                angle
            );
            assert!(angle.as_radians() >= -PI);
            assert!(angle.as_radians() < PI);
        }
    }

    #[test]
    fn test_normalizing_quarter_pi() {
        let mut angle = Angle::from_radians(PI / 4.);
        let expected = Angle::from_radians(PI / 4.);
        angle.normalize();
        println!("expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn test_normalizing_three_quarters_pi() {
        let mut angle = Angle::from_radians(3. * PI / 4.);
        let expected = Angle::from_radians(3. * PI / 4.);
        angle.normalize();
        println!("expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn test_normalizing_minus_quarter_pi() {
        let mut angle = Angle::from_radians(-PI / 4.);
        let expected = Angle::from_radians(-PI / 4.);
        angle.normalize();
        println!("expected: {}, actual: {}", expected, angle);
        assert!(angle.eq_within(expected, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn test_display() {
        let angle = Angle::from_degrees(100.);
        assert_eq!(format!("{}", angle), "100.00 deg");
    }

    #[test]
    fn test_eq_within() {
        let starts = vec![-10., 0., 3., 7.0];
        let small_diffs = vec![-TEST_ACCURACY / 100., 0., TEST_ACCURACY / 100.];
        let large_diffs = vec![-TWO_PI, 0., TWO_PI, 100. * TWO_PI];
        for start in starts {
            for small_diff in &small_diffs {
                for large_diff in &large_diffs {
                    let rad1 = start;
                    let rad2 = start + small_diff + large_diff;
                    let angle1 = Angle::from_radians(rad1);
                    let angle2 = Angle::from_radians(rad2);
                    println!("Expecting {} == {}", angle1, angle2);
                    assert!(angle1.eq_within(angle2, TEST_ANGLE_ACCURACY));
                }
            }
        }
    }
}
