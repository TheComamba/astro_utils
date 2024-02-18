use serde::{Deserialize, Serialize};
use simple_si_units::base::{Distance, Luminosity, Mass, Temperature};

use super::star_data::StarData;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarDataEvolution {
    lifestage_evolution: Option<StarDataLifestageEvolution>,
}

impl StarDataEvolution {
    pub const NONE: StarDataEvolution = StarDataEvolution {
        lifestage_evolution: None,
    };

    pub(crate) fn new(lifestage_evolution: Option<StarDataLifestageEvolution>) -> Self {
        Self {
            lifestage_evolution,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct StarDataLifestageEvolution {
    mass_per_year: Option<Mass<f64>>,
    radius_per_year: Option<Distance<f64>>,
    luminous_intensity_per_year: Option<Luminosity<f64>>,
    temperature_per_year: Option<Temperature<f64>>,
}

impl StarDataLifestageEvolution {
    pub(crate) fn new(now: &StarData, then: &StarData, years: f64) -> Self {
        let mass_per_year = match (now.mass, then.mass) {
            (Some(now_mass), Some(then_mass)) => Some((now_mass - then_mass) / years),
            _ => None,
        };
        let radius_per_year = match (now.radius, then.radius) {
            (Some(now_radius), Some(then_radius)) => Some((now_radius - then_radius) / years),
            _ => None,
        };
        let luminous_intensity_per_year = match (now.luminous_intensity, then.luminous_intensity) {
            (Some(now_luminous_intensity), Some(then_luminous_intensity)) => {
                Some((now_luminous_intensity - then_luminous_intensity) / years)
            }
            _ => None,
        };
        let temperature_per_year = match (now.temperature, then.temperature) {
            (Some(now_temperature), Some(then_temperature)) => {
                Some((now_temperature - then_temperature) / years)
            }
            _ => None,
        };
        Self {
            mass_per_year,
            radius_per_year,
            luminous_intensity_per_year,
            temperature_per_year,
        }
    }
}
