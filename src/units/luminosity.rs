use super::luminous_intensity::SOLAR_LUMINOUS_INTENSITY;

const SOLAR_LUMINOSITY: f64 = 3.828e26; //Watts

pub(crate) fn luminous_intensity_to_luminosity(
    luminous_intensity: &Luminosity<f64>,
) -> Luminosity<f64> {
    SOLAR_LUMINOSITY / SOLAR_LUMINOUS_INTENSITY.cd * luminous_intensity
}
