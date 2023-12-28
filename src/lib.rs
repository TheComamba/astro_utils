pub type Float = f32;
pub(crate) const PI: Float = std::f32::consts::PI;
pub(crate) const TWO_PI: Float = 2. * PI;

pub mod coordinates;
pub mod kepler_orbit;
pub mod solar_system_data;
pub mod units;

#[cfg(test)]
pub(crate) mod tests {
    use crate::{
        units::{angle::Angle, length::Length, mass::Mass, time::Time},
        Float,
    };

    pub(crate) const TEST_ACCURACY: Float = 1e-5;
    pub(crate) const TEST_ANGLE_ACCURACY: Angle = Angle::from_radians(TEST_ACCURACY);
    pub(crate) const TEST_LENGTH_ACCURACY: Length = Length::from_meters(TEST_ACCURACY);
    pub(crate) const TEST_MASS_ACCURACY: Mass = Mass::from_kilograms(TEST_ACCURACY);
    pub(crate) const TEST_TIME_ACCURACY: Time = Time::from_seconds(TEST_ACCURACY);
}
