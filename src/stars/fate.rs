use serde::{Deserialize, Serialize};
use simple_si_units::base::{Distance, Luminosity, Mass, Temperature, Time};

use crate::{
    astro_display::AstroDisplay,
    units::{distance::SOLAR_RADIUS, luminous_intensity::absolute_magnitude_to_luminous_intensity},
};

use super::data::StarData;

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

    pub(crate) fn apply_to_mass(&self, mass: Mass<f64>) -> Mass<f64> {
        match self {
            StarFate::WhiteDwarf => 0.3 * mass, // rough estimate after shedding outer layers
            StarFate::TypeIISupernova => {
                if mass < Mass::from_solar_mass(25.) {
                    Mass::from_solar_mass(1.4) // Typical Neutron Star
                } else {
                    Mass::from_solar_mass(7.0) // Typical Black Hole
                }
            }
        }
    }

    pub(crate) fn apply_to_radius(&self) -> Distance<f64> {
        match self {
            StarFate::WhiteDwarf => 0.0084 * SOLAR_RADIUS, // Sirius B
            StarFate::TypeIISupernova => Distance::from_km(20.), // Neutron star or black hole
        }
    }

    pub(crate) fn apply_to_luminous_intensity(
        &self,
        data: &StarData,
        time_since_death: Time<f64>,
    ) -> Luminosity<f64> {
        match self {
            StarFate::WhiteDwarf => absolute_magnitude_to_luminous_intensity(11.18), // Sirius B
            StarFate::TypeIISupernova => {
                type_2_supernova_luminous_intensity(time_since_death, data)
            }
        }
    }

    pub(crate) fn apply_to_temperature(
        &self,
        data: &StarData,
        time_since_death: Time<f64>,
    ) -> Temperature<f64> {
        match self {
            StarFate::WhiteDwarf => Temperature::from_celsius(25_000.), // Sirius B
            StarFate::TypeIISupernova => type_2_supernova_temperature(time_since_death, data),
        }
    }
}

fn type_2_supernova_luminous_intensity(
    time_since_death: Time<f64>,
    data: &StarData,
) -> Luminosity<f64> {
    todo!()
}

fn type_2_supernova_temperature(time_since_death: Time<f64>, data: &StarData) -> Temperature<f64> {
    todo!()
}

impl AstroDisplay for StarFate {
    fn astro_display(&self) -> String {
        match self {
            StarFate::WhiteDwarf => "White Dwarf".to_string(),
            StarFate::TypeIISupernova => "Type II Supernova".to_string(),
        }
    }
}
