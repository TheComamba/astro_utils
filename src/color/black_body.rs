pub(crate) fn planck_radiant_emittance(
    wavelength: Distance<f64>,
    temperature: Temperature<f64>,
) -> f64 {
    const H: f64 = 6.62607015e-34;
    const C: f64 = 299792458.0;
    const K: f64 = 1.380649e-23;
    let lambda = wavelength.to_meters();
    let t = temperature.K;
    let a = 2.0 * H * C * C;
    let exp_arg = H * C / (lambda * K * t);
    let numerator = a / (lambda * lambda * lambda * lambda * lambda);
    let denominator = exp_arg.exp() - 1.0;
    numerator / denominator
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestExpectance {
        wavelength: Distance<f64>,
        temperature: Temperature<f64>,
        expected: f64,
    }

    #[test]
    fn test_planck_radiant_emittance() {
        let expectations = vec![
            TestExpectance {
                wavelength: Distance::from_nm(100.),
                temperature: Temperature::from_K(5000.0),
                expected: 3.792e6,
            },
            TestExpectance {
                wavelength: Distance::from_nm(500.),
                temperature: Temperature::from_K(5000.0),
                expected: 1.211e13,
            },
            TestExpectance {
                wavelength: Distance::from_nm(1_000.),
                temperature: Temperature::from_K(5000.0),
                expected: 7.102e12,
            },
            TestExpectance {
                wavelength: Distance::from_nm(100.),
                temperature: Temperature::from_K(6000.0),
                expected: 4.589e8,
            },
            TestExpectance {
                wavelength: Distance::from_nm(500.),
                temperature: Temperature::from_K(6000.0),
                expected: 3.176e13,
            },
            TestExpectance {
                wavelength: Distance::from_nm(1000.),
                temperature: Temperature::from_K(6000.0),
                expected: 1.191e13,
            },
        ];

        for expectation in expectations {
            let result = planck_radiant_emittance(expectation.wavelength, expectation.temperature);
            let expected = expectation.expected;
            let ratio = result / expected;
            println!(
                "wavelength: {}, temperature: {}, expected: {}, result: {}, ratio: {}",
                expectation.wavelength.to_nm(),
                expectation.temperature.K,
                expected,
                result,
                ratio
            );
            assert!((ratio - 1.).abs() < 1e-3);
        }
    }
}
