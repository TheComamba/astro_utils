use crate::units::{
    distance::METERS_PER_SUN_RADII, luminosity::WATTS_PER_SOLAR_LUMINOSITY,
    mass::KG_PER_SOLAR_MASSES,
};
use simple_si_units::base::{Distance, Luminosity, Mass};

pub mod planets;
pub mod stars;

pub const SUN_LUMINOSITY: Luminosity<f64> = Luminosity {
    cd: WATTS_PER_SOLAR_LUMINOSITY,
};

pub const SUN_MASS: Mass<f64> = Mass {
    kg: KG_PER_SOLAR_MASSES,
};

pub const SUN_RADIUS: Distance<f64> = Distance {
    m: METERS_PER_SUN_RADII,
};
