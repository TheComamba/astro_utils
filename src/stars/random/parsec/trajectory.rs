use serde::{Deserialize, Serialize};
use simple_si_units::base::{Luminosity, Mass, Time};

use crate::{
    coordinates::cartesian::CartesianCoordinates,
    stars::{
        data::StarData,
        evolution::{StarDataEvolution, StarDataLifestageEvolution},
        fate::{StarFate, TYPE_II_SUPERNOVA_PEAK_MAGNITUDE},
        random::random_stars::DIMMEST_ILLUMINANCE,
    },
    units::{
        luminous_intensity::{
            absolute_magnitude_to_luminous_intensity, LUMINOSITY_ZERO, SOLAR_LUMINOUS_INTENSITY,
        },
        mass::MASS_ZERO,
        time::TIME_ZERO,
    },
};

use super::line::ParsedParsecLine;

#[derive(Deserialize, Serialize, Clone)]
pub(super) struct Trajectory {
    params: Vec<ParsedParsecLine>,
    pub(super) initial_mass: Mass<f64>,
    pub(super) lifetime: Time<f64>,
    pub(super) peak_lifetime_luminous_intensity: Luminosity<f64>,
}

impl Trajectory {
    pub(super) const EMPTY: Trajectory = Trajectory {
        params: Vec::new(),
        initial_mass: MASS_ZERO,
        lifetime: TIME_ZERO,
        peak_lifetime_luminous_intensity: LUMINOSITY_ZERO,
    };

    pub(super) fn new(params: Vec<ParsedParsecLine>) -> Self {
        let initial_mass = Mass::from_solar_mass(params[0].mass_in_solar_masses);
        let lifetime = Time::from_yr(params.last().unwrap().age_in_years);
        let peak_lifetime_luminous_intensity =
            get_peak_lifetime_luminous_intensity(&params, initial_mass);

        Self {
            params,
            initial_mass,
            lifetime,
            peak_lifetime_luminous_intensity,
        }
    }

    pub(super) fn get_params_by_index(&self, index: usize) -> Option<&ParsedParsecLine> {
        self.params.get(index)
    }

    #[cfg(test)]
    pub(super) fn get_params_by_index_unchecked(&self, index: usize) -> &ParsedParsecLine {
        &self.params[index]
    }

    pub(super) fn get_closest_params_index(&self, actual_age_in_years: f64) -> usize {
        if actual_age_in_years < self.params[0].age_in_years {
            return Self::this_or_next_age_index(self, 0, actual_age_in_years);
        }

        let mut age_index = 1;
        while self.params[age_index].age_in_years < actual_age_in_years {
            age_index *= 2;
            if age_index >= self.params.len() {
                age_index = self.params.len() - 2;
                break;
            }
        }

        while self.params[age_index].age_in_years > actual_age_in_years {
            age_index -= 1;
        }

        Self::this_or_next_age_index(self, age_index, actual_age_in_years)
    }

    fn this_or_next_age_index(&self, age_index: usize, actual_age_in_years: f64) -> usize {
        let this_age = self.params[age_index].age_in_years;
        let diff_to_this = actual_age_in_years - this_age;
        let next_age = self.params[age_index + 1].age_in_years;
        let diff_to_next = next_age - actual_age_in_years;
        if diff_to_this <= diff_to_next {
            age_index
        } else {
            age_index + 1
        }
    }

    pub(super) fn get_lifestage_evolution(
        &self,
        age_index: usize,
        current_params: &ParsedParsecLine,
        star: &StarData,
    ) -> StarDataLifestageEvolution {
        let other_params = if age_index == 0 {
            &self.params[age_index + 1]
        } else {
            &self.params[age_index - 1]
        };
        let star_at_other_time = other_params.to_star(star.pos.clone());
        let years = current_params.age_in_years - other_params.age_in_years;
        let lifestage_evolution = StarDataLifestageEvolution::new(star, &star_at_other_time, years);
        lifestage_evolution
    }

    pub(super) fn get_evolution(
        &self,
        current_params: &ParsedParsecLine,
        lifestage_evolution: StarDataLifestageEvolution,
    ) -> StarDataEvolution {
        let age_at_epoch = Some(Time::from_yr(current_params.age_in_years));
        let fate = StarFate::new(self.initial_mass);
        let evolution =
            StarDataEvolution::new(Some(lifestage_evolution), age_at_epoch, self.lifetime, fate);
        evolution
    }

    #[cfg(test)]
    pub(super) fn get_params(&self) -> &Vec<ParsedParsecLine> {
        &self.params
    }

    pub(super) fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    pub(super) fn is_ever_visible(&self, pos: &CartesianCoordinates) -> bool {
        let min_luminous_intensity = Luminosity {
            cd: DIMMEST_ILLUMINANCE.lux * pos.length_squared().m2,
        };
        self.peak_lifetime_luminous_intensity >= min_luminous_intensity
    }
}

fn get_peak_lifetime_luminous_intensity(
    params: &Vec<ParsedParsecLine>,
    initial_mass: Mass<f64>,
) -> Luminosity<f64> {
    let peak_lifetime_luminous_intensity = params
        .iter()
        .map(|p| p.luminous_intensity_in_solar)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    if initial_mass < Mass::from_solar_mass(8.) {
        peak_lifetime_luminous_intensity * SOLAR_LUMINOUS_INTENSITY
    } else {
        absolute_magnitude_to_luminous_intensity(TYPE_II_SUPERNOVA_PEAK_MAGNITUDE)
    }
}
