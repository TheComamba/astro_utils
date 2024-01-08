use super::{
    parsec_data::{ParsecData, ParsecLine},
    star::Star,
};
use crate::{
    coordinates::cartesian::CartesianCoordinates,
    error::AstroUtilError,
    units::{length::Length, mass::Mass},
    Float,
};
use rand::{distributions::Uniform, rngs::ThreadRng, Rng};

// https://en.wikipedia.org/wiki/Stellar_density
const STARS_PER_LY_CUBED: Float = 0.004;
const DIMMEST_VISIBLE_MAGNITUDE: Float = 6.5;

pub fn generate_random_stars(max_distance: Length) -> Result<Vec<Star>, AstroUtilError> {
    let parsec_data = ParsecData::new()?;
    let number_of_stars_in_cube = STARS_PER_LY_CUBED * (max_distance * 2.).as_light_years().powi(3);
    let number_of_stars_in_cube = number_of_stars_in_cube as usize;
    let mut rng = rand::thread_rng();
    let distr = Uniform::new(
        -max_distance.as_astronomical_units(),
        max_distance.as_astronomical_units(),
    );
    let mut stars = Vec::new();
    for _ in 0..number_of_stars_in_cube {
        if let Some(star) =
            generate_visible_random_star(&parsec_data, max_distance, &mut rng, &distr)
        {
            stars.push(star);
        }
    }
    Ok(stars)
}

fn generate_visible_random_star(
    parsec_data: &ParsecData,
    max_distance: Length,
    rng: &mut ThreadRng,
    distr: &Uniform<Float>,
) -> Option<Star> {
    let pos = generate_random_3d_position(rng, distr);
    let distance = pos.length();
    if distance > max_distance {
        return None;
    }
    let mass = generate_random_mass();
    let trajectory = parsec_data.get_trajectory(mass.as_solar_masses());
    let current_params = pick_random_age(&trajectory);
    let mut star = current_params.to_star_at_origin();

    let apparent_magnitude = star
        .luminosity
        .to_illuminance(&distance)
        .as_apparent_magnitude();
    if apparent_magnitude > DIMMEST_VISIBLE_MAGNITUDE {
        None
    } else {
        star.distance = distance;
        star.direction_in_ecliptic = pos.to_direction();
        Some(star)
    }
}

fn pick_random_age(trajectory: &Vec<ParsecLine>) -> &ParsecLine {
    let mut rng: ThreadRng = rand::thread_rng();
    let life_expectancy = ParsecData::get_life_expectancy_in_years(trajectory);
    let age_in_years = rng.gen_range(0..=life_expectancy);
    ParsecData::get_closest_params(trajectory, age_in_years as Float)
}

fn generate_random_3d_position(
    rng: &mut ThreadRng,
    distr: &Uniform<Float>,
) -> CartesianCoordinates {
    let x = Length::from_astronomical_units(rng.sample(distr));
    let y = Length::from_astronomical_units(rng.sample(distr));
    let z = Length::from_astronomical_units(rng.sample(distr));
    CartesianCoordinates::new(x, y, z)
}

// Selpeter Law
fn generate_random_mass() -> Mass {
    todo!("Implement generate_random_mass")
}
