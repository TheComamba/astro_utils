use crate::Float;

pub struct Declination {
    pub(super) degrees: Float,
    pub(super) minutes: Float,
    pub(super) seconds: Float,
}

impl Declination {
    pub const fn new(degrees: Float, minutes: Float, seconds: Float) -> Self {
        Self {
            degrees,
            minutes,
            seconds,
        }
    }
}
