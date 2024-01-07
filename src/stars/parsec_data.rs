use std::{collections::HashMap, fs::File, io::{copy, Bytes, Read}, path::PathBuf};

use crate::{error::AstroUtilError, Float};

struct ParsecLine {
    age: Float,
    log_l: Float,
    log_te: Float,
    log_r: Float,
}

pub struct ParsecData {
    mass_to_data: HashMap<Float, Vec<ParsecLine>>,
}

fn download_parsec_data() -> Result<(), AstroUtilError> {
    let filepath = PathBuf::from("/tmp/Z0.01.tag.gz");
    let target = "https://people.sissa.it/~sbressan/CAF09_V1.2S_M36_LT/no_phase/Z0.01.tar.gz";
    let mut response = reqwest::blocking::get(target).map_err(AstroUtilError::Connection)?;
    let mut file = File::create(filepath).map_err(AstroUtilError::FileSystem)?;
    copy(&mut response, &mut file).map_err(AstroUtilError::FileSystem)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_download_parsec_data() {
        super::download_parsec_data().unwrap();
    }
}