use std::f64::consts::PI;

pub mod color;
pub mod coordinates;
pub mod error;
pub mod planets;
pub mod real_data;
pub mod stars;
pub mod units;

#[cfg(test)]
pub(crate) mod tests {
    pub(crate) const TEST_ACCURACY: f64 = 1e-5;

    pub(crate) fn eq_within(actual: f64, expected: f64, accuracy: f64) -> bool {
        if (actual - expected).abs() >= accuracy {
            println!("Test assertion a == b failed:");
            println!(
                "actual: {}\nexpected: {}\naccuracy: {}",
                actual, expected, accuracy
            );
            false
        } else {
            true
        }
    }

    pub(crate) fn eq(actual: f64, expected: f64) -> bool {
        eq_within(actual, expected, TEST_ACCURACY)
    }
}
