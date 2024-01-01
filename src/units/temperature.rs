use crate::Float;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Temperature {
    pub(super) kelvin: Float,
}

impl Temperature {
    pub fn from_kelvin(kelvin: Float) -> Temperature {
        Temperature { kelvin }
    }

    pub fn get_kelvin(&self) -> Float {
        self.kelvin
    }
}
