use crate::{
    color::sRGBColor,
    coordinates::{cartesian::CartesianCoordinates, direction::Direction},
    units::{length::Length, luminosity::Luminosity, mass::Mass, temperature::Temperature},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Star {
    pub(super) name: String,
    pub(super) mass: Mass,
    pub(super) radius: Length,
    pub(super) luminosity: Luminosity,
    pub(super) temperature: Temperature,
    pub(super) color: sRGBColor,
    pub(super) distance: Length,
    pub(super) direction_in_ecliptic: Direction,
}

impl Star {
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

    pub const fn get_direction_in_ecliptic(&self) -> &Direction {
        &self.direction_in_ecliptic
    }

    pub fn calculate_position(&self) -> CartesianCoordinates {
        self.direction_in_ecliptic.to_cartesian(self.distance)
    }
}
