use crate::{
    units::conversions::{KG_PER_SOLAR_MASSES, METERS_PER_SUN_RADII},
    Float,
};
use simple_si_units::base::{Distance, Mass};

pub mod planets;
pub mod stars;

pub const SUN_MASS: Mass<Float> = Mass {
    kg: KG_PER_SOLAR_MASSES,
};
pub const SUN_RADIUS: Distance<Float> = Distance {
    m: METERS_PER_SUN_RADII,
};
