use crate::{error::AstroUtilError, stars::random::random_stars::METALLICITY_INDEX};

use parsec_access::getters::get_masses_in_solar;
use rand_distr::weighted::WeightedAliasIndex;

const MIN_MASS_FOR_HYDROGEN_FUSION: f64 = 0.08;

pub(crate) fn get_mass_index_distribution() -> Result<WeightedAliasIndex<f64>, AstroUtilError> {
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
    use uom::si::{f64::Length, length::light_year};

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
        let gen_masses = (0..num_stars).map(|_| masses[distribution.sample(&mut rand::rng())]);
        let mut thresholds = Vec::new();
        for i in 0..masses.len() - 1 {
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
        let max_distance = Length::new::<light_year>(1000.);
        let num_stars = number_in_sphere(STARS_PER_LY_CUBED, max_distance);
        println!("Number of stars: {}", num_stars);
        let distribution = get_mass_index_distribution().unwrap();
        let num_supermassive_stars = (0..num_stars)
            .into_par_iter()
            .map(|_| {
                let mut rng = rand::rng();
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
