pub const DISTANCE_ZERO: Distance<Float> = Distance { m: 0. };
pub const METERS_PER_SUN_RADII: Float = 6.957e8;
pub const METERS_PER_ASTRONOMICAL_UNIT: Float = 1.496e11;
pub const METERS_PER_LIGHT_YEAR: Float = 9.461e15;

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
}
