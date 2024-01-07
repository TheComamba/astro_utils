use crate::{error::AstroUtilError, Float};
use directories::ProjectDirs;
use flate2::read::GzDecoder;
use std::{
    collections::HashMap,
    fs::File,
    io::{copy, Bytes, Read},
    path::PathBuf,
};
use tar::Archive;

struct ParsecLine {
    age: Float,
    log_l: Float,
    log_te: Float,
    log_r: Float,
}

pub struct ParsecData {
    mass_to_data: HashMap<Float, Vec<ParsecLine>>,
}

fn get_project_dirs() -> Result<ProjectDirs, AstroUtilError> {
    ProjectDirs::from("", "the_comamba", "astro_utils").ok_or(AstroUtilError::FileSystem(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Could not get project dirs",
    )))
}

fn download_parsec_data() -> Result<(), AstroUtilError> {
    let project_dirs = get_project_dirs()?;
    let data_dir = project_dirs.data_dir();
    let data_dir = data_dir
        .to_str()
        .ok_or(AstroUtilError::FileSystem(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not convert data dir to string",
        )))?;
    println!("Downloading PARSEC data to {}", data_dir);
    let target = "https://people.sissa.it/~sbressan/CAF09_V1.2S_M36_LT/no_phase/Z0.01.tar.gz";
    let mut response = reqwest::blocking::get(target).map_err(AstroUtilError::Connection)?;
    let gz_decoder = GzDecoder::new(&mut response);
    let mut archive = Archive::new(gz_decoder);
    archive
        .unpack(data_dir)
        .map_err(AstroUtilError::FileSystem)?;
    Ok(())
}

fn ensure_parsec_data() -> Result<(), AstroUtilError> {
    let project_dirs = get_project_dirs()?;
    let data_dir = project_dirs.data_dir();
    let filepath = data_dir.join(PathBuf::from("Z0.01"));
    if !filepath.exists() {
        download_parsec_data()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_parsec_data() {
        ensure_parsec_data().unwrap();
    }
}
