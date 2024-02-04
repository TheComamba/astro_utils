use simple_si_units::base::Distance;

pub const DISTANCE_ZERO: Distance<f64> = Distance { m: 0. };
pub(crate) const METERS_PER_NANOMETER: f64 = 1e-9;
pub(crate) const METERS_PER_MICROMETER: f64 = 1e-6;
pub(crate) const METERS_PER_MILLIMETER: f64 = 1e-3;
pub(crate) const METERS_PER_KILOMETER: f64 = 1e3;
pub(crate) const METERS_PER_EARTH_RADII: f64 = 6.371e6;
pub(crate) const METERS_PER_SUN_RADII: f64 = 6.957e8;
pub(crate) const METERS_PER_ASTRONOMICAL_UNIT: f64 = 1.496e11;
pub(crate) const METERS_PER_LIGHT_YEAR: f64 = 9.461e15;

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

pub fn display_distance_in_units(distance: Distance<f64>, units: DistanceUnit) -> String {
    match units {
        DistanceUnit::Nanometers => format!("{:2} nm", distance.m / METERS_PER_NANOMETER),
        DistanceUnit::Micrometers => format!("{:2} μm", distance.m / METERS_PER_MICROMETER),
        DistanceUnit::Millimeters => format!("{:2} mm", distance.m / METERS_PER_MILLIMETER),
        DistanceUnit::Meters => format!("{:2} m", distance.m),
        DistanceUnit::Kilometers => format!("{:2} km", distance.m / METERS_PER_KILOMETER),
        DistanceUnit::EarthRadii => format!("{:2} R🜨", distance.m / METERS_PER_EARTH_RADII),
        DistanceUnit::SunRadii => format!("{:2} R☉", distance.m / METERS_PER_SUN_RADII),
        DistanceUnit::AstronomicalUnits => {
            format!("{:2} AU", distance.m / METERS_PER_ASTRONOMICAL_UNIT)
        }
        DistanceUnit::LightYears => format!("{:2} ly", distance.m / METERS_PER_LIGHT_YEAR),
    }
}

pub fn diplay_distance(distance: Distance<f64>) -> String {
    let units = if distance.to_lyr().abs() > 0.099 {
        DistanceUnit::LightYears
    } else if distance.to_au().abs() > 0.099 {
        DistanceUnit::AstronomicalUnits
    } else if distance.to_sun_radii().abs() > 0.099 {
        DistanceUnit::SunRadii
    } else if distance.to_earth_radii().abs() > 0.099 {
        DistanceUnit::EarthRadii
    } else if distance.to_km().abs() > 0.099 {
        DistanceUnit::Kilometers
    } else if distance.to_meters().abs() > 0.099 {
        DistanceUnit::Meters
    } else if distance.to_millimeters().abs() > 0.099 {
        DistanceUnit::Millimeters
    } else if distance.to_micrometers().abs() > 0.099 {
        DistanceUnit::Micrometers
    } else {
        DistanceUnit::Nanometers
    };
    display_distance_in_units(distance, units)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_display() {
        let nm = Distance::from_nm(1.23);
        assert_eq!(format!("{}", nm), "1.23 nm");
        let m = Distance::from_meters(1.23);
        assert_eq!(format!("{}", m), "1.23 m");
        let km = Distance::from_km(1.23);
        assert_eq!(format!("{}", km), "1.23 km");
        let earth_radii = Distance::from_earth_radii(1.23);
        assert_eq!(format!("{}", earth_radii), "1.23 R🜨");
        let sun_radii = Distance::from_sun_radii(1.23);
        assert_eq!(format!("{}", sun_radii), "1.23 R☉");
        let astronomical_units = Distance::from_astronomical_units(1.23);
        assert_eq!(format!("{}", astronomical_units), "1.23 AU");
        let light_years = Distance::from_lyr(1.23);
        assert_eq!(format!("{}", light_years), "1.23 ly");
    }

    #[test]
    fn test_length_negative_display() {
        let nm = Distance::from_nm(-1.23);
        assert_eq!(format!("{}", nm), "-1.23 nm");
        let m = Distance::from_meters(-1.23);
        assert_eq!(format!("{}", m), "-1.23 m");
        let km = Distance::from_km(-1.23);
        assert_eq!(format!("{}", km), "-1.23 km");
        let earth_radii = Distance::from_earth_radii(-1.23);
        assert_eq!(format!("{}", earth_radii), "-1.23 R🜨");
        let sun_radii = Distance::from_sun_radii(-1.23);
        assert_eq!(format!("{}", sun_radii), "-1.23 R☉");
        let astronomical_units = Distance::from_astronomical_units(-1.23);
        assert_eq!(format!("{}", astronomical_units), "-1.23 AU");
        let light_years = Distance::from_lyr(-1.23);
        assert_eq!(format!("{}", light_years), "-1.23 ly");
    }

    #[test]
    fn test_length_display_thresholds() {
        let nm = Distance::from_nm(1000.);
        assert_eq!(format!("{}", nm), "1000.00 nm");
        let m = Distance::from_meters(1.);
        assert_eq!(format!("{}", m), "1.00 m");
        let km = Distance::from_km(1.);
        assert_eq!(format!("{}", km), "1.00 km");
        let earth_radii = Distance::from_earth_radii(1.);
        assert_eq!(format!("{}", earth_radii), "1.00 R🜨");
        let sun_radii = Distance::from_sun_radii(0.1);
        assert_eq!(format!("{}", sun_radii), "0.10 R☉");
        let astronomical_units = Distance::from_astronomical_units(0.01);
        assert_eq!(format!("{}", astronomical_units), "0.01 AU");
        let light_years = Distance::from_lyr(0.01);
        assert_eq!(format!("{}", light_years), "0.01 ly");
    }
}
