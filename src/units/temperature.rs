use std::fmt::Display;

use crate::Float;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Temperature {
    pub(super) kelvin: Float,
}

impl Temperature {
    pub const fn from_kelvin(kelvin: Float) -> Temperature {
        Temperature { kelvin }
    }

    pub const fn as_kelvin(&self) -> Float {
        self.kelvin
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0} K", self.kelvin)
    }
}
