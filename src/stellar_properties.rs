use std::borrow::Cow;

use crate::units::{length::Length, luminosity::Luminosity, mass::Mass, temperature::Temperature};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct StellarProperties {
    name: Cow<'static, str>,
    radius: Length,
    mass: Mass,
    absolute_magnitude: Luminosity,
    temperature: Temperature,
}

impl StellarProperties {
    pub const fn new_const(
        name: &'static str,
        radius: Length,
        mass: Mass,
        absolute_magnitude: Luminosity,
        temperature: Temperature,
    ) -> StellarProperties {
        StellarProperties {
            name: Cow::Borrowed(name),
            radius,
            mass,
            absolute_magnitude,
            temperature,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub const fn get_radius(&self) -> Length {
        self.radius
    }

    pub const fn get_mass(&self) -> Mass {
        self.mass
    }

    pub const fn get_absolute_magnitude(&self) -> Luminosity {
        self.absolute_magnitude
    }

    pub const fn get_temperature(&self) -> Temperature {
        self.temperature
    }
}
