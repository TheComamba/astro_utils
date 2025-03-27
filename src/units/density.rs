use crate::astro_display::AstroDisplay;

impl AstroDisplay for Density<f64> {
    fn astro_display(&self) -> String {
        format!("{:.0} kg/m³", self.kgpm3)
    }
}
