use uom::si::{
    f64::{Length, LuminousIntensity},
    length::parsec,
    luminous_intensity::candela,
};

use crate::astro_display::AstroDisplay;

use super::illuminance::{apparent_magnitude_to_illuminance, illuminance_to_apparent_magnitude};

pub fn SOLAR_LUMINOUS_INTENSITY() -> LuminousIntensity {
    LuminousIntensity::new::<candela>(2.98e27)
}

pub fn luminous_intensity_to_solar_luminosities(luminous_intensity: LuminousIntensity) -> f64 {
    (luminous_intensity / SOLAR_LUMINOUS_INTENSITY()).into()
}

pub fn absolute_magnitude_to_luminous_intensity(absolute_magnitude: f64) -> LuminousIntensity {
    let ten_pc = Length::new::<parsec>(10.);
    let illuminance = apparent_magnitude_to_illuminance(absolute_magnitude);
    illuminance_to_luminous_intensity(&illuminance, &ten_pc)
}

pub fn luminous_intensity_to_absolute_magnitude(luminous_intensity: LuminousIntensity) -> f64 {
    let ten_pc = Length::new::<parsec>(10.);
    let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &ten_pc);
    illuminance_to_apparent_magnitude(&illuminance)
}

pub fn luminous_intensity_to_illuminance(
    luminous_intensity: &LuminousIntensity,
    distance: &Length,
) -> Illuminance<f64> {
    luminous_intensity * SolidAngle::from_steradians(1.) / (distance * distance)
}

pub fn illuminance_to_luminous_intensity(
    illuminance: &Illuminance<f64>,
    distance: &Length,
) -> LuminousIntensity {
    illuminance * (distance * distance) / SolidAngle::from_steradians(1.)
}

impl AstroDisplay for LuminousIntensity {
    fn astro_display(&self) -> String {
        let absolute_magnitude = luminous_intensity_to_absolute_magnitude(*self);
        format!("{:.2} abs. mag.", absolute_magnitude)
    }
}

#[cfg(test)]
mod tests {
    use uom::si::length::meter;

    use super::*;
    use crate::tests::{eq, eq_within};

    const REAL_DATA_TEST_ACCURACY: f64 = 0.05;
    const ILLUMINANCE_AT_UNIT_DISTANCE: f64 = 1.;

    #[test]
    fn illuminance_roundtrip() {
        for i in -10..10 {
            let input = i as f64;
            let luminous_intensity = LuminousIntensity::new::<candela>(input);
            let distance = Length::new::<meter>(1.);
            let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &distance);
            let output = illuminance_to_luminous_intensity(&illuminance, &distance);
            assert!(eq(input, output.value));
        }
    }

    #[test]
    fn absolute_magnitude_roundtrip() {
        for i in -10..10 {
            let input = i as f64;
            let luminous_intensity = absolute_magnitude_to_luminous_intensity(input);
            let output = luminous_intensity_to_absolute_magnitude(luminous_intensity);
            assert!(eq(input, output));
        }
    }

    #[test]
    fn illuminance_of_1_cd_source_at_1_m() {
        let luminous_intensity = LuminousIntensity::new::<candela>(1.);
        let distance = Length::new::<meter>(1.);
        let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &distance);
        let actual = illuminance.to_lux();
        let expected = ILLUMINANCE_AT_UNIT_DISTANCE;
        assert!(eq(actual, expected));
    }

    #[test]
    fn illuminance_is_proportional_to_luminous_intensity() {
        for i in 1..10 {
            let cd = i as f64;
            let luminous_intensity = LuminousIntensity::new::<candela>(cd);
            let distance = Length::new::<meter>(1.);
            let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &distance);
            let expected = cd * ILLUMINANCE_AT_UNIT_DISTANCE;
            let actual = illuminance.to_lux();
            assert!(eq(actual, expected));
        }
    }

    #[test]
    fn illuminance_is_inversely_proportional_to_distance_squared() {
        for d in 1..10 {
            let distance = Length::new::<meter>(d as f64);
            let luminous_intensity = LuminousIntensity::new::<candela>(1.);
            let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &distance);
            let expected = ILLUMINANCE_AT_UNIT_DISTANCE / (d * d) as f64;
            let actual = illuminance.to_lux();
            assert!(eq(actual, expected));
        }
    }

    #[test]
    fn apparent_and_absolute_magnitude_at_ten_parsecs_are_equal() {
        let ten_pc = Length::new::<parsec>(10.);
        for i in -10..10 {
            let input = i as f64;
            let luminous_intensity = absolute_magnitude_to_luminous_intensity(input);
            let illuminance = luminous_intensity_to_illuminance(&luminous_intensity, &ten_pc);
            let apparent_magnitude = illuminance_to_apparent_magnitude(&illuminance);
            let absolute_magnitude = luminous_intensity_to_absolute_magnitude(luminous_intensity);
            assert!(eq(apparent_magnitude, absolute_magnitude));
        }
    }

    #[test]
    fn test_the_sun() {
        let sun_abs_mag = luminous_intensity_to_absolute_magnitude(SOLAR_LUMINOUS_INTENSITY());
        let expected = 4.83;
        assert!(eq_within(sun_abs_mag, expected, REAL_DATA_TEST_ACCURACY));
    }

    #[test]
    fn test_sirius() {
        let sun_abs_mag =
            luminous_intensity_to_absolute_magnitude(22. * SOLAR_LUMINOUS_INTENSITY());
        let expected = 1.43;
        assert!(eq_within(sun_abs_mag, expected, REAL_DATA_TEST_ACCURACY));
    }
}
