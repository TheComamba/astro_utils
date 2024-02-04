// https://www.astro.princeton.edu/~gk/A403/constants.pdf

use simple_si_units::{
    base::{Distance, Luminosity},
    electromagnetic::Illuminance,
    geometry::Angle,
};

use crate::Float;

pub mod conversions;
pub mod formatting;

pub const ANGLE_ZERO: Angle<Float> = Angle { rad: 0. };
pub const DISTANCE_ZERO: Distance<Float> = Distance { m: 0. };
pub const ILLUMINANCE_ZERO: Illuminance<Float> = Illuminance { lux: 0. };
pub const LUMINOSITY_ZERO: Luminosity<Float> = Luminosity { cd: 0. };
