use crate::Float;

pub struct RightAscension {
    pub(super) hours: Float,
    pub(super) minutes: Float,
    pub(super) seconds: Float,
}

impl RightAscension {
    pub const fn new(hours: Float, minutes: Float, seconds: Float) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }
}
