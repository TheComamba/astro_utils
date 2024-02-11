use crate::astro_display::AstroDisplay;
use simple_si_units::mechanical::Density;

impl AstroDisplay for Density<f64> {
    fn astro_display(&self) -> String {
        format!("{:.2} kg/m³", self.kgpm3)
    }
}
