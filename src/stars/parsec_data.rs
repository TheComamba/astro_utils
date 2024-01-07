use crate::{error::AstroUtilError, Float};
use directories::ProjectDirs;
use flate2::read::GzDecoder;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use tar::Archive;

enum Metallicity {
    Z0_01,
}

impl ToString for Metallicity {
    fn to_string(&self) -> String {
        match self {
            Metallicity::Z0_01 => "Z0.01",
        }
        .to_string()
    }
}

struct ParsecLine {
    age: Float,
    log_l: Float,
    log_te: Float,
    log_r: Float,
}

pub struct ParsecData {
    data: Vec<Vec<ParsecLine>>,
}

impl ParsecData {
    const SORTED_MASSES: [Float; 100] = [
        0.09, 0.10, 0.12, 0.14, 0.16, 0.20, 0.25, 0.30, 0.35, 0.40, 0.45, 0.50, 0.55, 0.60, 0.65,
        0.70, 0.75, 0.80, 0.85, 0.90, 0.95, 1.00, 1.05, 1.10, 1.15, 1.20, 1.25, 1.30, 1.35, 1.40,
        1.45, 1.50, 1.55, 1.60, 1.65, 1.70, 1.75, 1.80, 1.85, 1.90, 1.95, 2.00, 2.05, 2.10, 2.15,
        2.20, 2.25, 2.30, 2.40, 2.60, 2.80, 3.00, 3.20, 3.40, 3.60, 3.80, 4.00, 4.20, 4.40, 4.60,
        4.80, 5.00, 5.20, 5.40, 5.60, 5.80, 6.00, 6.20, 6.40, 7.00, 8.00, 9.00, 10.0, 12.0, 14.0,
        16.0, 18.0, 20.0, 24.0, 28.0, 30.0, 35.0, 40.0, 45.0, 50.0, 55.0, 60.0, 65.0, 70.0, 75.0,
        80.0, 90.0, 95.0, 100.0, 120.0, 130.0, 200.0, 250.0, 300.0, 350.0,
    ];

    pub(super) fn new() -> Result<ParsecData, AstroUtilError> {
        const METALLICITY: Metallicity = Metallicity::Z0_01;
        const MASS_INDEX: usize = 1;
        const AGE_INDEX: usize = 2;
        const LOG_L_INDEX: usize = 3;
        const LOG_TE_INDEX: usize = 4;
        const LOG_R_INDEX: usize = 5;

        Self::ensure_files(METALLICITY)?;

        let mut parsec_data = ParsecData {
            data: Vec::with_capacity(Self::SORTED_MASSES.len()),
        };
        for _ in Self::SORTED_MASSES.iter() {
            parsec_data.data.push(Vec::new());
        }

        let project_dirs = get_project_dirs()?;
        let data_dir = project_dirs.data_dir();
        let folder_path = data_dir.join(PathBuf::from(METALLICITY.to_string()));
        let filepaths = fs::read_dir(folder_path).map_err(AstroUtilError::Io)?;
        for entry in filepaths {
            let file_path = entry.map_err(AstroUtilError::Io)?.path();
            let file = File::open(&file_path).map_err(AstroUtilError::Io)?;
            let reader = BufReader::new(file);
            let mut mass_position = None;

            for line in reader.lines() {
                let line = line.map_err(AstroUtilError::Io)?;
                let entries: Vec<&str> = line.split_whitespace().collect();
                if mass_position.is_none() {
                    let mass_entry = entries
                        .get(MASS_INDEX)
                        .ok_or(AstroUtilError::ParsecDataNotAvailable)?;
                    if let Ok(mass_value) = mass_entry.parse::<Float>() {
                        mass_position = Some(Self::get_closest_mass_index(mass_value));
                    }
                }
                if let Some(mass_position) = &mass_position {
                    let age_entry = entries
                        .get(AGE_INDEX)
                        .ok_or(AstroUtilError::ParsecDataNotAvailable)?;
                    let log_l_entry = entries
                        .get(LOG_L_INDEX)
                        .ok_or(AstroUtilError::ParsecDataNotAvailable)?;
                    let log_te_entry = entries
                        .get(LOG_TE_INDEX)
                        .ok_or(AstroUtilError::ParsecDataNotAvailable)?;
                    let log_r_entry = entries
                        .get(LOG_R_INDEX)
                        .ok_or(AstroUtilError::ParsecDataNotAvailable)?;
                    if let (Ok(age), Ok(log_l), Ok(log_te), Ok(log_r)) = (
                        age_entry.parse::<Float>(),
                        log_l_entry.parse::<Float>(),
                        log_te_entry.parse::<Float>(),
                        log_r_entry.parse::<Float>(),
                    ) {
                        let parsec_line = ParsecLine {
                            age,
                            log_l,
                            log_te,
                            log_r,
                        };
                        let data = parsec_data
                            .data
                            .get_mut(*mass_position)
                            .ok_or(AstroUtilError::ParsecDataNotAvailable)?;
                        data.push(parsec_line);
                    }
                }
            }
        }

        Ok(parsec_data)
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

    fn download(metallicity: Metallicity) -> Result<(), AstroUtilError> {
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
            + &metallicity.to_string()
            + ".tar.gz";
        let mut response = reqwest::blocking::get(target).map_err(AstroUtilError::Connection)?;
        let gz_decoder = GzDecoder::new(&mut response);
        let mut archive = Archive::new(gz_decoder);
        archive.unpack(data_dir).map_err(AstroUtilError::Io)?;
        Ok(())
    }

    fn ensure_files(metallicity: Metallicity) -> Result<(), AstroUtilError> {
        let project_dirs = get_project_dirs()?;
        let data_dir = project_dirs.data_dir();
        let path = data_dir.join(PathBuf::from(metallicity.to_string()));
        if !path.exists() {
            Self::download(metallicity)?;
        }
        Ok(())
    }

    fn get_trajectory(&self, mass: Float) -> &Vec<ParsecLine> {
        let mass_index = Self::get_closest_mass_index(mass);
        &self.data[mass_index]
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

    #[test]
    fn test_download_parsec_data() {
        ParsecData::ensure_files(Metallicity::Z0_01).unwrap();
    }

    #[test]
    fn test_read_parsec_data() {
        let parsec_data = ParsecData::new().unwrap();
        assert!(parsec_data.data.len() > 0);
    }

    #[test]
    fn map_masses_are_mapped_to_themselves() {
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
