use astro_coords::direction::Direction;
use astro_units::illuminance::Illuminance;
use uom::si::{
    acceleration::standard_gravity,
    angle::degree,
    f64::{Acceleration, Angle, MassDensity},
    mass_density::kilogram_per_cubic_meter,
};

use crate::{astro_display::AstroDisplay, units::illuminance::illuminance_to_apparent_magnitude};

impl AstroDisplay for Acceleration {
    fn astro_display(&self) -> String {
        format!("{:.2} g", self.get::<standard_gravity>())
    }
}

impl AstroDisplay for Angle {
    fn astro_display(&self) -> String {
        format!("{:.2} °", self.get::<degree>())
    }
}

impl AstroDisplay for Direction {
    fn astro_display(&self) -> String {
        self.to_string()
    }
}

impl AstroDisplay for f64 {
    fn astro_display(&self) -> String {
        format!("{:.2}", self)
    }
}

impl AstroDisplay for Illuminance {
    fn astro_display(&self) -> String {
        let apparent_magnitude = illuminance_to_apparent_magnitude(*self);
        format!("{:.2} app. mag.", apparent_magnitude)
    }
}

impl AstroDisplay for MassDensity {
    fn astro_display(&self) -> String {
        format!("{:.0} kg/m³", self.get::<kilogram_per_cubic_meter>())
    }
}

impl AstroDisplay for String {
    fn astro_display(&self) -> String {
        self.clone()
    }
}

impl<T> AstroDisplay for Option<T>
where
    T: AstroDisplay,
{
    fn astro_display(&self) -> String {
        match self {
            Some(value) => value.astro_display(),
            None => String::new(),
        }
    }
}

impl<T> AstroDisplay for &T
where
    T: AstroDisplay,
{
    fn astro_display(&self) -> String {
        (*self).astro_display()
    }
}
