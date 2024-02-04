use crate::{
    f64,
    units::conversions::{KG_PER_SOLAR_MASSES, METERS_PER_SUN_RADII},
};
use simple_si_units::base::{Distance, Mass};

pub mod planets;
pub mod stars;

pub const SUN_MASS: Mass<f64> = Mass {
    kg: KG_PER_SOLAR_MASSES,
};
pub const SUN_RADIUS: Distance<f64> = Distance {
    m: METERS_PER_SUN_RADII,
};
