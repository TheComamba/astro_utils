use serde::{Deserialize, Serialize};
use simple_si_units::base::{Distance, Luminosity, Mass, Temperature};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarPhysicalParameters {
    pub(super) mass: Option<Mass<f64>>,
    pub(super) radius: Option<Length>,
    pub(super) luminous_intensity: Luminosity<f64>,
    pub(super) temperature: Temperature<f64>,
}

impl StarPhysicalParameters {
    pub fn new(
        mass: Option<Mass<f64>>,
        radius: Option<Length>,
        luminous_intensity: Luminosity<f64>,
        temperature: Temperature<f64>,
    ) -> Self {
        Self {
            mass,
            radius,
            luminous_intensity,
            temperature,
        }
    }

    pub fn mass(&self) -> Option<Mass<f64>> {
        self.mass
    }

    pub fn radius(&self) -> Option<Length> {
        self.radius
    }

    pub fn luminous_intensity(&self) -> Luminosity<f64> {
        self.luminous_intensity
    }

    pub fn temperature(&self) -> Temperature<f64> {
        self.temperature
    }
}
