use super::data::ParsecData;
use super::line::ParsecLine;
use crate::coordinates::cartesian::CartesianCoordinates;
use crate::stars::random::random_stars::DIMMEST_ILLUMINANCE;
use crate::stars::star_data::StarData;
use crate::stars::star_data_evolution::{StarDataEvolution, StarDataLifestageEvolution};

impl ParsecData {
    pub(crate) const SORTED_MASSES: [f64; 100] = [
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

    pub(crate) fn get_trajectory_via_index(&self, i: usize) -> &Vec<ParsecLine> {
        &self.data[i]
    }

    pub(super) fn get_life_expectancy_in_years(trajectory: &[ParsecLine]) -> u32 {
        trajectory.last().unwrap().age as u32
    }

    pub(super) fn is_filled(&self) -> bool {
        let mut is_filled = !self.data.is_empty();
        for trajectory in self.data.iter() {
            is_filled = is_filled && !trajectory.is_empty();
        }
        is_filled
    }

    pub(crate) fn get_star_data_if_visible(
        trajectory: &[ParsecLine],
        age_in_years: f64,
        pos: CartesianCoordinates,
    ) -> Option<StarData> {
        if age_in_years > Self::get_life_expectancy_in_years(trajectory) as f64 {
            return None;
        }
        let current_params_index = ParsecData::get_closest_params_index(trajectory, age_in_years);
        let current_params = &trajectory[current_params_index];

        let distance_squared = pos.x() * pos.x() + pos.y() * pos.y() + pos.z() * pos.z();
        let min_luminous_intensity = DIMMEST_ILLUMINANCE * distance_squared;
        let luminous_intensity = current_params.get_luminous_intensity().cd;
        if luminous_intensity < min_luminous_intensity.lm {
            return None;
        }

        let mut star = current_params.to_star_at_origin();

        star.distance = pos.length();
        star.pos = pos.to_ecliptic();

        let other_params = if current_params_index == 0 {
            &trajectory[current_params_index + 1]
        } else {
            &trajectory[current_params_index - 1]
        };
        let star_at_other_time = other_params.to_star_at_origin();
        let years = current_params.age - other_params.age;
        let lifestage_evolution =
            StarDataLifestageEvolution::new(&star, &star_at_other_time, years);
        star.evolution = StarDataEvolution::new(Some(lifestage_evolution));

        Some(star)
    }

    pub(super) fn get_closest_params_index(
        trajectory: &[ParsecLine],
        actual_age_in_years: f64,
    ) -> usize {
        let mut age_index = 0;
        for (i, line) in trajectory.iter().enumerate() {
            if line.age > actual_age_in_years || i == trajectory.len() - 1 {
                age_index = i;
                break;
            }
        }
        if age_index == 0 {
            0
        } else {
            let previous_age = trajectory[age_index - 1].age;
            let diff_to_prev = actual_age_in_years - previous_age;
            let next_age = trajectory[age_index].age;
            let diff_to_next = next_age - actual_age_in_years;
            if diff_to_prev <= diff_to_next {
                age_index - 1
            } else {
                age_index
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stars::random::parsec::data::PARSEC_DATA;

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
            for (age_index, params) in trajectory.iter().enumerate() {
                let expected_age = params.age;
                let params_index = ParsecData::get_closest_params_index(&trajectory, expected_age);
                let received_age = trajectory[params_index].age;
                assert!(
                    (received_age - expected_age).abs() < 1e-8,
                    "Expected age {} should be exactly the same as received age {} (Mass index {}, Age index {})",
                    expected_age, received_age, mass_index, age_index
                );

                let little_less = expected_age - 0e-5;
                let params_index = ParsecData::get_closest_params_index(&trajectory, little_less);
                let received_age = trajectory[params_index].age;
                assert!(
                    (received_age - expected_age).abs() < 1e-8,
                    "Expected age {} should be a little less than received age {} (Mass index {}, Age index {})",
                    expected_age, received_age, mass_index, age_index
                );

                let little_more = expected_age + 0e-5;
                let params_index = ParsecData::get_closest_params_index(&trajectory, little_more);
                let received_age = trajectory[params_index].age;
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
        let trajectory = {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            (*parsec_data.get_trajectory_via_index(mass_index)).clone()
        };
        let age_in_years = 0.;
        let star = ParsecData::get_star_data_if_visible(
            &trajectory,
            age_in_years,
            CartesianCoordinates::ORIGIN,
        )
        .unwrap();
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
            .K
            .is_finite());
    }

    #[test]
    fn old_star_has_finite_evolution() {
        let mass_index = ParsecData::SORTED_MASSES.len() - 1;
        let trajectory = {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            (*parsec_data.get_trajectory_via_index(mass_index)).clone()
        };
        let age_in_years = ParsecData::get_life_expectancy_in_years(&trajectory) as f64;
        let star = ParsecData::get_star_data_if_visible(
            &trajectory,
            age_in_years,
            CartesianCoordinates::ORIGIN,
        )
        .unwrap();
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
            .K
            .is_finite());
    }

    #[test]
    fn mass_is_always_lost() {
        let mass_index = ParsecData::SORTED_MASSES.len() - 1;
        let trajectory = {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            (*parsec_data.get_trajectory_via_index(mass_index)).clone()
        };
        let age_in_years = ParsecData::get_life_expectancy_in_years(&trajectory) as f64 / 2.;
        let star = ParsecData::get_star_data_if_visible(
            &trajectory,
            age_in_years,
            CartesianCoordinates::ORIGIN,
        )
        .unwrap();
        assert!(star.evolution.get_lifestage_mass_per_year().kg < 0.);
    }
}
