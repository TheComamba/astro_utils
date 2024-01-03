use crate::units::{
    length::{Length, AU_PER_SUN_RADII},
    mass::{Mass, KILOGRAMS_PER_SOLAR_MASS},
};

pub mod planets;
pub mod stars;

pub const SUN_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_SOLAR_MASS);
pub const SUN_RADIUS: Length = Length::from_astronomical_units(AU_PER_SUN_RADII);
