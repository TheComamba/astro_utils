use crate::astro_display::AstroDisplay;

use super::DISPLAY_THRESHOLD;
use simple_si_units::base::Distance;

pub const DISTANCE_ZERO: Length = Distance { m: 0. };
pub const EARTH_RADIUS: Length = Distance { m: 6.371e6 };
pub const SOLAR_RADIUS: Length = Distance { m: 6.957e8 };
pub const ASTRONOMICAL_UNIT: Length = Distance { m: 1.496e11 };
pub const LIGHT_YEAR: Length = Distance { m: 9.461e15 };

pub enum DistanceUnit {
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

pub fn display_distance_in_units(distance: &Length, units: DistanceUnit) -> String {
    match units {
        DistanceUnit::Nanometers => format!("{:.2} nm", distance.to_nm()),
        DistanceUnit::Micrometers => format!("{:.2} μm", distance.to_um()),
        DistanceUnit::Millimeters => format!("{:.2} mm", distance.to_mm()),
        DistanceUnit::Meters => format!("{:.2} m", distance.m),
        DistanceUnit::Kilometers => format!("{:.2} km", distance.to_km()),
        DistanceUnit::EarthRadii => format!("{:.2} R🜨", distance_to_earth_radii(distance)),
        DistanceUnit::SunRadii => format!("{:.2} R☉", distance_to_sun_radii(distance)),
        DistanceUnit::AstronomicalUnits => format!("{:.2} AU", distance.to_au()),
        DistanceUnit::LightYears => format!("{:.2} lyr", distance.to_lyr()),
    }
}

impl AstroDisplay for Length {
    fn astro_display(&self) -> String {
        let units = if self.to_lyr().abs() > DISPLAY_THRESHOLD {
            DistanceUnit::LightYears
        } else if self.to_au().abs() > DISPLAY_THRESHOLD {
            DistanceUnit::AstronomicalUnits
        } else if distance_to_sun_radii(self).abs() > DISPLAY_THRESHOLD {
            DistanceUnit::SunRadii
        } else if distance_to_earth_radii(self).abs() > DISPLAY_THRESHOLD {
            DistanceUnit::EarthRadii
        } else if self.to_km().abs() > DISPLAY_THRESHOLD {
            DistanceUnit::Kilometers
        } else if self.to_meters().abs() > DISPLAY_THRESHOLD {
            DistanceUnit::Meters
        } else if self.to_mm().abs() > DISPLAY_THRESHOLD {
            DistanceUnit::Millimeters
        } else if self.to_um().abs() > DISPLAY_THRESHOLD {
            DistanceUnit::Micrometers
        } else {
            DistanceUnit::Nanometers
        };
        display_distance_in_units(self, units)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_display() {
        let d = Distance::from_nm(1.23);
        assert_eq!(d.astro_display(), "1.23 nm");
        let d = Distance::from_meters(1.23);
        assert_eq!(d.astro_display(), "1.23 m");
        let d = Distance::from_km(1.23);
        assert_eq!(d.astro_display(), "1.23 km");
        let d = 1.23 as f64 * EARTH_RADIUS;
        assert_eq!(d.astro_display(), "1.23 R🜨");
        let d = 1.23 as f64 * SOLAR_RADIUS;
        assert_eq!(d.astro_display(), "1.23 R☉");
        let d = Distance::from_au(1.23);
        assert_eq!(d.astro_display(), "1.23 AU");
        let d = Distance::from_lyr(1.23);
        assert_eq!(d.astro_display(), "1.23 lyr");
    }

    #[test]
    fn test_distance_negative_display() {
        let d = Distance::from_nm(-1.23);
        assert_eq!(d.astro_display(), "-1.23 nm");
        let d: Length = Distance::from_meters(-1.23);
        assert_eq!(d.astro_display(), "-1.23 m");
        let d: Length = Distance::from_km(-1.23);
        assert_eq!(d.astro_display(), "-1.23 km");
        let d = -1.23 as f64 * EARTH_RADIUS;
        assert_eq!(d.astro_display(), "-1.23 R🜨");
        let d = -1.23 as f64 * SOLAR_RADIUS;
        assert_eq!(d.astro_display(), "-1.23 R☉");
        let d = Distance::from_au(-1.23);
        assert_eq!(d.astro_display(), "-1.23 AU");
        let d = Distance::from_lyr(-1.23);
        assert_eq!(d.astro_display(), "-1.23 lyr");
    }

    #[test]
    fn test_distance_display_thresholds() {
        let d = Distance::from_um(0.1);
        assert_eq!(d.astro_display(), "0.10 μm");
        let d = Distance::from_mm(0.1);
        assert_eq!(d.astro_display(), "0.10 mm");
        let d = Distance::from_meters(0.1);
        assert_eq!(d.astro_display(), "0.10 m");
        let d = Distance::from_km(0.1);
        assert_eq!(d.astro_display(), "0.10 km");
        let d = 0.1 as f64 * EARTH_RADIUS;
        assert_eq!(d.astro_display(), "0.10 R🜨");
        let d = 0.1 as f64 * SOLAR_RADIUS;
        assert_eq!(d.astro_display(), "0.10 R☉");
        let d = Distance::from_au(0.1);
        assert_eq!(d.astro_display(), "0.10 AU");
        let d = Distance::from_lyr(0.1);
        assert_eq!(d.astro_display(), "0.10 lyr");
    }
}
