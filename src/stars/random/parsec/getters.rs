use astro_coords::cartesian::Cartesian;
use parsec_access::getters::{
    get_closest_age_index, get_masses_in_solar, get_parameters, get_trajectory,
};
use parsec_access::line::ParsecLine;
use parsec_access::trajectory::Trajectory;
use uom::si::f64::{LuminousIntensity, Mass, Time};
use uom::si::luminous_intensity::candela;
use uom::si::time::year;

use crate::stars::data::StarData;
use crate::stars::evolution::{StarDataEvolution, StarDataLifestageEvolution};
use crate::stars::fate::{StarFate, TYPE_II_SUPERNOVA_PEAK_MAGNITUDE};
use crate::stars::physical_parameters::StarPhysicalParameters;
use crate::stars::random::random_stars::{dimmest_illuminance, get_min_age, METALLICITY_INDEX};
use crate::units::luminous_intensity::{
    absolute_magnitude_to_luminous_intensity, solar_luminous_intensity,
};
use crate::units::mass::solar_mass;
use crate::units::time::kiloyear;

pub(crate) fn get_star_data_if_visible(
    mass_index: usize,
    age: Time,
    pos: Cartesian,
) -> Option<StarData> {
    let trajectory = get_trajectory(METALLICITY_INDEX, mass_index);
    let was_alive_10_millenia_ago = age - Time::new::<kiloyear>(10.) < trajectory.lifetime;
    if !was_alive_10_millenia_ago {
        return None;
    }

    let age_index = get_closest_age_index(METALLICITY_INDEX, mass_index, age);
    let params = get_parameters(METALLICITY_INDEX, mass_index, age_index);

    let is_currently_visible = is_visible(params, &pos);
    if is_currently_visible {
        return Some(get_star(mass_index, age, pos));
    }
    let has_visible_death_within_10k_years = is_visible_supernova(trajectory, &pos)
        && age + Time::new::<kiloyear>(10.) > trajectory.lifetime;
    if has_visible_death_within_10k_years {
        return Some(get_star(mass_index, age, pos));
    }
    None
}

fn is_visible(line: &ParsecLine, pos: &Cartesian) -> bool {
    let min_luminous_intensity = dimmest_illuminance() * pos.length_squared();
    line.luminosity_in_solar * solar_luminous_intensity() >= min_luminous_intensity
}

fn is_visible_supernova(trajectory: &Trajectory, pos: &Cartesian) -> bool {
    if trajectory.initial_mass < Mass::new::<solar_mass>(8.) {
        return false;
    }
    let min_luminous_intensity = dimmest_illuminance() * pos.length_squared();
    let supernova_luminous_intensity =
        absolute_magnitude_to_luminous_intensity(TYPE_II_SUPERNOVA_PEAK_MAGNITUDE);
    supernova_luminous_intensity >= min_luminous_intensity
}

pub(crate) fn get_most_luminous_intensity_possible(max_age: Time) -> LuminousIntensity {
    let mut max_luminous_intensity = LuminousIntensity::new::<candela>(0.);
    let min_age = get_min_age(max_age);
    let masses = get_masses_in_solar(METALLICITY_INDEX);
    for (mass_index, _mass) in masses.iter().enumerate() {
        let trajectory = get_trajectory(METALLICITY_INDEX, mass_index);
        if min_age > trajectory.lifetime {
            continue;
        }
        if trajectory.initial_mass > Mass::new::<solar_mass>(8.)
            && (min_age..max_age).contains(&trajectory.lifetime)
        {
            return absolute_magnitude_to_luminous_intensity(TYPE_II_SUPERNOVA_PEAK_MAGNITUDE);
        }
        let min_age_index = get_closest_age_index(METALLICITY_INDEX, mass_index, min_age);
        let max_age_index = get_closest_age_index(METALLICITY_INDEX, mass_index, max_age);
        for age_index in min_age_index..=max_age_index {
            let params = get_parameters(METALLICITY_INDEX, mass_index, age_index);
            let luminous_intensity = params.luminosity_in_solar * solar_luminous_intensity();
            if luminous_intensity > max_luminous_intensity {
                max_luminous_intensity = luminous_intensity;
            }
        }
    }
    max_luminous_intensity
}

fn get_star(mass_index: usize, age: Time, pos: Cartesian) -> StarData {
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
    let luminous_intensity = params.luminosity_in_solar * solar_luminous_intensity();
    let physical_parameters = StarPhysicalParameters {
        mass: Some(params.mass),
        luminous_intensity,
        temperature: params.temperature,
        radius: Some(params.radius),
    };
    let mut evolution = StarDataEvolution::none();
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
    let year_difference =
        star.evolution.age?.get::<year>() - other_star.evolution.age?.get::<year>();
    let lifestage_evolution = StarDataLifestageEvolution::new(star, &other_star, year_difference);
    Some(lifestage_evolution)
}

#[cfg(test)]
mod tests {
    use astro_coords::direction::Direction;
    use uom::si::{f64::Length, length::light_year};

    use super::*;
    use crate::{
        astro_display::AstroDisplay,
        real_data::stars::all::get_many_stars,
        units::{luminous_intensity::luminous_intensity_to_illuminance, time::gigayear},
    };
    use parsec_access::getters::get_closest_mass_index;

    #[test]
    fn infant_star_has_valid_evolution() {
        assert!(parsec_access::getters::is_data_ready());
        let mass_index = get_masses_in_solar(METALLICITY_INDEX).len() - 1;
        let star = get_star(mass_index, Time::new::<year>(0.), Cartesian::origin());
        assert!(star
            .evolution
            .get_lifestage_luminous_intensity_per_year()
            .value
            .is_finite());
        assert!(star
            .evolution
            .get_lifestage_mass_per_year()
            .value
            .is_finite());
        assert!(star
            .evolution
            .get_lifestage_radius_per_year()
            .value
            .is_finite());
        assert!(star
            .evolution
            .get_lifestage_temperature_per_year()
            .value
            .is_finite());
    }

    #[test]
    fn old_star_has_finite_evolution() {
        assert!(parsec_access::getters::is_data_ready());
        let mass_index = get_masses_in_solar(METALLICITY_INDEX).len() - 1;
        let age = get_trajectory(METALLICITY_INDEX, mass_index).lifetime;
        let star = get_star(mass_index, age, Cartesian::origin());
        assert!(star
            .evolution
            .get_lifestage_luminous_intensity_per_year()
            .value
            .is_finite());
        assert!(star
            .evolution
            .get_lifestage_mass_per_year()
            .value
            .is_finite());
        assert!(star
            .evolution
            .get_lifestage_radius_per_year()
            .value
            .is_finite());
        assert!(star
            .evolution
            .get_lifestage_temperature_per_year()
            .value
            .is_finite());
    }

    #[test]
    fn all_real_stars_are_visible() {
        assert!(parsec_access::getters::is_data_ready());
        let stars = get_many_stars().into_iter().map(|s| s.to_star_data());
        let mut failures = 0;
        for star in stars {
            let illuminance = luminous_intensity_to_illuminance(
                star.get_luminous_intensity_at_epoch(),
                star.get_distance_at_epoch(),
            );
            let mass = star.params.mass;
            let age = star.get_age_at_epoch();
            if mass.is_none() || age.is_none() {
                continue;
            }
            let mass = mass.unwrap();
            let age = age.unwrap();
            let mass_index = get_closest_mass_index(METALLICITY_INDEX, mass);
            let pos = star.pos.clone();
            let generated = get_star_data_if_visible(mass_index, age, pos);
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
        assert!(parsec_access::getters::is_data_ready());
        let pos = Direction::Z.to_cartesian(Length::new::<light_year>(1000.));
        let age = Time::new::<gigayear>(1.);
        for mass_index in 0..30 {
            let star = get_star_data_if_visible(mass_index, age, pos.clone());
            assert!(
                star.is_none(),
                "Star {:?} is visible at 1000 lyr, while it should not be.",
                star
            );
        }
    }
}
