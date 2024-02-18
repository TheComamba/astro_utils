use super::star_appearance::StarAppearance;
use crate::color::srgb::sRGBColor;
use serde::{Deserialize, Serialize};
use simple_si_units::{base::Time, electromagnetic::Illuminance};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarAppearanceEvolution {
    lifestage_evolution: Option<LifestageEvolution>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct LifestageEvolution {
    illuminance_per_year: Illuminance<f64>,
    color_per_year: sRGBColor,
}

impl StarAppearanceEvolution {
    pub const NONE: StarAppearanceEvolution = StarAppearanceEvolution {
        lifestage_evolution: None,
    };

    pub(crate) fn new(lifestage_evolution: Option<LifestageEvolution>) -> Self {
        Self {
            lifestage_evolution,
        }
    }
}

impl LifestageEvolution {
    pub(crate) fn new(
        now: &StarAppearance,
        then: &StarAppearance,
        time_difference: Time<f64>,
    ) -> Self {
        let illuminance_per_year =
            (now.get_illuminance() - then.get_illuminance()) / time_difference.to_yr();
        let color_per_year = now.get_color() - then.get_color();
        Self {
            illuminance_per_year,
            color_per_year,
        }
    }
}
