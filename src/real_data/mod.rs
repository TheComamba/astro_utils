use crate::units::{distance::METERS_PER_SUN_RADII, mass::KG_PER_SOLAR_MASSES};
use simple_si_units::base::{Distance, Mass};

pub mod planets;
pub mod stars;

pub const SUN_MASS: Mass<f64> = Mass {
    kg: KG_PER_SOLAR_MASSES,
};
pub const SUN_RADIUS: Distance<f64> = Distance {
    m: METERS_PER_SUN_RADII,
};
