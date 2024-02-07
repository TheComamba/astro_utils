use crate::astro_display::AstroDisplay;
use simple_si_units::base::Temperature;

pub const TEMPERATURE_ZERO: Temperature<f64> = Temperature { K: 0. };

impl AstroDisplay for Temperature<f64> {
    fn astro_display(&self) -> String {
        format!("{:.2} K", self.K)
    }
}
