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
}
