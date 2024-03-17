use std::fmt;

#[derive(Debug)]
pub enum AstroUtilError {
    Connection(reqwest::Error),
    DataNotAvailable(String),
    GaiaAccess(gaia_access::error::GaiaError),
    Io(std::io::Error),
    Json(serde_json::Error),
    MutexPoison,
    NormalizingZeroVector,
    RmpSerialization(rmp_serde::encode::Error),
    RmpDeserialization(rmp_serde::decode::Error),
}

impl fmt::Display for AstroUtilError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstroUtilError::Connection(err) => write!(f, "Connection error: {}", err),
            AstroUtilError::DataNotAvailable(data) => write!(f, "Data {} not available", data),
            AstroUtilError::GaiaAccess(err) => write!(f, "Gaia access error: {:?}", err),
            AstroUtilError::Io(err) => write!(f, "I/O error: {}", err),
            AstroUtilError::Json(err) => write!(f, "JSON error: {}", err),
            AstroUtilError::MutexPoison => write!(f, "Mutex poisoned"),
            AstroUtilError::NormalizingZeroVector => {
                write!(f, "Encountered a zero vector while normalizing")
            }
            AstroUtilError::RmpSerialization(err) => {
                write!(f, "MessagePack serialization error: {}", err)
            }
            AstroUtilError::RmpDeserialization(err) => {
                write!(f, "MessagePack deserialization error: {}", err)
            }
        }
    }
}

impl From<reqwest::Error> for AstroUtilError {
    fn from(err: reqwest::Error) -> Self {
        AstroUtilError::Connection(err)
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

impl<T> From<&T> for AstroUtilError
where
    T: From<AstroUtilError>,
{
    fn from(err: &T) -> Self {
        err.into()
    }
}
