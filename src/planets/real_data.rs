use super::orbit_parameters::OrbitParameters;
use crate::{
    color::sRGBColor,
    coordinates::earth_equatorial::EarthEquatorialCoordinates,
    units::{angle::Angle, length::Length, mass::Mass, time::Time},
    Float,
};

pub struct RealData {
    pub(crate) name: &'static str,
    pub(crate) orbit: OrbitParameters,
    pub(crate) geometric_albedo: Float,
    pub(crate) bond_albedo: Option<Float>,
    pub(crate) color: sRGBColor,
    pub(crate) radius: Length,
    pub(crate) mass: Mass,
    pub(crate) siderial_rotation_period: Time,
    pub(crate) axis_tilt: Angle,
    pub(crate) rotation_axis: EarthEquatorialCoordinates,
}
