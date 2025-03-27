use std::fmt::Display;
use std::ops::{Add, Mul, Neg, Sub};

use crate::astro_display::AstroDisplay;
use serde::{ser::SerializeTuple, Serializer};
use serde::{Deserialize, Serialize};
use uom::si::f64::ThermodynamicTemperature;

use super::xyz::XYZColor;

#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct sRGBColor {
    R: f64,
    G: f64,
    B: f64,
}

impl AstroDisplay for sRGBColor {
    fn astro_display(&self) -> String {
        let (r, g, b) = self.maximized_sRGB_tuple();
        format!("({:.2}, {:.2}, {:.2})", r, g, b)
    }
}

impl Display for sRGBColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.astro_display())
    }
}

impl sRGBColor {
    #[cfg(test)]
    pub(crate) const WHITE: Self = sRGBColor::from_sRGB(1., 1., 1.);
    const SERIALIZATION_ACCURACY: f64 = 1e-2;

    fn as_array(&self) -> [f64; 3] {
        [self.R, self.G, self.B]
    }

    fn from_array(array: [f64; 3]) -> Result<Self, String> {
        #[allow(non_snake_case)]
        let [R, G, B] = array;
        Ok(sRGBColor { R, G, B })
    }

    #[allow(non_snake_case)]
    pub const fn from_sRGB(R: f64, G: f64, B: f64) -> sRGBColor {
        sRGBColor { R, G, B }
    }

    pub fn from_temperature(temperature: ThermodynamicTemperature) -> sRGBColor {
        XYZColor::from_temperature(temperature).to_sRGB()
    }

    #[allow(non_snake_case)]
    pub fn maximized_sRGB_tuple(&self) -> (f64, f64, f64) {
        let max = self.R.max(self.G).max(self.B);
        (self.R / max, self.G / max, self.B / max)
    }
}

impl Serialize for sRGBColor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let array = self.as_array();
        let mut tuple_serializer = serializer.serialize_tuple(3)?;
        for value in &array {
            let value =
                (value / Self::SERIALIZATION_ACCURACY).round() * Self::SERIALIZATION_ACCURACY;
            tuple_serializer.serialize_element(&value)?;
        }
        tuple_serializer.end()
    }
}

impl<'de> Deserialize<'de> for sRGBColor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let array = <[f64; 3]>::deserialize(deserializer)?;
        sRGBColor::from_array(array).map_err(serde::de::Error::custom)
    }
}

impl Add for &sRGBColor {
    type Output = sRGBColor;

    fn add(self, other: Self) -> sRGBColor {
        sRGBColor {
            R: self.R + other.R,
            G: self.G + other.G,
            B: self.B + other.B,
        }
    }
}

impl Sub for &sRGBColor {
    type Output = sRGBColor;

    fn sub(self, other: Self) -> sRGBColor {
        sRGBColor {
            R: self.R - other.R,
            G: self.G - other.G,
            B: self.B - other.B,
        }
    }
}

impl Neg for &sRGBColor {
    type Output = sRGBColor;

    fn neg(self) -> sRGBColor {
        sRGBColor {
            R: -self.R,
            G: -self.G,
            B: -self.B,
        }
    }
}

impl Mul<f64> for &sRGBColor {
    type Output = sRGBColor;

    fn mul(self, scalar: f64) -> sRGBColor {
        sRGBColor {
            R: self.R * scalar,
            G: self.G * scalar,
            B: self.B * scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::eq_within;

    use super::*;

    const COLOR_TEST_ACCURACY: f64 = 0.06;

    fn maximized_color_tuple(color: (f64, f64, f64)) -> (f64, f64, f64) {
        let max = color.0.max(color.1).max(color.2);
        (color.0 / max, color.1 / max, color.2 / max)
    }

    #[test]
    fn fivehundred_kelvin_is_red() {
        let color = sRGBColor::from_temperature(Temperature::from_K(500.0));
        let expected = maximized_color_tuple((0.9, 0.1, 0.));
        let actual = color.maximized_sRGB_tuple();
        println!("expected: {:?}", expected);
        println!("actual: {:?}", actual);
        assert!(eq_within(expected.0, actual.0, COLOR_TEST_ACCURACY));
        assert!(eq_within(expected.1, actual.1, COLOR_TEST_ACCURACY));
        assert!(eq_within(expected.2, actual.2, COLOR_TEST_ACCURACY));
    }

    #[test]
    fn fivethousandfivehounded_kelvin_is_white() {
        let color = sRGBColor::from_temperature(Temperature::from_K(5500.0));
        let expected = maximized_color_tuple((1., 1., 1.));
        let actual = color.maximized_sRGB_tuple();
        println!("expected: {:?}", expected);
        println!("actual: {:?}", actual);
        assert!(eq_within(expected.0, actual.0, COLOR_TEST_ACCURACY));
        assert!(eq_within(expected.1, actual.1, COLOR_TEST_ACCURACY));
        assert!(eq_within(expected.2, actual.2, COLOR_TEST_ACCURACY));
    }

    #[test]
    fn fourtythousand_kelvin_is_blue() {
        let color = sRGBColor::from_temperature(Temperature::from_K(40_000.0));
        let expected = maximized_color_tuple((0.25, 0.5, 1.0));
        let actual = color.maximized_sRGB_tuple();
        println!("expected: {:?}", expected);
        println!("actual: {:?}", actual);
        assert!(eq_within(expected.0, actual.0, COLOR_TEST_ACCURACY));
        assert!(eq_within(expected.1, actual.1, COLOR_TEST_ACCURACY));
        assert!(eq_within(expected.2, actual.2, COLOR_TEST_ACCURACY));
    }

    #[test]
    fn serialization() {
        let color = sRGBColor::from_sRGB(1.23, -0.01, 1e-8);
        let serialized = serde_json::to_string(&color).unwrap();
        println!("{:?}", color);
        println!("{}", serialized);
        assert_eq!(serialized, "[1.23,-0.01,0.0]");

        let deserialized: sRGBColor = serde_json::from_str(&serialized).unwrap();
        println!("{:?}", deserialized);
        assert!(eq_within(
            deserialized.R,
            color.R,
            sRGBColor::SERIALIZATION_ACCURACY
        ));
        assert!(eq_within(
            deserialized.G,
            color.G,
            sRGBColor::SERIALIZATION_ACCURACY
        ));
        assert!(eq_within(
            deserialized.B,
            color.B,
            sRGBColor::SERIALIZATION_ACCURACY
        ));
    }
}
