use uom::si::f64::Length;

use crate::astro_display::AstroDisplay;

use super::DISPLAY_THRESHOLD;

pub const DISTANCE_ZERO: Length = Length { m: 0. };
pub const EARTH_RADIUS: Length = Length { m: 6.371e6 };
pub const SOLAR_RADIUS: Length = Length { m: 6.957e8 };
pub const ASTRONOMICAL_UNIT: Length = Length { m: 1.496e11 };
pub const LIGHT_YEAR: Length = Length { m: 9.461e15 };

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

pub fn distance_to_earth_radii(distance: &Length) -> f64 {
    distance / &EARTH_RADIUS
}

pub fn distance_to_sun_radii(distance: &Length) -> f64 {
    distance / &SOLAR_RADIUS
}

pub fn display_distance_in_units(distance: &Length, units: LengthUnit) -> String {
    match units {
        LengthUnit::Nanometers => format!("{:.2} nm", distance.get::<nanometer>()),
        LengthUnit::Micrometers => format!("{:.2} μm", distance.to_um()),
        LengthUnit::Millimeters => format!("{:.2} mm", distance.to_mm()),
        LengthUnit::Meters => format!("{:.2} m", distance.m),
        LengthUnit::Kilometers => format!("{:.2} km", distance.to_km()),
        LengthUnit::EarthRadii => format!("{:.2} R🜨", distance_to_earth_radii(distance)),
        LengthUnit::SunRadii => format!("{:.2} R☉", distance_to_sun_radii(distance)),
        LengthUnit::AstronomicalUnits => format!("{:.2} AU", distance.to_au()),
        LengthUnit::LightYears => format!("{:.2} lyr", distance.to_lyr()),
    }
}

impl AstroDisplay for Length {
    fn astro_display(&self) -> String {
        let units = if self.to_lyr().abs() > DISPLAY_THRESHOLD {
            LengthUnit::LightYears
        } else if self.to_au().abs() > DISPLAY_THRESHOLD {
            LengthUnit::AstronomicalUnits
        } else if distance_to_sun_radii(self).abs() > DISPLAY_THRESHOLD {
            LengthUnit::SunRadii
        } else if distance_to_earth_radii(self).abs() > DISPLAY_THRESHOLD {
            LengthUnit::EarthRadii
        } else if self.to_km().abs() > DISPLAY_THRESHOLD {
            LengthUnit::Kilometers
        } else if self.get::<meter>().abs() > DISPLAY_THRESHOLD {
            LengthUnit::Meters
        } else if self.to_mm().abs() > DISPLAY_THRESHOLD {
            LengthUnit::Millimeters
        } else if self.to_um().abs() > DISPLAY_THRESHOLD {
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
        let d = Length::from_meters(1.23);
        assert_eq!(d.astro_display(), "1.23 m");
        let d = Length::new::<kilometer>(1.23);
        assert_eq!(d.astro_display(), "1.23 km");
        let d = 1.23 as f64 * EARTH_RADIUS;
        assert_eq!(d.astro_display(), "1.23 R🜨");
        let d = 1.23 as f64 * SOLAR_RADIUS;
        assert_eq!(d.astro_display(), "1.23 R☉");
        let d = Length::from_au(1.23);
        assert_eq!(d.astro_display(), "1.23 AU");
        let d = Length::from_lyr(1.23);
        assert_eq!(d.astro_display(), "1.23 lyr");
    }

    #[test]
    fn test_distance_negative_display() {
        let d = Length::new::<nanometer>(-1.23);
        assert_eq!(d.astro_display(), "-1.23 nm");
        let d: Length = Length::from_meters(-1.23);
        assert_eq!(d.astro_display(), "-1.23 m");
        let d: Length = Length::new::<kilometer>(-1.23);
        assert_eq!(d.astro_display(), "-1.23 km");
        let d = -1.23 as f64 * EARTH_RADIUS;
        assert_eq!(d.astro_display(), "-1.23 R🜨");
        let d = -1.23 as f64 * SOLAR_RADIUS;
        assert_eq!(d.astro_display(), "-1.23 R☉");
        let d = Length::from_au(-1.23);
        assert_eq!(d.astro_display(), "-1.23 AU");
        let d = Length::from_lyr(-1.23);
        assert_eq!(d.astro_display(), "-1.23 lyr");
    }

    #[test]
    fn test_distance_display_thresholds() {
        let d = Length::from_um(0.1);
        assert_eq!(d.astro_display(), "0.10 μm");
        let d = Length::from_mm(0.1);
        assert_eq!(d.astro_display(), "0.10 mm");
        let d = Length::from_meters(0.1);
        assert_eq!(d.astro_display(), "0.10 m");
        let d = Length::new::<kilometer>(0.1);
        assert_eq!(d.astro_display(), "0.10 km");
        let d = 0.1 as f64 * EARTH_RADIUS;
        assert_eq!(d.astro_display(), "0.10 R🜨");
        let d = 0.1 as f64 * SOLAR_RADIUS;
        assert_eq!(d.astro_display(), "0.10 R☉");
        let d = Length::from_au(0.1);
        assert_eq!(d.astro_display(), "0.10 AU");
        let d = Length::from_lyr(0.1);
        assert_eq!(d.astro_display(), "0.10 lyr");
    }
}
