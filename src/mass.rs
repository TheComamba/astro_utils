use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use crate::Float;

pub static KILOGRAMS_PER_EARTH_MASS: Float = 5.972e24;
pub static KILOGRAMS_PER_JUPITER_MASS: Float = 1.898e27;
pub static KILOGRAMS_PER_SOLAR_MASS: Float = 1.989e30;

pub struct Mass {
    kilograms: Float,
}

impl Mass {
    pub fn from_kilograms(kilograms: Float) -> Mass {
        Mass { kilograms }
    }

    pub fn from_earth_masses(earth_masses: Float) -> Mass {
        Mass {
            kilograms: earth_masses * KILOGRAMS_PER_EARTH_MASS,
        }
    }

    pub fn from_jupiter_masses(jupiter_masses: Float) -> Mass {
        Mass {
            kilograms: jupiter_masses * KILOGRAMS_PER_JUPITER_MASS,
        }
    }

    pub fn from_solar_masses(solar_masses: Float) -> Mass {
        Mass {
            kilograms: solar_masses * KILOGRAMS_PER_SOLAR_MASS,
        }
    }

    pub fn as_kilograms(&self) -> Float {
        self.kilograms
    }

    pub fn as_earth_masses(&self) -> Float {
        self.kilograms / KILOGRAMS_PER_EARTH_MASS
    }

    pub fn as_jupiter_masses(&self) -> Float {
        self.kilograms / KILOGRAMS_PER_JUPITER_MASS
    }

    pub fn as_solar_masses(&self) -> Float {
        self.kilograms / KILOGRAMS_PER_SOLAR_MASS
    }
}

impl Display for Mass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.kilograms > KILOGRAMS_PER_SOLAR_MASS {
            write!(f, "{} solar masses", self.as_solar_masses())
        } else if self.kilograms > KILOGRAMS_PER_JUPITER_MASS {
            write!(f, "{} Jupiter masses", self.as_jupiter_masses())
        } else if self.kilograms > KILOGRAMS_PER_EARTH_MASS {
            write!(f, "{} Earth masses", self.as_earth_masses())
        } else {
            write!(f, "{} kg", self.kilograms)
        }
    }
}

impl Add for Mass {
    type Output = Mass;

    fn add(self, other: Mass) -> Mass {
        Mass {
            kilograms: self.kilograms + other.kilograms,
        }
    }
}

impl Sub for Mass {
    type Output = Mass;

    fn sub(self, other: Mass) -> Mass {
        Mass {
            kilograms: self.kilograms - other.kilograms,
        }
    }
}
