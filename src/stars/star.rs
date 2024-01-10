use crate::{
    color::sRGBColor,
    coordinates::direction::Direction,
    units::{
        illuminance::Illuminance, length::Length, luminosity::Luminosity, mass::Mass,
        temperature::Temperature, time::Time,
    },
    Float,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarAppearance {
    pub(super) name: String,
    pub(super) illuminance: Illuminance,
    pub(super) color: sRGBColor,
    pub(super) direction_in_ecliptic: Direction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarData {
    pub(super) name: String,
    pub(super) mass: Option<Mass>,
    pub(super) radius: Option<Length>,
    pub(super) luminosity: Option<Luminosity>,
    pub(super) temperature: Option<Temperature>,
    pub(super) age: Option<Time>,
    pub(super) distance: Option<Length>,
    pub(super) direction_in_ecliptic: Direction,
}

impl StarAppearance {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub const fn get_color(&self) -> &sRGBColor {
        &self.color
    }

    pub const fn get_direction_in_ecliptic(&self) -> &Direction {
        &self.direction_in_ecliptic
    }

    pub(super) fn apparently_the_same(&self, other: &Self) -> bool {
        const DIRECTION_ACCURACY: Float = 1e-5;

        let illuminance_ration = self.illuminance.as_lux() / other.illuminance.as_lux();
        illuminance_ration < 0.1
            || illuminance_ration > 10.0
                && self
                    .direction_in_ecliptic
                    .eq_within(&other.direction_in_ecliptic, DIRECTION_ACCURACY)
    }
}

impl StarData {
    pub const fn get_radius(&self) -> Option<Length> {
        self.radius
    }

    pub const fn get_mass(&self) -> &Option<Mass> {
        &self.mass
    }

    pub const fn get_absolute_magnitude(&self) -> &Option<Luminosity> {
        &self.luminosity
    }

    pub const fn get_temperature(&self) -> &Option<Temperature> {
        &self.temperature
    }

    pub const fn get_age(&self) -> &Option<Time> {
        &self.age
    }

    pub const fn get_distance(&self) -> &Option<Length> {
        &self.distance
    }

    #[cfg(test)]
    pub(crate) fn similar_within_order_of_magnitude(&self, other: &Self) -> bool {
        let mass_ratio = match (self.mass, other.mass) {
            (Some(self_mass), Some(other_mass)) => self_mass / other_mass,
            _ => 1.0,
        };
        let radius_ratio = match (self.radius, other.radius) {
            (Some(self_radius), Some(other_radius)) => self_radius / other_radius,
            _ => 1.0,
        };
        let luminosity_difference = match (self.luminosity, other.luminosity) {
            (Some(self_luminosity), Some(other_luminosity)) => {
                (self_luminosity.as_absolute_magnitude() - other_luminosity.as_absolute_magnitude())
                    .abs()
            }
            _ => 0.0,
        };
        let temperature_ratio = match (self.temperature, other.temperature) {
            (Some(self_temperature), Some(other_temperature)) => {
                self_temperature / other_temperature
            }
            _ => 1.0,
        };
        let age_ratio = match (self.age, other.age) {
            (Some(self_age), Some(other_age)) => self_age / other_age,
            _ => 1.0,
        };
        let mut result = true;
        if mass_ratio < 0.1 || mass_ratio > 10.0 {
            println!(
                "mass1: {}, mass2: {}, ratio: {}",
                self.mass.unwrap(),
                other.mass.unwrap(),
                mass_ratio
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
                self.luminosity.unwrap(),
                other.luminosity.unwrap(),
                luminosity_difference
            );
            result = false;
        }
        if temperature_ratio < 0.1 || temperature_ratio > 10.0 {
            println!(
                "temperature1: {}, temperature2: {}, ratio: {}",
                self.temperature.unwrap(),
                other.temperature.unwrap(),
                temperature_ratio
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
