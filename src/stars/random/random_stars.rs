use crate::{
    coordinates::{cartesian::CartesianCoordinates, direction::Direction},
    error::AstroUtilError,
    stars::{random::parsec::data::PARSEC_DATA, star_data::StarData},
    units::distance::DISTANCE_ZERO,
};
use rand::{
    distributions::{Uniform, WeightedIndex},
    rngs::ThreadRng,
    Rng,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use simple_si_units::{
    base::{Distance, Time},
    electromagnetic::Illuminance,
};
use std::f64::consts::PI;

use super::parsec::data::ParsecData;

// https://en.wikipedia.org/wiki/Stellar_density
// Adjusted a little bit
const STARS_PER_LY_CUBED: f64 = 3.33e-5;
pub(super) const DIMMEST_ILLUMINANCE: Illuminance<f64> = Illuminance { lux: 6.5309e-9 };
const AGE_OF_MILKY_WAY_THIN_DISK: Time<f64> = Time {
    s: 8.8e9 * 365.25 * 24. * 3600.,
};

pub fn generate_random_stars(max_distance: Distance<f64>) -> Result<Vec<StarData>, AstroUtilError> {
    let number_of_stars_in_sphere =
        STARS_PER_LY_CUBED * 4. / 3. * PI * max_distance.to_lyr().powi(3);
    let number_of_stars_in_sphere = number_of_stars_in_sphere as usize;

    let pos_distr = get_pos_distribution(max_distance);
    let mass_index_distr = get_mass_distribution();
    let age_distr = get_age_distribution();

    let parsec_data_mutex = PARSEC_DATA
        .lock()
        .map_err(|_| AstroUtilError::MutexPoison)?;
    let parsec_data = parsec_data_mutex.as_ref()?;

    const MAX_CHUNKSIZE: usize = 1_000_000;
    let mut remaining = number_of_stars_in_sphere;
    let mut stars = Vec::new();
    while remaining > MAX_CHUNKSIZE {
        let chunk = generate_certain_number_of_random_stars(
            MAX_CHUNKSIZE,
            parsec_data,
            max_distance,
            pos_distr,
            mass_index_distr.clone(),
            age_distr,
        );
        stars.extend(chunk);
        remaining -= MAX_CHUNKSIZE;

        let finished = number_of_stars_in_sphere - remaining;
        let fraction = finished as f64 / number_of_stars_in_sphere as f64;
        println!(
            "Generated {:2.2e} of {:2.2e} stars ({:2.0}%) and kept {:2.2e}",
            finished,
            number_of_stars_in_sphere,
            fraction * 100.,
            stars.len()
        );
    }
    let chunk = generate_certain_number_of_random_stars(
        remaining,
        parsec_data,
        max_distance,
        pos_distr,
        mass_index_distr,
        age_distr,
    );
    stars.extend(chunk);

    Ok(stars)
}

fn generate_certain_number_of_random_stars(
    number: usize,
    parsec_data: &ParsecData,
    max_distance: Distance<f64>,
    pos_distr: Uniform<f64>,
    mass_index_distr: WeightedIndex<f64>,
    age_distr: Uniform<f64>,
) -> Vec<StarData> {
    (0..=number)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            generate_visible_random_star(
                parsec_data,
                max_distance,
                &mut rng,
                &pos_distr,
                &mass_index_distr,
                &age_distr,
            )
        })
        .filter_map(|star| star)
        .collect::<Vec<StarData>>()
}

pub fn generate_random_star(
    max_distance: Option<Distance<f64>>,
) -> Result<StarData, AstroUtilError> {
    let mut rng = rand::thread_rng();
    let max_distance_or_1 = max_distance.unwrap_or(Distance { m: 1. });
    let pos_distr = get_pos_distribution(max_distance_or_1);
    let mass_index_distr = get_mass_distribution();
    let age_dist = get_age_distribution();

    let parsec_data_mutex = PARSEC_DATA
        .lock()
        .map_err(|_| AstroUtilError::MutexPoison)?;
    let parsec_data = parsec_data_mutex.as_ref()?;
    let mut star = generate_visible_random_star(
        parsec_data,
        max_distance_or_1,
        &mut rng,
        &pos_distr,
        &mass_index_distr,
        &age_dist,
    );
    while star.is_none() {
        star = generate_visible_random_star(
            parsec_data,
            max_distance_or_1,
            &mut rng,
            &pos_distr,
            &mass_index_distr,
            &age_dist,
        );
    }
    let mut star = star.unwrap();
    if max_distance.is_none() {
        star.distance = DISTANCE_ZERO;
    }
    Ok(star)
}

fn generate_visible_random_star(
    parsec_data: &ParsecData,
    max_distance: Distance<f64>,
    rng: &mut ThreadRng,
    pos_distr: &Uniform<f64>,
    mass_index_distr: &WeightedIndex<f64>,
    age_dist: &Uniform<f64>,
) -> Option<StarData> {
    let pos = random_point_in_sphere(rng, pos_distr, max_distance);
    let mass_index = rng.sample(mass_index_distr);
    let trajectory = parsec_data.get_trajectory_via_index(mass_index);
    let age_in_years = rng.sample(age_dist);
    ParsecData::get_star_data_if_visible(trajectory, age_in_years, pos)
}

fn get_pos_distribution(max_distance: Distance<f64>) -> Uniform<f64> {
    Uniform::new(-max_distance.m, max_distance.m)
}

fn get_mass_distribution() -> WeightedIndex<f64> {
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

fn get_age_distribution() -> Uniform<f64> {
    Uniform::new(0., AGE_OF_MILKY_WAY_THIN_DISK.to_yr())
}

fn random_point_in_sphere(
    rng: &mut ThreadRng,
    distr: &Uniform<f64>,
    max_distance: Distance<f64>,
) -> CartesianCoordinates {
    let (mut x, mut y, mut z) = (rng.sample(distr), rng.sample(distr), rng.sample(distr));
    while x * x + y * y + z * z > max_distance.m * max_distance.m {
        (x, y, z) = (rng.sample(distr), rng.sample(distr), rng.sample(distr));
    }
    let x = Distance { m: x };
    let y = Distance { m: y };
    let z = Distance { m: z };
    CartesianCoordinates::new(x, y, z)
}

pub(crate) fn random_direction(rng: &mut ThreadRng) -> Direction {
    let distr = Uniform::new(-1., 1.);
    let mut point = random_point_in_sphere(rng, &distr, Distance { m: 1. });
    let mut dir = point.to_direction();
    while dir.is_err() {
        point = random_point_in_sphere(rng, &distr, Distance { m: 1. });
        dir = point.to_direction();
    }
    dir.unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{
        astro_display::AstroDisplay, tests::eq,
        units::illuminance::illuminance_to_apparent_magnitude,
    };

    use super::*;
    use std::time::Instant;

    #[test]
    fn dimmest_illuminance_is_magnitude_6_5() {
        let dimmest = illuminance_to_apparent_magnitude(&DIMMEST_ILLUMINANCE);
        assert!(eq(dimmest, 6.5));
    }

    #[test]
    #[ignore]
    fn generate_random_stars_stress_test() {
        let _ = PARSEC_DATA.lock(); // Load the parsec data.

        let max_distance = Distance::from_lyr(10000.);
        let max_seconds = 60;

        let start = Instant::now();
        let stars = generate_random_stars(max_distance).unwrap();
        let duration = start.elapsed();
        println!(
            "Generated {} stars within {} in {:?}",
            stars.len(),
            max_distance.astro_display(),
            duration
        );
        assert!(stars.len() > 1000);
        assert!(duration.as_secs() < max_seconds);
    }

    #[test]
    fn generating_a_distant_random_star() {
        let max_distance = Distance::from_lyr(1000.);
        let _ = generate_random_star(Some(max_distance)).unwrap();
    }

    #[test]
    fn generated_stars_are_not_further_away_than_max_distance() {
        let max_distance = Distance::from_lyr(100.);
        let stars = generate_random_stars(max_distance).unwrap();
        for star in stars {
            assert!(star.distance < max_distance * 1.01);
        }
    }
}
