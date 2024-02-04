pub const LUMINOSITY_ZERO: Luminosity<f64> = Luminosity { cd: 0. };

pub(crate) const PRACICALLY_ZERO: Self = Self {
    absolute_magnitude: -100.,
};

pub const fn from_absolute_magnitude(absolute_magnitude: f64) -> Luminosity {
    Luminosity { absolute_magnitude }
}

pub fn from_solar_luminosities(solar_luminosities: f64) -> Luminosity {
    let absolute_magnitude = 4.8 - 2.5 * solar_luminosities.log10();
    Luminosity { absolute_magnitude }
}

pub fn from_watts(watts: f64) -> Luminosity {
    Self::from_solar_luminosities(watts * SOLAR_LUMINOSITY_PER_WATT)
}

pub const fn to_absolute_magnitude(&self) -> f64 {
    self.absolute_magnitude
}

pub fn to_solar_luminosities(&self) -> f64 {
    let exponent = (4.8 - self.absolute_magnitude) / 2.5;
    (10. as f64).powf(exponent)
}

pub fn to_watts(&self) -> f64 {
    self.to_solar_luminosities() * WATTS_PER_SOLAR_LUMINOSITY
}

pub fn to_illuminance(&self, distance: &Distance) -> Illuminance {
    let apparent_magnitude = self.absolute_magnitude + 5. * distance.to_parsecs().log10() - 5.;
    Illuminance::from_apparent_magnitude(apparent_magnitude)
}

#[cfg(test)]
pub(crate) fn eq_within(&self, other: &Self, accuracy: Self) -> bool {
    (self.absolute_magnitude - other.absolute_magnitude).abs() < accuracy.absolute_magnitude
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_absolute_magnitude() {
        for i in -10..10 {
            let input = i as f64;
            let luminosity = Luminosity::from_absolute_magnitude(input);
            let output = luminosity.to_absolute_magnitude();
            println!("input: {}, output: {}", input, output);
            assert!((input - output).abs() < TEST_ACCURACY);
        }
    }

    #[test]
    fn test_apparent_magnitudes() {
        for lum in -10..10 {
            for dist in 1..10 {
                let input = lum as f64;
                let distance = Distance::from_lyr(dist as f64);
                let luminosity =
                    Illuminance::from_apparent_magnitude(input).to_luminosity(&distance);
                let output = luminosity.to_illuminance(&distance).to_apparent_magnitude();
                println!("distance: {}", distance);
                println!("input: {}, output: {}", input, output);
                assert!((input - output).abs() < TEST_ACCURACY);
            }
        }
    }

    #[test]
    fn test_solar_luminosities() {
        for i in -10..10 {
            let input = (i as f64).exp();
            let luminosity = Luminosity::from_solar_luminosities(input);
            let output = luminosity.to_solar_luminosities();
            println!("input: {}, output: {}", input, output);
            assert!((input - output).abs() < TEST_ACCURACY * input);
        }
    }

    #[test]
    fn test_watts() {
        for i in -10..10 {
            let input = (i as f64).exp() * WATTS_PER_SOLAR_LUMINOSITY;
            let luminosity = Luminosity::from_watts(input);
            let output = luminosity.to_watts();
            println!("input: {}, output: {}", input, output);
            assert!((input - output).abs() < TEST_ACCURACY * input);
        }
    }

    #[test]
    fn apparent_and_absolute_magnitude_at_ten_parsecs_are_equal() {
        for i in -10..10 {
            let input = i as f64;
            let luminosity = Luminosity::from_absolute_magnitude(input);
            let distance = Distance::from_parsecs(10.);
            let apparent_magnitude = luminosity.to_illuminance(&distance).to_apparent_magnitude();
            let absolute_magnitude = luminosity.to_absolute_magnitude();
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
            .to_illuminance(&Distance::from_astronomical_units(1.))
            .to_apparent_magnitude();
        let expected_apparent_magnitude = -26.74;
        println!(
            "Apparnet Magnitude:\nexpected: {},\nactual: {}",
            expected_apparent_magnitude, sun_apparent_magnitude
        );
        assert!(
            (expected_apparent_magnitude - sun_apparent_magnitude).abs() < REAL_DATA_TEST_ACCURACY
        );

        let sun_solar_luminosities = sun.to_solar_luminosities();
        let expected_solar_luminosities = 1.;
        println!(
            "Solar Luminosities:\nexpected: {},\nactual: {}",
            expected_solar_luminosities, sun_solar_luminosities
        );
        assert!(
            (expected_solar_luminosities - sun_solar_luminosities).abs() < REAL_DATA_TEST_ACCURACY
        );

        let sun_watts = sun.to_watts();
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
            .to_illuminance(&&Distance::from_lyr(8.6))
            .to_apparent_magnitude();
        let expected_apparent_magnitude = -1.46;
        println!(
            "Apparnet Magnitude:\nexpected: {},\nactual: {}",
            expected_apparent_magnitude, sirius_apparent_magnitude
        );
        assert!(
            (expected_apparent_magnitude - sirius_apparent_magnitude).abs()
                < REAL_DATA_TEST_ACCURACY
        );

        let sirius_solar_luminosities = sirius.to_solar_luminosities();
        let expected_solar_luminosities = 22.3;
        println!(
            "Solar Luminosities:\nexpected: {},\nactual: {}",
            expected_solar_luminosities, sirius_solar_luminosities
        );
        assert!(
            (expected_solar_luminosities - sirius_solar_luminosities).abs()
                < REAL_DATA_TEST_ACCURACY
        );

        let sirius_watts = sirius.to_watts();
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
