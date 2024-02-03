pub type Float = f32;
pub(crate) const PI: Float = std::f32::consts::PI;
pub(crate) const TWO_PI: Float = 2. * PI;

pub mod color;
pub mod coordinates;
pub mod error;
pub mod planets;
pub mod real_data;
pub mod stars;
pub mod units;

#[cfg(test)]
pub(crate) mod tests {
    use crate::Float;

    pub(crate) const TEST_ACCURACY: Float = 1e-5;

    pub(crate) fn eq_within(a: Float, b: Float, accuracy: Float) -> bool {
        (a - b).abs() <= accuracy
    }

    pub(crate) fn eq(a: Float, b: Float) -> bool {
        eq_within(a, b, TEST_ACCURACY)
    }
}
