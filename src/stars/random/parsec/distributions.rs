use super::{data::ParsecData, line::ParsedParsecLine};
use crate::stars::random::random_stars::AGE_OF_MILKY_WAY_THIN_DISK;
use rand::{distributions::Distribution, rngs::ThreadRng};
use rand_distr::WeightedAliasIndex;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const MIN_MASS_FOR_HYDROGEN_FUSION: f64 = 0.08;

pub(crate) struct ParsecDistribution {
    mass_distribution: WeightedAliasIndex<f64>,
    age_distributions: Vec<WeightedAliasIndex<f64>>,
}

impl ParsecDistribution {
    pub(crate) fn new(parsec: &ParsecData) -> Self {
        let mass_distribution = get_mass_distribution();

        let max_age_in_years = AGE_OF_MILKY_WAY_THIN_DISK.to_yr();
        let age_distributions = parsec
            .data
            .par_iter()
            .map(|trajectory| get_age_distribution(trajectory, max_age_in_years))
            .collect();

        ParsecDistribution {
            mass_distribution,
            age_distributions,
        }
    }

    pub(crate) fn get_random_mass_index(&self, rng: &mut ThreadRng) -> usize {
        self.mass_distribution.sample(rng)
    }

    pub(crate) fn get_random_age_index(&self, mass_index: usize, rng: &mut ThreadRng) -> usize {
        self.age_distributions[mass_index].sample(rng)
    }
}

fn get_mass_distribution() -> WeightedAliasIndex<f64> {
    let weights = kroupa_weights();
    WeightedAliasIndex::new(weights).unwrap()
}

fn kroupa_weights() -> Vec<f64> {
    let mut weights = Vec::new();
    for m in ParsecData::SORTED_MASSES {
        let weight = kroupa_mass_distribution(m);
        weights.push(weight);
    }
    weights
}

fn kroupa_mass_distribution(m_in_solar_masses: f64) -> f64 {
    const NORMALIZATION: f64 = 0.124969;
    if m_in_solar_masses < MIN_MASS_FOR_HYDROGEN_FUSION {
        return 0.; // Brown dwarfs
    }
    let (alpha, prefactor) = if m_in_solar_masses <= 0.5 {
        (1.3, 0.5f64.powf(-2.3) / 0.5f64.powf(-1.3))
    } else if m_in_solar_masses <= 1. {
        (2.3, 1.)
    } else {
        (2.7, 1.)
    };
    prefactor * m_in_solar_masses.powf(-alpha) * NORMALIZATION
}

fn get_age_distribution(
    trajectory: &Vec<ParsedParsecLine>,
    max_age_in_years: f64,
) -> WeightedAliasIndex<f64> {
    let mut weights = Vec::new();
    for i in 0..=trajectory.len() {
        let previous_age = if i > 0 {
            trajectory[i - 1].age_in_years
        } else {
            0.
        };
        if previous_age >= max_age_in_years {
            break;
        }
        let current_age = if i < trajectory.len() {
            trajectory[i].age_in_years
        } else {
            max_age_in_years
        };
        weights.push(current_age - previous_age);
    }
    WeightedAliasIndex::new(weights).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!((integral - 1.).abs() < 0.01, "Integral is {}", integral);
    }

    #[test]
    fn kroupa_weights_are_ordered() {
        let weights = kroupa_weights();
        for i in 1..weights.len() {
            assert!(
                weights[i - 1] >= weights[i],
                "Weight {} is larger than weight {}: {}, {}",
                i - 1,
                i,
                weights[i - 1],
                weights[i]
            );
        }
    }
}
