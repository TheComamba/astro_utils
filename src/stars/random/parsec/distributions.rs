use super::{data::ParsecData, line::ParsecLine};
use crate::stars::random::random_stars::AGE_OF_MILKY_WAY_THIN_DISK;
use rand::{distributions::Distribution, rngs::ThreadRng};
use rand_distr::WeightedAliasIndex;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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
    let mut weights = Vec::new();
    for m in ParsecData::SORTED_MASSES {
        let weight = kroupa_mass_distribution(m);
        weights.push(weight);
    }
    WeightedAliasIndex::new(weights).unwrap()
}

fn kroupa_mass_distribution(m_in_solar_masses: f64) -> f64 {
    let alpha = if m_in_solar_masses <= 0.08 {
        0.3
    } else if m_in_solar_masses <= 0.5 {
        1.3
    } else if m_in_solar_masses <= 1. {
        2.3
    } else {
        2.7
    };
    m_in_solar_masses.powf(-alpha)
}

fn get_age_distribution(
    trajectory: &Vec<ParsecLine>,
    max_age_in_years: f64,
) -> WeightedAliasIndex<f64> {
    let mut weights = Vec::new();
    for i in 0..=trajectory.len() {
        let previous_age = if i > 0 { trajectory[i - 1].age } else { 0. };
        if previous_age >= max_age_in_years {
            break;
        }
        let current_age = if i < trajectory.len() {
            trajectory[i].age
        } else {
            max_age_in_years
        };
        weights.push(current_age - previous_age);
    }
    WeightedAliasIndex::new(weights).unwrap()
}
