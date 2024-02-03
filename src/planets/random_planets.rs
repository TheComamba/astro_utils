use super::{orbit_parameters::OrbitParameters, planet_data::PlanetData};
use crate::{
    color::sRGBColor,
    real_data::planets::*,
    stars::random_stars::random_direction,
    units::{angle::Angle, length::Length, mass::Mass, time::Time},
};
use rand::Rng;

pub fn generate_random_planet() -> PlanetData {
    let mut rng = rand::thread_rng();

    let name = String::new();

    let min = MERCURY.radius.as_kilometers() * 0.5;
    let max = JUPITER.radius.as_kilometers() * 2.0;
    let radius = Length::from_kilometers(rng.gen_range(min..max));

    let min = SATURN.mass.as_jupiter_masses() / SATURN.radius.as_jupiter_radii().powi(3) * 0.9;
    let max = EARTH.mass.as_jupiter_masses() / EARTH.radius.as_jupiter_radii().powi(3) * 1.1;
    let density = rng.gen_range(min..max);

    let mass = Mass::from_jupiter_masses(density * radius.as_jupiter_radii().powi(3));

    let geometric_albedo = rng.gen_range(0.0..1.0);

    let color = sRGBColor::from_sRGB(
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
    );

    let rotation_period = Time::from_hours(rng.gen_range((-500.)..(500.)));

    let min = MERCURY.orbit.semi_major_axis.as_astronomical_units() * 0.5;
    let max = NEPTUNE.orbit.semi_major_axis.as_astronomical_units() * 2.0;
    let semi_major_axis = Length::from_astronomical_units(rng.gen_range(min..max));

    let min = 0.;
    let max = PLUTO.orbit.eccentricity * 2.0;
    let eccentricity = rng.gen_range(min..max);

    let min = -PLUTO.orbit.inclination.as_degrees();
    let max = PLUTO.orbit.inclination.as_degrees();
    let inclination = Angle::from_degrees(rng.gen_range(min..max));

    let longitude_of_ascending_node = Angle::from_degrees(rng.gen_range(0.0..360.0));

    let argument_of_periapsis = Angle::from_degrees(rng.gen_range(0.0..360.0));

    let rotation_axis = random_direction(&mut rng);

    PlanetData {
        name,
        mass,
        radius,
        geometric_albedo,
        color,
        orbital_parameters: OrbitParameters::new(
            semi_major_axis,
            eccentricity,
            inclination,
            longitude_of_ascending_node,
            argument_of_periapsis,
        ),
        sideral_rotation_period: rotation_period,
        rotation_axis,
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
