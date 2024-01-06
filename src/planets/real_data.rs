use super::orbit_parameters::OrbitParameters;
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
