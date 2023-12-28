pub type Float = f32;
pub(crate) static PI: Float = std::f32::consts::PI;
pub(crate) static TWO_PI: Float = 2.0 * PI;
pub(crate) static PI_HALF: Float = 0.5 * PI;

pub mod coordinates;
pub mod kepler_orbit;
pub mod solar_system_data;
pub mod units;
