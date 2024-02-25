use super::data::ParsecData;
use crate::{
    coordinates::ecliptic::EclipticCoordinates,
    error::AstroUtilError,
    stars::{data::StarData, data_evolution::StarDataEvolution},
    units::{
        distance::{distance_to_sun_radii, DISTANCE_ZERO, SOLAR_RADIUS},
        luminous_intensity::SOLAR_LUMINOUS_INTENSITY,
    },
};
use serde::{Deserialize, Serialize};
use simple_si_units::base::{Distance, Mass, Temperature, Time};

pub(super) struct ParsecLine {
    mass: f64,
    age: f64,
    log_l: f64,
    log_te: f64,
    log_r: f64,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct ParsedParsecLine {
    pub(super) mass_in_solar_masses: f64,
    pub(super) age_in_years: f64,
    pub(super) luminous_intensity_in_solar: f64,
    pub(super) temperature_in_kelvin: f64,
    pub(super) radius_in_solar_radii: f64,
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
                }
                .parse();
                let data = parsec_data
                    .data
                    .get_mut(*mass_position)
                    .ok_or(AstroUtilError::DataNotAvailable)?;
                data.push(parsec_line);
            } else {
                return Err(AstroUtilError::DataNotAvailable);
            }
        };
        Ok(())
    }

    fn parse(self) -> ParsedParsecLine {
        let radius = Distance::from_cm(10f64.powf(self.log_r));
        ParsedParsecLine {
            mass_in_solar_masses: self.mass,
            age_in_years: self.age,
            luminous_intensity_in_solar: 10f64.powf(self.log_l),
            temperature_in_kelvin: 10f64.powf(self.log_te),
            radius_in_solar_radii: distance_to_sun_radii(&radius),
        }
    }
}

impl ParsedParsecLine {
    pub(super) fn to_star_at_origin(&self) -> StarData {
        let mass = Mass::from_solar_mass(self.mass_in_solar_masses);
        let age = Time::from_yr(self.age_in_years);
        let luminous_intensity = self.luminous_intensity_in_solar * SOLAR_LUMINOUS_INTENSITY;
        let temperature = Temperature::from_K(self.temperature_in_kelvin);
        let radius = self.radius_in_solar_radii * SOLAR_RADIUS;
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
}
