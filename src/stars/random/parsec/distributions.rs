use super::data::ParsecData;
use crate::stars::random::random_stars::AGE_OF_MILKY_WAY_THIN_DISK;
use rand::distributions::{Uniform, WeightedIndex};

pub(crate) fn get_mass_distribution() -> WeightedIndex<f64> {
    let mut weights = Vec::new();
    for m in ParsecData::SORTED_MASSES {
        let weight = kroupa_mass_distribution(m);
        weights.push(weight);
    }
    WeightedIndex::new(weights).unwrap()
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

pub(crate) fn get_age_distribution() -> Uniform<f64> {
    Uniform::new(0., AGE_OF_MILKY_WAY_THIN_DISK.to_yr())
}
