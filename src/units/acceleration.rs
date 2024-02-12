use crate::astro_display::AstroDisplay;
use simple_si_units::mechanical::Acceleration;

pub const EARTH_SURFACE_GRAVITY: Acceleration<f64> = Acceleration { mps2: 9.80665 };

pub fn acceleration_to_earth_gravity_units(acceleration: &Acceleration<f64>) -> f64 {
    acceleration / &EARTH_SURFACE_GRAVITY
}

impl AstroDisplay for Acceleration<f64> {
    fn astro_display(&self) -> String {
        format!("{:.2e} g", acceleration_to_earth_gravity_units(self))
    }
}
