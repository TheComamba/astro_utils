use super::data::ParsecData;
use super::line::ParsecLine;
use crate::error::AstroUtilError;
use crate::units::mass;
use directories::ProjectDirs;
use flate2::read::GzDecoder;
use rmp_serde;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use tar::Archive;

impl ParsecData {
    const METALLICITY: &'static str = "Z0.01";
    const FILENAME: &'static str = "Z0.01.rmp";

    pub(super) fn new() -> Result<ParsecData, AstroUtilError> {
        let project_dirs = get_project_dirs()?;
        let data_dir = project_dirs.data_dir();
        let file_path = data_dir.join(Self::FILENAME);

        if file_path.exists() {
            println!("Reading PARSEC data from {}", file_path.display());
            let file = File::open(&file_path).map_err(AstroUtilError::Io)?;
            let parsec_data: ParsecData =
                rmp_serde::from_read(file).map_err(AstroUtilError::RmpDeserialization)?;
            if parsec_data.is_filled() {
                Ok(parsec_data)
            } else {
                Err(AstroUtilError::DataNotAvailable)
            }
        } else {
            Self::ensure_data_files()?;
            let folder_path = data_dir.join(PathBuf::from(Self::METALLICITY));
            let filepaths = fs::read_dir(folder_path).map_err(AstroUtilError::Io)?;
            let mut parsec_data = ParsecData {
                data: Vec::with_capacity(Self::SORTED_MASSES.len()),
            };
            for _ in Self::SORTED_MASSES.iter() {
                parsec_data.data.push(Vec::new());
            }
            for entry in filepaths {
                Self::read_file(entry, &mut parsec_data)?;
            }
            println!("Writing PARSEC data to {}", file_path.display());
            let file = File::create(&file_path).map_err(AstroUtilError::Io)?;
            let buffer =
                rmp_serde::to_vec(&parsec_data).map_err(AstroUtilError::RmpSerialization)?;
            let mut writer = BufWriter::new(file);
            writer.write_all(&buffer).map_err(AstroUtilError::Io)?;
            if parsec_data.is_filled() {
                Ok(parsec_data)
            } else {
                Err(AstroUtilError::DataNotAvailable)
            }
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

    fn ensure_data_files() -> Result<(), AstroUtilError> {
        let project_dirs = get_project_dirs()?;
        let data_dir = project_dirs.data_dir();
        let path = data_dir.join(PathBuf::from(Self::METALLICITY));
        if !path.exists() {
            Self::download()?;
        }
        Ok(())
    }

    fn read_file(
        entry: Result<fs::DirEntry, std::io::Error>,
        parsec_data: &mut ParsecData,
    ) -> Result<(), AstroUtilError> {
        let file_path = entry.map_err(AstroUtilError::Io)?.path();
        let file = File::open(file_path).map_err(AstroUtilError::Io)?;
        let reader = BufReader::new(file);
        let mut mass_position = None;
        for line in reader.lines() {
            ParsecLine::read(line, &mut mass_position, parsec_data)?;
        }
        Ok(())
    }
}

fn get_project_dirs() -> Result<ProjectDirs, AstroUtilError> {
    ProjectDirs::from("", "the_comamba", "astro_utils").ok_or(AstroUtilError::Io(
        std::io::Error::new(std::io::ErrorKind::Other, "Could not get project dirs"),
    ))
}
