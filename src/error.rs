use std::fmt;
#[derive(Debug)]
pub enum AstroUtilError {
    NormalizingZeroVector,
    Connection(reqwest::Error),
    Io(std::io::Error),
    Json(serde_json::Error),
    RmpSerialization(rmp_serde::encode::Error),
    RmpDeserialization(rmp_serde::decode::Error),
    MutexPoison,
    DataNotAvailable,
}

impl fmt::Display for AstroUtilError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstroUtilError::NormalizingZeroVector => {
                write!(f, "Encountered a zero vector while normalizing")
            }
            AstroUtilError::Connection(err) => write!(f, "Connection error: {}", err),
            AstroUtilError::Io(err) => write!(f, "I/O error: {}", err),
            AstroUtilError::Json(err) => write!(f, "JSON error: {}", err),
            AstroUtilError::RmpSerialization(err) => {
                write!(f, "MessagePack serialization error: {}", err)
            }
            AstroUtilError::RmpDeserialization(err) => {
                write!(f, "MessagePack deserialization error: {}", err)
            }
            AstroUtilError::DataNotAvailable => write!(f, "Data not available"),
            AstroUtilError::MutexPoison => write!(f, "Mutex poisoned"),
        }
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
