use astro_coords::cartesian::Cartesian;
use parsec_access::getters::{
    get_closest_age_index, get_masses_in_solar, get_parameters, get_trajectory,
};
use simple_si_units::base::{Distance, Luminosity, Mass, Time};

use crate::{
    stars::fate::TYPE_II_SUPERNOVA_PEAK_MAGNITUDE,
    units::{
        distance::DISTANCE_ZERO,
        luminous_intensity::{
            absolute_magnitude_to_luminous_intensity, LUMINOSITY_ZERO, SOLAR_LUMINOUS_INTENSITY,
        },
    },
};

use super::random_stars::{
    get_min_age, number_in_sphere, AGE_OF_MILKY_WAY_THIN_DISK, DIMMEST_ILLUMINANCE,
    METALLICITY_INDEX, NUMBER_OF_STARS_FORMED_IN_NURSERY, STARS_PER_LY_CUBED, STELLAR_VELOCITY,
};

pub(super) struct GenerationParams {
    pub(super) pos: Cartesian,
    pub(super) max_age: Time<f64>,
    pub(super) radius: Distance<f64>,
    pub(super) number: usize,
}

impl GenerationParams {
    pub(super) fn old_stars(max_distance: Distance<f64>) -> Self {
        let pos = Cartesian::ORIGIN;
        let max_age = AGE_OF_MILKY_WAY_THIN_DISK;
        let radius = max_distance;
        let number = number_in_sphere(STARS_PER_LY_CUBED, radius);
        GenerationParams {
            pos,
            max_age,
            radius,
            number,
        }
    }

    pub(super) fn nursery(pos: Cartesian, max_age: Time<f64>) -> Self {
        let radius = STELLAR_VELOCITY * max_age;
        let number = NUMBER_OF_STARS_FORMED_IN_NURSERY;
        GenerationParams {
            pos,
            max_age,
            radius,
            number,
        }
    }

    pub(super) fn adjust_distance_for_performance(&mut self) {
        let original_radius = self.radius;
        let most_luminous_intensity = self.get_most_luminous_intensity_possible();
        let required_distance = Distance {
            m: (most_luminous_intensity.cd / DIMMEST_ILLUMINANCE.lux).sqrt(),
        };
        let distance_to_origin = self.pos.length();
        let closest_possible = distance_to_origin - self.radius;
        let farthest_possible = distance_to_origin + self.radius;
        if distance_to_origin > self.radius {
            if closest_possible > required_distance {
                self.radius = DISTANCE_ZERO;
            }
        } else if farthest_possible > required_distance {
            self.radius = required_distance - distance_to_origin
        }
        self.number = (self.number as f64 * (self.radius / original_radius).powi(3)) as usize;
    }

    pub(crate) fn get_most_luminous_intensity_possible(&self) -> Luminosity<f64> {
        let mut max_luminous_intensity = LUMINOSITY_ZERO;
        let max_age = self.max_age;
        let min_age = get_min_age(max_age);
        let masses = get_masses_in_solar(METALLICITY_INDEX);
        for (mass_index, _mass) in masses.iter().enumerate() {
            let trajectory = get_trajectory(METALLICITY_INDEX, mass_index);
            if min_age > trajectory.lifetime {
                continue;
            }
            if trajectory.initial_mass > Mass::from_solar_mass(8.)
                && (min_age..max_age).contains(&trajectory.lifetime)
            {
                return absolute_magnitude_to_luminous_intensity(TYPE_II_SUPERNOVA_PEAK_MAGNITUDE);
            }
            let min_age_index = get_closest_age_index(METALLICITY_INDEX, mass_index, min_age);
            let max_age_index = get_closest_age_index(METALLICITY_INDEX, mass_index, max_age);
            for age_index in min_age_index..=max_age_index {
                let params = get_parameters(METALLICITY_INDEX, mass_index, age_index);
                let luminous_intensity = params.luminosity_in_solar * SOLAR_LUMINOUS_INTENSITY;
                if luminous_intensity > max_luminous_intensity {
                    max_luminous_intensity = luminous_intensity;
                }
            }
        }
        max_luminous_intensity
    }
}

#[cfg(test)]
mod tests {
    use astro_coords::direction::Direction;
    use parsec_access::getters::is_data_ready;

    use crate::{
        tests::{eq_within, TEST_ACCURACY},
        units::time::TEN_MILLENIA,
    };

    use super::*;

    #[test]
    fn large_distance_for_old_stars_is_adjusted() {
        let max_distance = Distance::from_lyr(10_000.);
        let mut params = GenerationParams::old_stars(max_distance);
        assert!(is_data_ready());
        params.adjust_distance_for_performance();
        assert!(params.radius < max_distance);
    }

    #[test]
    fn short_distance_for_old_stars_is_not_adjusted() {
        let max_distance = Distance::from_lyr(10.);
        let mut params = GenerationParams::old_stars(max_distance);
        assert!(is_data_ready());
        params.adjust_distance_for_performance();
        assert!(eq_within(
            params.radius.to_lyr(),
            max_distance.to_lyr(),
            TEST_ACCURACY
        ));
    }

    #[test]
    fn old_stars_far_away_are_adjusted() {
        let max_age = AGE_OF_MILKY_WAY_THIN_DISK;
        let origin = Direction::Z.to_cartesian(Distance::from_lyr(10_000.));
        let mut params = GenerationParams::nursery(origin, max_age);
        assert!(is_data_ready());
        params.adjust_distance_for_performance();
        assert!(params.radius.m < 1.);
    }

    #[test]
    fn young_stars_far_away_are_not_adjusted() {
        let max_age = TEN_MILLENIA;
        let origin = Direction::Z.to_cartesian(Distance::from_lyr(1000.));
        let mut params = GenerationParams::nursery(origin, max_age);
        let max_distance = params.radius;
        assert!(is_data_ready());
        params.adjust_distance_for_performance();
        assert!(eq_within(
            params.radius.to_lyr(),
            max_distance.to_lyr(),
            TEST_ACCURACY
        ));
    }
}
