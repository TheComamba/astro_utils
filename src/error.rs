#[derive(Debug)]
pub enum AstroUtilError {
    NormalizingZeroVector,
    Connection(reqwest::Error),
    Io(std::io::Error),
    Json(serde_json::Error),
    RmpSerialization(rmp_serde::encode::Error),
    RmpDeserialization(rmp_serde::decode::Error),
    DataNotAvailable,
}
