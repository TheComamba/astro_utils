use uom::si::{
    f64::{Length, ThermodynamicTemperature},
    length::{meter, nanometer},
};

use crate::color::black_body::planck_radiant_emittance;

fn tilted_gaussian(lambda: f64, mean: f64, sigma1: f64, sigma2: f64) -> f64 {
    let nominator = -0.5 * (lambda - mean).powi(2);
    if lambda < mean {
        let denominator = sigma1.powi(2);
        (nominator / denominator).exp()
    } else {
        let denominator = sigma2.powi(2);
        (nominator / denominator).exp()
    }
}

pub(crate) fn x_color_matching(lambda: Length) -> f64 {
    1.056 * tilted_gaussian(lambda.get::<nanometer>(), 599.8, 37.9, 31.0)
        + 0.362 * tilted_gaussian(lambda.get::<nanometer>(), 442.0, 16.0, 26.7)
        + -0.065 * tilted_gaussian(lambda.get::<nanometer>(), 501.1, 20.4, 26.2)
}

pub(crate) fn y_color_matching(lambda: Length) -> f64 {
    0.821 * tilted_gaussian(lambda.get::<nanometer>(), 568.8, 46.9, 40.5)
        + 0.286 * tilted_gaussian(lambda.get::<nanometer>(), 530.9, 16.3, 31.1)
}

pub(crate) fn z_color_matching(lambda: Length) -> f64 {
    1.217 * tilted_gaussian(lambda.get::<nanometer>(), 437.0, 11.8, 36.0)
        + 0.681 * tilted_gaussian(lambda.get::<nanometer>(), 459.0, 26.0, 13.8)
}

pub(crate) fn convolute_with_black_body(
    fun: Box<dyn Fn(Length) -> f64>,
    temperature: ThermodynamicTemperature,
) -> f64 {
    let step = Length::new::<nanometer>(1.);
    let mut sum = 0.;
    let mut lambda = Length::new::<nanometer>(380.);
    while lambda.get::<nanometer>() < 780. {
        let value = fun(lambda);
        let planck = planck_radiant_emittance(lambda, temperature);
        sum += value * planck;
        lambda += step;
    }
    sum * step.get::<meter>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_matching_functions_are_between_zero_and_two() {
        let mut lambda = Length::new::<nanometer>(380.);
        while lambda.get::<nanometer>() < 780. {
            let x = x_color_matching(lambda);
            let y = y_color_matching(lambda);
            let z = z_color_matching(lambda);
            assert!(x >= 0. && x <= 2.);
            assert!(y >= 0. && y <= 2.);
            assert!(z >= 0. && z <= 2.);
            lambda += Length::new::<nanometer>(1.);
        }
    }
}
