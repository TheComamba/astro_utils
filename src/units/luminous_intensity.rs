use astro_units::luminous_intensity::luminous_intensity_to_absolute_magnitude;
use uom::si::f64::LuminousIntensity;

use crate::astro_display::AstroDisplay;

impl AstroDisplay for LuminousIntensity {
    fn astro_display(&self) -> String {
        let absolute_magnitude = luminous_intensity_to_absolute_magnitude(*self);
        format!("{:.2} abs. mag.", absolute_magnitude)
    }
}
