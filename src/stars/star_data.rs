use super::star_appearance::StarAppearance;
use crate::{color::sRGBColor, coordinates::direction::Direction, units::ILLUMINANCE_ZERO, Float};
use serde::{Deserialize, Serialize};
use simple_si_units::base::{Distance, Luminosity, Mass, Temperature, Time};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarData {
    pub(super) name: String,
    pub(super) mass: Option<Mass<Float>>,
    pub(super) radius: Option<Distance<Float>>,
    pub(super) luminosity: Option<Luminosity<Float>>,
    pub(super) temperature: Option<Temperature<Float>>,
    pub(super) age: Option<Time<Float>>,
    pub(super) distance: Option<Distance<Float>>,
    pub(super) direction_in_ecliptic: Direction,
}

impl StarData {
    pub fn new(
        name: String,
        mass: Option<Mass<Float>>,
        radius: Option<Distance<Float>>,
        luminosity: Option<Luminosity<Float>>,
        temperature: Option<Temperature<Float>>,
        age: Option<Time<Float>>,
        distance: Option<Distance<Float>>,
        direction_in_ecliptic: Direction,
    ) -> Self {
        Self {
            name,
            mass,
            radius,
            luminosity,
            temperature,
            age,
            distance,
            direction_in_ecliptic,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub const fn get_radius(&self) -> &Option<Distance<Float>> {
        &self.radius
    }

    pub const fn get_mass(&self) -> &Option<Mass<Float>> {
        &self.mass
    }

    pub const fn get_luminosity(&self) -> &Option<Luminosity<Float>> {
        &self.luminosity
    }

    pub const fn get_temperature(&self) -> &Option<Temperature<Float>> {
        &self.temperature
    }

    pub const fn get_age(&self) -> &Option<Time<Float>> {
        &self.age
    }

    pub const fn get_distance(&self) -> &Option<Distance<Float>> {
        &self.distance
    }

    pub const fn get_direction_in_ecliptic(&self) -> &Direction {
        &self.direction_in_ecliptic
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_mass(&mut self, mass: Option<Mass<Float>>) {
        self.mass = mass;
    }

    pub fn set_radius(&mut self, radius: Option<Distance<Float>>) {
        self.radius = radius;
    }

    pub fn set_luminosity(&mut self, luminosity: Option<Luminosity<Float>>) {
        self.luminosity = luminosity;
    }

    pub fn set_temperature(&mut self, temperature: Option<Temperature<Float>>) {
        self.temperature = temperature;
    }

    pub fn set_age(&mut self, age: Option<Time<Float>>) {
        self.age = age;
    }

    pub fn set_distance(&mut self, distance: Option<Distance<Float>>) {
        self.distance = distance;
    }

    pub fn set_direction_in_ecliptic(&mut self, direction_in_ecliptic: Direction) {
        self.direction_in_ecliptic = direction_in_ecliptic;
    }

    pub fn to_star_appearance(&self) -> StarAppearance {
        let illuminance = match (self.luminosity, self.distance) {
            (Some(luminosity), Some(distance)) => luminosity.to_illuminance(&distance),
            _ => ILLUMINANCE_ZERO,
        };
        let color = match self.temperature {
            Some(temperature) => sRGBColor::from_temperature(temperature),
            None => sRGBColor::DEFAULT,
        };
        StarAppearance {
            name: self.name.clone(),
            illuminance,
            color,
            direction_in_ecliptic: self.direction_in_ecliptic.clone(),
        }
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
