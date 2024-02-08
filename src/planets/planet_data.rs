use super::orbit_parameters::OrbitParameters;
use crate::{
    color::sRGBColor,
    coordinates::{cartesian::CartesianCoordinates, direction::Direction},
    error::AstroUtilError,
    planets::planet_brightness::planet_brightness,
    stars::{star_appearance::StarAppearance, star_data::StarData},
    units::luminous_intensity::LUMINOSITY_ZERO,
};
use serde::{Deserialize, Serialize};
use simple_si_units::{
    base::{Distance, Mass, Time},
    geometry::Angle,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetData {
    pub(super) name: String,
    pub(super) mass: Mass<f64>,
    pub(super) radius: Distance<f64>,
    pub(super) geometric_albedo: f64,
    pub(super) color: sRGBColor,
    pub(super) sideral_rotation_period: Time<f64>,
    pub(super) orbital_parameters: OrbitParameters,
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
        mass: Mass<f64>,
        radius: Distance<f64>,
        geometric_albedo: f64,
        color: sRGBColor,
        sideral_rotation_period: Time<f64>,
        orbital_parameters: OrbitParameters,
        rotation_axis: Direction,
    ) -> Self {
        PlanetData {
            name,
            mass,
            radius,
            geometric_albedo,
            color,
            sideral_rotation_period,
            orbital_parameters,
            rotation_axis,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_mass(&self) -> Mass<f64> {
        self.mass
    }

    pub fn get_radius(&self) -> Distance<f64> {
        self.radius
    }

    pub fn get_geometric_albedo(&self) -> f64 {
        self.geometric_albedo
    }

    pub fn get_color(&self) -> &sRGBColor {
        &self.color
    }

    pub fn get_orbital_parameters(&self) -> &OrbitParameters {
        &self.orbital_parameters
    }

    pub fn get_sideral_rotation_period(&self) -> Time<f64> {
        self.sideral_rotation_period
    }

    pub fn get_rotation_axis(&self) -> &Direction {
        &self.rotation_axis
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_mass(&mut self, mass: Mass<f64>) {
        self.mass = mass;
    }

    pub fn set_radius(&mut self, radius: Distance<f64>) {
        self.radius = radius;
    }

    pub fn set_geometric_albedo(&mut self, geometric_albedo: f64) {
        self.geometric_albedo = geometric_albedo;
    }

    pub fn set_color(&mut self, color: sRGBColor) {
        self.color = color;
    }

    pub fn set_semi_major_axis(&mut self, semi_major_axis: Distance<f64>) {
        self.orbital_parameters.semi_major_axis = semi_major_axis;
    }

    pub fn set_eccentricity(&mut self, eccentricity: f64) {
        self.orbital_parameters.eccentricity = eccentricity;
    }

    pub fn set_inclination(&mut self, inclination: Angle<f64>) {
        self.orbital_parameters.inclination = inclination;
    }

    pub fn set_longitude_of_ascending_node(&mut self, longitude_of_ascending_node: Angle<f64>) {
        self.orbital_parameters.longitude_of_ascending_node = longitude_of_ascending_node;
    }

    pub fn set_argument_of_periapsis(&mut self, argument_of_periapsis: Angle<f64>) {
        self.orbital_parameters.argument_of_periapsis = argument_of_periapsis;
    }

    pub fn set_sideral_rotation_period(&mut self, sideral_rotation_period: Time<f64>) {
        self.sideral_rotation_period = sideral_rotation_period;
    }

    pub fn set_rotation_axis(&mut self, rotation_axis: Direction) {
        self.rotation_axis = rotation_axis;
    }

    pub fn to_star_appearance(
        &self,
        central_body: &StarData,
        planet_pos: &CartesianCoordinates,
        observer_position: &CartesianCoordinates,
    ) -> Result<StarAppearance, AstroUtilError> {
        let central_body_luminous_intensity = central_body
            .get_luminous_intensity()
            .unwrap_or(LUMINOSITY_ZERO);
        let brightness = planet_brightness(
            central_body_luminous_intensity,
            &CartesianCoordinates::ORIGIN,
            planet_pos,
            observer_position,
            self.radius,
            self.geometric_albedo,
        )?;
        let relative_position = planet_pos - observer_position;
        Ok(StarAppearance {
            name: self.name.clone(),
            illuminance: brightness,
            color: self.color.clone(),
            direction_in_ecliptic: relative_position.to_direction()?,
        })
    }
}
