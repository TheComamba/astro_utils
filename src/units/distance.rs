use simple_si_units::base::Distance;

pub const DISTANCE_ZERO: Distance<f64> = Distance { m: 0. };

pub const EARTH_RADIUS: Distance<f64> = Distance { m: 6.371e6 };
pub const SOLAR_RADIUS: Distance<f64> = Distance { m: 6.957e8 };
pub const ASTRONOMICAL_UNIT: Distance<f64> = Distance { m: 1.496e11 };
pub const LIGHT_YEAR: Distance<f64> = Distance { m: 9.461e15 };

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

pub(crate) fn distance_to_earth_radii(distance: Distance<f64>) -> f64 {
    distance / EARTH_RADIUS
}

pub(crate) fn distance_to_sun_radii(distance: Distance<f64>) -> f64 {
    distance / SOLAR_RADIUS
}

pub fn display_distance_in_units(distance: Distance<f64>, units: DistanceUnit) -> String {
    match units {
        DistanceUnit::Nanometers => format!("{:2} nm", distance.to_nm()),
        DistanceUnit::Micrometers => format!("{:2} μm", distance.to_um()),
        DistanceUnit::Millimeters => format!("{:2} mm", distance.to_mm()),
        DistanceUnit::Meters => format!("{:2} m", distance.m),
        DistanceUnit::Kilometers => format!("{:2} km", distance.to_km()),
        DistanceUnit::EarthRadii => format!("{:2} R🜨", distance / EARTH_RADIUS),
        DistanceUnit::SunRadii => format!("{:2} R☉", distance / SOLAR_RADIUS),
        DistanceUnit::AstronomicalUnits => format!("{:2} AU", distance.to_au()),
        DistanceUnit::LightYears => format!("{:2} lyr", distance.to_lyr()),
    }
}

pub fn diplay_distance(distance: Distance<f64>) -> String {
    let units = if distance.to_lyr().abs() > 0.099 {
        DistanceUnit::LightYears
    } else if distance.to_au().abs() > 0.099 {
        DistanceUnit::AstronomicalUnits
    } else if distance_to_sun_radii(distance).abs() > 0.099 {
        DistanceUnit::SunRadii
    } else if distance_to_earth_radii(distance).abs() > 0.099 {
        DistanceUnit::EarthRadii
    } else if distance.to_km().abs() > 0.099 {
        DistanceUnit::Kilometers
    } else if distance.to_meters().abs() > 0.099 {
        DistanceUnit::Meters
    } else if distance.to_mm().abs() > 0.099 {
        DistanceUnit::Millimeters
    } else if distance.to_um().abs() > 0.099 {
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
        let earth_radii = 1.23 * EARTH_RADIUS;
        assert_eq!(format!("{}", earth_radii), "1.23 R🜨");
        let sun_radii = 1.23 * SOLAR_RADIUS;
        assert_eq!(format!("{}", sun_radii), "1.23 R☉");
        let astronomical_units = Distance::from_au(1.23);
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
        let earth_radii = -1.23 * EARTH_RADIUS;
        assert_eq!(format!("{}", earth_radii), "-1.23 R🜨");
        let sun_radii = -1.23 * SOLAR_RADIUS;
        assert_eq!(format!("{}", sun_radii), "-1.23 R☉");
        let astronomical_units = Distance::from_au(-1.23);
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
        let earth_radii = EARTH_RADIUS;
        assert_eq!(format!("{}", earth_radii), "1.00 R🜨");
        let sun_radii = 0.1 * SOLAR_RADIUS;
        assert_eq!(format!("{}", sun_radii), "0.10 R☉");
        let astronomical_units = Distance::from_au(0.01);
        assert_eq!(format!("{}", astronomical_units), "0.01 AU");
        let light_years = Distance::from_lyr(0.01);
        assert_eq!(format!("{}", light_years), "0.01 ly");
    }
}
