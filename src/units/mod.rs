// https://www.astro.princeton.edu/~gk/A403/constants.pdf

pub mod length;
pub mod luminous_intensity;
pub mod mass;
pub mod solid_angle;
pub mod temperature;
pub mod time;
pub mod velocity;

pub(super) const DISPLAY_THRESHOLD: f64 = 0.085;

#[cfg(test)]
pub(crate) mod tests {
    use uom::si::{angle::radian, f64::Angle};

    use crate::tests::TEST_ACCURACY;

    pub(crate) fn angle_test_accuracy() -> Angle {
        Angle::new::<radian>(TEST_ACCURACY)
    }

    #[cfg(test)]
    pub(crate) fn angle_eq(actual: Angle, expected: Angle) -> bool {
        use astro_units::angle::angle_eq_within;
        use uom::si::angle::radian;

        use crate::tests::TEST_ACCURACY;

        angle_eq_within(actual, expected, Angle::new::<radian>(TEST_ACCURACY))
    }
}
