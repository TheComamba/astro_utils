use parsec_access::getters::{
    get_closest_age_index, get_masses_in_solar, get_parameters, get_trajectory,
};
use rand_distr::WeightedAliasIndex;
use simple_si_units::base::{Luminosity, Mass, Time};

use crate::{
    error::AstroUtilError,
    stars::fate::TYPE_II_SUPERNOVA_PEAK_MAGNITUDE,
    units::luminous_intensity::{
        absolute_magnitude_to_luminous_intensity, LUMINOSITY_ZERO, SOLAR_LUMINOUS_INTENSITY,
    },
};

use super::random_stars::{get_min_age, METALLICITY_INDEX};

const MIN_MASS_FOR_HYDROGEN_FUSION: f64 = 0.08;

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

pub(super) fn get_mass_index_distribution() -> Result<WeightedAliasIndex<f64>, AstroUtilError> {
    let weights = kroupa_weights();
    WeightedAliasIndex::new(weights).map_err(AstroUtilError::from)
}

fn kroupa_weights() -> Vec<f64> {
    let masses = get_masses_in_solar(METALLICITY_INDEX);
    let mut weights = Vec::new();
    for m in 0..masses.len() {
        let lower = if m == 0 {
            0.
        } else {
            geometric_mean(masses[m - 1], masses[m])
        };
        let upper = if m == masses.len() - 1 {
            1000.
        } else {
            geometric_mean(masses[m], masses[m + 1])
        };
        let weight = integrate_kroupa(lower, upper);
        weights.push(weight);
    }
    weights
}

fn geometric_mean(a: f64, b: f64) -> f64 {
    (a * b).sqrt()
}

fn kroupa_mass_distribution(m_in_solar_masses: f64) -> f64 {
    const NORMALIZATION: f64 = 0.12499960249873866;
    if m_in_solar_masses < MIN_MASS_FOR_HYDROGEN_FUSION {
        return 0.; // Brown dwarfs
    }
    let (alpha, prefactor) = if m_in_solar_masses <= 0.5 {
        (1.3, 0.5f64.powf(-2.3) / 0.5f64.powf(-1.3))
    } else if m_in_solar_masses <= 1. {
        (2.3, 1.)
    } else if m_in_solar_masses <= 20. {
        (2.7, 1.)
    } else {
        (5., 20f64.powf(-2.7) / 20f64.powf(-5.)) //Adjusted high mass tail
    };
    prefactor * m_in_solar_masses.powf(-alpha) * NORMALIZATION
}

fn integrate_kroupa(lower: f64, upper: f64) -> f64 {
    let mut integral = 0.;
    let mut x = lower;
    while x < upper {
        let dx = (upper - x).min(0.01);
        integral += kroupa_mass_distribution(x) * dx;
        x += dx;
    }
    integral
}

#[cfg(test)]
mod tests {
    use rand_distr::Distribution;
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    use simple_si_units::base::Distance;

    use crate::stars::random::random_stars::{number_in_sphere, STARS_PER_LY_CUBED};

    use super::*;

    #[test]
    fn kroupa_is_smooth() {
        let stepsize = 0.01;
        let mut mass = MIN_MASS_FOR_HYDROGEN_FUSION;
        let mut last = kroupa_mass_distribution(mass);
        while mass < 100. {
            mass += stepsize;
            let current = kroupa_mass_distribution(mass);
            let derivative = (current - last) / stepsize;
            assert!(
                derivative < 0.01,
                "Kroupa is not smooth at {} solar masses:\nlast: {}\ncurrent: {}\nderivative: {}",
                mass,
                last,
                current,
                derivative
            );
            last = current;
        }
    }

    #[test]
    fn kroupa_integrates_to_1() {
        let lower = 0.0;
        let upper = 2000.;
        let integral = integrate_kroupa(lower, upper);
        assert!(
            (integral - 1.).abs() < 1e-5,
            "Integral is {},\nso normalization should be {}",
            integral,
            1. / integral
        );
    }

    #[test]
    fn kroupa_integral_and_sampling_agree() {
        let masses = get_masses_in_solar(METALLICITY_INDEX);
        let num_stars = 100_000;
        let distribution = get_mass_index_distribution().unwrap();
        let gen_masses =
            (0..num_stars).map(|_| masses[distribution.sample(&mut rand::thread_rng())]);
        let mut thresholds = Vec::new();
        for i in 0..gen_masses.len() - 1 {
            thresholds.push(geometric_mean(masses[i], masses[i + 1]));
        }
        for threshold in thresholds {
            let count = gen_masses.clone().filter(|&m| m >= threshold).count();
            let uncertainty = 10. / (count as f64).sqrt();
            let fraction = count as f64 / num_stars as f64;
            let integral = integrate_kroupa(threshold as f64, 1000.);
            let lower = integral - uncertainty;
            let upper = integral + uncertainty;
            assert!(
                (lower..upper).contains(&fraction),
                "Threshold mass is {}, fraction is {} not within [{},{}]",
                threshold,
                fraction,
                lower,
                upper
            );
        }
    }

    #[test]
    fn there_are_less_than_10_supermassive_stars_within_1000_lyr() {
        // The closest star above 50 Sun masses ist 3000 lyr away.
        let masses = get_masses_in_solar(METALLICITY_INDEX);
        let max_distance = Distance::from_lyr(1000.);
        let num_stars = number_in_sphere(STARS_PER_LY_CUBED, max_distance);
        println!("Number of stars: {}", num_stars);
        let distribution = get_mass_index_distribution().unwrap();
        let num_supermassive_stars = (0..num_stars)
            .into_par_iter()
            .map(|_| {
                let mut rng = rand::thread_rng();
                masses[distribution.sample(&mut rng)]
            })
            .filter(|&m| m >= 99.)
            .count();
        assert!(
            num_supermassive_stars < 10,
            "There are {} supermassive stars",
            num_supermassive_stars
        );
    }
}
