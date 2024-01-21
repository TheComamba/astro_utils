use crate::{
    color::sRGBColor, coordinates::direction::Direction, units::illuminance::Illuminance, Float,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarAppearance {
    pub(crate) name: String,
    pub(crate) illuminance: Illuminance,
    pub(crate) color: sRGBColor,
    pub(crate) direction_in_ecliptic: Direction,
}

impl StarAppearance {
    pub fn new(
        name: String,
        illuminance: Illuminance,
        color: sRGBColor,
        direction_in_ecliptic: Direction,
    ) -> Self {
        Self {
            name,
            illuminance,
            color,
            direction_in_ecliptic,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub const fn get_illuminance(&self) -> &Illuminance {
        &self.illuminance
    }

    pub const fn get_color(&self) -> &sRGBColor {
        &self.color
    }

    pub const fn get_direction_in_ecliptic(&self) -> &Direction {
        &self.direction_in_ecliptic
    }

    pub fn set_direction_in_ecliptic(&mut self, direction: Direction) {
        self.direction_in_ecliptic = direction;
    }

    pub(super) fn apparently_the_same(&self, other: &Self) -> bool {
        const DIRECTION_ACCURACY: Float = 1e-5;

        let illuminance_ration = self.illuminance.as_lux() / other.illuminance.as_lux();
        illuminance_ration < 0.1
            || illuminance_ration > 10.0
                && self
                    .direction_in_ecliptic
                    .eq_within(&other.direction_in_ecliptic, DIRECTION_ACCURACY)
    }
}
