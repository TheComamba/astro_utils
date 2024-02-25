use serde::{Deserialize, Serialize};
use simple_si_units::{
    base::{Distance, Luminosity, Mass, Temperature, Time},
    electromagnetic::Illuminance,
};

use crate::{
    color::srgb::sRGBColor,
    units::{
        distance::DISTANCE_ZERO, illuminance::IRRADIANCE_ZERO, luminous_intensity::LUMINOSITY_ZERO,
    },
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum StarFate {
    WhiteDwarf,
    TypeIISupernova,
}

impl StarFate {
    pub(crate) fn new(initial_mass: Mass<f64>) -> Self {
        if initial_mass < Mass::from_solar_mass(8.) {
            StarFate::WhiteDwarf
        } else {
            StarFate::TypeIISupernova
        }
    }

    pub(crate) fn apply_to_mass(&self, mass: Mass<f64>, time_since_death: Time<f64>) -> Mass<f64> {
        mass
    }

    pub(crate) fn apply_to_radius(
        &self,
        radius: Distance<f64>,
        time_since_death: Time<f64>,
    ) -> Distance<f64> {
        DISTANCE_ZERO
    }

    pub(crate) fn apply_to_luminous_intensity(
        &self,
        luminous_intensity: Luminosity<f64>,
        time_since_death: Time<f64>,
    ) -> Luminosity<f64> {
        LUMINOSITY_ZERO
    }

    pub(crate) fn apply_to_temperature(
        &self,
        temperature: Temperature<f64>,
        time_since_death: Time<f64>,
    ) -> Temperature<f64> {
        temperature
    }

    pub(crate) fn apply_to_illuminance(
        &self,
        illuminance: Illuminance<f64>,
        time_since_death: Time<f64>,
    ) -> Illuminance<f64> {
        IRRADIANCE_ZERO
    }

    pub(crate) fn apply_to_color(
        &self,
        color: sRGBColor,
        time_since_death: Time<f64>,
    ) -> sRGBColor {
        color
    }
}
