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

    pub(crate) fn eq_within(a: f64, b: f64, accuracy: f64) -> bool {
        (a - b).abs() <= accuracy
    }

    pub(crate) fn eq(a: f64, b: f64) -> bool {
        eq_within(a, b, TEST_ACCURACY)
    }
}
