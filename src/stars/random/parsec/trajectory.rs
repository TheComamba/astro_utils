use astro_coordinates::cartesian::CartesianCoordinates;
use serde::{Deserialize, Serialize};
use simple_si_units::base::{Luminosity, Mass, Temperature, Time};

use crate::{
    stars::{
        data::StarData,
        evolution::{StarDataEvolution, StarDataLifestageEvolution},
        fate::{StarFate, TYPE_II_SUPERNOVA_PEAK_MAGNITUDE},
        physical_parameters::StarPhysicalParameters,
        random::random_stars::DIMMEST_ILLUMINANCE,
    },
    units::{
        distance::SOLAR_RADIUS,
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
        let lifetime = match params.last() {
            Some(last) => Time::from_yr(last.age_in_years),
            None => TIME_ZERO,
        };
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

    #[cfg(test)]
    pub(super) fn get_params(&self) -> &Vec<ParsedParsecLine> {
        &self.params
    }

    pub(super) fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    pub(super) fn is_visible_supernova(&self, pos: &CartesianCoordinates) -> bool {
        if self.initial_mass < Mass::from_solar_mass(8.) {
            return false;
        }
        let min_luminous_intensity = Luminosity {
            cd: DIMMEST_ILLUMINANCE.lux * pos.length_squared().m2,
        };
        self.peak_lifetime_luminous_intensity >= min_luminous_intensity
    }

    pub(super) fn to_star(&self, age: Time<f64>, pos: CartesianCoordinates) -> StarData {
        let age_index = self.get_closest_params_index(age.to_yr());
        let mut star = self.to_star_without_evolution(age_index, pos.clone());
        let other_age_index = if age_index == 0 {
            age_index + 1
        } else {
            age_index - 1
        };
        let other_star = self.to_star_without_evolution(other_age_index, pos);

        let lifestage_evolution = get_lifestage_evolution(&star, other_star);
        let fate = StarFate::new(self.initial_mass);
        star.evolution =
            StarDataEvolution::new(lifestage_evolution, Some(age), self.lifetime, fate);
        star
    }

    fn to_star_without_evolution(&self, age_index: usize, pos: CartesianCoordinates) -> StarData {
        let params = self.get_params_by_index_unchecked(age_index);
        let mass = Mass::from_solar_mass(params.mass_in_solar_masses);
        let luminous_intensity = params.luminous_intensity_in_solar * SOLAR_LUMINOUS_INTENSITY;
        let temperature = Temperature::from_K(params.temperature_in_kelvin);
        let radius = params.radius_in_solar_radii * SOLAR_RADIUS;
        let physical_parameters = StarPhysicalParameters {
            mass: Some(mass),
            luminous_intensity,
            temperature,
            radius: Some(radius),
        };
        let mut evolution = StarDataEvolution::NONE;
        evolution.age = Some(Time::from_yr(params.age_in_years));
        StarData {
            name: "".to_string(),
            params: physical_parameters,
            pos,
            constellation: None,
            evolution,
        }
    }
}

fn get_lifestage_evolution(
    star: &StarData,
    other_star: StarData,
) -> Option<StarDataLifestageEvolution> {
    let year_difference = star.evolution.age?.to_yr() - other_star.evolution.age?.to_yr();
    let lifestage_evolution = StarDataLifestageEvolution::new(star, &other_star, year_difference);
    Some(lifestage_evolution)
}

fn get_peak_lifetime_luminous_intensity(
    params: &[ParsedParsecLine],
    initial_mass: Mass<f64>,
) -> Luminosity<f64> {
    let peak_lifetime_luminous_intensity = params
        .iter()
        .map(|p| p.luminous_intensity_in_solar)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or_default();
    if initial_mass < Mass::from_solar_mass(8.) {
        peak_lifetime_luminous_intensity * SOLAR_LUMINOUS_INTENSITY
    } else {
        absolute_magnitude_to_luminous_intensity(TYPE_II_SUPERNOVA_PEAK_MAGNITUDE)
    }
}
