// https://www.astro.princeton.edu/~gk/A403/constants.pdf

pub mod angle;
pub mod distance;
pub mod illuminance;
pub mod luminous_intensity;
pub mod mass;
pub mod time;

pub(super) const DISPLAY_THRESHOLD: f64 = 0.099;

#[cfg(test)]
pub(crate) mod tests {
    use crate::tests::TEST_ACCURACY;
    use simple_si_units::geometry::Angle;

    pub(crate) const ANGLE_TEST_ACCURACY: Angle<f64> = Angle { rad: TEST_ACCURACY };
}
