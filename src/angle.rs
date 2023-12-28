use std::{
    fmt::Display,
    ops::{Add, Sub},
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

impl Angle {
    pub fn from_radians(radian: Float) -> Angle {
        let mut angle = Angle { radian };
        angle.normalize();
        angle
    }

    pub fn from_degrees(degree: Float) -> Angle {
        let mut angle = Angle {
            radian: degree * RADIANS_PER_DEGREE,
        };
        angle.normalize();
        angle
    }

    pub fn from_arcsecs(arcsec: Float) -> Angle {
        let mut angle = Angle {
            radian: arcsec * RADIAN_PER_ARCSEC,
        };
        angle.normalize();
        angle
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
        let max = if self.radian > other.radian {
            self.radian.abs()
        } else {
            other.radian.abs()
        };
        let diff = (self.radian - other.radian).abs();
        diff <= max * relative_accuracy
    }

    fn normalize(&mut self) {
        while self.radian < 0.0 {
            self.radian += TWO_PI;
        }
        while self.radian >= TWO_PI {
            self.radian -= TWO_PI;
        }
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, other: Angle) -> Angle {
        Angle::from_radians(self.radian + other.radian)
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, other: Angle) -> Angle {
        Angle::from_radians(self.radian - other.radian)
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.2} rad", self.as_radians())
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
    fn test_moduloness() {
        let angle1 = Angle::from_radians(1.0);
        let angle2 = Angle::from_radians(TWO_PI + 1.0);
        let angle3 = Angle::from_radians(-TWO_PI + 1.0);
        assert!(angle1.eq_within(angle2, TEST_ACCURACY));
        assert!(angle1.eq_within(angle3, TEST_ACCURACY));
    }

    #[test]
    fn test_display() {
        let angle = Angle::from_radians(1.0);
        assert_eq!(format!("{}", angle), "1.00 rad");
    }
}
