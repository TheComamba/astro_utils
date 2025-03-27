use std::f64::consts::PI;

use rand::Rng;
use uom::si::f64::{Length, Time};

use crate::{
    color::srgb::sRGBColor, real_data::planets::*, stars::random::random_stars::random_direction,
};

use super::{
    orbit_parameters::OrbitParameters, physical_parameters::PlanetPhysicalParameters,
    planet_data::PlanetData,
};

pub fn generate_random_planet() -> PlanetData {
    let mut rng = rand::thread_rng();

    let name = String::new();

    let min = MERCURY.radius * 0.5;
    let max = JUPITER.radius * 2.0;
    let radius = Length {
        m: rng.gen_range(min.m..max.m),
    };

    let min = SATURN.mass / (4. / 3. * PI * SATURN.radius * SATURN.radius * SATURN.radius) * 0.9;
    let max = EARTH.mass / (4. / 3. * PI * EARTH.radius * EARTH.radius * EARTH.radius) * 1.1;
    let density = Density {
        kgpm3: rng.gen_range(min.kgpm3..max.kgpm3),
    };

    let mass = density * (4. / 3. * PI * radius * radius * radius);

    let geometric_albedo = rng.gen_range(0.0..1.0);

    let color = sRGBColor::from_sRGB(
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
    );

    let min = -500.;
    let max = 500.;
    let rotation_period = Time::from_hr(rng.gen_range(min..max));

    let min = MERCURY.orbit.semi_major_axis * 0.5;
    let max = NEPTUNE.orbit.semi_major_axis * 2.0;
    let semi_major_axis = Length {
        m: rng.gen_range(min.m..max.m),
    };

    let min = 0.;
    let max = PLUTO.orbit.eccentricity * 2.0;
    let eccentricity = rng.gen_range(min..max);

    let min = -PLUTO.orbit.inclination.to_degrees();
    let max = PLUTO.orbit.inclination.to_degrees();
    let inclination = Angle::from_degrees(rng.gen_range(min..max));

    let longitude_of_ascending_node = Angle::from_degrees(rng.gen_range(0.0..360.0));

    let argument_of_periapsis = Angle::from_degrees(rng.gen_range(0.0..360.0));

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
