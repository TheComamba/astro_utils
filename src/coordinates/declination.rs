use crate::Float;

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
}
