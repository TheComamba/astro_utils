use crate::Float;
use simple_si_units::base::{Distance, Mass, Time};

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

pub fn display_distance_in_units(distance: Distance<Float>, units: DistanceUnit) -> String {
    match units {
        DistanceUnit::Nanometers => format!("{:2} nm", distance.to_nanometers()),
        DistanceUnit::Micrometers => format!("{:2} μm", distance.to_micrometers()),
        DistanceUnit::Millimeters => format!("{:2} mm", distance.to_millimeters()),
        DistanceUnit::Meters => format!("{:2} m", distance.to_meters()),
        DistanceUnit::Kilometers => format!("{:2} km", distance.to_kilometers()),
        DistanceUnit::EarthRadii => format!("{:2} R🜨", distance.to_earth_radii()),
        DistanceUnit::SunRadii => format!("{:2} R☉", distance.to_sun_radii()),
        DistanceUnit::AstronomicalUnits => format!("{:2} AU", distance.to_astronomical_units()),
        DistanceUnit::LightYears => format!("{:2} ly", distance.to_light_years()),
    }
}

pub fn diplay_distance(distance: Distance<Float>) -> String {
    let units = if distance.as_light_years().abs() > 0.099 {
        DistanceUnit::LightYears
    } else if distance.as_astronomical_units().abs() > 0.099 {
        DistanceUnit::AstronomicalUnits
    } else if distance.as_sun_radii().abs() > 0.099 {
        DistanceUnit::SunRadii
    } else if distance.as_earth_radii().abs() > 0.099 {
        DistanceUnit::EarthRadii
    } else if distance.as_kilometers().abs() > 0.099 {
        DistanceUnit::Kilometers
    } else if distance.as_meters().abs() > 0.099 {
        DistanceUnit::Meters
    } else if distance.as_millimeters().abs() > 0.099 {
        DistanceUnit::Millimeters
    } else if distance.as_micrometers().abs() > 0.099 {
        DistanceUnit::Micrometers
    } else {
        DistanceUnit::Nanometers
    };
    display_distance_in_units(distance, units)
}

pub enum MassUnit {
    Kilograms,
    EarthMasses,
    SolarMasses,
}

pub fn display_mass_in_units(mass: Mass<Float>, units: MassUnit) -> String {
    match units {
        MassUnit::Kilograms => format!("{:2} kg", mass.to_kilograms()),
        MassUnit::EarthMasses => format!("{:2} M🜨", mass.to_earth_masses()),
        MassUnit::SolarMasses => format!("{:2} M☉", mass.to_solar_masses()),
    }
}

pub fn display_mass(mass: Mass<Float>) -> String {
    let units = if mass.as_solar_masses().abs() > 0.099 {
        MassUnit::SolarMasses
    } else if mass.as_earth_masses().abs() > 0.099 {
        MassUnit::EarthMasses
    } else {
        MassUnit::Kilograms
    };
    display_mass_in_units(mass, units)
}

pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Years,
    ThousandYears,
    MillionYears,
    BillionYears,
}

pub fn display_time_in_units(time: Time<Float>, units: TimeUnit) -> String {
    match units {
        TimeUnit::Seconds => format!("{:2} sec", time.to_seconds()),
        TimeUnit::Minutes => format!("{:2} min", time.to_minutes()),
        TimeUnit::Hours => format!("{:2} hrs", time.to_hours()),
        TimeUnit::Days => format!("{:2} days", time.to_days()),
        TimeUnit::Years => format!("{:2} yrs", time.to_years()),
        TimeUnit::ThousandYears => format!("{:2} kyr", time.to_thousand_years()),
        TimeUnit::MillionYears => format!("{:2} Myrs", time.to_million_years()),
        TimeUnit::BillionYears => format!("{:2} Gyrs", time.to_billion_years()),
    }
}

pub fn display_time(time: Time<Float>) -> String {
    let units = if time.as_billion_years().abs() > 0.099 {
        TimeUnit::BillionYears
    } else if time.as_million_years().abs() > 0.099 {
        TimeUnit::MillionYears
    } else if time.as_thousand_years().abs() > 0.099 {
        TimeUnit::ThousandYears
    } else if time.as_years().abs() > 0.099 {
        TimeUnit::Years
    } else if time.as_days().abs() > 0.099 {
        TimeUnit::Days
    } else if time.as_hours().abs() > 0.099 {
        TimeUnit::Hours
    } else if time.as_minutes().abs() > 0.099 {
        TimeUnit::Minutes
    } else {
        TimeUnit::Seconds
    };
    display_time_in_units(time, units)
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
