#[derive(Debug)]
pub enum AstroUtilError {
    NormalizingZeroVector,
    Connection(reqwest::Error),
    Io(std::io::Error),
    ParsecDataNotAvailable,
}
