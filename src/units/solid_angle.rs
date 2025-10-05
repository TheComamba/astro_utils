use uom::si::{f64::SolidAngle, solid_angle::steradian};

use crate::astro_display::AstroDisplay;

impl AstroDisplay for SolidAngle {
    fn astro_display(&self) -> String {
        format!("{:.2} sr", self.get::<steradian>())
    }
}
