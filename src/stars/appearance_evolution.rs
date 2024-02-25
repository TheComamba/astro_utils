use super::fate::StarFate;
use crate::{
    color::srgb::sRGBColor,
    units::{illuminance::IRRADIANCE_ZERO, time::TIME_ZERO},
};
use serde::{Deserialize, Serialize};
use simple_si_units::{base::Time, electromagnetic::Illuminance};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarAppearanceEvolution {
    lifestage_evolution: Option<StarAppearanceLifestageEvolution>,
    pub(super) lifetime: Time<f64>,
    pub(super) fate: StarFate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct StarAppearanceLifestageEvolution {
    pub(super) illuminance_per_year: Illuminance<f64>,
    pub(super) color_per_year: sRGBColor,
}

impl StarAppearanceEvolution {
    pub const NONE: StarAppearanceEvolution = StarAppearanceEvolution {
        lifestage_evolution: None,
        lifetime: TIME_ZERO,
        fate: StarFate::WhiteDwarf,
    };

    pub(crate) fn new(
        lifestage_evolution: Option<StarAppearanceLifestageEvolution>,
        lifetime: Time<f64>,
        fate: StarFate,
    ) -> Self {
        Self {
            lifestage_evolution,
            lifetime,
            fate,
        }
    }

    pub(crate) fn apply_to_illuminance(
        &self,
        illuminance: Illuminance<f64>,
        time: Time<f64>,
    ) -> Illuminance<f64> {
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return illuminance + lifestage_evolution.illuminance_per_year * time.to_yr();
        }
        illuminance
    }

    pub(crate) fn apply_to_color(&self, color: sRGBColor, time: Time<f64>) -> sRGBColor {
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return color + &lifestage_evolution.color_per_year * time.to_yr();
        }
        color
    }

    pub fn get_lifestage_illuminance_per_year(&self) -> Illuminance<f64> {
        self.lifestage_evolution
            .as_ref()
            .map(|lifestage_evolution| lifestage_evolution.illuminance_per_year)
            .unwrap_or(IRRADIANCE_ZERO)
    }

    pub fn get_lifestage_color_per_year(&self) -> sRGBColor {
        self.lifestage_evolution
            .as_ref()
            .map(|lifestage_evolution| lifestage_evolution.color_per_year)
            .unwrap_or(sRGBColor::BLACK)
    }
}
