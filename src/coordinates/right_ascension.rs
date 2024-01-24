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
