use crate::{
    length::{Length, METERS_PER_EARTH_RADIUS, METERS_PER_JUPITER_RADIUS, METERS_PER_SUN_RADIUS},
    mass::{Mass, KILOGRAMS_PER_EARTH_MASS, KILOGRAMS_PER_JUPITER_MASS, KILOGRAMS_PER_SOLAR_MASS},
};

pub static SUN_RADIUS: Length = Length::from_meters(METERS_PER_SUN_RADIUS);
pub static SUN_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_SOLAR_MASS);

pub static EARTH_SEMI_MAJOR_AXIS: Length = Length::from_meters(149_598_023_000.0);
pub static EARTH_RADIUS: Length = Length::from_meters(METERS_PER_EARTH_RADIUS);
pub static EARTH_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_EARTH_MASS);

pub static JUPITER_SEMI_MAJOR_AXIS: Length = Length::from_meters(778_479_000_000.0);
pub static JUPITER_RADIUS: Length = Length::from_meters(METERS_PER_JUPITER_RADIUS);
pub static JUPITER_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_JUPITER_MASS);

pub static MOON_SEMI_MAJOR_AXIS: Length = Length::from_meters(384_399_000.0);
pub static MOON_RADIUS: Length = Length::from_meters(1_737_400.0);
pub static MOON_MASS: Mass = Mass::from_kilograms(7.342e22);
