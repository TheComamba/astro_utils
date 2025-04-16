use astro_coords::cartesian::Cartesian;
use parsec_access::getters::{get_closest_age_index, get_parameters, get_trajectory};
use simple_si_units::base::Time;

use crate::{
    stars::{
        data::StarData,
        evolution::{StarDataEvolution, StarDataLifestageEvolution},
        fate::StarFate,
        physical_parameters::StarPhysicalParameters,
        random::random_stars::METALLICITY_INDEX,
    },
    units::luminous_intensity::SOLAR_LUMINOUS_INTENSITY,
};

pub(super) fn get_star(mass_index: usize, age: Time<f64>, pos: Cartesian) -> StarData {
    let age_index = get_closest_age_index(METALLICITY_INDEX, mass_index, age);
    let mut star = star_without_evolution(mass_index, age_index, pos.clone());
    let other_age_index = if age_index == 0 {
        age_index + 1
    } else {
        age_index - 1
    };
    let other_star = star_without_evolution(mass_index, other_age_index, pos);

    let lifestage_evolution = get_lifestage_evolution(&star, other_star);
    let trajectory = get_trajectory(METALLICITY_INDEX, mass_index);
    let fate = StarFate::new(trajectory.initial_mass);
    star.evolution =
        StarDataEvolution::new(lifestage_evolution, Some(age), trajectory.lifetime, fate);
    star
}

fn star_without_evolution(mass_index: usize, age_index: usize, pos: Cartesian) -> StarData {
    let params = get_parameters(METALLICITY_INDEX, mass_index, age_index);
    let luminous_intensity = params.luminosity_in_solar * SOLAR_LUMINOUS_INTENSITY;
    let physical_parameters = StarPhysicalParameters {
        mass: Some(params.mass),
        luminous_intensity,
        temperature: params.temperature,
        radius: Some(params.radius),
    };
    let mut evolution = StarDataEvolution::NONE;
    evolution.age = Some(params.age);
    StarData {
        name: "".to_string(),
        params: physical_parameters,
        pos,
        constellation: None,
        evolution,
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
