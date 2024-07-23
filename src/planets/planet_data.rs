use super::{orbit_parameters::OrbitParameters, physical_parameters::PlanetPhysicalParameters};
use crate::{
    color::srgb::sRGBColor,
    error::AstroUtilError,
    planets::planet_brightness::planet_brightness,
    stars::{appearance::StarAppearance, data::StarData},
};
use astro_coords::{cartesian::Cartesian, direction::Direction};
use serde::{Deserialize, Serialize};
use simple_si_units::{
    base::{Distance, Mass, Time},
    geometry::Angle,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetData {
    pub(super) name: String,
    pub(super) params: PlanetPhysicalParameters,
    pub(super) orbital_parameters: OrbitParameters,
}

impl PartialEq for PlanetData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PlanetData {
    pub fn new(
        name: String,
        params: PlanetPhysicalParameters,
        orbital_parameters: OrbitParameters,
    ) -> Self {
        PlanetData {
            name,
            params,
            orbital_parameters,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_mass(&self) -> Mass<f64> {
        self.params.mass
    }

    pub fn get_radius(&self) -> Distance<f64> {
        self.params.radius
    }

    pub fn get_geometric_albedo(&self) -> f64 {
        self.params.geometric_albedo
    }

    pub fn get_color(&self) -> &sRGBColor {
        &self.params.color
    }

    pub fn get_orbital_parameters(&self) -> &OrbitParameters {
        &self.orbital_parameters
    }

    pub fn get_sideral_rotation_period(&self) -> Time<f64> {
        self.params.sideral_rotation_period
    }

    pub fn get_rotation_axis(&self) -> &Direction {
        &self.params.rotation_axis
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_mass(&mut self, mass: Mass<f64>) {
        self.params.mass = mass;
    }

    pub fn set_radius(&mut self, radius: Distance<f64>) {
        self.params.radius = radius;
    }

    pub fn set_geometric_albedo(&mut self, geometric_albedo: f64) {
        self.params.geometric_albedo = geometric_albedo;
    }

    pub fn set_color(&mut self, color: sRGBColor) {
        self.params.color = color;
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
        self.params.sideral_rotation_period = sideral_rotation_period;
    }

    pub fn set_rotation_axis(&mut self, rotation_axis: Direction) {
        self.params.rotation_axis = rotation_axis;
    }

    pub fn to_star_appearance(
        &self,
        central_body: &StarData,
        planet_pos: &Cartesian,
        observer_position: &Cartesian,
        time_since_epoch: Time<f64>,
    ) -> Result<StarAppearance, AstroUtilError> {
        let central_body_luminous_intensity = central_body.get_luminous_intensity_at_epoch();
        let brightness = planet_brightness(
            central_body_luminous_intensity,
            &Cartesian::ORIGIN,
            planet_pos,
            observer_position,
            self.params.radius,
            self.params.geometric_albedo,
        )?;
        let relative_position = planet_pos - observer_position;
        let pos = relative_position.to_ecliptic()?;
        Ok(StarAppearance {
            name: self.name.clone(),
            illuminance: brightness,
            color: self.params.color,
            pos,
            time_since_epoch,
        })
    }
}
