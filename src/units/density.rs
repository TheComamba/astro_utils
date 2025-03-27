use uom::si::f64::MassDensity;

use crate::astro_display::AstroDisplay;

impl AstroDisplay for MassDensity {
    fn astro_display(&self) -> String {
        format!("{:.0} kg/m³", self.kgpm3)
    }
}
