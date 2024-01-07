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
use rand::{rngs::ThreadRng, Rng};

// https://en.wikipedia.org/wiki/Stellar_density
const STARS_PER_LY_CUBED: Float = 0.004;
const DIMMEST_VISIBLE_MAGNITUDE: Float = 6.5;

pub fn generate_random_stars(max_distance: Length) -> Result<Vec<Star>, AstroUtilError> {
    let parsec_data = ParsecData::new()?;
    let number_of_stars_in_cube = STARS_PER_LY_CUBED * (max_distance * 2.).as_light_years().powi(3);
    let number_of_stars_in_cube = number_of_stars_in_cube as usize;
    let mut stars = Vec::new();
    for _ in 0..number_of_stars_in_cube {
        if let Some(star) = generate_visible_random_star(&parsec_data, max_distance) {
            stars.push(star);
        }
    }
    Ok(stars)
}

fn generate_visible_random_star(parsec_data: &ParsecData, max_distance: Length) -> Option<Star> {
    let pos = generate_random_3d_position(max_distance);
    let distance = pos.length();
    if distance > max_distance {
        return None;
    }
    let mass = generate_random_mass();
    let trajectory = parsec_data.get_trajectory(mass.as_solar_masses());
    let current_params = pick_random_age(&trajectory);
    let mut star = current_params.to_star_at_origin(mass);

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
    let max_number = trajectory.len() - 1;
    let i = rng.gen_range(0..=max_number);
    &trajectory[i]
}

fn generate_random_3d_position(max_distance: Length) -> CartesianCoordinates {
    todo!("Implement generate_random_3d_position")
}

// Selpeter Law
fn generate_random_mass() -> Mass {
    todo!("Implement generate_random_mass")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::stars::STARS_TO_TWO_POINT_FIVE_APPARENT_MAG;

    // #[test]
    // fn test_calulate_star() {
    //     for data in STARS_TO_TWO_POINT_FIVE_APPARENT_MAG.iter() {
    //         if let Some(age) = data.age {
    //             let mass = data.mass;
    //             let calculated_star = calulate_star_at_origin(mass, age);
    //             let real_star = data.to_star();
    //             println!("calculated_star: {:?}", calculated_star);
    //             println!("real_star: {:?}", real_star);
    //             assert!(calculated_star.similar_within_order_of_magnitude(&real_star));
    //         }
    //     }
    // }
}
