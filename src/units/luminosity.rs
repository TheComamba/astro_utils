use uom::si::{
    f64::{LuminousIntensity, Power},
    power::watt,
};

use super::luminous_intensity::solar_luminous_intensity;

#[inline(always)]
fn solar_luminosity() -> Power {
    Power::new::<watt>(3.828e26)
}

pub(crate) fn luminous_intensity_to_luminosity(
    luminous_intensity: LuminousIntensity,
) -> Luminosity {
    (luminous_intensity / solar_luminous_intensity()) * solar_luminosity()
}
