// https://www.astro.princeton.edu/~gk/A403/constants.pdf

pub mod acceleration;
pub mod angle;
pub mod density;
pub mod illuminance;
pub mod length;
pub mod luminosity;
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

    pub(crate) fn ANGLE_TEST_ACCURACY() -> Angle {
        Angle::new::<radian>(TEST_ACCURACY)
    }
}
