use std::f64::consts::PI;

use astro_coords::earth_equatorial::EarthEquatorial;
use uom::si::f64::{Angle, Length, Mass, MassDensity, Time};

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
    pub mass: Mass,
    pub siderial_rotation_period: Time,
    pub axis_tilt: Angle,
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

    pub fn mass_density(&self) -> MassDensity {
        self.mass / (4.0 / 3.0 * PI * self.radius * self.radius * self.radius)
    }
}

#[cfg(test)]
mod tests {
    use uom::si::mass_density::kilogram_per_cubic_meter;

    use crate::real_data::planets::*;

    use super::*;

    #[test]
    fn mass_density_of_real_planets() {
        fn expect_mass_density(planet: RealData, expected: MassDensity) {
            assert!(
                (planet.mass_density() - expected)
                    .abs()
                    .get::<kilogram_per_cubic_meter>()
                    < 0.1
            );
        }

        //TODO: Look up actual numbers
        let expected = MassDensity::new::<kilogram_per_cubic_meter>(13.6);
        expect_mass_density(mercury(), expected);
        let expected = MassDensity::new::<kilogram_per_cubic_meter>(5.2);
        expect_mass_density(venus(), expected);
        let expected = MassDensity::new::<kilogram_per_cubic_meter>(5.5);
        expect_mass_density(earth(), expected);
        let expected = MassDensity::new::<kilogram_per_cubic_meter>(3.9);
        expect_mass_density(mars(), expected);
        let expected = MassDensity::new::<kilogram_per_cubic_meter>(3.9);
        expect_mass_density(ceres(), expected);
        let expected = MassDensity::new::<kilogram_per_cubic_meter>(1.3);
        expect_mass_density(jupiter(), expected);
        let expected = MassDensity::new::<kilogram_per_cubic_meter>(0.7);
        expect_mass_density(saturn(), expected);
        let expected = MassDensity::new::<kilogram_per_cubic_meter>(1.2);
        expect_mass_density(uranus(), expected);
        let expected = MassDensity::new::<kilogram_per_cubic_meter>(1.6);
        expect_mass_density(neptune(), expected);
        let expected = MassDensity::new::<kilogram_per_cubic_meter>(2.0);
        expect_mass_density(pluto(), expected);
        let expected = MassDensity::new::<kilogram_per_cubic_meter>(2.0);
        expect_mass_density(luna(), expected);
    }
}
