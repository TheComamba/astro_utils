use uom::si::{f64::ThermodynamicTemperature, thermodynamic_temperature::kelvin};

use crate::astro_display::AstroDisplay;

impl AstroDisplay for ThermodynamicTemperature {
    fn astro_display(&self) -> String {
        format!("{:.0} K", self.get::<kelvin>())
    }
}
