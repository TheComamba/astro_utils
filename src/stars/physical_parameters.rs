use serde::{Deserialize, Serialize};
use uom::si::f64::{Length, Mass, ThermodynamicTemperature};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarPhysicalParameters {
    pub(super) mass: Option<Mass>,
    pub(super) radius: Option<Length>,
    pub(super) luminous_intensity: Luminosity<f64>,
    pub(super) temperature: ThermodynamicTemperature,
}

impl StarPhysicalParameters {
    pub fn new(
        mass: Option<Mass>,
        radius: Option<Length>,
        luminous_intensity: Luminosity<f64>,
        temperature: ThermodynamicTemperature,
    ) -> Self {
        Self {
            mass,
            radius,
            luminous_intensity,
            temperature,
        }
    }

    pub fn mass(&self) -> Option<Mass> {
        self.mass
    }

    pub fn radius(&self) -> Option<Length> {
        self.radius
    }

    pub fn luminous_intensity(&self) -> Luminosity<f64> {
        self.luminous_intensity
    }

    pub fn temperature(&self) -> ThermodynamicTemperature {
        self.temperature
    }
}
