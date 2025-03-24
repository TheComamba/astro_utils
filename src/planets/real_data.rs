use astro_coords::earth_equatorial::EarthEquatorial;

use crate::color::srgb::sRGBColor;

use super::{
    orbit_parameters::OrbitParameters, physical_parameters::PlanetPhysicalParameters,
    planet_data::PlanetData,
};

pub struct RealData {
    pub name: &'static str,
    pub orbit: OrbitParameters,
    pub geometric_albedo: f64,
    pub bond_albedo: Option<f64>,
    pub color: sRGBColor,
    pub radius: Length,
    pub mass: Mass<f64>,
    pub siderial_rotation_period: Time<f64>,
    pub axis_tilt: Angle<f64>,
    pub rotation_axis: EarthEquatorial,
}

impl RealData {
    pub fn to_planet_data(&self) -> PlanetData {
        let params = PlanetPhysicalParameters::new(
            self.mass,
            self.radius,
            self.geometric_albedo,
            self.color,
            self.siderial_rotation_period,
            self.rotation_axis.to_direction(),
        );
        PlanetData {
            name: self.name.to_string(),
            params,
            orbital_parameters: self.orbit.clone(),
        }
    }
}
