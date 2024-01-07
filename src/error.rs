#[derive(Debug)]
pub enum AstroUtilError {
    NormalizingZeroVector,
    Connection(reqwest::Error),
    FileSystem(std::io::Error),
}
