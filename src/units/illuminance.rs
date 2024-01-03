use std::fmt::Display;

use super::{length::Length, luminosity::Luminosity};
use crate::Float;

pub struct Illuminance {
    pub(crate) apparent_magnitude: Float,
}

impl Illuminance {
    pub const fn from_apparent_magnitude(apparent_magnitude: Float) -> Illuminance {
        Illuminance { apparent_magnitude }
    }

    pub const fn get_apparent_magnitude(&self) -> Float {
        self.apparent_magnitude
    }

    pub fn to_luminosity(&self, distance: &Length) -> Luminosity {
        let absolute_magnitude = self.apparent_magnitude - 5. * distance.as_parsecs().log10() + 5.;
        Luminosity::from_absolute_magnitude(absolute_magnitude)
    }
}

impl Display for Illuminance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} app. mag.", self.apparent_magnitude)
    }
}
