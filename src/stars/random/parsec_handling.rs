use parsec_access::getters::{
    get_closest_age_index, get_masses_in_solar, get_parameters, get_trajectory,
};
use simple_si_units::base::{Luminosity, Mass, Time};

use crate::{
    stars::fate::TYPE_II_SUPERNOVA_PEAK_MAGNITUDE,
    units::luminous_intensity::{
        absolute_magnitude_to_luminous_intensity, LUMINOSITY_ZERO, SOLAR_LUMINOUS_INTENSITY,
    },
};

use super::random_stars::{get_min_age, METALLICITY_INDEX};

pub(super) fn get_most_luminous_intensity_possible(max_age: Time<f64>) -> Luminosity<f64> {
    let mut max_luminous_intensity = LUMINOSITY_ZERO;
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
