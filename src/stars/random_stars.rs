use std::time::Instant;

use super::{
    parsec_data::{ParsecData, ParsecLine},
    star::StarData,
};
use crate::{
    coordinates::cartesian::CartesianCoordinates, error::AstroUtilError, units::length::Length,
    Float,
};
use rand::{
    distributions::{Uniform, WeightedIndex},
    rngs::ThreadRng,
    Rng,
};

// https://en.wikipedia.org/wiki/Stellar_density
const STARS_PER_LY_CUBED: Float = 0.004;
const DIMMEST_VISIBLE_MAGNITUDE: Float = 6.5;

pub fn generate_random_stars(max_distance: Length) -> Result<Vec<StarData>, AstroUtilError> {
    let parsec_data = ParsecData::new()?;
    let number_of_stars_in_cube = STARS_PER_LY_CUBED * (max_distance * 2.).as_light_years().powi(3);
    let number_of_stars_in_cube = number_of_stars_in_cube as usize;
    let mut rng = rand::thread_rng();
    let pos_distr = get_pos_distribution(max_distance);
    let mass_distr = get_mass_distribution();
    let mut stars = Vec::new();

    let start = Instant::now();
    for _ in 0..number_of_stars_in_cube {
        if let Some(star) = generate_visible_random_star(
            &parsec_data,
            max_distance,
            &mut rng,
            &pos_distr,
            &mass_distr,
        ) {
            stars.push(star);
        }
    }
    let duration = start.elapsed();
    println!(
        "\nGenerated {} stars within {} in {:?}\n",
        stars.len(),
        max_distance,
        duration
    );

    Ok(stars)
}

fn get_pos_distribution(max_distance: Length) -> Uniform<f32> {
    Uniform::new(
        -max_distance.as_astronomical_units(),
        max_distance.as_astronomical_units(),
    )
}

fn get_mass_distribution() -> WeightedIndex<Float> {
    let mut weights = Vec::new();
    for m in ParsecData::SORTED_MASSES {
        let weight = kroupa_mass_distribution(m);
        weights.push(weight);
    }
    WeightedIndex::new(weights).unwrap()
}

fn kroupa_mass_distribution(m_in_sun_masses: Float) -> Float {
    let alpha = if m_in_sun_masses <= 0.08 {
        0.3
    } else if m_in_sun_masses <= 0.5 {
        1.3
    } else if m_in_sun_masses <= 1. {
        2.3
    } else {
        2.7
    };
    m_in_sun_masses.powf(-alpha)
}

fn generate_visible_random_star(
    parsec_data: &ParsecData,
    max_distance: Length,
    rng: &mut ThreadRng,
    pos_distr: &Uniform<Float>,
    mass_index_distr: &WeightedIndex<Float>,
) -> Option<StarData> {
    let pos = generate_random_3d_position(rng, pos_distr);
    let distance = pos.length();
    if distance > max_distance {
        return None;
    }
    let mass_index = rng.sample(mass_index_distr);
    let trajectory = parsec_data.get_trajectory_via_index(mass_index);
    let current_params = pick_random_age(&trajectory);
    let apparent_magnitude = current_params.get_apparent_magnitude(&distance);
    if apparent_magnitude > DIMMEST_VISIBLE_MAGNITUDE {
        return None;
    }

    let mut star = current_params.to_star_at_origin();
    star.distance = Some(distance);
    star.direction_in_ecliptic = pos.to_direction();
    Some(star)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_random_stars_stress_test() {
        // Generating 70_000 stars within 1000 ly takes round about 17 seconds in release mode.
        // But this should not take too long and also work in debug mode.
        // Furthermore, the parsec data needs to be loaded and parsed which takes some seconds.
        let max_distance = Length::from_light_years(200.);
        let max_seconds = 60;

        // Downloading files should not count against the time.
        ParsecData::ensure_data_files().unwrap();
        let start = Instant::now();
        let stars = generate_random_stars(max_distance).unwrap();
        let duration = start.elapsed();
        println!("duration: {:?}", duration);
        assert!(stars.len() > 1000);
        assert!(duration.as_secs() < max_seconds);
    }
}
