use super::star_data::StarData;
use crate::coordinates::direction::Direction;
use crate::{error::AstroUtilError, Float};
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
    mass: Float,
    age: Float,
    log_l: Float,
    log_te: Float,
    log_r: Float,
}

#[derive(Deserialize, Serialize)]
pub(super) struct ParsecData {
    data: Vec<Vec<ParsecLine>>,
}

impl ParsecData {
    const METALLICITY: &'static str = "Z0.01";
    const FILENAME: &'static str = "Z0.01.rmp";
    pub(super) const SORTED_MASSES: [Float; 100] = [
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

    fn get_closest_mass_index(mass: Float) -> usize {
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
            if let Ok(mass_value) = mass_entry.parse::<Float>() {
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
                mass_entry.parse::<Float>(),
                age_entry.parse::<Float>(),
                log_l_entry.parse::<Float>(),
                log_te_entry.parse::<Float>(),
                log_r_entry.parse::<Float>(),
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
        mass: Mass<Float>,
        age_in_years: Float,
    ) -> &ParsecLine {
        let mut mass_index = Self::get_closest_mass_index(mass.as_solar_masses());
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
        actual_age_in_years: Float,
    ) -> &ParsecLine {
        let mut closest_age = Float::MAX;
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
        let luminosity = self.get_luminosity();
        let temperature = self.get_temperature();
        let radius = self.get_radius();
        StarData {
            name: "".to_string(),
            mass: Some(mass),
            age: Some(age),
            luminosity: Some(luminosity),
            temperature: Some(temperature),
            radius: Some(radius),
            distance: None,
            direction_in_ecliptic: Direction::Z,
        }
    }

    pub(super) fn get_mass(&self) -> Mass<Float> {
        Mass::from_solar_masses(self.mass)
    }

    pub(super) fn get_age(&self) -> Time<Float> {
        Time::from_years(self.age)
    }

    pub(super) fn get_luminosity(&self) -> Luminosity<Float> {
        let lum = 10f32.powf(self.log_l);
        Luminosity::from_solar_luminosities(lum)
    }

    pub(super) fn get_apparent_magnitude(&self, distance: &Distance<Float>) -> Float {
        let lum = self.get_luminosity();
        let ill = lum.to_illuminance(&distance);
        ill.as_apparent_magnitude()
    }

    pub(super) fn get_temperature(&self) -> Temperature<Float> {
        let temp = 10f32.powf(self.log_te);
        Temperature::from_kelvin(temp)
    }

    pub(super) fn get_radius(&self) -> Distance<Float> {
        let radius = 10f32.powf(self.log_r);
        Distance::from_centimeters(radius)
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
    };

    #[test]
    fn test_caluclate_sun() {
        let parsec_data = ParsecData::new().unwrap();
        let mass = SUN_DATA.mass;
        let age = SUN_DATA.age.unwrap();
        let current_params =
            parsec_data.get_params_for_current_mass_and_age(mass.unwrap(), age.as_years());
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
            "calculated luminosity: {}, real luminosity: {}",
            calculated_sun.get_luminosity().unwrap(),
            real_sun.get_luminosity().unwrap()
        );
        println!(
            "calculated temperature: {}, real temperature: {}",
            calculated_sun.get_temperature().unwrap(),
            real_sun.get_temperature().unwrap()
        );
        assert!(eq_within(
            calculated_sun.get_mass().unwrap().to_solar_masses(),
            real_sun.get_mass().unwrap(),
            1e-2
        ));
        assert!(eq_within(
            calculated_sun.get_radius().unwrap().to_solar_radii(),
            &real_sun.get_radius().unwrap(),
            1e-1
        ));
        assert!(eq_within(
            calculated_sun.get_luminosity().unwrap(),
            &real_sun.get_luminosity().unwrap(),
            0.
        ));
        assert!(eq_within(
            calculated_sun.get_temperature().unwrap(),
            &real_sun.get_temperature().unwrap(),
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
                let age = age.as_years();
                let mass_index = ParsecData::get_closest_mass_index(mass.as_solar_masses());
                let trajectory = parsec_data.get_trajectory_via_index(mass_index);
                let age_expectancy = ParsecData::get_life_expectancy_in_years(trajectory);
                let age_expectancy = Time::from_years(age_expectancy as Float);
                if age_expectancy < Time::from_billion_years(0.3) {
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
        const SMALL_OFFSET: Float = 1e-4;
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
