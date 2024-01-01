use crate::units::mass::{Mass, KILOGRAMS_PER_SOLAR_MASS};

pub mod planets;
pub mod stars;

pub const SUN_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_SOLAR_MASS);
