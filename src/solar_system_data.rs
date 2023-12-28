use crate::units::length::{
    Length, METERS_PER_EARTH_RADIUS, METERS_PER_JUPITER_RADIUS, METERS_PER_SUN_RADIUS,
};
use crate::units::mass::{
    Mass, KILOGRAMS_PER_EARTH_MASS, KILOGRAMS_PER_JUPITER_MASS, KILOGRAMS_PER_SOLAR_MASS,
};

pub const SUN_RADIUS: Length = Length::from_meters(METERS_PER_SUN_RADIUS);
pub const SUN_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_SOLAR_MASS);

pub const EARTH_SEMI_MAJOR_AXIS: Length = Length::from_meters(149_598_023_000.);
pub const EARTH_RADIUS: Length = Length::from_meters(METERS_PER_EARTH_RADIUS);
pub const EARTH_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_EARTH_MASS);

pub const JUPITER_SEMI_MAJOR_AXIS: Length = Length::from_meters(778_479_000_000.);
pub const JUPITER_RADIUS: Length = Length::from_meters(METERS_PER_JUPITER_RADIUS);
pub const JUPITER_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_JUPITER_MASS);

pub const MOON_SEMI_MAJOR_AXIS: Length = Length::from_meters(384_399_000.);
pub const MOON_RADIUS: Length = Length::from_meters(1_737_400.);
pub const MOON_MASS: Mass = Mass::from_kilograms(7.342e22);
