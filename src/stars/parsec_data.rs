use super::star_data::StarData;
use crate::coordinates::direction::Direction;
use crate::error::AstroUtilError;
use crate::units::illuminance::illuminance_to_apparent_magnitude;
use crate::units::luminous_intensity::{
    luminous_intensity_to_illuminance, SOLAR_LUMINOUS_INTENSITY,
};
use crate::units::mass::SOLAR_MASS;
use directories::ProjectDirs;
use flate2::read::GzDecoder;
use rmp_serde;
use serde::{Deserialize, Serialize};
use simple_si_units::base::{Distance, Luminosity, Mass, Temperature, Time};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use tar::Archive;

#[derive(Deserialize, Serialize)]
pub(super) struct ParsecLine {
    mass: f64,
    age: f64,
    log_l: f64,
    log_te: f64,
    log_r: f64,
}

#[derive(Deserialize, Serialize)]
pub(super) struct ParsecData {
    data: Vec<Vec<ParsecLine>>,
}

impl ParsecData {
    const METALLICITY: &'static str = "Z0.01";
    const FILENAME: &'static str = "Z0.01.rmp";
    pub(super) const SORTED_MASSES: [f64; 100] = [
        0.09, 0.10, 0.12, 0.14, 0.16, 0.20, 0.25, 0.30, 0.35, 0.40, 0.45, 0.50, 0.55, 0.60, 0.65,
        0.70, 0.75, 0.80, 0.85, 0.90, 0.95, 1.00, 1.05, 1.10, 1.15, 1.20, 1.25, 1.30, 1.35, 1.40,
        1.45, 1.50, 1.55, 1.60, 1.65, 1.70, 1.75, 1.80, 1.85, 1.90, 1.95, 2.00, 2.05, 2.10, 2.15,
        2.20, 2.25, 2.30, 2.40, 2.60, 2.80, 3.00, 3.20, 3.40, 3.60, 3.80, 4.00, 4.20, 4.40, 4.60,
        4.80, 5.00, 5.20, 5.40, 5.60, 5.80, 6.00, 6.20, 6.40, 7.00, 8.00, 9.00, 10.0, 12.0, 14.0,
        16.0, 18.0, 20.0, 24.0, 28.0, 30.0, 35.0, 40.0, 45.0, 50.0, 55.0, 60.0, 65.0, 70.0, 75.0,
        80.0, 90.0, 95.0, 100.0, 120.0, 130.0, 200.0, 250.0, 300.0, 350.0,
    ];
    const MASS_INDEX: usize = 1;
    const AGE_INDEX: usize = 2;
    const LOG_L_INDEX: usize = 3;
    const LOG_TE_INDEX: usize = 4;
    const LOG_R_INDEX: usize = 5;

    pub(super) fn new() -> Result<ParsecData, AstroUtilError> {
        let project_dirs = get_project_dirs()?;
        let data_dir = project_dirs.data_dir();
        let file_path = data_dir.join(Self::FILENAME);

        if file_path.exists() {
            println!("Reading PARSEC data from {}", file_path.display());
            let file = File::open(&file_path).map_err(AstroUtilError::Io)?;
            let parsec_data: ParsecData =
                rmp_serde::from_read(file).map_err(AstroUtilError::RmpDeserialization)?;
            Ok(parsec_data)
        } else {
            Self::ensure_data_files()?;
            let mut parsec_data = ParsecData {
                data: Vec::with_capacity(Self::SORTED_MASSES.len()),
            };
            for _ in Self::SORTED_MASSES.iter() {
                parsec_data.data.push(Vec::new());
            }
            let folder_path = data_dir.join(PathBuf::from(Self::METALLICITY));
            let filepaths = fs::read_dir(folder_path).map_err(AstroUtilError::Io)?;
            for entry in filepaths {
                Self::read_file(entry, &mut parsec_data)?;
            }
            println!("Writing PARSEC data to {}", file_path.display());
            let file = File::create(&file_path).map_err(AstroUtilError::Io)?;
            let buffer =
                rmp_serde::to_vec(&parsec_data).map_err(AstroUtilError::RmpSerialization)?;
            let mut writer = BufWriter::new(file);
            writer.write_all(&buffer).map_err(AstroUtilError::Io)?;
            Ok(parsec_data)
        }
    }

    fn get_closest_mass_index(mass: f64) -> usize {
        let mut min_index = 0;
        let mut max_index = Self::SORTED_MASSES.len() - 1;
        while max_index - min_index > 1 {
            let mid_index = (max_index + min_index) / 2;
            let mid_mass = Self::SORTED_MASSES[mid_index];
            if mass > mid_mass {
                min_index = mid_index;
            } else {
                max_index = mid_index;
            }
        }
        if (mass - Self::SORTED_MASSES[min_index]).abs()
            < (mass - Self::SORTED_MASSES[max_index]).abs()
        {
            min_index
        } else {
            max_index
        }
    }

    fn download() -> Result<(), AstroUtilError> {
        let project_dirs = get_project_dirs()?;
        let data_dir = project_dirs.data_dir();
        let data_dir = data_dir
            .to_str()
            .ok_or(AstroUtilError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not convert data dir to string",
            )))?;
        println!("Downloading PARSEC data to {}", data_dir);
        let target = "https://people.sissa.it/~sbressan/CAF09_V1.2S_M36_LT/no_phase/".to_string()
            + Self::METALLICITY
            + ".tar.gz";
        let mut response = reqwest::blocking::get(target).map_err(AstroUtilError::Connection)?;
        let gz_decoder = GzDecoder::new(&mut response);
        let mut archive = Archive::new(gz_decoder);
        archive.unpack(data_dir).map_err(AstroUtilError::Io)?;
        Ok(())
    }

    pub(super) fn ensure_data_files() -> Result<(), AstroUtilError> {
        let project_dirs = get_project_dirs()?;
        let data_dir = project_dirs.data_dir();
        let path = data_dir.join(PathBuf::from(Self::METALLICITY));
        if !path.exists() {
            Self::download()?;
        }
        Ok(())
    }

    pub(super) fn get_trajectory_via_index(&self, i: usize) -> &Vec<ParsecLine> {
        &self.data[i]
    }

    fn read_file(
        entry: Result<fs::DirEntry, std::io::Error>,
        parsec_data: &mut ParsecData,
    ) -> Result<(), AstroUtilError> {
        let file_path = entry.map_err(AstroUtilError::Io)?.path();
        let file = File::open(&file_path).map_err(AstroUtilError::Io)?;
        let reader = BufReader::new(file);
        let mut mass_position = None;
        Ok(for line in reader.lines() {
            Self::read_line(line, &mut mass_position, parsec_data)?;
        })
    }

    fn read_line(
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
                *mass_position = Some(Self::get_closest_mass_index(mass_value));
            }
        }
        Ok(if let Some(mass_position) = &*mass_position {
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
        })
    }

    pub(super) fn get_life_expectancy_in_years(trajectory: &Vec<ParsecLine>) -> u32 {
        trajectory.last().unwrap().age as u32
    }

    #[cfg(test)]
    pub(super) fn get_params_for_current_mass_and_age(
        &self,
        mass: Mass<f64>,
        age_in_years: f64,
    ) -> &ParsecLine {
        use crate::units::mass::mass_to_solar_masses;

        let mut mass_index = Self::get_closest_mass_index(mass_to_solar_masses(mass));
        let mut trajectory = &self.data[mass_index];
        let mut params = Self::get_closest_params(trajectory, age_in_years);
        while params.get_mass() < mass && mass_index < Self::SORTED_MASSES.len() - 1 {
            mass_index += 1;
            trajectory = &self.data[mass_index];
            params = Self::get_closest_params(trajectory, age_in_years);
        }
        params
    }

    pub(super) fn get_closest_params(
        trajectory: &Vec<ParsecLine>,
        actual_age_in_years: f64,
    ) -> &ParsecLine {
        let mut closest_age = f64::MAX;
        let mut age_index = 0;
        for (i, line) in trajectory.iter().enumerate() {
            let age_difference = (line.age - actual_age_in_years).abs();
            if age_difference < closest_age {
                closest_age = age_difference;
                age_index = i;
            }
        }
        &trajectory[age_index]
    }
}

impl ParsecLine {
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
            temperature: Some(temperature),
            radius: Some(radius),
            distance: None,
            direction_in_ecliptic: Direction::Z,
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

    pub(super) fn get_apparent_magnitude(&self, distance: &Distance<f64>) -> f64 {
        let lum = self.get_luminous_intensity();
        let ill = luminous_intensity_to_illuminance(&lum, &distance);
        illuminance_to_apparent_magnitude(&ill)
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

fn get_project_dirs() -> Result<ProjectDirs, AstroUtilError> {
    ProjectDirs::from("", "the_comamba", "astro_utils").ok_or(AstroUtilError::Io(
        std::io::Error::new(std::io::ErrorKind::Other, "Could not get project dirs"),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        real_data::stars::{BRIGHTEST_STARS, SUN_DATA},
        tests::eq_within,
        units::{distance::SOLAR_RADIUS, mass::mass_to_solar_masses, time::BILLION_YEARS},
    };

    #[test]
    fn test_caluclate_sun() {
        let parsec_data = ParsecData::new().unwrap();
        let mass = SUN_DATA.mass;
        let age = SUN_DATA.age.unwrap();
        let current_params =
            parsec_data.get_params_for_current_mass_and_age(mass.unwrap(), age.to_yr());
        let calculated_sun = current_params.to_star_at_origin();
        let real_sun = SUN_DATA.to_star_data();
        println!(
            "calculated mass: {}, real mass: {}",
            calculated_sun.get_mass().unwrap(),
            real_sun.get_mass().unwrap()
        );
        println!(
            "calculated radius: {}, real radius: {}",
            calculated_sun.get_radius().unwrap(),
            real_sun.get_radius().unwrap()
        );
        println!(
            "calculated luminous_intensity: {}, real luminous_intensity: {}",
            calculated_sun.get_luminous_intensity().unwrap(),
            real_sun.get_luminous_intensity().unwrap()
        );
        println!(
            "calculated temperature: {}, real temperature: {}",
            calculated_sun.get_temperature().unwrap(),
            real_sun.get_temperature().unwrap()
        );
        assert!(eq_within(
            calculated_sun.get_mass().unwrap().kg,
            real_sun.get_mass().unwrap().kg,
            1e-2 * SOLAR_MASS.kg
        ));
        assert!(eq_within(
            calculated_sun.get_radius().unwrap().m,
            real_sun.get_radius().unwrap().m,
            1e-1 * SOLAR_RADIUS.m
        ));
        assert!(eq_within(
            calculated_sun.get_luminous_intensity().unwrap().cd,
            real_sun.get_luminous_intensity().unwrap().cd,
            0.5 * SOLAR_LUMINOUS_INTENSITY.cd
        ));
        assert!(eq_within(
            calculated_sun.get_temperature().unwrap().K,
            real_sun.get_temperature().unwrap().K,
            500.
        ));
    }

    #[test]
    fn test_calculate_star() {
        let parsec_data = ParsecData::new().unwrap();
        let mut num_success = 0;
        let mut num_fail = 0;
        for data in BRIGHTEST_STARS.iter() {
            if let (Some(age), Some(mass)) = (data.age, data.mass) {
                let age = age.to_yr();
                let mass_index = ParsecData::get_closest_mass_index(mass_to_solar_masses(mass));
                let trajectory = parsec_data.get_trajectory_via_index(mass_index);
                let age_expectancy = ParsecData::get_life_expectancy_in_years(trajectory);
                let age_expectancy = Time::from_yr(age_expectancy as f64);
                if age_expectancy < 0.3 * BILLION_YEARS {
                    // Numerics get really unstable for stars with short life expectancies.
                    continue;
                }

                let current_params = parsec_data.get_params_for_current_mass_and_age(mass, age);
                let calculated_star = current_params.to_star_at_origin();
                let real_star = data.to_star_data();
                if calculated_star.similar_within_order_of_magnitude(&real_star) {
                    num_success += 1;
                } else {
                    println!("Comparing data for {} failed.\n\n", data.common_name);
                    num_fail += 1;
                }
            }
        }
        println!("\nnum_success: {}", num_success);
        println!("num_fail: {}", num_fail);
        assert!(num_success > num_fail)
    }

    #[test]
    fn masses_are_mapped_to_themselves() {
        const SMALL_OFFSET: f64 = 1e-4;
        for expected_mass in ParsecData::SORTED_MASSES.iter() {
            let mass = *expected_mass;
            let mass_index = ParsecData::get_closest_mass_index(mass);
            let mapped_mass = ParsecData::SORTED_MASSES[mass_index];
            println!("mass: {}, mapped_mass: {}", mass, mapped_mass);
            assert!((expected_mass - mapped_mass).abs() < SMALL_OFFSET);

            let mass = *expected_mass + SMALL_OFFSET;
            let mass_index = ParsecData::get_closest_mass_index(mass);
            let mapped_mass = ParsecData::SORTED_MASSES[mass_index];
            println!("mass: {}, mapped_mass: {}", mass, mapped_mass);
            assert!((expected_mass - mapped_mass).abs() < SMALL_OFFSET);

            let mass = *expected_mass - SMALL_OFFSET;
            let mass_index = ParsecData::get_closest_mass_index(mass);
            let mapped_mass = ParsecData::SORTED_MASSES[mass_index];
            println!("mass: {}, mapped_mass: {}", mass, mapped_mass);
            assert!((expected_mass - mapped_mass).abs() < SMALL_OFFSET);
        }
    }
}
