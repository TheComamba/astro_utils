use super::star::Star;
use crate::{
    coordinates::cartesian::CartesianCoordinates,
    units::{length::Length, luminosity::Luminosity, mass::Mass, time::Time},
    Float,
};

// https://en.wikipedia.org/wiki/Stellar_density
const STARS_PER_LY_CUBED: Float = 0.004;
const DIMMEST_VISIBLE_MAGNITUDE: Float = 6.5;

pub fn generate_random_starts(max_distance: Length) -> Vec<Star> {
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

fn generate_visible_random_star(max_distance: Length) -> Option<Star> {
    let pos = generate_random_3d_position(max_distance);
    let distance = pos.length();
    if distance > max_distance {
        return None;
    }
    let (mass, luminosity) = generate_random_star_ting_point();
    if luminosity.to_illuminance(&distance).as_apparent_magnitude() > DIMMEST_VISIBLE_MAGNITUDE {
        return None;
    }
    let age_expectancy = calculate_age_axpectancy(mass, luminosity);
    let age = generate_random_age(age_expectancy);
    let mut star = calulate_star_at_origin(mass,  age);
    star.distance = distance;
    star.direction_in_ecliptic = pos.to_direction();
    Some(star)
}

fn generate_random_star_ting_point() -> (Mass, Luminosity) {
    let mass = generate_random_mass();
    let luminosity = calulate_luminosity(mass);
    (mass, luminosity)
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
fn calculate_age_axpectancy(mass: Mass) -> Time {
    todo!("Implement generate_random_luminosity")
}

fn generate_random_age(age_expectancy: Time) -> Time {
    // const MAX_AGE: Time = Time::from_years(10e9);
    todo!("Implement generate_random_age")
}

fn calulate_star_at_origin(mass: Mass, age: Time) -> Star {
    todo!("Implement calulate_other_star_properties")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::stars::STARS_TO_TWO_POINT_FIVE_APPARENT_MAG;

    #[test]
    fn test_calulate_star() {
        for data in STARS_TO_TWO_POINT_FIVE_APPARENT_MAG.iter() {
            if let Some(age) = data.age {
                let mass = data.mass;
                let calculated_star = calulate_star_at_origin(mass,  age);
                let real_star = data.to_star();
                println!("calculated_star: {:?}", calculated_star);
                println!("real_star: {:?}", real_star);
                assert!(calculated_star.similar_within_order_of_magnitude(&real_star));
            }
        }
    }
}
