use astro_coords::direction::Direction;
use serde::{Deserialize, Serialize};

use crate::color::srgb::sRGBColor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetPhysicalParameters {
    pub(super) mass: Mass<f64>,
    pub(super) radius: Length,
    pub(super) geometric_albedo: f64,
    pub(super) color: sRGBColor,
    pub(super) sideral_rotation_period: Time<f64>,
    pub(super) rotation_axis: Direction,
}

impl PlanetPhysicalParameters {
    pub fn new(
        mass: Mass<f64>,
        radius: Length,
        geometric_albedo: f64,
        color: sRGBColor,
        sideral_rotation_period: Time<f64>,
        rotation_axis: Direction,
    ) -> Self {
        Self {
            mass,
            radius,
            geometric_albedo,
            color,
            sideral_rotation_period,
            rotation_axis,
        }
    }

    pub fn mass(&self) -> Mass<f64> {
        self.mass
    }

    pub fn radius(&self) -> Length {
        self.radius
    }

    pub fn geometric_albedo(&self) -> f64 {
        self.geometric_albedo
    }

    pub fn color(&self) -> &sRGBColor {
        &self.color
    }

    pub fn sideral_rotation_period(&self) -> Time<f64> {
        self.sideral_rotation_period
    }

    pub fn rotation_axis(&self) -> &Direction {
        &self.rotation_axis
    }
}
