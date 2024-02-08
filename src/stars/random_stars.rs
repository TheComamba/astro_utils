use super::{
    parsec_data::{ParsecData, ParsecLine},
    star_data::StarData,
};
use crate::{
    coordinates::{cartesian::CartesianCoordinates, direction::Direction},
    error::AstroUtilError,
    stars::parsec_data::PARSEC_DATA,
};
use rand::{
    distributions::{Uniform, WeightedIndex},
    rngs::ThreadRng,
    Rng,
};
use simple_si_units::base::Distance;
use std::f64::consts::PI;

// https://en.wikipedia.org/wiki/Stellar_density
const STARS_PER_LY_CUBED: f64 = 0.004;
const DIMMEST_VISIBLE_MAGNITUDE: f64 = 6.5;

pub fn generate_random_stars(max_distance: Distance<f64>) -> Result<Vec<StarData>, AstroUtilError> {
    println!("generating random stars...");

    let number_of_stars_in_sphere =
        STARS_PER_LY_CUBED * 4. / 3. * PI * max_distance.to_lyr().powi(3);
    let number_of_stars_in_sphere = number_of_stars_in_sphere as usize;
    let max_distance_in_au_squared = max_distance.to_au().powi(2);
    let mut rng = rand::thread_rng();
    let pos_distr = get_pos_distribution(max_distance);
    let mass_index_distr = get_mass_distribution();
    let mut stars = Vec::new();
    {
        let parsec_data_mutex = PARSEC_DATA
            .lock()
            .map_err(|_| AstroUtilError::MutexPoison)?;
        let parsec_data = parsec_data_mutex.as_ref()?;
        for _ in 0..number_of_stars_in_sphere {
            if let Some(star) = generate_visible_random_star(
                parsec_data,
                max_distance_in_au_squared,
                &mut rng,
                &pos_distr,
                &mass_index_distr,
            ) {
                stars.push(star);
            }
        }
    }

    Ok(stars)
}

pub fn generate_random_star(
    max_distance: Option<Distance<f64>>,
) -> Result<StarData, AstroUtilError> {
    println!("generating random star...");
    let mut rng = rand::thread_rng();
    let max_distance_in_au_squared = max_distance.map(|d| d.to_au().powi(2)).unwrap_or(1.);
    let pos_distr = get_pos_distribution(max_distance.unwrap_or(Distance::from_au(1.)));
    let mass_index_distr = get_mass_distribution();

    let parsec_data_mutex = PARSEC_DATA
        .lock()
        .map_err(|_| AstroUtilError::MutexPoison)?;
    let parsec_data = parsec_data_mutex.as_ref()?;
    let mut star = generate_visible_random_star(
        parsec_data,
        max_distance_in_au_squared,
        &mut rng,
        &pos_distr,
        &mass_index_distr,
    );
    while star.is_none() {
        star = generate_visible_random_star(
            parsec_data,
            max_distance_in_au_squared,
            &mut rng,
            &pos_distr,
            &mass_index_distr,
        );
    }
    let mut star = star.unwrap();
    if max_distance.is_none() {
        star.distance = None;
    }
    Ok(star)
}

fn generate_visible_random_star(
    parsec_data: &ParsecData,
    max_distance_in_au_squared: f64,
    rng: &mut ThreadRng,
    pos_distr: &Uniform<f64>,
    mass_index_distr: &WeightedIndex<f64>,
) -> Option<StarData> {
    println!("generating visible random star...");
    let pos = random_point_in_sphere(rng, pos_distr, max_distance_in_au_squared);
    let mass_index = rng.sample(mass_index_distr);
    let trajectory = parsec_data.get_trajectory_via_index(mass_index);
    let current_params = pick_random_age(trajectory);
    let distance = pos.length();
    let apparent_magnitude = current_params.get_apparent_magnitude(&distance);
    if apparent_magnitude > DIMMEST_VISIBLE_MAGNITUDE {
        return None;
    }
    let mut star = current_params.to_star_at_origin();
    star.distance = Some(distance);
    star.direction_in_ecliptic = pos.to_direction().unwrap_or(random_direction(rng));
    Some(star)
}

fn get_pos_distribution(max_distance: Distance<f64>) -> Uniform<f64> {
    println!("getting pos distribution...");
    Uniform::new(-max_distance.to_au(), max_distance.to_au())
}

fn get_mass_distribution() -> WeightedIndex<f64> {
    println!(" getting mass distribution...");
    let mut weights = Vec::new();
    for m in ParsecData::SORTED_MASSES {
        let weight = kroupa_mass_distribution(m);
        weights.push(weight);
    }
    WeightedIndex::new(weights).unwrap()
}

fn kroupa_mass_distribution(m_in_solar_masses: f64) -> f64 {
    let alpha = if m_in_solar_masses <= 0.08 {
        0.3
    } else if m_in_solar_masses <= 0.5 {
        1.3
    } else if m_in_solar_masses <= 1. {
        2.3
    } else {
        2.7
    };
    m_in_solar_masses.powf(-alpha)
}

pub(crate) fn random_direction(rng: &mut ThreadRng) -> Direction {
    println!(" random direction...");
    let distr = Uniform::new(-1., 1.);
    let mut point = random_point_in_sphere(rng, &distr, 1.);
    let mut dir = point.to_direction();
    while dir.is_err() {
        point = random_point_in_sphere(rng, &distr, 1.);
        dir = point.to_direction();
    }
    dir.unwrap()
}

fn random_point_in_sphere(
    rng: &mut ThreadRng,
    distr: &Uniform<f64>,
    max_distance_in_au_squared: f64,
) -> CartesianCoordinates {
    println!(" random point in sphere...");
    let (mut x, mut y, mut z) = (rng.sample(distr), rng.sample(distr), rng.sample(distr));
    while x * x + y * y + z * z > max_distance_in_au_squared {
        (x, y, z) = (rng.sample(distr), rng.sample(distr), rng.sample(distr));
    }
    let x = Distance::from_au(rng.sample(distr));
    let y = Distance::from_au(rng.sample(distr));
    let z = Distance::from_au(rng.sample(distr));
    CartesianCoordinates::new(x, y, z)
}

fn pick_random_age(trajectory: &[ParsecLine]) -> &ParsecLine {
    println!(" pick random age...");
    let mut rng: ThreadRng = rand::thread_rng();
    let life_expectancy = ParsecData::get_life_expectancy_in_years(trajectory);
    let age_in_years = rng.gen_range(0..=life_expectancy);
    ParsecData::get_closest_params(trajectory, age_in_years as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn generate_random_stars_stress_test() {
        println!(" generate random stars stress test...");
        // Generating 70_000 stars within 1000 ly takes round about 17 seconds in release mode.
        // But this should not take too long and also work in debug mode.
        // Furthermore, the parsec data needs to be loaded and parsed which takes some seconds.
        let max_distance = Distance::from_lyr(200.);
        let max_seconds = 60;

        let start = Instant::now();
        let stars = generate_random_stars(max_distance).unwrap();
        let duration = start.elapsed();
        println!("duration: {:?}", duration);
        println!("stars: {}", stars.len());
        assert!(stars.len() > 1000);
        assert!(duration.as_secs() < max_seconds);
    }

    #[test]
    fn generating_a_distant_random_star() {
        println!(" generating a distant random star...");
        let max_distance = Distance::from_lyr(1000.);
        let _ = generate_random_star(Some(max_distance)).unwrap();
    }
}
