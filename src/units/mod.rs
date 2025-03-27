// https://www.astro.princeton.edu/~gk/A403/constants.pdf

pub mod acceleration;
pub mod angle;
pub mod density;
pub mod distance;
pub mod illuminance;
pub mod luminosity;
pub mod luminous_intensity;
pub mod mass;
pub mod solid_angle;
pub mod temperature;
pub mod time;
pub mod velocity;

pub(super) const DISPLAY_THRESHOLD: f64 = 0.099;

#[cfg(test)]
pub(crate) mod tests {
    use uom::si::f64::Angle;

    use crate::tests::TEST_ACCURACY;

    pub(crate) const ANGLE_TEST_ACCURACY: Angle = Angle { rad: TEST_ACCURACY };
}
