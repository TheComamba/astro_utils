use super::parsec_data::ParsecData;
use crate::{
    coordinates::ecliptic::EclipticCoordinates,
    error::AstroUtilError,
    stars::{star_data::StarData, star_data_evolution::StarDataEvolution},
    units::{
        distance::DISTANCE_ZERO,
        luminous_intensity::{luminous_intensity_to_illuminance, SOLAR_LUMINOUS_INTENSITY},
        mass::SOLAR_MASS,
    },
};
use serde::{Deserialize, Serialize};
use simple_si_units::{
    base::{Distance, Luminosity, Mass, Temperature, Time},
    electromagnetic::Illuminance,
};

#[derive(Deserialize, Serialize)]
pub(super) struct ParsecLine {
    mass: f64,
    pub(super) age: f64,
    log_l: f64,
    log_te: f64,
    log_r: f64,
}

impl ParsecLine {
    const MASS_INDEX: usize = 1;
    const AGE_INDEX: usize = 2;
    const LOG_L_INDEX: usize = 3;
    const LOG_TE_INDEX: usize = 4;
    const LOG_R_INDEX: usize = 5;

    pub(super) fn read(
        line: Result<String, std::io::Error>,
        mass_position: &mut Option<usize>,
        parsec_data: &mut ParsecData,
    ) -> Result<(), AstroUtilError> {
        let line = line.map_err(AstroUtilError::Io)?;
        let entries: Vec<&str> = line.split_whitespace().collect();
        let mass_entry = entries
            .get(Self::MASS_INDEX)
            .ok_or(AstroUtilError::DataNotAvailable)?;
        if mass_position.is_none() {
            if let Ok(mass_value) = mass_entry.parse::<f64>() {
                *mass_position = Some(ParsecData::get_closest_mass_index(mass_value));
            }
        }
        if let Some(mass_position) = &*mass_position {
            let age_entry = entries
                .get(Self::AGE_INDEX)
                .ok_or(AstroUtilError::DataNotAvailable)?;
            let log_l_entry = entries
                .get(Self::LOG_L_INDEX)
                .ok_or(AstroUtilError::DataNotAvailable)?;
            let log_te_entry = entries
                .get(Self::LOG_TE_INDEX)
                .ok_or(AstroUtilError::DataNotAvailable)?;
            let log_r_entry = entries
                .get(Self::LOG_R_INDEX)
                .ok_or(AstroUtilError::DataNotAvailable)?;
            if let (Ok(mass), Ok(age), Ok(log_l), Ok(log_te), Ok(log_r)) = (
                mass_entry.parse::<f64>(),
                age_entry.parse::<f64>(),
                log_l_entry.parse::<f64>(),
                log_te_entry.parse::<f64>(),
                log_r_entry.parse::<f64>(),
            ) {
                let parsec_line = ParsecLine {
                    mass,
                    age,
                    log_l,
                    log_te,
                    log_r,
                };
                let data = parsec_data
                    .data
                    .get_mut(*mass_position)
                    .ok_or(AstroUtilError::DataNotAvailable)?;
                data.push(parsec_line);
            }
        };
        Ok(())
    }

    pub(super) fn to_star_at_origin(&self) -> StarData {
        let mass = self.get_mass();
        let age = self.get_age();
        let luminous_intensity = self.get_luminous_intensity();
        let temperature = self.get_temperature();
        let radius = self.get_radius();
        StarData {
            name: "".to_string(),
            mass: Some(mass),
            age: Some(age),
            luminous_intensity: Some(luminous_intensity),
            temperature: temperature,
            radius: Some(radius),
            distance: DISTANCE_ZERO,
            pos: EclipticCoordinates::Z_DIRECTION,
            constellation: None,
            evolution: StarDataEvolution::NONE,
        }
    }

    pub(super) fn get_mass(&self) -> Mass<f64> {
        self.mass * SOLAR_MASS
    }

    pub(super) fn get_age(&self) -> Time<f64> {
        Time::from_yr(self.age)
    }

    pub(super) fn get_luminous_intensity(&self) -> Luminosity<f64> {
        let lum = 10f64.powf(self.log_l);
        lum * SOLAR_LUMINOUS_INTENSITY
    }

    pub(super) fn get_illuminance(&self, distance: &Distance<f64>) -> Illuminance<f64> {
        let lum = self.get_luminous_intensity();
        luminous_intensity_to_illuminance(&lum, distance)
    }

    pub(super) fn get_temperature(&self) -> Temperature<f64> {
        let temp = 10f64.powf(self.log_te);
        Temperature::from_K(temp)
    }

    pub(super) fn get_radius(&self) -> Distance<f64> {
        let radius = 10f64.powf(self.log_r);
        Distance::from_cm(radius)
    }
}
