use crate::{
    color::sRGBColor,
    coordinates::{cartesian::CartesianCoordinates, direction::Direction},
    units::{
        length::Length, luminosity::Luminosity, mass::Mass, temperature::Temperature, time::Time,
    },
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
    pub(super) age: Option<Time>,
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

    pub const fn get_age(&self) -> &Option<Time> {
        &self.age
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

    #[cfg(test)]
    pub(crate) fn similar_within_order_of_magnitude(&self, other: &Self) -> bool {
        let mass_ratio = self.mass / other.mass;
        let radius_ratio = self.radius / other.radius;
        let luminosity_difference = (self.luminosity.as_absolute_magnitude()
            - other.luminosity.as_absolute_magnitude())
        .abs();
        let temperature_ratio = self.temperature / other.temperature;
        let age_ratio = match (self.age, other.age) {
            (Some(self_age), Some(other_age)) => self_age / other_age,
            _ => 1.0,
        };
        if mass_ratio < 0.1 || mass_ratio > 10.0 {
            println!("mass_ratio: {}", mass_ratio);
            false
        } else if radius_ratio < 0.1 || radius_ratio > 10.0 {
            println!("radius_ratio: {}", radius_ratio);
            false
        } else if luminosity_difference > 1.0 {
            println!("luminosity_difference: {}", luminosity_difference);
            false
        } else if temperature_ratio < 0.1 || temperature_ratio > 10.0 {
            println!("temperature_ratio: {}", temperature_ratio);
            false
        } else if age_ratio < 0.1 || age_ratio > 10.0 {
            println!("age_ratio: {}", age_ratio);
            false
        } else {
            true
        }
    }
}
