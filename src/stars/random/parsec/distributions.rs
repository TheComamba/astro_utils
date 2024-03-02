use super::data::ParsecData;
use rand::{distributions::Distribution, rngs::ThreadRng};
use rand_distr::WeightedAliasIndex;

const MIN_MASS_FOR_HYDROGEN_FUSION: f64 = 0.08;

pub(crate) struct ParsecDistribution {
    mass_distribution: WeightedAliasIndex<f64>,
}

impl ParsecDistribution {
    pub(crate) fn new() -> Self {
        let mass_distribution = get_mass_distribution();
        ParsecDistribution { mass_distribution }
    }

    pub(crate) fn get_random_mass_index(&self, rng: &mut ThreadRng) -> usize {
        self.mass_distribution.sample(rng)
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

    #[test]
    fn about_2_permille_of_stars_explode() {
        let integral = integrate_kroupa(8.0, 80.);
        assert!(
            integral > 0.0020 && integral < 0.0022,
            "Integral is {}",
            integral
        );
    }
}
