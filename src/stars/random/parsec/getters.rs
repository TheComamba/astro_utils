use astro_coords::cartesian::Cartesian;
use uom::si::f64::{Mass, Time};
use uom::si::time::year;

use crate::stars::data::StarData;
use crate::stars::fate::TYPE_II_SUPERNOVA_PEAK_MAGNITUDE;
use crate::stars::random::random_stars::get_min_age;
use crate::units::luminous_intensity::{
    absolute_magnitude_to_luminous_intensity, solar_luminous_intensity,
};

use super::data::ParsecData;
use super::trajectory::Trajectory;

impl ParsecData {
    pub(super) const SORTED_MASSES: [f64; 100] = [
        0.09, 0.10, 0.12, 0.14, 0.16, 0.20, 0.25, 0.30, 0.35, 0.40, 0.45, 0.50, 0.55, 0.60, 0.65,
        0.70, 0.75, 0.80, 0.85, 0.90, 0.95, 1.00, 1.05, 1.10, 1.15, 1.20, 1.25, 1.30, 1.35, 1.40,
        1.45, 1.50, 1.55, 1.60, 1.65, 1.70, 1.75, 1.80, 1.85, 1.90, 1.95, 2.00, 2.05, 2.10, 2.15,
        2.20, 2.25, 2.30, 2.40, 2.60, 2.80, 3.00, 3.20, 3.40, 3.60, 3.80, 4.00, 4.20, 4.40, 4.60,
        4.80, 5.00, 5.20, 5.40, 5.60, 5.80, 6.00, 6.20, 6.40, 7.00, 8.00, 9.00, 10.0, 12.0, 14.0,
        16.0, 18.0, 20.0, 24.0, 28.0, 30.0, 35.0, 40.0, 45.0, 50.0, 55.0, 60.0, 65.0, 70.0, 75.0,
        80.0, 90.0, 95.0, 100.0, 120.0, 130.0, 200.0, 250.0, 300.0, 350.0,
    ];

    pub(super) fn get_closest_mass_index(mass: f64) -> usize {
        let mut min_index = 0;
        let mut max_index = Self::SORTED_MASSES.len() - 1;
        while max_index - min_index > 1 {
            let mid_index = (max_index + min_index) / 2;
            let mid_mass = Self::SORTED_MASSES[mid_index];
            if mass > mid_mass {
                min_index = mid_index;
            } else {
                max_index = mid_index;
            }
        }
        if (mass - Self::SORTED_MASSES[min_index]).abs()
            < (mass - Self::SORTED_MASSES[max_index]).abs()
        {
            min_index
        } else {
            max_index
        }
    }

    pub(super) fn get_trajectory_via_index(&self, i: usize) -> &Trajectory {
        &self.data[i]
    }

    pub(super) fn is_filled(&self) -> bool {
        let mut is_filled = !self.data.is_empty();
        for trajectory in self.data.iter() {
            is_filled = is_filled && !trajectory.is_empty();
        }
        is_filled
    }

    pub(crate) fn get_star_data_if_visible(
        &self,
        mass_index: usize,
        age: Time,
        pos: Cartesian,
    ) -> Option<StarData> {
        let trajectory = self.get_trajectory_via_index(mass_index);
        let was_alive_10_millenia_ago = age - Time::new::<kiloyear>(10.) < trajectory.lifetime;
        if !was_alive_10_millenia_ago {
            return None;
        }

        let age_index = trajectory.get_closest_params_index(age.get::<year>());
        let params = trajectory.get_params_by_index(age_index)?;

        let is_currently_visible = params.is_visible(&pos);
        let has_visible_death_within_10k_years = trajectory.is_visible_supernova(&pos)
            && age + Time::new::<kiloyear>(10.) > trajectory.lifetime;
        if is_currently_visible || has_visible_death_within_10k_years {
            Some(trajectory.to_star(age, pos))
        } else {
            None
        }
    }

    pub(crate) fn get_most_luminous_intensity_possible(&self, max_age: Time) -> LuminousIntensity {
        let mut max_luminous_intensity = LuminousIntensity::new::<candela>(0.);
        let min_age = get_min_age(max_age);
        for trajectory in self.data.iter() {
            if min_age > trajectory.lifetime {
                continue;
            }
            if (min_age..max_age).contains(&trajectory.lifetime)
                && trajectory.initial_mass > Mass::new::<solar_mass>(8.)
            {
                return absolute_magnitude_to_luminous_intensity(TYPE_II_SUPERNOVA_PEAK_MAGNITUDE);
            }
            let min_age_index = trajectory.get_closest_params_index(min_age.get::<year>());
            let max_age_index = trajectory.get_closest_params_index(max_age.get::<year>());
            for age_index in min_age_index..=max_age_index {
                let params = trajectory.get_params_by_index_unchecked(age_index);
                let luminous_intensity =
                    params.luminous_intensity_in_solar * solar_luminous_intensity;
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

    use super::*;
    use crate::{
        astro_display::AstroDisplay, real_data::stars::all::get_many_stars,
        stars::random::parsec::data::PARSEC_DATA,
        units::luminous_intensity::luminous_intensity_to_illuminance,
    };

    #[test]
    fn masses_are_mapped_to_themselves() {
        const SMALL_OFFSET: f64 = 1e-4;
        for expected_mass in ParsecData::SORTED_MASSES.iter() {
            let mass = *expected_mass;
            let mass_index = ParsecData::get_closest_mass_index(mass);
            let mapped_mass = ParsecData::SORTED_MASSES[mass_index];
            println!("mass: {}, mapped_mass: {}", mass, mapped_mass);
            assert!((expected_mass - mapped_mass).abs() < SMALL_OFFSET);

            let mass = *expected_mass + SMALL_OFFSET;
            let mass_index = ParsecData::get_closest_mass_index(mass);
            let mapped_mass = ParsecData::SORTED_MASSES[mass_index];
            println!("mass: {}, mapped_mass: {}", mass, mapped_mass);
            assert!((expected_mass - mapped_mass).abs() < SMALL_OFFSET);

            let mass = *expected_mass - SMALL_OFFSET;
            let mass_index = ParsecData::get_closest_mass_index(mass);
            let mapped_mass = ParsecData::SORTED_MASSES[mass_index];
            println!("mass: {}, mapped_mass: {}", mass, mapped_mass);
            assert!((expected_mass - mapped_mass).abs() < SMALL_OFFSET);
        }
    }

    #[test]
    fn closest_params_map_to_correct_age() {
        for mass_index in 0..ParsecData::SORTED_MASSES.len() {
            let trajectory = {
                let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
                let parsec_data = parsec_data_mutex.as_ref().unwrap();
                (*parsec_data.get_trajectory_via_index(mass_index)).clone()
            };
            for (age_index, params) in trajectory.get_params().iter().enumerate() {
                let expected_age = params.age_in_years;
                let params_index = trajectory.get_closest_params_index(expected_age);
                let received_age = trajectory.get_params()[params_index].age_in_years;
                assert!(
                    (received_age - expected_age).abs() < 1e-8,
                    "Expected age {} should be exactly the same as received age {} (Mass index {}, Age index {})",
                    expected_age, received_age, mass_index, age_index
                );

                let little_less = expected_age - 0e-5;
                let params_index = trajectory.get_closest_params_index(little_less);
                let received_age = trajectory.get_params()[params_index].age_in_years;
                assert!(
                    (received_age - expected_age).abs() < 1e-8,
                    "Expected age {} should be a little less than received age {} (Mass index {}, Age index {})",
                    expected_age, received_age, mass_index, age_index
                );

                let little_more = expected_age + 0e-5;
                let params_index = trajectory.get_closest_params_index(little_more);
                let received_age = trajectory.get_params()[params_index].age_in_years;
                assert!(
                    (received_age - expected_age).abs() < 1e-8,
                    "Expected age {} should be a little more than received age {} (Mass index {}, Age index {})",
                    expected_age, received_age, mass_index, age_index
                );
            }
        }
    }

    #[test]
    fn infant_star_has_valid_evolution() {
        let mass_index = ParsecData::SORTED_MASSES.len() - 1;
        let star = {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            parsec_data
                .get_star_data_if_visible(mass_index, Time::new::<year>(0.), Cartesian::origin())
                .unwrap()
        };
        assert!(star
            .evolution
            .get_lifestage_luminous_intensity_per_year()
            .cd
            .is_finite());
        assert!(star.evolution.get_lifestage_mass_per_year().kg.is_finite());
        assert!(star.evolution.get_lifestage_radius_per_year().m.is_finite());
        assert!(star
            .evolution
            .get_lifestage_temperature_per_year()
            .get::<kelvin>()
            .is_finite());
    }

    #[test]
    fn old_star_has_finite_evolution() {
        let mass_index = ParsecData::SORTED_MASSES.len() - 1;
        let star = {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            let trajectory = &parsec_data.get_trajectory_via_index(mass_index);
            let age = trajectory.lifetime;
            parsec_data.get_star_data_if_visible(mass_index, age, Cartesian::origin())
        };
        let star = star.unwrap();
        assert!(star
            .evolution
            .get_lifestage_luminous_intensity_per_year()
            .cd
            .is_finite());
        assert!(star.evolution.get_lifestage_mass_per_year().kg.is_finite());
        assert!(star.evolution.get_lifestage_radius_per_year().m.is_finite());
        assert!(star
            .evolution
            .get_lifestage_temperature_per_year()
            .get::<kelvin>()
            .is_finite());
    }

    #[test]
    fn mass_is_always_lost() {
        let mass_index = ParsecData::SORTED_MASSES.len() - 1;
        let star = {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            let trajectory = &parsec_data.get_trajectory_via_index(mass_index);
            let age = trajectory.lifetime / 2.;
            parsec_data
                .get_star_data_if_visible(mass_index, age, Cartesian::origin())
                .unwrap()
        };
        assert!(star.evolution.get_lifestage_mass_per_year().kg < 0.);
    }

    #[test]
    fn lifetimes_of_real_stars_are_within_limits() {
        let stars = get_many_stars();
        for star in stars {
            let mass = star.mass;
            let lifetime = star.lifetime;
            let mass_index = ParsecData::get_closest_mass_index(mass.get::<solar>());
            let expected_lifetime = {
                let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
                let parsec_data = parsec_data_mutex.as_ref().unwrap();
                let trajectory = parsec_data.get_trajectory_via_index(mass_index);
                trajectory.lifetime
            };
            let ratio = lifetime / expected_lifetime;
            assert!(
                (0.99..1.01).contains(&ratio),
                "Star {} is deviant\nlifetime: {} Gyr,\nexpected lifetime: {} Gyr,\nratio: {:.2}\n",
                star.astronomical_name,
                lifetime.astro_display(),
                expected_lifetime.astro_display(),
                ratio
            );
        }
    }

    #[test]
    fn lifetime_mostly_decreases_with_mass() {
        let trajectories = {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            parsec_data.data.clone()
        };
        for (i, trajectory) in trajectories.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let lifetime = trajectory.lifetime;
            let previous_lifetime = trajectories[i - 1].lifetime;
            assert!(
                lifetime < 1.2 * previous_lifetime,
                "Lifetime of star {} is {} years, while lifetime of star {} is {} years",
                i,
                lifetime.astro_display(),
                i - 1,
                previous_lifetime.astro_display()
            );
        }
    }

    #[test]
    fn all_real_stars_are_visible() {
        let stars = get_many_stars().into_iter().map(|s| s.to_star_data());
        let mut failures = 0;
        for star in stars {
            let illuminance = luminous_intensity_to_illuminance(
                &star.get_luminous_intensity_at_epoch(),
                &star.get_distance_at_epoch(),
            );
            let mass = star.params.mass;
            let age = star.get_age_at_epoch();
            if mass.is_none() || age.is_none() {
                continue;
            }
            let mass = mass.unwrap();
            let age = age.unwrap();
            let mass_index = ParsecData::get_closest_mass_index(mass.get::<solar>());
            let pos = star.pos.clone();
            let generated = {
                let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
                let parsec_data = parsec_data_mutex.as_ref().unwrap();
                parsec_data.get_star_data_if_visible(mass_index, age, pos)
            };
            if generated.is_none() {
                failures += 1;
                println!(
                    "Star {} with illuminance {} is not generated by ParsecData.",
                    star.get_name(),
                    illuminance.astro_display()
                );
            }
        }
        assert!(
            failures < 12,
            "There are {} stars that are not generated by ParsecData.",
            failures
        );
    }

    #[test]
    fn distant_small_stars_are_invisible() {
        let pos = Direction::Z.to_cartesian(Length::from_lyr(1000.));
        let age = Time::new::<gigayear>(1.);
        for mass_index in 0..30 {
            let star = {
                let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
                let parsec_data = parsec_data_mutex.as_ref().unwrap();
                parsec_data.get_star_data_if_visible(mass_index, age, pos.clone())
            };
            assert!(
                star.is_none(),
                "Star {:?} is visible at 1000 lyr, while it should not be.",
                star
            );
        }
    }
}
