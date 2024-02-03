use std::fmt::Display;

use crate::units::{
    mass::{Mass, *},
    time::{Time, *},
    Distance::{Length, *},
};

impl Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.au.abs() > 0.0099 * AU_PER_LIGHT_YEARS {
            write!(f, "{:.2} ly", self.as_light_years())
        } else if self.au.abs() > 0.0099 {
            write!(f, "{:.2} AU", self.as_astronomical_units())
        } else if self.au.abs() > 0.099 * AU_PER_SUN_RADII {
            write!(f, "{:.2} R☉", self.as_sun_radii())
        } else if self.au.abs() > 0.99 * AU_PER_EARTH_RADII {
            write!(f, "{:.2} R🜨", self.as_earth_radii())
        } else if self.au.abs() > 0.99 * AU_PER_KILOMETERS {
            write!(f, "{:.2} km", self.as_kilometers())
        } else if self.au.abs() > 1001. * AU_PER_NANOMETERS {
            write!(f, "{:.2} m", self.as_meters())
        } else {
            write!(f, "{:.2} nm", self.as_nanometers())
        }
    }
}

impl Display for Mass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.kilograms.abs() > 0.099 * KILOGRAMS_PER_SOLAR_MASS {
            write!(f, "{:.2} M☉", self.as_solar_masses())
        } else if self.kilograms.abs() > 0.0099 * KILOGRAMS_PER_EARTH_MASS {
            write!(f, "{:.2} M🜨", self.as_earth_masses())
        } else if self.kilograms.abs() > 0.99e-5 * KILOGRAMS_PER_EARTH_MASS {
            write!(f, "{:.5} M🜨", self.as_earth_masses())
        } else {
            write!(f, "{:.2} kg", self.kilograms)
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.seconds.abs() > 0.99 * SECONDS_PER_BILLION_YEARS {
            write!(f, "{:.2} Gyrs", self.as_billion_years())
        } else if self.seconds.abs() > 0.99 * SECONDS_PER_MILLION_YEARS {
            write!(f, "{:.2} Myrs", self.as_million_years())
        } else if self.seconds.abs() > 0.99 * SECONDS_PER_MILLENIUM {
            write!(f, "{:.2} kyr", self.as_thousand_years())
        } else if self.seconds.abs() > 0.99 * SECONDS_PER_YEAR {
            write!(f, "{:.2} yrs", self.as_years())
        } else if self.seconds.abs() > 0.99 * SECONDS_PER_DAY {
            write!(f, "{:.2} days", self.as_days())
        } else if self.seconds.abs() > 0.99 * SECONDS_PER_HOUR {
            write!(f, "{:.2} hrs", self.as_hours())
        } else if self.seconds.abs() > 0.99 * SECONDS_PER_MINUTE {
            write!(f, "{:.2} min", self.as_minutes())
        } else {
            write!(f, "{:.2} sec", self.as_seconds())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_display() {
        let nm = Distance::from_nanometers(1.23);
        assert_eq!(format!("{}", nm), "1.23 nm");
        let m = Distance::from_meters(1.23);
        assert_eq!(format!("{}", m), "1.23 m");
        let km = Distance::from_kilometers(1.23);
        assert_eq!(format!("{}", km), "1.23 km");
        let earth_radii = Distance::from_earth_radii(1.23);
        assert_eq!(format!("{}", earth_radii), "1.23 R🜨");
        let sun_radii = Distance::from_sun_radii(1.23);
        assert_eq!(format!("{}", sun_radii), "1.23 R☉");
        let astronomical_units = Distance::from_astronomical_units(1.23);
        assert_eq!(format!("{}", astronomical_units), "1.23 AU");
        let light_years = Distance::from_light_years(1.23);
        assert_eq!(format!("{}", light_years), "1.23 ly");
    }

    #[test]
    fn test_length_negative_display() {
        let nm = Distance::from_nanometers(-1.23);
        assert_eq!(format!("{}", nm), "-1.23 nm");
        let m = Distance::from_meters(-1.23);
        assert_eq!(format!("{}", m), "-1.23 m");
        let km = Distance::from_kilometers(-1.23);
        assert_eq!(format!("{}", km), "-1.23 km");
        let earth_radii = Distance::from_earth_radii(-1.23);
        assert_eq!(format!("{}", earth_radii), "-1.23 R🜨");
        let sun_radii = Distance::from_sun_radii(-1.23);
        assert_eq!(format!("{}", sun_radii), "-1.23 R☉");
        let astronomical_units = Distance::from_astronomical_units(-1.23);
        assert_eq!(format!("{}", astronomical_units), "-1.23 AU");
        let light_years = Distance::from_light_years(-1.23);
        assert_eq!(format!("{}", light_years), "-1.23 ly");
    }

    #[test]
    fn test_length_display_thresholds() {
        let nm = Distance::from_nanometers(1000.);
        assert_eq!(format!("{}", nm), "1000.00 nm");
        let m = Distance::from_meters(1.);
        assert_eq!(format!("{}", m), "1.00 m");
        let km = Distance::from_kilometers(1.);
        assert_eq!(format!("{}", km), "1.00 km");
        let earth_radii = Distance::from_earth_radii(1.);
        assert_eq!(format!("{}", earth_radii), "1.00 R🜨");
        let sun_radii = Distance::from_sun_radii(0.1);
        assert_eq!(format!("{}", sun_radii), "0.10 R☉");
        let astronomical_units = Distance::from_astronomical_units(0.01);
        assert_eq!(format!("{}", astronomical_units), "0.01 AU");
        let light_years = Distance::from_light_years(0.01);
        assert_eq!(format!("{}", light_years), "0.01 ly");
    }

    #[test]
    fn test_mass_display() {
        let mass = Mass::from_kilograms(1.23);
        assert_eq!(mass.to_string(), "1.23 kg");
        let mass = Mass::from_earth_masses(1.23);
        assert_eq!(mass.to_string(), "1.23 M🜨");
        let mass = Mass::from_solar_masses(1.23);
        assert_eq!(mass.to_string(), "1.23 M☉");
    }

    #[test]
    fn test_mass_negative_display() {
        let mass = Mass::from_kilograms(-1.23);
        assert_eq!(mass.to_string(), "-1.23 kg");
        let mass = Mass::from_earth_masses(-1.23);
        assert_eq!(mass.to_string(), "-1.23 M🜨");
        let mass = Mass::from_solar_masses(-1.23);
        assert_eq!(mass.to_string(), "-1.23 M☉");
    }

    #[test]
    fn test_mass_display_thresholds() {
        let mass = Mass::from_kilograms(1.);
        assert_eq!(mass.to_string(), "1.00 kg");
        let mass = Mass::from_earth_masses(1e-5);
        assert_eq!(mass.to_string(), "0.00001 M🜨");
        let mass = Mass::from_earth_masses(0.01);
        assert_eq!(mass.to_string(), "0.01 M🜨");
        let mass = Mass::from_solar_masses(0.1);
        assert_eq!(mass.to_string(), "0.10 M☉");
    }

    #[test]
    fn test_time_display() {
        let time = Time::from_seconds(1.);
        assert_eq!(format!("{}", time), "1.00 sec");
        let time = Time::from_minutes(1.);
        assert_eq!(format!("{}", time), "1.00 min");
        let time = Time::from_hours(1.);
        assert_eq!(format!("{}", time), "1.00 hrs");
        let time = Time::from_days(1.);
        assert_eq!(format!("{}", time), "1.00 days");
        let time = Time::from_years(1.);
        assert_eq!(format!("{}", time), "1.00 yrs");
        let time = Time::from_thousand_years(1.);
        assert_eq!(format!("{}", time), "1.00 kyr");
        let time = Time::from_million_years(1.);
        assert_eq!(format!("{}", time), "1.00 Myrs");
        let time = Time::from_billion_years(1.);
        assert_eq!(format!("{}", time), "1.00 Gyrs");
    }

    #[test]
    fn test_time_negative_display() {
        let time = Time::from_seconds(-1.);
        assert_eq!(format!("{}", time), "-1.00 sec");
        let time = Time::from_minutes(-1.);
        assert_eq!(format!("{}", time), "-1.00 min");
        let time = Time::from_hours(-1.);
        assert_eq!(format!("{}", time), "-1.00 hrs");
        let time = Time::from_days(-1.);
        assert_eq!(format!("{}", time), "-1.00 days");
        let time = Time::from_years(-1.);
        assert_eq!(format!("{}", time), "-1.00 yrs");
        let time = Time::from_thousand_years(-1.);
        assert_eq!(format!("{}", time), "-1.00 kyr");
        let time = Time::from_million_years(-1.);
        assert_eq!(format!("{}", time), "-1.00 Myrs");
        let time = Time::from_billion_years(-1.);
        assert_eq!(format!("{}", time), "-1.00 Gyrs");
    }
}
