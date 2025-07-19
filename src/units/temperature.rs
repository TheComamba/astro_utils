use uom::si::{
    f64::{TemperatureInterval, ThermodynamicTemperature},
    temperature_interval, thermodynamic_temperature,
};

use crate::astro_display::AstroDisplay;

impl AstroDisplay for ThermodynamicTemperature {
    fn astro_display(&self) -> String {
        format!("{:.0} K", self.get::<thermodynamic_temperature::kelvin>())
    }
}

impl AstroDisplay for TemperatureInterval {
    fn astro_display(&self) -> String {
        format!("{:.0} K", self.get::<temperature_interval::kelvin>())
    }
}
