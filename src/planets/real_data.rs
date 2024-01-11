use super::{orbit_parameters::OrbitParameters, planet_data::PlanetData};
use crate::{
    color::sRGBColor,
    coordinates::earth_equatorial::EarthEquatorialCoordinates,
    units::{angle::Angle, length::Length, mass::Mass, time::Time},
    Float,
};

pub struct RealData {
    pub name: &'static str,
    pub orbit: OrbitParameters,
    pub geometric_albedo: Float,
    pub bond_albedo: Option<Float>,
    pub color: sRGBColor,
    pub radius: Length,
    pub mass: Mass,
    pub siderial_rotation_period: Time,
    pub axis_tilt: Angle,
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
