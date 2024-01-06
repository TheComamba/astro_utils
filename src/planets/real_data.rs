use super::{orbit_parameters::OrbitParameters, planet::Planet};
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
    pub fn to_planet(real_data: &RealData) -> Planet {
        Planet {
            name: real_data.name.to_string(),
            mass: real_data.mass,
            orbital_parameters: real_data.orbit.clone(),
            radius: real_data.radius,
            geometric_albedo: real_data.geometric_albedo,
            color: real_data.color.clone(),
            sideral_rotation_period: real_data.siderial_rotation_period,
            rotation_axis: real_data.rotation_axis.to_direction(),
        }
    }
}
