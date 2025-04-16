use astro_coords::cartesian::Cartesian;
use uom::si::f64::{Length, Time};

use crate::units::length::solar_radii;

use super::{
    parsec::getters::get_most_luminous_intensity_possible,
    random_stars::{
        age_of_milky_way_thin_disk, dimmest_illuminance, number_in_sphere, stellar_velocity,
        NUMBER_OF_STARS_FORMED_IN_NURSERY, STARS_PER_LY_CUBED,
    },
};

pub(super) struct GenerationParams {
    pub(super) pos: Cartesian,
    pub(super) max_age: Time,
    pub(super) radius: Length,
    pub(super) number: usize,
}

impl GenerationParams {
    pub(super) fn old_stars(max_distance: Length) -> Self {
        let pos = Cartesian::origin();
        let max_age = age_of_milky_way_thin_disk();
        let radius = max_distance;
        let number = number_in_sphere(STARS_PER_LY_CUBED, radius);
        GenerationParams {
            pos,
            max_age,
            radius,
            number,
        }
    }

    pub(super) fn nursery(pos: Cartesian, max_age: Time) -> Self {
        let radius = stellar_velocity() * max_age;
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
        let most_luminous_intensity = get_most_luminous_intensity_possible(self.max_age);
        let required_distance = Length {
            m: (most_luminous_intensity.cd / dimmest_illuminance().get::<lux>()).sqrt(),
        };
        let distance_to_origin = self.pos.length();
        let closest_possible = distance_to_origin - self.radius;
        let farthest_possible = distance_to_origin + self.radius;
        if distance_to_origin > self.radius {
            if closest_possible > required_distance {
                self.radius = Length::new::<solar_radii>(0.);
            }
        } else if farthest_possible > required_distance {
            self.radius = required_distance - distance_to_origin
        }
        self.number = (self.number as f64 * (self.radius / original_radius).powi(3)) as usize;
    }
}

#[cfg(test)]
mod tests {
    use astro_coords::direction::Direction;
    use parsec_access::getters::is_data_ready;
    use uom::si::length::light_year;

    use crate::{
        tests::{eq_within, TEST_ACCURACY},
        units::time::kiloyear,
    };

    use super::*;

    #[test]
    fn large_distance_for_old_stars_is_adjusted() {
        let max_distance = Length::new::<light_year>(10_000.);
        let mut params = GenerationParams::old_stars(max_distance);
        assert!(is_data_ready());
        params.adjust_distance_for_performance();
        assert!(params.radius < max_distance);
    }

    #[test]
    fn short_distance_for_old_stars_is_not_adjusted() {
        let max_distance = Length::new::<light_year>(10.);
        let mut params = GenerationParams::old_stars(max_distance);
        assert!(is_data_ready());
        params.adjust_distance_for_performance();
        assert!(eq_within(
            params.radius.get::<light_year>(),
            max_distance.get::<light_year>(),
            TEST_ACCURACY
        ));
    }

    #[test]
    fn old_stars_far_away_are_adjusted() {
        let max_age = age_of_milky_way_thin_disk;
        let origin = Direction::Z.to_cartesian(Length::new::<light_year>(10_000.));
        let mut params = GenerationParams::nursery(origin, max_age);
        assert!(is_data_ready());
        params.adjust_distance_for_performance();
        assert!(params.radius.m < 1.);
    }

    #[test]
    fn young_stars_far_away_are_not_adjusted() {
        let max_age = Time::new::<kiloyear>(10.);
        let origin = Direction::Z.to_cartesian(Length::new::<light_year>(1000.));
        let mut params = GenerationParams::nursery(origin, max_age);
        let max_distance = params.radius;
        assert!(is_data_ready());
        params.adjust_distance_for_performance();
        assert!(eq_within(
            params.radius.get::<light_year>(),
            max_distance.get::<light_year>(),
            TEST_ACCURACY
        ));
    }
}
