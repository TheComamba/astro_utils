use crate::{
    color::sRGBColor,
    coordinates::{
        declination::Declination, earth_equatorial::EarthEquatorialCoordinates,
        right_ascension::RightAscension,
    },
    units::{length::Length, luminosity::Luminosity, mass::Mass, temperature::Temperature},
};

use super::star::Star;

pub struct RealData {
    pub name: &'static str,
    pub mass: Mass,
    pub radius: Length,
    pub luminosity: Luminosity,
    pub temperature: Temperature,
    pub right_ascension: RightAscension,
    pub declination: Declination,
    pub distance: Length,
}

impl RealData {
    pub fn to_star(data: RealData) -> Star {
        let ra = data.right_ascension.to_angle();
        let dec = data.declination.to_angle();
        let dir = EarthEquatorialCoordinates::new(ra, dec).to_direction();
        Star {
            name: data.name.to_string(),
            mass: data.mass,
            radius: data.radius,
            luminosity: data.luminosity,
            temperature: data.temperature,
            color: sRGBColor::from_temperature(data.temperature),
            distance: data.distance,
            direction_in_ecliptic: dir,
        }
    }
}
