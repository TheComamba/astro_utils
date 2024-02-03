use crate::Float;
use simple_si_units::base::{Distance, Mass};

pub mod planets;
pub mod stars;

pub const SUN_MASS: Mass<Float> = Mass { kg: 1.988e30 };
pub const SUN_RADIUS: Distance<Float> = Distance { m: 6.957e8 };
