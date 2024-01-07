use crate::{
    color::sRGBColor,
    coordinates::{
        declination::Declination, earth_equatorial::EarthEquatorialCoordinates,
        right_ascension::RightAscension,
    },
    units::{
        length::Length, luminosity::Luminosity, mass::Mass, temperature::Temperature, time::Time,
    },
};

use super::star::Star;

pub struct RealData {
    pub name: &'static str,
    pub mass: Mass,
    pub radius: Length,
    pub luminosity: Luminosity,
    pub temperature: Temperature,
    pub age: Option<Time>,
    pub right_ascension: RightAscension,
    pub declination: Declination,
    pub distance: Length,
}

impl RealData {
    pub fn to_star(&self) -> Star {
        let ra = self.right_ascension.to_angle();
        let dec = self.declination.to_angle();
        let dir = EarthEquatorialCoordinates::new(ra, dec).to_direction();
        Star {
            name: self.name.to_string(),
            mass: self.mass,
            radius: self.radius,
            luminosity: self.luminosity,
            temperature: self.temperature,
            color: sRGBColor::from_temperature(self.temperature),
            distance: self.distance,
            direction_in_ecliptic: dir,
        }
    }
}
