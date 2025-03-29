use uom::si::{f64::MassDensity, mass_density::kilogram_per_cubic_meter};

use crate::astro_display::AstroDisplay;

impl AstroDisplay for MassDensity {
    fn astro_display(&self) -> String {
        format!("{:.0} kg/m³", self.get::<kilogram_per_cubic_meter>())
    }
}
