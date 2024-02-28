use std::ops::Range;

use serde::{Deserialize, Serialize};
use simple_si_units::base::{Distance, Luminosity, Mass, Temperature, Time};

use crate::{
    astro_display::AstroDisplay,
    units::{
        distance::SOLAR_RADIUS,
        luminous_intensity::{
            absolute_magnitude_to_luminous_intensity, luminous_intensity_to_absolute_magnitude,
            LUMINOSITY_ZERO,
        },
        temperature::TEMPERATURE_ZERO,
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
        luminous_intensity: Luminosity<f64>,
        time_since_death: Time<f64>,
    ) -> Luminosity<f64> {
        match self {
            StarFate::WhiteDwarf => absolute_magnitude_to_luminous_intensity(11.18), // Sirius B
            StarFate::TypeIISupernova => {
                type_2_supernova_luminous_intensity(luminous_intensity, time_since_death)
            }
        }
    }

    pub(crate) fn apply_to_temperature(
        &self,
        temperature: Temperature<f64>,
        time_since_death: Time<f64>,
    ) -> Temperature<f64> {
        match self {
            StarFate::WhiteDwarf => Temperature::from_celsius(25_000.), // Sirius B
            StarFate::TypeIISupernova => {
                type_2_supernova_temperature(temperature, time_since_death)
            }
        }
    }
}

const SN_PHASE_1_INCREASE: Range<f64> = 0.0..10.0;
const SN_PHASE_2_DECREASE: Range<f64> = 10.0..20.0;
const SN_PHASE_3_PLATEAU: Range<f64> = 20.0..110.0;

fn type_2_supernova_luminous_intensity(
    initial: Luminosity<f64>,
    time_since_death: Time<f64>,
) -> Luminosity<f64> {
    const PEAK_MAGNITUDE: f64 = -16.8;
    const PLATEAU_MAGNITUDE: f64 = -16.3;

    let days = time_since_death.to_days();
    if days < 0. {
        LUMINOSITY_ZERO
    } else if SN_PHASE_1_INCREASE.contains(&days) {
        let offset = luminous_intensity_to_absolute_magnitude(initial);
        let slope =
            (PEAK_MAGNITUDE - offset) / (SN_PHASE_1_INCREASE.end - SN_PHASE_1_INCREASE.start);
        let mag = offset + slope * days;
        absolute_magnitude_to_luminous_intensity(mag)
    } else if SN_PHASE_2_DECREASE.contains(&days) {
        let offset = PEAK_MAGNITUDE;
        let slope =
            (PLATEAU_MAGNITUDE - offset) / (SN_PHASE_2_DECREASE.end - SN_PHASE_2_DECREASE.start);
        let mag = offset + slope * days;
        absolute_magnitude_to_luminous_intensity(mag)
    } else if SN_PHASE_3_PLATEAU.contains(&days) {
        absolute_magnitude_to_luminous_intensity(PLATEAU_MAGNITUDE)
    } else {
        let offset = PLATEAU_MAGNITUDE;
        let slope = 1. / 60.;
        let mag = offset + slope * days;
        absolute_magnitude_to_luminous_intensity(mag)
    }
}

fn type_2_supernova_temperature(
    initial: Temperature<f64>,
    time_since_death: Time<f64>,
) -> Temperature<f64> {
    const PEAK_TEMPERATURE: Temperature<f64> = Temperature { K: 100_000. };
    const PLATEAU_TEMPERATURE: Temperature<f64> = Temperature { K: 4_500. };

    let days = time_since_death.to_days();
    if days < 0. {
        initial
    } else if SN_PHASE_1_INCREASE.contains(&days) {
        let offset = initial;
        let slope =
            (PEAK_TEMPERATURE - offset) / (SN_PHASE_1_INCREASE.end - SN_PHASE_1_INCREASE.start);
        offset + slope * days
    } else if SN_PHASE_2_DECREASE.contains(&days) {
        let offset = PEAK_TEMPERATURE;
        let slope =
            (PLATEAU_TEMPERATURE - offset) / (SN_PHASE_2_DECREASE.end - SN_PHASE_2_DECREASE.start);
        offset + slope * days
    } else if SN_PHASE_3_PLATEAU.contains(&days) {
        PLATEAU_TEMPERATURE
    } else {
        let offset = PLATEAU_TEMPERATURE;
        let slope = -PLATEAU_TEMPERATURE / Time::from_kyr(10.).to_days();
        let t = offset + slope * days;
        if t > TEMPERATURE_ZERO {
            t
        } else {
            TEMPERATURE_ZERO
        }
    }
}

impl AstroDisplay for StarFate {
    fn astro_display(&self) -> String {
        match self {
            StarFate::WhiteDwarf => "White Dwarf".to_string(),
            StarFate::TypeIISupernova => "Type II Supernova".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
