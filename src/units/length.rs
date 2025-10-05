use astro_units::length::{earth_radius, solar_radius};
use uom::si::{
    f64::Length,
    length::{astronomical_unit, kilometer, light_year, meter, micrometer, millimeter, nanometer},
};

use crate::astro_display::AstroDisplay;

use super::DISPLAY_THRESHOLD;

pub enum LengthUnit {
    Nanometers,
    Micrometers,
    Millimeters,
    Meters,
    Kilometers,
    EarthRadii,
    SunRadii,
    AstronomicalUnits,
    LightYears,
}

pub fn display_distance_in_units(distance: &Length, units: LengthUnit) -> String {
    match units {
        LengthUnit::Nanometers => format!("{:.2} nm", distance.get::<nanometer>()),
        LengthUnit::Micrometers => format!("{:.2} μm", distance.get::<micrometer>()),
        LengthUnit::Millimeters => format!("{:.2} mm", distance.get::<millimeter>()),
        LengthUnit::Meters => format!("{:.2} m", distance.get::<meter>()),
        LengthUnit::Kilometers => format!("{:.2} km", distance.get::<kilometer>()),
        LengthUnit::EarthRadii => format!("{:.2} R🜨", distance.get::<earth_radius>()),
        LengthUnit::SunRadii => format!("{:.2} R☉", distance.get::<solar_radius>()),
        LengthUnit::AstronomicalUnits => format!("{:.2} AU", distance.get::<astronomical_unit>()),
        LengthUnit::LightYears => format!("{:.2} lyr", distance.get::<light_year>()),
    }
}

impl AstroDisplay for Length {
    fn astro_display(&self) -> String {
        let units = if self.get::<light_year>().abs() > DISPLAY_THRESHOLD {
            LengthUnit::LightYears
        } else if self.get::<astronomical_unit>().abs() > DISPLAY_THRESHOLD {
            LengthUnit::AstronomicalUnits
        } else if self.get::<solar_radius>().abs() > DISPLAY_THRESHOLD {
            LengthUnit::SunRadii
        } else if self.get::<earth_radius>().abs() > DISPLAY_THRESHOLD {
            LengthUnit::EarthRadii
        } else if self.get::<kilometer>().abs() > DISPLAY_THRESHOLD {
            LengthUnit::Kilometers
        } else if self.get::<meter>().abs() > DISPLAY_THRESHOLD {
            LengthUnit::Meters
        } else if self.get::<millimeter>().abs() > DISPLAY_THRESHOLD {
            LengthUnit::Millimeters
        } else if self.get::<micrometer>().abs() > DISPLAY_THRESHOLD {
            LengthUnit::Micrometers
        } else {
            LengthUnit::Nanometers
        };
        display_distance_in_units(self, units)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_display() {
        let d = Length::new::<nanometer>(1.23);
        assert_eq!(d.astro_display(), "1.23 nm");
        let d = Length::new::<meter>(1.23);
        assert_eq!(d.astro_display(), "1.23 m");
        let d = Length::new::<kilometer>(1.23);
        assert_eq!(d.astro_display(), "1.23 km");
        let d = Length::new::<earth_radius>(1.23);
        assert_eq!(d.astro_display(), "1.23 R🜨");
        let d = Length::new::<solar_radius>(1.23);
        assert_eq!(d.astro_display(), "1.23 R☉");
        let d = Length::new::<astronomical_unit>(1.23);
        assert_eq!(d.astro_display(), "1.23 AU");
        let d = Length::new::<light_year>(1.23);
        assert_eq!(d.astro_display(), "1.23 lyr");
    }

    #[test]
    fn test_distance_negative_display() {
        let d = Length::new::<nanometer>(-1.23);
        assert_eq!(d.astro_display(), "-1.23 nm");
        let d: Length = Length::new::<meter>(-1.23);
        assert_eq!(d.astro_display(), "-1.23 m");
        let d: Length = Length::new::<kilometer>(-1.23);
        assert_eq!(d.astro_display(), "-1.23 km");
        let d: Length = Length::new::<earth_radius>(-1.23);
        assert_eq!(d.astro_display(), "-1.23 R🜨");
        let d: Length = Length::new::<solar_radius>(-1.23);
        assert_eq!(d.astro_display(), "-1.23 R☉");
        let d: Length = Length::new::<astronomical_unit>(-1.23);
        assert_eq!(d.astro_display(), "-1.23 AU");
        let d: Length = Length::new::<light_year>(-1.23);
        assert_eq!(d.astro_display(), "-1.23 lyr");
    }

    #[test]
    fn test_distance_display_thresholds() {
        let d = Length::new::<micrometer>(0.1);
        assert_eq!(d.astro_display(), "0.10 μm");
        let d = Length::new::<millimeter>(0.1);
        assert_eq!(d.astro_display(), "0.10 mm");
        let d = Length::new::<meter>(0.1);
        assert_eq!(d.astro_display(), "0.10 m");
        let d = Length::new::<kilometer>(0.1);
        assert_eq!(d.astro_display(), "0.10 km");
        let d = Length::new::<earth_radius>(0.1);
        assert_eq!(d.astro_display(), "0.10 R🜨");
        let d = Length::new::<solar_radius>(0.1);
        assert_eq!(d.astro_display(), "0.10 R☉");
        let d = Length::new::<astronomical_unit>(0.1);
        assert_eq!(d.astro_display(), "0.10 AU");
        let d = Length::new::<light_year>(0.1);
        assert_eq!(d.astro_display(), "0.10 lyr");
    }
}
