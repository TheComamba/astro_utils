use uom::si::{acceleration::standard_gravity, f64::Acceleration};

use crate::astro_display::AstroDisplay;

impl AstroDisplay for Acceleration {
    fn astro_display(&self) -> String {
        format!("{:.2} g", self.get::<standard_gravity>())
    }
}
