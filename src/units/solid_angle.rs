use crate::astro_display::AstroDisplay;
use simple_si_units::geometry::SolidAngle;

impl AstroDisplay for SolidAngle<f64> {
    fn astro_display(&self) -> String {
        format!("{:.2} sr", self.sr)
    }
}
