use crate::units::{length::Length, luminosity::Luminosity, mass::Mass, temperature::Temperature};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct StellarProperties {
    radius: Length,
    mass: Mass,
    absolute_magnitude: Luminosity,
    temperature: Temperature,
}

impl StellarProperties {
    pub fn new(
        radius: Length,
        mass: Mass,
        absolute_magnitude: Luminosity,
        temperature: Temperature,
    ) -> StellarProperties {
        StellarProperties {
            radius,
            mass,
            absolute_magnitude,
            temperature,
        }
    }

    pub fn get_radius(&self) -> Length {
        self.radius
    }

    pub fn get_mass(&self) -> Mass {
        self.mass
    }

    pub fn get_absolute_magnitude(&self) -> Luminosity {
        self.absolute_magnitude
    }

    pub fn get_temperature(&self) -> Temperature {
        self.temperature
    }
}
