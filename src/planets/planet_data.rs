use super::orbit_parameters::OrbitParameters;
use crate::{
    color::sRGBColor,
    coordinates::{cartesian::CartesianCoordinates, direction::Direction},
    planets::planet_brightness::planet_brightness,
    stars::{star_appearance::StarAppearance, star_data::StarData},
    units::{length::Length, luminosity::Luminosity, mass::Mass, time::Time},
    Float,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetData {
    pub(super) name: String,
    pub(super) mass: Mass,
    pub(super) radius: Length,
    pub(super) geometric_albedo: Float,
    pub(super) color: sRGBColor,
    pub(super) orbital_parameters: OrbitParameters,
    pub(super) sideral_rotation_period: Time,
    pub(super) rotation_axis: Direction,
}

impl PartialEq for PlanetData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PlanetData {
    pub fn new(
        name: String,
        mass: Mass,
        orbital_parameters: OrbitParameters,
        radius: Length,
        geometric_albedo: Float,
        color: sRGBColor,
        sideral_rotation_period: Time,
        rotation_axis: Direction,
    ) -> Self {
        PlanetData {
            name,
            mass,
            orbital_parameters,
            radius,
            geometric_albedo,
            color,
            sideral_rotation_period,
            rotation_axis,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_mass(&self) -> Mass {
        self.mass
    }

    pub fn get_radius(&self) -> Length {
        self.radius
    }

    pub fn get_geometric_albedo(&self) -> Float {
        self.geometric_albedo
    }

    pub fn get_color(&self) -> &sRGBColor {
        &self.color
    }

    pub fn get_orbital_parameters(&self) -> &OrbitParameters {
        &self.orbital_parameters
    }

    pub fn get_sideral_rotation_period(&self) -> Time {
        self.sideral_rotation_period
    }

    pub fn get_rotation_axis(&self) -> &Direction {
        &self.rotation_axis
    }

    pub fn to_star_appearance(
        &self,
        central_body: &StarData,
        time_since_epoch: &Time,
        observer_position: &CartesianCoordinates,
    ) -> StarAppearance {
        let central_body_luminosity = central_body
            .get_luminosity()
            .unwrap_or(Luminosity::PRACICALLY_ZERO);
        let planet_position =
            self.orbital_parameters
                .calculate_position(self.mass, central_body, *time_since_epoch);
        let brightness = planet_brightness(
            central_body_luminosity,
            &CartesianCoordinates::ORIGIN,
            &planet_position,
            observer_position,
            self.radius,
            self.geometric_albedo,
        );
        let relative_position = observer_position - &planet_position;
        StarAppearance {
            name: self.name.clone(),
            illuminance: brightness,
            color: self.color.clone(),
            direction_in_ecliptic: relative_position.to_direction(),
        }
    }
}
