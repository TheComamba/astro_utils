use uom::si::{
    f64::{LuminousIntensity, Power},
    power::watt,
};

use super::luminous_intensity::SOLAR_LUMINOUS_INTENSITY;

pub type Luminosity = Power;

fn SOLAR_LUMINOSITY() -> Power {
    Power::new::<watt>(3.828e26)
}

pub(crate) fn luminous_intensity_to_luminosity(
    luminous_intensity: LuminousIntensity,
) -> Luminosity {
    (luminous_intensity / SOLAR_LUMINOUS_INTENSITY()) * SOLAR_LUMINOSITY()
}
