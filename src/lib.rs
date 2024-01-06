pub type Float = f32;
pub(crate) const PI: Float = std::f32::consts::PI;
pub(crate) const TWO_PI: Float = 2. * PI;

pub mod color;
pub mod coordinates;
pub mod data;
pub mod error;
pub mod kepler_orbit;
pub mod orbit_orientation;
pub mod planet_brightness;
pub mod random_stars;
pub mod stellar_properties;
pub mod surface_normal;
pub mod units;

#[cfg(test)]
pub(crate) mod tests {
    use crate::{
        units::{
            angle::Angle,
            length::{Length, AU_PER_METER},
            mass::Mass,
            time::Time,
        },
        Float,
    };

    pub(crate) const TEST_ACCURACY: Float = 1e-5;
    pub(crate) const TEST_ANGLE_ACCURACY: Angle = Angle::from_radians(TEST_ACCURACY);
    pub(crate) const TEST_LENGTH_ACCURACY: Length =
        Length::from_astronomical_units(TEST_ACCURACY * AU_PER_METER);
    pub(crate) const TEST_MASS_ACCURACY: Mass = Mass::from_kilograms(TEST_ACCURACY);
    pub(crate) const TEST_TIME_ACCURACY: Time = Time::from_seconds(TEST_ACCURACY);
}
