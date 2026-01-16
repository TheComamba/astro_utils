use std::fmt;

#[derive(Debug)]
pub enum AstroUtilError {
    AstroCoordsError(astro_coords::error::AstroCoordsError),
    DataNotAvailable(String),
    GaiaAccess(gaia_access::error::GaiaError),
    Io(std::io::Error),
    Json(serde_json::Error),
    MutexPoison,
    RmpSerialization(rmp_serde::encode::Error),
    RmpDeserialization(rmp_serde::decode::Error),
    RandError(String),
}

impl fmt::Display for AstroUtilError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstroUtilError::AstroCoordsError(err) => write!(f, "AstroCoords error: {}", err),
            AstroUtilError::DataNotAvailable(data) => write!(f, "Data {} not available", data),
            AstroUtilError::GaiaAccess(err) => write!(f, "Gaia access error: {:?}", err),
            AstroUtilError::Io(err) => write!(f, "I/O error: {}", err),
            AstroUtilError::Json(err) => write!(f, "JSON error: {}", err),
            AstroUtilError::MutexPoison => write!(f, "Mutex poisoned"),
            AstroUtilError::RmpSerialization(err) => {
                write!(f, "MessagePack serialization error: {}", err)
            }
            AstroUtilError::RmpDeserialization(err) => {
                write!(f, "MessagePack deserialization error: {}", err)
            }
            AstroUtilError::RandError(err) => write!(f, "Rand error: {}", err),
        }
    }
}

impl From<astro_coords::error::AstroCoordsError> for AstroUtilError {
    fn from(err: astro_coords::error::AstroCoordsError) -> Self {
        AstroUtilError::AstroCoordsError(err)
    }
}

impl From<gaia_access::error::GaiaError> for AstroUtilError {
    fn from(err: gaia_access::error::GaiaError) -> Self {
        AstroUtilError::GaiaAccess(err)
    }
}

impl From<std::io::Error> for AstroUtilError {
    fn from(err: std::io::Error) -> Self {
        AstroUtilError::Io(err)
    }
}

impl From<serde_json::Error> for AstroUtilError {
    fn from(err: serde_json::Error) -> Self {
        AstroUtilError::Json(err)
    }
}

impl From<rmp_serde::encode::Error> for AstroUtilError {
    fn from(err: rmp_serde::encode::Error) -> Self {
        AstroUtilError::RmpSerialization(err)
    }
}

impl From<rmp_serde::decode::Error> for AstroUtilError {
    fn from(err: rmp_serde::decode::Error) -> Self {
        AstroUtilError::RmpDeserialization(err)
    }
}

impl From<rand_distr::weighted::Error> for AstroUtilError {
    fn from(err: rand_distr::weighted::Error) -> Self {
        AstroUtilError::RandError(err.to_string())
    }
}

impl From<rand_distr::uniform::Error> for AstroUtilError {
    fn from(err: rand_distr::uniform::Error) -> Self {
        AstroUtilError::RandError(err.to_string())
    }
}
