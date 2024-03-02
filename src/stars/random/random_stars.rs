use super::parsec::data::ParsecData;
use crate::{
    coordinates::{cartesian::CartesianCoordinates, direction::Direction},
    error::AstroUtilError,
    stars::{
        data::StarData,
        fate::StarFate,
        random::parsec::{data::PARSEC_DATA, distributions::ParsecDistribution},
    },
};
use rand::{distributions::Uniform, rngs::ThreadRng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use simple_si_units::{
    base::{Distance, Time},
    electromagnetic::Illuminance,
};
use std::f64::consts::PI;

// https://en.wikipedia.org/wiki/Stellar_density
// Adjusted, because Gaia does not resolve all binaries.
const STARS_PER_LY_CUBED: f64 = 0.004 / 1.12;
pub(super) const DIMMEST_ILLUMINANCE: Illuminance<f64> = Illuminance { lux: 6.5309e-9 };
pub(super) const AGE_OF_MILKY_WAY_THIN_DISK: Time<f64> = Time {
    s: 8.8e9 * 365.25 * 24. * 3600.,
};
// pub(super) const AGE_OF_UNIVERSE: Time<f64> = Time {
//     s: 1.38e10 * 365.25 * 24. * 3600.,
// };

pub fn generate_random_stars(max_distance: Distance<f64>) -> Result<Vec<StarData>, AstroUtilError> {
    let parsec_data_mutex = PARSEC_DATA
        .lock()
        .map_err(|_| AstroUtilError::MutexPoison)?;
    let parsec_data = parsec_data_mutex.as_ref()?;
    let parsec_distr = ParsecDistribution::new();

    let mut stars = vec![];
    let mut new_stars = generate_random_stars_in_sphere(
        parsec_data,
        &CartesianCoordinates::ORIGIN,
        max_distance,
        AGE_OF_MILKY_WAY_THIN_DISK,
        &parsec_distr,
    )?;

    loop {
        let supernovae = collect_supernovae(&new_stars);
        stars.append(&mut new_stars);
        if supernovae.is_empty() {
            break;
        }
        println!(
            "Generated {} new supernovae. Total star count {}",
            supernovae.len(),
            stars.len()
        );
        for supernova in supernovae {
            let mut stars_near_remnant =
                generate_stars_around_supernova_remnant(&supernova, parsec_data, &parsec_distr)?;
            new_stars.append(&mut stars_near_remnant);
        }
    }
    Ok(stars)
}

fn generate_stars_around_supernova_remnant(
    remnant: &StarData,
    parsec_data: &ParsecData,
    parsec_distr: &ParsecDistribution,
) -> Result<Vec<StarData>, AstroUtilError> {
    const TIME_UNTIL_STAR_FORMATION: Time<f64> = Time {
        s: 10_000. * 365.25 * 24. * 60. * 60.,
    }; // 10_000 years
    const DISTANCE_OF_STAR_FORMATION: Distance<f64> = Distance { m: 100. * 9.461e15 }; // 100 lyr

    let origin = remnant.pos.clone();
    let age =
        remnant.get_age_at_epoch().unwrap() - remnant.get_lifetime() - TIME_UNTIL_STAR_FORMATION;
    if age < -TIME_UNTIL_STAR_FORMATION {
        return Ok(vec![]);
    }
    generate_random_stars_in_sphere(
        parsec_data,
        &origin,
        DISTANCE_OF_STAR_FORMATION,
        age,
        parsec_distr,
    )
}

fn generate_random_stars_in_sphere(
    parsec_data: &ParsecData,
    origin: &CartesianCoordinates,
    max_distance: Distance<f64>,
    age: Time<f64>,
    parsec_distr: &ParsecDistribution,
) -> Result<Vec<StarData>, AstroUtilError> {
    const MAX_CHUNKSIZE: usize = 100_000_000;

    let number_of_stars_in_sphere = number_of_stars_in_sphere(max_distance);
    let mut remaining = number_of_stars_in_sphere;
    let mut stars = Vec::new();
    while remaining > MAX_CHUNKSIZE {
        let chunk = generate_certain_number_of_random_stars(
            MAX_CHUNKSIZE,
            parsec_data,
            origin,
            max_distance,
            age,
            &parsec_distr,
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
        origin,
        max_distance,
        age,
        &parsec_distr,
    );
    stars.extend(chunk);

    Ok(stars)
}

fn number_of_stars_in_sphere(max_distance: Distance<f64>) -> usize {
    let number_of_stars_in_sphere =
        STARS_PER_LY_CUBED * 4. / 3. * PI * max_distance.to_lyr().powi(3);
    let number_of_stars_in_sphere = number_of_stars_in_sphere as usize;
    number_of_stars_in_sphere
}

fn generate_certain_number_of_random_stars(
    number: usize,
    parsec_data: &ParsecData,
    origin: &CartesianCoordinates,
    max_distance: Distance<f64>,
    age: Time<f64>,
    parsec_distr: &ParsecDistribution,
) -> Vec<StarData> {
    (0..=number)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
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

fn collect_supernovae(stars: &Vec<StarData>) -> Vec<StarData> {
    stars
        .iter()
        .filter(|s| s.get_fate() == &StarFate::TypeIISupernova)
        .cloned()
        .collect()
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

        let max_distance = Distance::from_lyr(4000.);
        let max_seconds = 60;

        let start = Instant::now();
        let stars = generate_random_stars(max_distance).unwrap();
        let duration = start.elapsed();
        let number_of_visible_stars = stars.len() - collect_supernovae(&stars).len();
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
    fn about_2_permille_of_random_stars_have_gone_supernova() {
        let max_distance = Distance::from_lyr(1000.);
        let star_data: Vec<StarData> = generate_random_stars(max_distance).unwrap();
        let supernova_stars = collect_supernovae(&star_data).len();
        let total_stars = number_of_stars_in_sphere(max_distance);
        assert!(total_stars > 0);
        let portion = supernova_stars as f64 / total_stars as f64;
        assert!(
            (0.001..0.003).contains(&portion),
            "Generated {} stars, {} of which have gone supernova. This corresponds to a portion of {}",
            total_stars,
            supernova_stars,
            portion
        );
    }
}
