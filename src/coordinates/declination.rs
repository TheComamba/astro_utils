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
        let degrees = self.degrees as Float;
        let minutes = self.minutes as Float;
        let seconds = self.seconds as Float;

        Angle::from_degrees(degrees + minutes / 60. + seconds / 3600.)
    }
}
