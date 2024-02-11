use fraction::Fraction;
use simple_si_units::base::Time;

const RESONANCE_TOLERANCE: f64 = 0.01;
const RESONANCE_MAX_INT: u8 = 6;

fn orbital_resonance(period1: Time<f64>, period2: Time<f64>) -> Option<Fraction> {
    let large = period1.s.max(period2.s);
    let small = period1.s.min(period2.s);
    let ratio = small / large;
    for denominator in 1..=RESONANCE_MAX_INT {
        let numerator_float = ratio * denominator as f64;
        let numerator_int = numerator_float.round() as u8;
        let error = (numerator_float - numerator_int as f64).abs();
        if error < RESONANCE_TOLERANCE && numerator_int <= denominator {
            return Some(Fraction::new(numerator_int, denominator));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orbital_resonance_is_never_larger_than_1() {
        for i in 1..=RESONANCE_MAX_INT {
            for j in 1..=RESONANCE_MAX_INT {
                let period_1 = Time::from_seconds(i as f64);
                let period_2 = Time::from_seconds(j as f64);
                let resonance = orbital_resonance(period_1, period_2).unwrap();
                assert!(resonance <= Fraction::new(1u64, 1u64));
            }
        }
    }

    #[test]
    fn test_finding_orbital_resonance() {
        for small in 1..=RESONANCE_MAX_INT {
            for large in small..=RESONANCE_MAX_INT {
                let expected = Fraction::new(small, large);
                println!("{}/{} = {}", large, small, expected);
                let small = Time::from_seconds(small as f64);
                let large = Time::from_seconds(large as f64);
                let resonance = orbital_resonance(small, large);
                assert!(resonance.is_some());
                let resonance = resonance.unwrap();
                assert_eq!(resonance, expected);
            }
        }
    }

    #[test]
    fn tetst_not_finding_orbital_resonance() {
        let factor = 1.0 + RESONANCE_TOLERANCE * 2.;
        for small in 1..=RESONANCE_MAX_INT {
            for large in small..=RESONANCE_MAX_INT {
                let small = Time::from_seconds(small as f64);
                let large = Time::from_seconds(large as f64 * factor);
                let ratio = large.s / small.s;
                println!("{:.2}/{:.2} = {:.4}", large.s, small.s, ratio);
                let resonance = orbital_resonance(small, large);
                if let Some(resonance) = resonance {
                    println!("Found resonance: {}", resonance);
                }
                assert!(resonance.is_none());
            }
        }
    }
}
