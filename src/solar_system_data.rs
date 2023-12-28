use crate::{
    length::{Length, EARTH_RADII_PER_METER, JUPITER_RADII_PER_METER, SUN_RADII_PER_METER},
    mass::{
        Mass, EARTH_MASSES_PER_KILOGRAM, JUPITER_MASSES_PER_KILOGRAM, SOLAR_MASSES_PER_KILOGRAM,
    },
};

pub static SUN_RADIUS: Length = Length::from_meters(SUN_RADII_PER_METER);
pub static SUN_MASS: Mass = Mass::from_kilograms(SOLAR_MASSES_PER_KILOGRAM);

pub static EARTH_SEMI_MAJOR_AXIS: Length = Length::from_meters(149_598_023_000.0);
pub static EARTH_RADIUS: Length = Length::from_meters(EARTH_RADII_PER_METER);
pub static EARTH_MASS: Mass = Mass::from_kilograms(EARTH_MASSES_PER_KILOGRAM);

pub static JUPITER_SEMI_MAJOR_AXIS: Length = Length::from_meters(778_479_000_000.0);
pub static JUPITER_RADIUS: Length = Length::from_meters(JUPITER_RADII_PER_METER);
pub static JUPITER_MASS: Mass = Mass::from_kilograms(JUPITER_MASSES_PER_KILOGRAM);

pub static MOON_SEMI_MAJOR_AXIS: Length = Length::from_meters(384_399_000.0);
pub static MOON_RADIUS: Length = Length::from_meters(1_737_000.0);
pub static MOON_MASS: Mass = Mass::from_kilograms(7.34767309e22);
