use super::parsec::data::ParsecData;
use crate::{
    coordinates::{cartesian::CartesianCoordinates, direction::Direction},
    error::AstroUtilError,
    stars::{
        data::StarData,
        random::parsec::{data::PARSEC_DATA, distributions::ParsecDistribution},
    },
    units::{distance::DISTANCE_ZERO, time::TEN_MILLENIA},
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
const STARS_PER_LY_CUBED: f64 = 3e-4;
// https://ui.adsabs.harvard.edu/abs/1985ApJ...289..373S/abstract
// 6000 star forming regions are currently in the milky way
// Assuming that they form stars at a constant rate for 10 million years, and that the milky way is about 10 billion years old,
// this adds a factor of 1000.
const NURSERY_LIFETIME: Time<f64> = Time {
    s: 10_000_000. * 365.25 * 24. * 60. * 60.,
}; // 10_000_000 years
pub(super) const AGE_OF_MILKY_WAY_THIN_DISK: Time<f64> = Time {
    s: 8.8e9 * 365.25 * 24. * 3600.,
};
const NURSERIES_PER_LY_CUBED: f64 =
    6_000. / 8e12 * AGE_OF_MILKY_WAY_THIN_DISK.s / NURSERY_LIFETIME.s;
const NUMBER_OF_STARS_FORMED_IN_NURSERY: usize = 20_000;
const STELLAR_VELOCITY: Velocity<f64> = Velocity { mps: 20_000. };
pub(super) const DIMMEST_ILLUMINANCE: Illuminance<f64> = Illuminance { lux: 6.5309e-9 };

pub fn generate_random_stars(max_distance: Distance<f64>) -> Result<Vec<StarData>, AstroUtilError> {
    let parsec_data_mutex = PARSEC_DATA
        .lock()
        .map_err(|_| AstroUtilError::MutexPoison)?;
    let parsec_data = parsec_data_mutex.as_ref()?;
    let parsec_distr = ParsecDistribution::new();

    let number_star_forming_regions = number_in_sphere(NURSERIES_PER_LY_CUBED, max_distance) + 1;
    let age_distribution = Uniform::new(0., AGE_OF_MILKY_WAY_THIN_DISK.s);
    let stars = (0..number_star_forming_regions)
        .into_par_iter()
        .map(|i| {
            let mut rng = rand::thread_rng();
            if i == 0 {
                let origin = CartesianCoordinates::ORIGIN;
                let max_age = AGE_OF_MILKY_WAY_THIN_DISK;
                let min_age = min_age(max_age);
                let adjusted_distance = distance_adjusted_for_performance(
                    parsec_data,
                    min_age,
                    max_age,
                    &origin,
                    max_distance,
                );
                let number_of_stars_in_sphere =
                    number_in_sphere(STARS_PER_LY_CUBED, adjusted_distance);
                generate_certain_number_of_random_stars(
                    number_of_stars_in_sphere,
                    parsec_data,
                    &origin,
                    adjusted_distance,
                    max_age,
                    &parsec_distr,
                )
            } else {
                let nursery_pos = random_point_in_sphere(&mut rng, max_distance);
                let nursery_age = Time {
                    s: rng.sample(age_distribution),
                };
                let nursery_radius = STELLAR_VELOCITY * nursery_age;
                generate_certain_number_of_random_stars(
                    NUMBER_OF_STARS_FORMED_IN_NURSERY,
                    parsec_data,
                    &nursery_pos,
                    nursery_radius,
                    nursery_age,
                    &parsec_distr,
                )
            }
        })
        .flatten()
        .collect();
    Ok(stars)
}

fn min_age(max_age: Time<f64>) -> Time<f64> {
    max_age - NURSERY_LIFETIME - TEN_MILLENIA
}

fn number_in_sphere(num_per_lyr: f64, max_distance: Distance<f64>) -> usize {
    (num_per_lyr * 4. / 3. * PI * max_distance.to_lyr().powi(3)) as usize
}

fn generate_certain_number_of_random_stars(
    number: usize,
    parsec_data: &ParsecData,
    origin: &CartesianCoordinates,
    max_distance: Distance<f64>,
    max_age: Time<f64>,
    parsec_distr: &ParsecDistribution,
) -> Vec<StarData> {
    let age_distribution = Uniform::new(0., NURSERY_LIFETIME.s);
    (0..=number)
        .into_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let age = max_age
                - Time {
                    s: rng.sample(age_distribution),
                };
            generate_visible_random_star(
                parsec_data,
                origin,
                max_distance,
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

fn distance_adjusted_for_performance(
    parsec_data: &ParsecData,
    min_age: Time<f64>,
    max_age: Time<f64>,
    origin: &CartesianCoordinates,
    max_distance_from_origin: Distance<f64>,
) -> Distance<f64> {
    let most_luminous_intensity =
        parsec_data.get_most_luminous_intensity_possible(min_age, max_age);
    let required_distance = Distance {
        m: (most_luminous_intensity.cd / DIMMEST_ILLUMINANCE.lux).sqrt(),
    };
    let distance_to_origin = origin.length();
    let closest_possible = distance_to_origin - max_distance_from_origin;
    let farthest_possible = distance_to_origin + max_distance_from_origin;
    if distance_to_origin > max_distance_from_origin {
        if closest_possible > required_distance {
            DISTANCE_ZERO
        } else {
            max_distance_from_origin
        }
    } else {
        if farthest_possible > required_distance {
            required_distance - distance_to_origin
        } else {
            max_distance_from_origin
        }
    }
}

#[cfg(test)]
mod tests {
    use simple_si_units::base::Mass;

    use crate::{
        astro_display::AstroDisplay,
        stars::fate::StarFate,
        tests::{eq, eq_within, TEST_ACCURACY},
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

    #[test]
    fn large_distance_for_old_stars_is_adjusted() {
        let min_age = AGE_OF_MILKY_WAY_THIN_DISK - NURSERY_LIFETIME - TEN_MILLENIA;
        let max_age = AGE_OF_MILKY_WAY_THIN_DISK;
        let origin = CartesianCoordinates::ORIGIN;
        let max_distance = Distance::from_lyr(10_000.);
        let adjusted = {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            distance_adjusted_for_performance(parsec_data, min_age, max_age, &origin, max_distance)
        };
        assert!(adjusted < max_distance);
    }

    #[test]
    fn short_distance_for_old_stars_is_not_adjusted() {
        let min_age = AGE_OF_MILKY_WAY_THIN_DISK - NURSERY_LIFETIME - TEN_MILLENIA;
        let max_age = AGE_OF_MILKY_WAY_THIN_DISK;
        let origin = CartesianCoordinates::ORIGIN;
        let max_distance = Distance::from_lyr(10.);
        let adjusted = {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            distance_adjusted_for_performance(parsec_data, min_age, max_age, &origin, max_distance)
        };
        assert!(eq_within(
            adjusted.to_lyr(),
            max_distance.to_lyr(),
            TEST_ACCURACY
        ));
    }

    #[test]
    fn old_stars_far_away_are_adjusted() {
        let min_age = AGE_OF_MILKY_WAY_THIN_DISK - NURSERY_LIFETIME - TEN_MILLENIA;
        let max_age = AGE_OF_MILKY_WAY_THIN_DISK;
        let origin = Direction::Z.to_cartesian(Distance::from_lyr(10_000.));
        let max_distance = Distance::from_lyr(100.);
        let adjusted = {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            distance_adjusted_for_performance(parsec_data, min_age, max_age, &origin, max_distance)
        };
        assert!(adjusted.m < 1.);
    }

    #[test]
    fn young_stars_far_away_are_not_adjusted() {
        let min_age = TIME_ZERO;
        let max_age = NURSERY_LIFETIME;
        let origin = Direction::Z.to_cartesian(Distance::from_lyr(1000.));
        let max_distance = Distance::from_lyr(100.);
        let adjusted = {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            distance_adjusted_for_performance(parsec_data, min_age, max_age, &origin, max_distance)
        };
        assert!(eq_within(
            adjusted.to_lyr(),
            max_distance.to_lyr(),
            TEST_ACCURACY
        ));
    }
}
