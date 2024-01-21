use super::{star_appearance::StarAppearance, star_data::StarData};
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

pub struct RealData {
    pub name: &'static str,
    pub mass: Option<Mass>,
    pub radius: Option<Length>,
    pub luminosity: Luminosity,
    pub temperature: Option<Temperature>,
    pub age: Option<Time>,
    pub right_ascension: RightAscension,
    pub declination: Declination,
    pub distance: Length,
}

impl RealData {
    pub fn to_star_data(&self) -> StarData {
        let ra = self.right_ascension.to_angle();
        let dec = self.declination.to_angle();
        let direction_in_ecliptic = EarthEquatorialCoordinates::new(ra, dec).to_direction();
        StarData {
            name: self.name.to_string(),
            mass: self.mass,
            radius: self.radius,
            luminosity: Some(self.luminosity),
            temperature: self.temperature,
            age: self.age,
            distance: Some(self.distance),
            direction_in_ecliptic,
        }
    }

    pub fn to_star_appearance(&self) -> StarAppearance {
        let ra = self.right_ascension.to_angle();
        let dec = self.declination.to_angle();
        let direction_in_ecliptic = EarthEquatorialCoordinates::new(ra, dec).to_direction();
        let illuminance = self.luminosity.to_illuminance(&self.distance);
        let color = match self.temperature {
            Some(temperature) => sRGBColor::from_temperature(temperature),
            None => sRGBColor::DEFAULT,
        };
        StarAppearance {
            name: self.name.to_string(),
            illuminance,
            color,
            direction_in_ecliptic,
        }
    }
}
