use simple_si_units::{
    base::{Distance, Mass, Time},
    geometry::Angle,
};

use super::{orbit_parameters::OrbitParameters, planet_data::PlanetData};
use crate::{color::srgb::sRGBColor, coordinates::earth_equatorial::EarthEquatorialCoordinates};

pub struct RealData {
    pub name: &'static str,
    pub orbit: OrbitParameters,
    pub geometric_albedo: f64,
    pub bond_albedo: Option<f64>,
    pub color: sRGBColor,
    pub radius: Distance<f64>,
    pub mass: Mass<f64>,
    pub siderial_rotation_period: Time<f64>,
    pub axis_tilt: Angle<f64>,
    pub rotation_axis: EarthEquatorialCoordinates,
}

impl RealData {
    pub fn to_planet_data(&self) -> PlanetData {
        PlanetData {
            name: self.name.to_string(),
            mass: self.mass,
            orbital_parameters: self.orbit.clone(),
            radius: self.radius,
            geometric_albedo: self.geometric_albedo,
            color: self.color.clone(),
            sideral_rotation_period: self.siderial_rotation_period,
            rotation_axis: self.rotation_axis.to_direction(),
        }
    }
}
