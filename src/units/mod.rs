// https://www.astro.princeton.edu/~gk/A403/constants.pdf

pub mod conversions;
pub mod formatting;

pub const ANGLE_ZERO: Angle<Float> = Angle { rad: 0. };
pub const DISTANCE_ZERO: Distance<Float> = Distance { m: 0. };
pub const LUMINOSITY_ZERO: Luminosity<Float> = Luminosity { watts: 0. };
