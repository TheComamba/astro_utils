use super::{params::GenerationParams, parsec::data::ParsecData};
use crate::{
    coordinates::{cartesian::CartesianCoordinates, direction::Direction},
    error::AstroUtilError,
    stars::{
        data::StarData,
        random::parsec::{data::PARSEC_DATA, distributions::ParsecDistribution},
    },
    units::time::TEN_MILLENIA,
};
use rand::{distributions::Uniform, rngs::ThreadRng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use simple_si_units::{
    base::{Distance, Time},
    electromagnetic::Illuminance,
    mechanical::Velocity,
};
use std::f64::consts::PI;

// https://en.wikipedia.org/wiki/Stellar_density
// But more or less arbitrarily adjusted to reproduce Gaia data.
pub(super) const STARS_PER_LY_CUBED: f64 = 3e-3;
// https://ui.adsabs.harvard.edu/abs/1985ApJ...289..373S/abstract
// 6000 star forming regions are currently in the milky way
pub(super) const NURSERY_LIFETIME: Time<f64> = Time {
    s: 2e7 * 365.25 * 24. * 60. * 60.,
};
pub(super) const AGE_OF_MILKY_WAY_THIN_DISK: Time<f64> = Time {
    s: 8.8e9 * 365.25 * 24. * 3600.,
};
const NURSERIES_PER_LY_CUBED: f64 =
    6_000. / 8e12 * AGE_OF_MILKY_WAY_THIN_DISK.s / NURSERY_LIFETIME.s;
pub(super) const NUMBER_OF_STARS_FORMED_IN_NURSERY: usize = 20_000;
pub(super) const STELLAR_VELOCITY: Velocity<f64> = Velocity { mps: 20_000. };
pub(super) const DIMMEST_ILLUMINANCE: Illuminance<f64> = Illuminance { lux: 6.5309e-9 };

pub fn generate_random_stars(max_distance: Distance<f64>) -> Result<Vec<StarData>, AstroUtilError> {
    let parsec_data_mutex = PARSEC_DATA
        .lock()
        .map_err(|_| AstroUtilError::MutexPoison)?;
    let parsec_data = parsec_data_mutex.as_ref()?;
    let parsec_distr = ParsecDistribution::new();

    let number_star_forming_regions = number_in_sphere(NURSERIES_PER_LY_CUBED, max_distance) + 1;
    let age_distribution = Uniform::new(0., AGE_OF_MILKY_WAY_THIN_DISK.s);
    println!(
        "Number of star forming regions: {}",
        number_star_forming_regions
    );
    let stars = (0..number_star_forming_regions)
        .into_par_iter()
        .map(|i| {
            let mut rng = rand::thread_rng();
            let mut params = if i == 0 {
                GenerationParams::old_stars(max_distance)
            } else {
                let pos = random_point_in_sphere(&mut rng, max_distance);
                let max_age = Time {
                    s: rng.sample(age_distribution),
                };
                GenerationParams::nursery(pos, max_age)
            };
            params.adjust_distance_for_performance(parsec_data);
            generate_random_stars_with_params(params, parsec_data, &parsec_distr)
        })
        .flatten()
        .collect();
    Ok(stars)
}

pub(crate) fn get_min_age(max_age: Time<f64>) -> Time<f64> {
    max_age - NURSERY_LIFETIME - TEN_MILLENIA
}

pub(super) fn number_in_sphere(num_per_lyr: f64, max_distance: Distance<f64>) -> usize {
    (num_per_lyr * 4. / 3. * PI * max_distance.to_lyr().powi(3)) as usize
}

fn generate_random_stars_with_params(
    params: GenerationParams,
    parsec_data: &ParsecData,
    parsec_distr: &ParsecDistribution,
) -> Vec<StarData> {
    let age_distribution = Uniform::new(0., NURSERY_LIFETIME.s);
    (0..=params.number)
        .into_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let age = params.max_age
                - Time {
                    s: rng.sample(age_distribution),
                };
            generate_visible_random_star(
                parsec_data,
                &params.pos,
                params.radius,
                age,
                &mut rng,
                parsec_distr,
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

    let parsec_data_mutex = PARSEC_DATA
        .lock()
        .map_err(|_| AstroUtilError::MutexPoison)?;
    let parsec_data = parsec_data_mutex.as_ref()?;
    let parsec_distr = ParsecDistribution::new();

    let mut star = generate_visible_random_star(
        parsec_data,
        &CartesianCoordinates::ORIGIN,
        max_distance_or_1,
        AGE_OF_MILKY_WAY_THIN_DISK,
        &mut rng,
        &parsec_distr,
    );
    while star.is_none() {
        star = generate_visible_random_star(
            parsec_data,
            &CartesianCoordinates::ORIGIN,
            max_distance_or_1,
            AGE_OF_MILKY_WAY_THIN_DISK,
            &mut rng,
            &parsec_distr,
        );
    }
    let mut star = star.unwrap();
    if max_distance.is_none() {
        star.pos = CartesianCoordinates::ORIGIN;
    }
    Ok(star)
}

fn generate_visible_random_star(
    parsec_data: &ParsecData,
    origin: &CartesianCoordinates,
    max_distance: Distance<f64>,
    age: Time<f64>,
    rng: &mut ThreadRng,
    parsec_distr: &ParsecDistribution,
) -> Option<StarData> {
    let mass_index = parsec_distr.get_random_mass_index(rng);
    let pos = origin + &random_point_in_sphere(rng, max_distance);
    let star = parsec_data.get_star_data_if_visible(mass_index, age, pos)?;
    Some(star)
}

fn random_point_in_unit_sphere(rng: &mut ThreadRng) -> CartesianCoordinates {
    let distr = Uniform::new(-1., 1.);
    let (mut x, mut y, mut z) = (rng.sample(distr), rng.sample(distr), rng.sample(distr));
    while x * x + y * y + z * z > 1. {
        (x, y, z) = (rng.sample(distr), rng.sample(distr), rng.sample(distr));
    }
    let x = Distance { m: x };
    let y = Distance { m: y };
    let z = Distance { m: z };
    CartesianCoordinates::new(x, y, z)
}

fn random_point_in_sphere(
    rng: &mut ThreadRng,
    max_distance: Distance<f64>,
) -> CartesianCoordinates {
    let point = random_point_in_unit_sphere(rng);
    point * max_distance.m
}

pub(crate) fn random_direction(rng: &mut ThreadRng) -> Direction {
    let mut point = random_point_in_unit_sphere(rng);
    let mut dir = point.to_direction();
    while dir.is_err() {
        point = random_point_in_unit_sphere(rng);
        dir = point.to_direction();
    }
    dir.unwrap()
}

#[cfg(test)]
mod tests {
    use simple_si_units::base::Mass;

    use crate::{
        astro_display::AstroDisplay,
        stars::fate::StarFate,
        tests::eq,
        units::{illuminance::illuminance_to_apparent_magnitude, time::TIME_ZERO},
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

        let max_distance = Distance::from_lyr(20_000.);
        let max_seconds = 60;

        let start = Instant::now();
        let stars = generate_random_stars(max_distance).unwrap();
        let duration = start.elapsed();
        let number_of_visible_stars = stars.len();
        println!(
            "Generated {} stars within {} in {:?}",
            number_of_visible_stars,
            max_distance.astro_display(),
            duration
        );
        assert!(number_of_visible_stars > 6_000);
        assert!(number_of_visible_stars < 24_000);
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
            assert!(star.get_distance_at_epoch() < max_distance * 1.01);
        }
    }

    #[test]
    fn random_stars_have_a_non_vanishing_lifetime() {
        let max_distance = Distance::from_lyr(500.);
        let star_data: Vec<StarData> = generate_random_stars(max_distance).unwrap();
        for star in star_data {
            assert!(star.get_lifetime() > TIME_ZERO);
        }
    }

    #[test]
    fn random_stars_below_8_sun_masses_become_white_dwarfs() {
        let max_distance = Distance::from_lyr(500.);
        let star_data: Vec<StarData> = generate_random_stars(max_distance).unwrap();
        for star in star_data {
            if star.mass.unwrap() < Mass::from_solar_mass(8.0) {
                assert_eq!(star.get_fate(), &StarFate::WhiteDwarf);
            }
        }
    }

    #[test]
    fn random_stars_above_8_sun_masses_go_supernova() {
        let max_distance = Distance::from_lyr(500.);
        let star_data: Vec<StarData> = generate_random_stars(max_distance).unwrap();
        for star in star_data {
            if star.mass.unwrap() > Mass::from_solar_mass(8.0) {
                assert_eq!(star.get_fate(), &StarFate::TypeIISupernova);
            }
        }
    }

    #[test]
    fn random_stars_have_an_age() {
        let max_distance = Distance::from_lyr(500.);
        let star_data: Vec<StarData> = generate_random_stars(max_distance).unwrap();
        for star in star_data {
            assert!(star.get_age_at_epoch().is_some());
        }
    }
}
