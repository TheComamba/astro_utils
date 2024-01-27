use std::fmt::Display;

use crate::{units::angle::Angle, Float};

pub struct RightAscension {
    pub(super) hours: i8,
    pub(super) minutes: i8,
    pub(super) seconds: i8,
}

impl RightAscension {
    pub const fn new(hours: i8, minutes: i8, seconds: i8) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }

    pub fn to_angle(&self) -> Angle {
        let hours = self.hours as Float;
        let minutes = self.minutes as Float;
        let seconds = self.seconds as Float;

        Angle::from_degrees((hours + minutes / 60. + seconds / 3600.) * 15.)
    }
}

impl Display for RightAscension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}h{:02}m{:02}s",
            self.hours, self.minutes, self.seconds
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::TEST_ANGLE_ACCURACY;

    use super::*;

    #[test]
    fn one_second() {
        let dec = RightAscension::new(0, 0, 1);
        let expected = Angle::from_second_angle(1.);
        println!("{}", dec.to_angle().as_arcsecs());
        println!("{}", expected.as_arcsecs());
        assert!(dec.to_angle().eq_within(expected, TEST_ANGLE_ACCURACY));
    }
}
