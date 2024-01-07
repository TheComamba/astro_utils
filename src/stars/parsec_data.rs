use crate::{error::AstroUtilError, Float};
use directories::ProjectDirs;
use flate2::read::GzDecoder;
use std::{collections::HashMap, path::PathBuf};
use tar::Archive;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    mass_to_data: HashMap<String, Vec<ParsecLine>>,
}

fn get_project_dirs() -> Result<ProjectDirs, AstroUtilError> {
    ProjectDirs::from("", "the_comamba", "astro_utils").ok_or(AstroUtilError::Io(
        std::io::Error::new(std::io::ErrorKind::Other, "Could not get project dirs"),
    ))
}

pub(super) fn format_mass_like_parsec(mass: Float) -> String {
    if mass > 350. {
        "350.0".to_string()
    } else if mass > 165. {
        let mass = (mass / 50.).round() * 50.;
        format!("{:.1}", mass)
    } else if mass > 125. {
        "130.0".to_string()
    } else if mass > 104. {
        "120.0".to_string()
    } else if mass > 29. {
        let mass = (mass / 5.).round() * 5.;
        format!("{:.1}", mass)
    } else if mass > 26. {
        "28.0".to_string()
    } else if mass > 22. {
        "24.0".to_string()
    } else if mass > 9.5 {
        let mass = (mass / 2.).round() * 2.;
        format!("{:.1}", mass)
    } else if mass > 6.5 {
        let mass = mass.round();
        format!("{:.2}", mass)
    } else if mass > 2.34 {
        let mass = (mass / 0.2).round() * 0.2;
        format!("{:.2}", mass)
    } else if mass > 0.18 {
        let mass = (mass / 0.05).round() * 0.05;
        format!("{:.2}", mass)
    } else if mass > 0.095 {
        let mass = (mass / 0.02).round() * 0.02;
        format!("{:.2}", mass)
    } else {
        "0.09".to_string()
    }
}

fn download_parsec_data(metallicity: Metallicity) -> Result<(), AstroUtilError> {
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
    archive
        .unpack(data_dir)
        .map_err(AstroUtilError::Io)?;
    Ok(())
}

fn ensure_parsec_data(metallicity: Metallicity) -> Result<(), AstroUtilError> {
    let project_dirs = get_project_dirs()?;
    let data_dir = project_dirs.data_dir();
    let path = data_dir.join(PathBuf::from(metallicity.to_string()));
    if !path.exists() {
        download_parsec_data(metallicity)?;
    }
    Ok(())
}

fn read_parsec_data() -> Result<ParsecData, AstroUtilError> {
    const METALLICITY: Metallicity = Metallicity::Z0_01;
    const MASS_INDEX: usize = 1;
    const AGE_INDEX: usize = 2;
    const LOG_L_INDEX: usize = 3;
    const LOG_TE_INDEX: usize = 4;
    const LOG_R_INDEX: usize = 5;

    ensure_parsec_data(METALLICITY)?;

    let mut parsec_data = ParsecData {
        mass_to_data: HashMap::new(),
    };

    let project_dirs = get_project_dirs()?;
    let data_dir = project_dirs.data_dir();
    let path = data_dir.join(PathBuf::from(METALLICITY.to_string()));
    let filepaths = fs::read_dir(path).map_err(AstroUtilError::Io)?;
    for entry in filepaths {
        let file_path = entry.map_err(AstroUtilError::Io)?.path();
        let file = File::open(&file_path).map_err(AstroUtilError::Io)?;
        let reader = BufReader::new(file);
        let mut mass = None;

        for line in reader.lines() {
            let line = line.map_err(AstroUtilError::Io)?;
            let entries: Vec<&str> = line.split_whitespace().collect();
            if mass.is_none() {
                let mass_entry = entries.get(MASS_INDEX).ok_or(AstroUtilError::ParsecDataNotAvailable)?;
                if let Ok(mass_value) = mass_entry.parse::<Float>() {
                    mass = Some(format_mass_like_parsec(mass_value));
                }
            }
            if let Some(mass_value) = mass {
                let age_entry = entries.get(AGE_INDEX).ok_or(AstroUtilError::ParsecDataNotAvailable)?;
                let log_l_entry = entries.get(LOG_L_INDEX).ok_or(AstroUtilError::ParsecDataNotAvailable)?;
                let log_te_entry = entries.get(LOG_TE_INDEX).ok_or(AstroUtilError::ParsecDataNotAvailable)?;
                let log_r_entry = entries.get(LOG_R_INDEX).ok_or(AstroUtilError::ParsecDataNotAvailable)?;
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
                    let data = parsec_data.mass_to_data.entry(mass_value).or_insert(Vec::new());
                    data.push(parsec_line);
                }
            }
        
        }
    }

    Ok(parsec_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_parsec_data() {
        ensure_parsec_data(Metallicity::Z0_01).unwrap();
    }
}
