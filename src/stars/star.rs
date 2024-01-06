use super::real_data::RealData;
use crate::{
    color::sRGBColor,
    coordinates::{
        cartesian::CartesianCoordinates, direction::Direction,
        earth_equatorial::EarthEquatorialCoordinates,
    },
    units::{length::Length, luminosity::Luminosity, mass::Mass, temperature::Temperature},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Star {
    name: String,
    mass: Mass,
    radius: Length,
    luminosity: Luminosity,
    temperature: Temperature,
    color: sRGBColor,
    distance: Length,
    direction_in_ecliptic: Direction,
}

impl Star {
    pub fn from_data(data: RealData) -> Star {
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub const fn get_radius(&self) -> Length {
        self.radius
    }

    pub const fn get_mass(&self) -> Mass {
        self.mass
    }

    pub const fn get_absolute_magnitude(&self) -> Luminosity {
        self.luminosity
    }

    pub const fn get_temperature(&self) -> Temperature {
        self.temperature
    }

    pub const fn get_color(&self) -> &sRGBColor {
        &self.color
    }

    pub const fn get_distance(&self) -> Length {
        self.distance
    }

    pub fn calculate_position(&self) -> CartesianCoordinates {
        self.direction_in_ecliptic.to_cartesian(self.distance)
    }
}
