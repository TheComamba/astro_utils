use serde::{Deserialize, Serialize};
use simple_si_units::base::{Distance, Luminosity, Mass, Temperature, Time};

use crate::{
    color::srgb::sRGBColor,
    units::{
        distance::DISTANCE_ZERO,
        luminous_intensity::{luminous_intensity_to_illuminance, LUMINOSITY_ZERO},
        mass::MASS_ZERO,
        temperature::TEMPERATURE_ZERO,
        time::TIME_ZERO,
    },
};

use super::{
    appearance_evolution::{StarAppearanceEvolution, StarAppearanceLifestageEvolution},
    data::StarData,
    fate::StarFate,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarDataEvolution {
    lifestage_evolution: Option<StarDataLifestageEvolution>,
    pub(super) age: Option<Time<f64>>,
    pub(super) lifetime: Time<f64>,
    pub(super) fate: StarFate,
}

impl StarDataEvolution {
    pub const NONE: StarDataEvolution = StarDataEvolution {
        lifestage_evolution: None,
        age: None,
        lifetime: TIME_ZERO,
        fate: StarFate::WhiteDwarf,
    };

    pub(crate) fn new(
        lifestage_evolution: Option<StarDataLifestageEvolution>,
        age: Option<Time<f64>>,
        lifetime: Time<f64>,
        fate: StarFate,
    ) -> Self {
        Self {
            lifestage_evolution,
            age,
            lifetime,
            fate,
        }
    }

    pub(crate) fn apply_to_mass(&self, mass: Mass<f64>, time_since_epoch: Time<f64>) -> Mass<f64> {
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return mass + lifestage_evolution.mass_per_year * time_since_epoch.to_yr();
        }
        mass
    }

    pub(crate) fn apply_to_radius(
        &self,
        radius: Distance<f64>,
        time_since_epoch: Time<f64>,
    ) -> Distance<f64> {
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return radius + lifestage_evolution.radius_per_year * time_since_epoch.to_yr();
        }
        radius
    }

    pub(crate) fn apply_to_luminous_intensity(
        &self,
        luminous_intensity: Luminosity<f64>,
        time_since_epoch: Time<f64>,
    ) -> Luminosity<f64> {
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return luminous_intensity
                + lifestage_evolution.luminous_intensity_per_year * time_since_epoch.to_yr();
        }
        luminous_intensity
    }

    pub(crate) fn apply_to_temperature(
        &self,
        temperature: Temperature<f64>,
        time_since_epoch: Time<f64>,
    ) -> Temperature<f64> {
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return temperature
                + lifestage_evolution.temperature_per_year * time_since_epoch.to_yr();
        }
        temperature
    }

    pub(crate) fn to_star_appearance_evolution(
        &self,
        temperature_at_epoch: Temperature<f64>,
        distance: Distance<f64>,
    ) -> StarAppearanceEvolution {
        let lifestage_evolution = self
            .lifestage_evolution
            .as_ref()
            .map(|e| e.to_star_appearance_lifestage_evolution(temperature_at_epoch, distance));
        StarAppearanceEvolution::new(lifestage_evolution, self.lifetime, self.fate.clone())
    }

    pub fn get_lifestage_mass_per_year(&self) -> Mass<f64> {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.mass_per_year)
            .unwrap_or(MASS_ZERO)
    }

    pub fn get_lifestage_radius_per_year(&self) -> Distance<f64> {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.radius_per_year)
            .unwrap_or(DISTANCE_ZERO)
    }

    pub fn get_lifestage_luminous_intensity_per_year(&self) -> Luminosity<f64> {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.luminous_intensity_per_year)
            .unwrap_or(LUMINOSITY_ZERO)
    }

    pub fn get_lifestage_temperature_per_year(&self) -> Temperature<f64> {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.temperature_per_year)
            .unwrap_or(TEMPERATURE_ZERO)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct StarDataLifestageEvolution {
    mass_per_year: Mass<f64>,
    radius_per_year: Distance<f64>,
    luminous_intensity_per_year: Luminosity<f64>,
    temperature_per_year: Temperature<f64>,
}

impl StarDataLifestageEvolution {
    pub(crate) fn new(now: &StarData, then: &StarData, years: f64) -> Self {
        let mass_per_year = match (now.mass, then.mass) {
            (Some(now_mass), Some(then_mass)) => (now_mass - then_mass) / years,
            _ => MASS_ZERO,
        };
        let radius_per_year = match (now.radius, then.radius) {
            (Some(now_radius), Some(then_radius)) => (now_radius - then_radius) / years,
            _ => DISTANCE_ZERO,
        };
        let luminous_intensity_per_year = match (now.luminous_intensity, then.luminous_intensity) {
            (Some(now_luminous_intensity), Some(then_luminous_intensity)) => {
                (now_luminous_intensity - then_luminous_intensity) / years
            }
            _ => LUMINOSITY_ZERO,
        };
        let temperature_per_year = (now.temperature - then.temperature) / years;
        Self {
            mass_per_year,
            radius_per_year,
            luminous_intensity_per_year,
            temperature_per_year,
        }
    }

    fn to_star_appearance_lifestage_evolution(
        &self,
        temperature_at_epoch: Temperature<f64>,
        distance: Distance<f64>,
    ) -> StarAppearanceLifestageEvolution {
        let illuminance_per_year =
            luminous_intensity_to_illuminance(&self.luminous_intensity_per_year, &distance);
        let color_now = sRGBColor::from_temperature(temperature_at_epoch);
        let color_then =
            sRGBColor::from_temperature(temperature_at_epoch - self.temperature_per_year);
        let color_per_year = color_now - color_then;

        StarAppearanceLifestageEvolution {
            illuminance_per_year,
            color_per_year,
        }
    }
}
