use std::f64::consts::PI;

use astro_units::length::earth_radius;
use rand::Rng;
use uom::si::{
    angle::degree,
    f64::{Angle, Length, MassDensity, Time},
    length::astronomical_unit,
    mass_density::kilogram_per_cubic_meter,
    time::hour,
};

use crate::{
    color::srgb::sRGBColor, real_data::planets::*, stars::random::random_stars::random_direction,
};

use super::{
    orbit_parameters::OrbitParameters, physical_parameters::PlanetPhysicalParameters,
    planet_data::PlanetData,
};

pub fn generate_random_planet() -> PlanetData {
    let mut rng = rand::rng();

    let name = String::new();

    let min = mercury().radius.get::<earth_radius>() * 0.5;
    let max = jupiter().radius.get::<earth_radius>() * 2.0;
    let radius = Length::new::<earth_radius>(rng.random_range(min..max));

    let min = saturn().mass_density().get::<kilogram_per_cubic_meter>() * 0.9;
    let max = earth().mass_density().get::<kilogram_per_cubic_meter>() * 1.1;
    let density = MassDensity::new::<kilogram_per_cubic_meter>(rng.random_range(min..max));

    let mass = density * (4. / 3. * PI * radius * radius * radius);

    let geometric_albedo = rng.random_range(0.0..1.0);

    let color = sRGBColor::from_sRGB(
        rng.random_range(0.0..1.0),
        rng.random_range(0.0..1.0),
        rng.random_range(0.0..1.0),
    );

    let min = -500.;
    let max = 500.;
    let rotation_period = Time::new::<hour>(rng.random_range(min..max));

    let min = mercury().orbit.semi_major_axis.get::<astronomical_unit>() * 0.5;
    let max = neptune().orbit.semi_major_axis.get::<astronomical_unit>() * 2.0;
    let semi_major_axis = Length::new::<astronomical_unit>(rng.random_range(min..max));

    let min = 0.;
    let max = pluto().orbit.eccentricity * 2.0;
    let eccentricity = rng.random_range(min..max);

    let min = -pluto().orbit.inclination.get::<degree>();
    let max = pluto().orbit.inclination.get::<degree>();
    let inclination = Angle::new::<degree>(rng.random_range(min..max));

    let longitude_of_ascending_node = Angle::new::<degree>(rng.random_range(0.0..360.0));

    let argument_of_periapsis = Angle::new::<degree>(rng.random_range(0.0..360.0));

    let rotation_axis = random_direction(&mut rng);

    PlanetData {
        name,
        params: PlanetPhysicalParameters::new(
            mass,
            radius,
            geometric_albedo,
            color,
            rotation_period,
            rotation_axis,
        ),
        orbital_parameters: OrbitParameters::new(
            semi_major_axis,
            eccentricity,
            inclination,
            longitude_of_ascending_node,
            argument_of_periapsis,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_random_planet_does_not_crash() {
        let _ = generate_random_planet();
    }
}
