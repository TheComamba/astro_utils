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
    use uom::si::mass_density::gram_per_cubic_centimeter;

    use crate::real_data::planets::*;

    use super::*;

    #[test]
    fn mass_density_of_real_planets() {
        fn expect_mass_density(planet: RealData, expected: MassDensity) {
            assert!(
                (planet.mass_density() - expected)
                    .abs()
                    .get::<gram_per_cubic_centimeter>()
                    < 0.1
            );
        }

        let expected = MassDensity::new::<gram_per_cubic_centimeter>(5.427);
        expect_mass_density(mercury(), expected);

        let expected = MassDensity::new::<gram_per_cubic_centimeter>(5.243);
        expect_mass_density(venus(), expected);

        let expected = MassDensity::new::<gram_per_cubic_centimeter>(5.513);
        expect_mass_density(earth(), expected);

        let expected = MassDensity::new::<gram_per_cubic_centimeter>(3.9335);
        expect_mass_density(mars(), expected);

        let expected = MassDensity::new::<gram_per_cubic_centimeter>(2.1616);
        expect_mass_density(ceres(), expected);

        let expected = MassDensity::new::<gram_per_cubic_centimeter>(1.326);
        expect_mass_density(jupiter(), expected);

        let expected = MassDensity::new::<gram_per_cubic_centimeter>(0.687);
        expect_mass_density(saturn(), expected);

        let expected = MassDensity::new::<gram_per_cubic_centimeter>(1.27);
        expect_mass_density(uranus(), expected);

        let expected = MassDensity::new::<gram_per_cubic_centimeter>(1.638);
        expect_mass_density(neptune(), expected);

        let expected = MassDensity::new::<gram_per_cubic_centimeter>(1.853);
        expect_mass_density(pluto(), expected);

        let expected = MassDensity::new::<gram_per_cubic_centimeter>(3.344);
        expect_mass_density(luna(), expected);
    }
}
