use super::data::ParsecData;
use crate::{
    error::AstroUtilError,
    stars::random::random_stars::DIMMEST_ILLUMINANCE,
    units::{length::distance_to_sun_radii, luminous_intensity::solar_luminous_intensity},
};
use astro_coords::cartesian::Cartesian;
use serde::{Deserialize, Serialize};
use uom::si::{
    f64::{Length, LuminousIntensity},
    length::centimeter,
};

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
        lines: &mut Vec<ParsedParsecLine>,
    ) -> Result<(), AstroUtilError> {
        let line = line.map_err(AstroUtilError::Io)?;
        let entries: Vec<&str> = line.split_whitespace().collect();
        let mass_entry = entries
            .get(Self::MASS_INDEX)
            .ok_or(AstroUtilError::DataNotAvailable("mass".to_string()))?;
        if mass_position.is_none() {
            if let Ok(mass_value) = mass_entry.parse::<f64>() {
                *mass_position = Some(ParsecData::get_closest_mass_index(mass_value));
            } else {
                return Ok(());
            }
        }

        let age_entry = entries
            .get(Self::AGE_INDEX)
            .ok_or(AstroUtilError::DataNotAvailable("age".to_string()))?;
        let log_l_entry = entries
            .get(Self::LOG_L_INDEX)
            .ok_or(AstroUtilError::DataNotAvailable("log_l".to_string()))?;
        let log_te_entry = entries
            .get(Self::LOG_TE_INDEX)
            .ok_or(AstroUtilError::DataNotAvailable("log_te".to_string()))?;
        let log_r_entry = entries
            .get(Self::LOG_R_INDEX)
            .ok_or(AstroUtilError::DataNotAvailable("log_r".to_string()))?;
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

            lines.push(parsec_line);
        } else {
            return Err(AstroUtilError::DataNotAvailable(
                "[Parsing failed]".to_string(),
            ));
        }

        Ok(())
    }

    fn parse(self) -> ParsedParsecLine {
        let radius = Length::new::<centimeter>(10f64.powf(self.log_r));
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
    pub(super) fn is_visible(&self, pos: &Cartesian) -> bool {
        let min_luminous_intensity = LuminousIntensity {
            cd: DIMMEST_ILLUMINANCE.get::<lux>() * pos.length_squared().m2,
        };
        self.luminous_intensity_in_solar * solar_luminous_intensity >= min_luminous_intensity
    }
}
