pub type Float = f32;
pub static PI: Float = std::f32::consts::PI;
pub static TWO_PI: Float = 2.0 * PI;

pub mod coordinates;
pub mod kepler_orbit;
pub mod solar_system_data;
pub mod units;
