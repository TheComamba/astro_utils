#[derive(Debug)]
pub enum AstroUtilError {
    NormalizingZeroVector,
    Connection(reqwest::Error),
    Io(std::io::Error),
    Json(serde_json::Error),
    DataNotAvailable,
}
