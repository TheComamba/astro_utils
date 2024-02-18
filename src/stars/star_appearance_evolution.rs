use super::star_appearance::StarAppearance;
use crate::color::srgb::sRGBColor;
use simple_si_units::{base::Time, electromagnetic::Illuminance};

pub(crate) struct StarAppearanceEvolution {
    lifestage_evolution: Option<LifeStageEvolution>,
}

pub(crate) struct LifeStageEvolution {
    illuminance_per_year: Illuminance<f64>,
    color_per_year: sRGBColor,
}

impl StarAppearanceEvolution {
    pub(crate) fn new(lifestage_evolution: Option<LifeStageEvolution>) -> Self {
        Self {
            lifestage_evolution,
        }
    }
}

impl LifeStageEvolution {
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
