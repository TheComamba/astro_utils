use crate::units::{
    length::{Length, METERS_PER_SUN_RADIUS},
    mass::{Mass, KILOGRAMS_PER_SOLAR_MASS},
};

pub mod planets;
pub mod stars;

pub const SUN_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_SOLAR_MASS);
pub const SUN_RADIUS: Length = Length::from_meters(METERS_PER_SUN_RADIUS);
