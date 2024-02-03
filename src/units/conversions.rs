impl Angle {
    pub fn from_arcsecs(arcsec: Float) -> Angle {
        Angle {
            radian: arcsec * RADIAN_PER_ARCSEC,
        }
    }

    pub fn as_arcsecs(&self) -> Float {
        self.radian * ARCSECS_PER_RADIAN
    }

    pub fn from_second_angle(second_angle: Float) -> Angle {
        Angle {
            radian: second_angle * RADIANS_PER_SECOND_ANGLE,
        }
    }

    pub fn as_second_angle(&self) -> Float {
        self.radian * SECOND_ANGLE_PER_RADIAN
    }
    /*
     * Normalize the angle to a range of −π to +π radians, -180° to 180°.
     */
    pub fn normalize(&mut self) {
        self.radian = self.radian % TWO_PI;
        if self.radian > PI {
            self.radian -= TWO_PI;
        } else if self.radian < -PI {
            self.radian += TWO_PI;
        }
    }

    pub fn sin(&self) -> Float {
        self.radian.sin()
    }

    pub fn cos(&self) -> Float {
        self.radian.cos()
    }

    pub fn tan(&self) -> Float {
        self.radian.tan()
    }
}

impl Illuminance {
    pub const ZERO: Illuminance = Illuminance { lux: 0. };

    pub const fn from_lux(lux: Float) -> Illuminance {
        Illuminance { lux }
    }

    pub fn from_apparent_magnitude(apparent_magnitude: Float) -> Illuminance {
        let exponent = (-14.18 - apparent_magnitude) / 2.5;
        let lux = (10. as Float).powf(exponent);
        Illuminance { lux }
    }

    pub const fn as_lux(&self) -> Float {
        self.lux
    }

    pub fn as_apparent_magnitude(&self) -> Float {
        -14.18 - 2.5 * self.lux.log10()
    }

    pub fn to_luminosity(&self, distance: &Length) -> Luminosity {
        let absolute_magnitude =
            self.as_apparent_magnitude() - 5. * distance.as_parsecs().log10() + 5.;
        Luminosity::from_absolute_magnitude(absolute_magnitude)
    }

    pub fn to_luminance_flat_surface(&self) -> Luminance {
        let nit = self.lux / PI;
        Luminance::from_nit(nit)
    }

    #[cfg(test)]
    pub(crate) fn eq_within(&self, other: Illuminance, accuracy: Illuminance) -> bool {
        (self.lux - other.lux).abs() <= accuracy.lux
    }
}

impl Luminosity {
    pub(crate) const PRACICALLY_ZERO: Self = Self {
        absolute_magnitude: -100.,
    };

    pub const fn from_absolute_magnitude(absolute_magnitude: Float) -> Luminosity {
        Luminosity { absolute_magnitude }
    }

    pub fn from_solar_luminosities(solar_luminosities: Float) -> Luminosity {
        let absolute_magnitude = 4.8 - 2.5 * solar_luminosities.log10();
        Luminosity { absolute_magnitude }
    }

    pub fn from_watts(watts: Float) -> Luminosity {
        Self::from_solar_luminosities(watts * SOLAR_LUMINOSITY_PER_WATT)
    }

    pub const fn as_absolute_magnitude(&self) -> Float {
        self.absolute_magnitude
    }

    pub fn as_solar_luminosities(&self) -> Float {
        let exponent = (4.8 - self.absolute_magnitude) / 2.5;
        (10. as Float).powf(exponent)
    }

    pub fn as_watts(&self) -> Float {
        self.as_solar_luminosities() * WATTS_PER_SOLAR_LUMINOSITY
    }

    pub fn to_illuminance(&self, distance: &Length) -> Illuminance {
        let apparent_magnitude = self.absolute_magnitude + 5. * distance.as_parsecs().log10() - 5.;
        Illuminance::from_apparent_magnitude(apparent_magnitude)
    }

    #[cfg(test)]
    pub(crate) fn eq_within(&self, other: &Self, accuracy: Self) -> bool {
        (self.absolute_magnitude - other.absolute_magnitude).abs() < accuracy.absolute_magnitude
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::TEST_ACCURACY;

    const REAL_DATA_TEST_ACCURACY: Float = 0.05;

    #[test]
    fn test_apparent_magnitudes() {
        for magnitude in -10..10 {
            let input = magnitude as Float;
            let illuminance = Illuminance::from_apparent_magnitude(input);
            let output = illuminance.as_apparent_magnitude();
            println!("input: {}, output: {}", input, output);
            assert!((input - output).abs() < TEST_ACCURACY);
        }
    }

    #[test]
    fn test_sunlight() {
        let luminosity = Luminosity::from_solar_luminosities(1.);
        let distance = Length::from_astronomical_units(1.);
        let illuminance = luminosity.to_illuminance(&distance);
        let actual = illuminance.as_lux();
        let expected = 107_527.;
        println!("expected: {}, actual: {}", expected, actual);
        assert!((actual - expected).abs() < REAL_DATA_TEST_ACCURACY * expected);
    }

    use super::*;
    use crate::tests::TEST_ACCURACY;

    const REAL_DATA_TEST_ACCURACY: Float = 0.05;

    #[test]
    fn test_absolute_magnitude() {
        for i in -10..10 {
            let input = i as Float;
            let luminosity = Luminosity::from_absolute_magnitude(input);
            let output = luminosity.as_absolute_magnitude();
            println!("input: {}, output: {}", input, output);
            assert!((input - output).abs() < TEST_ACCURACY);
        }
    }

    #[test]
    fn test_apparent_magnitudes() {
        for lum in -10..10 {
            for dist in 1..10 {
                let input = lum as Float;
                let distance = Length::from_light_years(dist as Float);
                let luminosity =
                    Illuminance::from_apparent_magnitude(input).to_luminosity(&distance);
                let output = luminosity.to_illuminance(&distance).as_apparent_magnitude();
                println!("distance: {}", distance);
                println!("input: {}, output: {}", input, output);
                assert!((input - output).abs() < TEST_ACCURACY);
            }
        }
    }

    #[test]
    fn test_solar_luminosities() {
        for i in -10..10 {
            let input = (i as Float).exp();
            let luminosity = Luminosity::from_solar_luminosities(input);
            let output = luminosity.as_solar_luminosities();
            println!("input: {}, output: {}", input, output);
            assert!((input - output).abs() < TEST_ACCURACY * input);
        }
    }

    #[test]
    fn test_watts() {
        for i in -10..10 {
            let input = (i as Float).exp() * WATTS_PER_SOLAR_LUMINOSITY;
            let luminosity = Luminosity::from_watts(input);
            let output = luminosity.as_watts();
            println!("input: {}, output: {}", input, output);
            assert!((input - output).abs() < TEST_ACCURACY * input);
        }
    }

    #[test]
    fn apparent_and_absolute_magnitude_at_ten_parsecs_are_equal() {
        for i in -10..10 {
            let input = i as Float;
            let luminosity = Luminosity::from_absolute_magnitude(input);
            let distance = Length::from_parsecs(10.);
            let apparent_magnitude = luminosity.to_illuminance(&distance).as_apparent_magnitude();
            let absolute_magnitude = luminosity.as_absolute_magnitude();
            println!("input: {}", input);
            println!("apparent magnitude: {}", apparent_magnitude);
            println!("absolute magnitude: {}", absolute_magnitude);
            assert!((apparent_magnitude - absolute_magnitude).abs() < TEST_ACCURACY);
        }
    }

    #[test]
    fn test_the_sun() {
        let sun = Luminosity::from_absolute_magnitude(4.83);

        let sun_apparent_magnitude = sun
            .to_illuminance(&Length::from_astronomical_units(1.))
            .as_apparent_magnitude();
        let expected_apparent_magnitude = -26.74;
        println!(
            "Apparnet Magnitude:\nexpected: {},\nactual: {}",
            expected_apparent_magnitude, sun_apparent_magnitude
        );
        assert!(
            (expected_apparent_magnitude - sun_apparent_magnitude).abs() < REAL_DATA_TEST_ACCURACY
        );

        let sun_solar_luminosities = sun.as_solar_luminosities();
        let expected_solar_luminosities = 1.;
        println!(
            "Solar Luminosities:\nexpected: {},\nactual: {}",
            expected_solar_luminosities, sun_solar_luminosities
        );
        assert!(
            (expected_solar_luminosities - sun_solar_luminosities).abs() < REAL_DATA_TEST_ACCURACY
        );

        let sun_watts = sun.as_watts();
        let expected_watts = WATTS_PER_SOLAR_LUMINOSITY;
        println!(
            "Watts:\nexpected: {},\nactual: {}",
            expected_watts, sun_watts
        );
        assert!(
            (expected_watts - sun_watts).abs()
                < REAL_DATA_TEST_ACCURACY * WATTS_PER_SOLAR_LUMINOSITY
        );
    }

    #[test]
    fn test_sirius() {
        let sirius = Luminosity::from_absolute_magnitude(1.43);

        let sirius_apparent_magnitude = sirius
            .to_illuminance(&&Length::from_light_years(8.6))
            .as_apparent_magnitude();
        let expected_apparent_magnitude = -1.46;
        println!(
            "Apparnet Magnitude:\nexpected: {},\nactual: {}",
            expected_apparent_magnitude, sirius_apparent_magnitude
        );
        assert!(
            (expected_apparent_magnitude - sirius_apparent_magnitude).abs()
                < REAL_DATA_TEST_ACCURACY
        );

        let sirius_solar_luminosities = sirius.as_solar_luminosities();
        let expected_solar_luminosities = 22.3;
        println!(
            "Solar Luminosities:\nexpected: {},\nactual: {}",
            expected_solar_luminosities, sirius_solar_luminosities
        );
        assert!(
            (expected_solar_luminosities - sirius_solar_luminosities).abs()
                < REAL_DATA_TEST_ACCURACY
        );

        let sirius_watts = sirius.as_watts();
        let expected_watts = 22.3 * WATTS_PER_SOLAR_LUMINOSITY;
        println!(
            "Watts:\nexpected: {},\nactual: {}",
            expected_watts, sirius_watts
        );
        assert!(
            (expected_watts - sirius_watts).abs()
                < REAL_DATA_TEST_ACCURACY * WATTS_PER_SOLAR_LUMINOSITY
        );
    }
}
