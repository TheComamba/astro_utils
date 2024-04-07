use simple_si_units::base::{Distance, Time};

use crate::{coordinates::cartesian::CartesianCoordinates, units::distance::DISTANCE_ZERO};

use super::{
    parsec::data::ParsecData,
    random_stars::{
        number_in_sphere, AGE_OF_MILKY_WAY_THIN_DISK, DIMMEST_ILLUMINANCE,
        NUMBER_OF_STARS_FORMED_IN_NURSERY, STARS_PER_LY_CUBED, STELLAR_VELOCITY,
    },
};

pub(super) struct GenerationParams {
    pub(super) pos: CartesianCoordinates,
    pub(super) max_age: Time<f64>,
    pub(super) radius: Distance<f64>,
    pub(super) number: usize,
}

impl GenerationParams {
    pub(super) fn old_stars(max_distance: Distance<f64>) -> Self {
        let pos = CartesianCoordinates::ORIGIN;
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

    pub(super) fn nursery(pos: CartesianCoordinates, max_age: Time<f64>) -> Self {
        let radius = STELLAR_VELOCITY * max_age;
        let number = NUMBER_OF_STARS_FORMED_IN_NURSERY;
        GenerationParams {
            pos,
            max_age,
            radius,
            number,
        }
    }

    pub(super) fn adjust_distance_for_performance(&mut self, parsec_data: &ParsecData) {
        let original_radius = self.radius;
        let most_luminous_intensity =
            parsec_data.get_most_luminous_intensity_possible(self.max_age);
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
}

#[cfg(test)]
mod tests {
    use crate::{
        coordinates::direction::Direction,
        stars::random::parsec::data::PARSEC_DATA,
        tests::{eq_within, TEST_ACCURACY},
        units::time::TEN_MILLENIA,
    };

    use super::*;

    #[test]
    fn large_distance_for_old_stars_is_adjusted() {
        let max_distance = Distance::from_lyr(10_000.);
        let mut params = GenerationParams::old_stars(max_distance);
        {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            params.adjust_distance_for_performance(parsec_data)
        };
        assert!(params.radius < max_distance);
    }

    #[test]
    fn short_distance_for_old_stars_is_not_adjusted() {
        let max_distance = Distance::from_lyr(10.);
        let mut params = GenerationParams::old_stars(max_distance);
        {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            params.adjust_distance_for_performance(parsec_data)
        };
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
        {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            params.adjust_distance_for_performance(parsec_data)
        };
        assert!(params.radius.m < 1.);
    }

    #[test]
    fn young_stars_far_away_are_not_adjusted() {
        let max_age = TEN_MILLENIA;
        let origin = Direction::Z.to_cartesian(Distance::from_lyr(1000.));
        let mut params = GenerationParams::nursery(origin, max_age);
        let max_distance = params.radius;
        {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            params.adjust_distance_for_performance(parsec_data)
        };
        assert!(eq_within(
            params.radius.to_lyr(),
            max_distance.to_lyr(),
            TEST_ACCURACY
        ));
    }
}
