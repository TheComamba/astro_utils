use crate::{
    color::sRGBColor,
    coordinates::{cartesian::CartesianCoordinates, direction::Direction},
    units::{
        length::Length, luminosity::Luminosity, mass::Mass, temperature::Temperature, time::Time,
    },
    Float,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Star {
    pub(super) name: String,
    pub(super) mass: Mass,
    pub(super) radius: Option<Length>,
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

    pub const fn get_radius(&self) -> Option<Length> {
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

    pub(super) fn apparently_the_same(&self, other: &Self) -> bool {
        const LUMINOSITY_ACCURACY: Float = 1.;
        const DIRECTION_ACCURACY: Float = 1e-5;

        let luminosity_difference = (self.luminosity.as_absolute_magnitude()
            - other.luminosity.as_absolute_magnitude())
        .abs();
        if luminosity_difference < LUMINOSITY_ACCURACY
            || self
                .direction_in_ecliptic
                .eq_within(&other.direction_in_ecliptic, DIRECTION_ACCURACY)
        {
            println!(
                "These two stars were deemed to appear the same: \n{:?}\n{:?}",
                self, other
            );
            true
        } else {
            false
        }
    }

    #[cfg(test)]
    pub(crate) fn similar_within_order_of_magnitude(&self, other: &Self) -> bool {
        let mass_ratio = self.mass / other.mass;
        let radius_ratio = match (self.radius, other.radius) {
            (Some(self_radius), Some(other_radius)) => self_radius / other_radius,
            _ => 1.0,
        };
        let luminosity_difference = (self.luminosity.as_absolute_magnitude()
            - other.luminosity.as_absolute_magnitude())
        .abs();
        let temperature_ratio = self.temperature / other.temperature;
        let age_ratio = match (self.age, other.age) {
            (Some(self_age), Some(other_age)) => self_age / other_age,
            _ => 1.0,
        };
        let mut result = true;
        if mass_ratio < 0.1 || mass_ratio > 10.0 {
            println!(
                "mass1: {}, mass2: {}, ratio: {}",
                self.mass, other.mass, mass_ratio
            );
            result = false;
        }
        if radius_ratio < 0.1 || radius_ratio > 10.0 {
            println!(
                "radius1: {}, radius2: {}, ratio: {}",
                self.radius.unwrap(),
                other.radius.unwrap(),
                radius_ratio
            );
            result = false;
        }
        if luminosity_difference > 1.0 {
            println!(
                "luminosity1: {}, luminosity2: {}, difference: {}",
                self.luminosity, other.luminosity, luminosity_difference
            );
            result = false;
        }
        if temperature_ratio < 0.1 || temperature_ratio > 10.0 {
            println!(
                "temperature1: {}, temperature2: {}, ratio: {}",
                self.temperature, other.temperature, temperature_ratio
            );
            result = false;
        }
        if age_ratio < 0.1 || age_ratio > 10.0 {
            println!(
                "age1: {}, age2: {}, ratio: {}",
                self.age.unwrap(),
                other.age.unwrap(),
                age_ratio
            );
            result = false;
        }
        result
    }
}
