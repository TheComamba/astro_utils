use astro_coords::direction::Direction;
use serde::{Deserialize, Serialize};
use uom::si::f64::{Length, Mass, Time};

use crate::color::srgb::sRGBColor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetPhysicalParameters {
    pub(super) mass: Mass,
    pub(super) radius: Length,
    pub(super) geometric_albedo: f64,
    pub(super) color: sRGBColor,
    pub(super) sideral_rotation_period: Time,
    pub(super) rotation_axis: Direction,
}

impl PlanetPhysicalParameters {
    pub fn new(
        mass: Mass,
        radius: Length,
        geometric_albedo: f64,
        color: sRGBColor,
        sideral_rotation_period: Time,
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

    pub fn mass(&self) -> Mass {
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

    pub fn sideral_rotation_period(&self) -> Time {
        self.sideral_rotation_period
    }

    pub fn rotation_axis(&self) -> &Direction {
        &self.rotation_axis
    }
}
