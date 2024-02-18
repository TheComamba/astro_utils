use super::star_appearance::StarAppearance;
use crate::{
    color::srgb::sRGBColor,
    coordinates::ecliptic::EclipticCoordinates,
    units::{illuminance::IRRADIANCE_ZERO, luminous_intensity::luminous_intensity_to_illuminance},
};
use serde::{Deserialize, Serialize};
use simple_si_units::base::{Distance, Luminosity, Mass, Temperature, Time};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarData {
    pub(super) name: String,
    pub(super) constellation: Option<String>,
    pub(super) mass: Option<Mass<f64>>,
    pub(super) radius: Option<Distance<f64>>,
    pub(super) luminous_intensity: Option<Luminosity<f64>>,
    pub(super) temperature: Option<Temperature<f64>>,
    pub(super) age: Option<Time<f64>>,
    pub(super) distance: Option<Distance<f64>>,
    pub(super) pos: EclipticCoordinates,
}

impl StarData {
    pub fn new(
        name: String,
        constellation: Option<String>,
        mass: Option<Mass<f64>>,
        radius: Option<Distance<f64>>,
        luminous_intensity: Option<Luminosity<f64>>,
        temperature: Option<Temperature<f64>>,
        age: Option<Time<f64>>,
        distance: Option<Distance<f64>>,
        pos: EclipticCoordinates,
    ) -> Self {
        Self {
            name,
            mass,
            radius,
            luminous_intensity,
            temperature,
            age,
            distance,
            pos,
            constellation,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_constellation(&self) -> &Option<String> {
        &self.constellation
    }

    pub const fn get_radius(&self) -> &Option<Distance<f64>> {
        &self.radius
    }

    pub const fn get_mass(&self) -> &Option<Mass<f64>> {
        &self.mass
    }

    pub const fn get_luminous_intensity(&self) -> &Option<Luminosity<f64>> {
        &self.luminous_intensity
    }

    pub const fn get_temperature(&self) -> &Option<Temperature<f64>> {
        &self.temperature
    }

    pub const fn get_age(&self) -> &Option<Time<f64>> {
        &self.age
    }

    pub const fn get_distance(&self) -> &Option<Distance<f64>> {
        &self.distance
    }

    pub const fn get_pos(&self) -> &EclipticCoordinates {
        &self.pos
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_constellation(&mut self, constellation: Option<String>) {
        self.constellation = constellation;
    }

    pub fn set_mass(&mut self, mass: Option<Mass<f64>>) {
        self.mass = mass;
    }

    pub fn set_radius(&mut self, radius: Option<Distance<f64>>) {
        self.radius = radius;
    }

    pub fn set_luminous_intensity(&mut self, luminous_intensity: Option<Luminosity<f64>>) {
        self.luminous_intensity = luminous_intensity;
    }

    pub fn set_temperature(&mut self, temperature: Option<Temperature<f64>>) {
        self.temperature = temperature;
    }

    pub fn set_age(&mut self, age: Option<Time<f64>>) {
        self.age = age;
    }

    pub fn set_distance(&mut self, distance: Option<Distance<f64>>) {
        self.distance = distance;
    }

    pub fn set_pos(&mut self, pos: EclipticCoordinates) {
        self.pos = pos;
    }

    pub fn to_star_appearance(&self) -> StarAppearance {
        let illuminance = match (self.luminous_intensity, self.distance) {
            (Some(luminous_intensity), Some(distance)) => {
                luminous_intensity_to_illuminance(&luminous_intensity, &distance)
            }
            _ => IRRADIANCE_ZERO,
        };
        let color = match self.temperature {
            Some(temperature) => sRGBColor::from_temperature(temperature),
            None => sRGBColor::DEFAULT,
        };
        StarAppearance {
            name: self.name.clone(),
            illuminance,
            color,
            pos: self.pos.clone(),
        }
    }

    #[cfg(test)]
    pub(crate) fn similar_within_order_of_magnitude(&self, other: &Self) -> bool {
        use crate::units::luminous_intensity::luminous_intensity_to_absolute_magnitude;

        let mass_ratio = match (self.mass, other.mass) {
            (Some(self_mass), Some(other_mass)) => self_mass / other_mass,
            _ => 1.0,
        };
        let radius_ratio = match (self.radius, other.radius) {
            (Some(self_radius), Some(other_radius)) => self_radius / other_radius,
            _ => 1.0,
        };
        let luminous_intensity_difference =
            match (self.luminous_intensity, other.luminous_intensity) {
                (Some(self_luminous_intensity), Some(other_luminous_intensity)) => {
                    (luminous_intensity_to_absolute_magnitude(self_luminous_intensity)
                        - luminous_intensity_to_absolute_magnitude(other_luminous_intensity))
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
        if luminous_intensity_difference > 1.0 {
            println!(
                "luminous_intensity1: {}, luminous_intensity2: {}, difference: {}",
                self.luminous_intensity.unwrap(),
                other.luminous_intensity.unwrap(),
                luminous_intensity_difference
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
