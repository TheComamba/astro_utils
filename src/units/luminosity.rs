use crate::Float;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Luminosity {
    magnitude: Float,
}

impl Luminosity {
    pub fn from_magnitude(magnitude: Float) -> Luminosity {
        Luminosity { magnitude }
    }

    pub fn get_magnitude(&self) -> Float {
        self.magnitude
    }
}
