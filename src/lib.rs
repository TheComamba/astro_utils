pub type Float = f32;
pub static PI: Float = std::f32::consts::PI;
pub static TWO_PI: Float = 2.0 * PI;

pub mod angle;
pub mod kepler_orbit;
pub mod length;
pub mod mass;
pub mod solar_system_data;
pub mod time;
