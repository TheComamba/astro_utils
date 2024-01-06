use crate::{
    coordinates::cartesian::CartesianCoordinates,
    stellar_properties::StellarProperties,
    units::{
        illuminance::Illuminance, length::Length, luminosity::Luminosity, mass::Mass, time::Time,
    },
    Float,
};

// https://en.wikipedia.org/wiki/Stellar_density
const STARS_PER_LY_CUBED: Float = 0.004;
const DIMMEST_VISIBLE_MAGNITUDE: Float = 6.5;

pub fn generate_random_starts(max_distance: Length) -> Vec<StellarProperties> {
    let number_of_stars_in_cube = STARS_PER_LY_CUBED * (max_distance * 2.).as_light_years().powi(3);
    let number_of_stars_in_cube = number_of_stars_in_cube as usize;
    let mut stars = Vec::new();
    for _ in 0..number_of_stars_in_cube {
        if let Some(star) = generate_visible_random_star(max_distance) {
            stars.push(star);
        }
    }
    stars
}

fn generate_visible_random_star(max_distance: Length) -> Option<StellarProperties> {
    let pos = generate_random_3d_position(max_distance);
    if pos.length() > max_distance {
        return None;
    }
    let star = generate_random_star(max_distance);
    let apparent_magnitude = star.get_absolute_magnitude().to_illuminance(&pos.length());
    if apparent_magnitude.as_apparent_magnitude() > DIMMEST_VISIBLE_MAGNITUDE {
        return None;
    }
    // star.pos = pos;
    Some(star)
}

fn generate_random_star(max_distance: Length) -> StellarProperties {
    let mass = generate_random_mass();
    let luminosity = calulate_luminosity(mass);
    let age_expectancy = calculate_age_axpectancy(mass, luminosity);
    let age = generate_random_age(age_expectancy);
    todo!("Implement generate_random_star")
}

fn generate_random_3d_position(max_distance: Length) -> CartesianCoordinates {
    todo!("Implement generate_random_3d_position")
}

// Selpeter Law
fn generate_random_mass() -> Mass {
    todo!("Implement generate_random_mass")
}

// https://en.wikipedia.org/wiki/Mass%E2%80%93luminosity_relation
fn calulate_luminosity(mass: Mass) -> Luminosity {
    todo!("Implement generate_random_luminosity")
}

// Nuclear timescale
fn calculate_age_axpectancy(mass: Mass, luminosity: Luminosity) -> Time {
    todo!("Implement generate_random_luminosity")
}

fn generate_random_age(age_expectancy: Time) -> Time {
    // const MAX_AGE: Time = Time::from_years(10e9);
    todo!("Implement generate_random_age")
}
