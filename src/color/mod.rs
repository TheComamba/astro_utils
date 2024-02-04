use self::color_matching_functions::*;
use serde::{Deserialize, Serialize};
use simple_si_units::base::{Distance, Temperature};
use std::fmt::Display;

pub mod black_body;
pub(crate) mod color_matching_functions;

/*
 * https://engineering.purdue.edu/~bouman/ece637/notes/pdf/Tristimulus.pdf
 * Page 15 dfines the transformation matrix from RGB to XYZ
 * Page 19 defines the transformation matrix from XYZ to RGB
 */
#[allow(non_upper_case_globals)]
const sRGB_TO_XYZ: [[f64; 3]; 3] = [
    [0.490, 0.310, 0.200],
    [0.177, 0.813, 0.010],
    [0.000, 0.010, 0.990],
];

#[allow(non_upper_case_globals)]
const XYZ_TO_sRGB: [[f64; 3]; 3] = [
    [2.3644, -0.8958, -0.4686],
    [-0.5148, 1.4252, 0.0896],
    [0.0052, -0.0144, 1.0092],
];

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct sRGBColor {
    R: f64,
    G: f64,
    B: f64,
}

impl Display for sRGBColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (r, g, b) = self.maximized_sRGB_tuple();
        write!(f, "({:.2}, {:.2}, {:.2})", r, g, b)
    }
}

#[allow(non_snake_case)]
pub struct XYZColor {
    X: f64,
    Y: f64,
    Z: f64,
}

impl sRGBColor {
    pub(crate) const DEFAULT: Self = sRGBColor::from_sRGB(1., 1., 1.);

    fn to_array(&self) -> [f64; 3] {
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

    pub fn from_temperature(temperature: Temperature<f64>) -> sRGBColor {
        XYZColor::from_temperature(temperature).to_sRGB()
    }

    #[allow(non_snake_case)]
    pub fn maximized_sRGB_tuple(&self) -> (f64, f64, f64) {
        let max = self.R.max(self.G).max(self.B);
        (self.R / max, self.G / max, self.B / max)
    }

    #[allow(non_snake_case)]
    pub fn to_XYZ(&self) -> XYZColor {
        let X =
            sRGB_TO_XYZ[0][0] * self.R + sRGB_TO_XYZ[0][1] * self.G + sRGB_TO_XYZ[0][2] * self.B;
        let Y =
            sRGB_TO_XYZ[1][0] * self.R + sRGB_TO_XYZ[1][1] * self.G + sRGB_TO_XYZ[1][2] * self.B;
        let Z =
            sRGB_TO_XYZ[2][0] * self.R + sRGB_TO_XYZ[2][1] * self.G + sRGB_TO_XYZ[2][2] * self.B;
        XYZColor { X, Y, Z }
    }
}

impl Serialize for sRGBColor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_array().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for sRGBColor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let array = <[f64; 3]>::deserialize(deserializer)?;
        sRGBColor::from_array(array).map_err(serde::de::Error::custom)
    }
}

impl XYZColor {
    #[allow(non_snake_case)]
    fn from_XYZ(X: f64, Y: f64, Z: f64) -> XYZColor {
        XYZColor { X, Y, Z }
    }

    #[allow(non_snake_case)]
    pub(super) fn from_temperature(temperature: Temperature<f64>) -> Self {
        let x_fun = Box::new(|lambda: Distance<f64>| x_color_matching(lambda));
        let y_fun = Box::new(|lambda: Distance<f64>| y_color_matching(lambda));
        let z_fun = Box::new(|lambda: Distance<f64>| z_color_matching(lambda));
        let X = convolute_with_black_body(x_fun, temperature);
        let Y = convolute_with_black_body(y_fun, temperature);
        let Z = convolute_with_black_body(z_fun, temperature);
        XYZColor::from_XYZ(X, Y, Z)
    }

    #[allow(non_snake_case)]
    pub fn to_sRGB(&self) -> sRGBColor {
        let R =
            XYZ_TO_sRGB[0][0] * self.X + XYZ_TO_sRGB[0][1] * self.Y + XYZ_TO_sRGB[0][2] * self.Z;
        let G =
            XYZ_TO_sRGB[1][0] * self.X + XYZ_TO_sRGB[1][1] * self.Y + XYZ_TO_sRGB[1][2] * self.Z;
        let B =
            XYZ_TO_sRGB[2][0] * self.X + XYZ_TO_sRGB[2][1] * self.Y + XYZ_TO_sRGB[2][2] * self.Z;
        sRGBColor::from_sRGB(R, G, B)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ACCURACY: f64 = 0.06;

    fn maximized_color_tuple(color: (f64, f64, f64)) -> (f64, f64, f64) {
        let max = color.0.max(color.1).max(color.2);
        (color.0 / max, color.1 / max, color.2 / max)
    }

    #[test]
    fn fivehundred_kelvin_is_red() {
        let color = sRGBColor::from_temperature(Temperature::from_kelvin(500.0));
        let expected = maximized_color_tuple((0.9, 0.1, 0.));
        let actual = color.maximized_sRGB_tuple();
        println!("expected: {:?}", expected);
        println!("actual: {:?}", actual);
        assert!((expected.0 - actual.0).abs() < TEST_ACCURACY);
        assert!((expected.1 - actual.1).abs() < TEST_ACCURACY);
        assert!((expected.2 - actual.2).abs() < TEST_ACCURACY);
    }

    #[test]
    fn fivethousandfivehounded_kelvin_is_white() {
        let color = sRGBColor::from_temperature(Temperature::from_kelvin(5500.0));
        let expected = maximized_color_tuple((1., 1., 1.));
        let actual = color.maximized_sRGB_tuple();
        println!("expected: {:?}", expected);
        println!("actual: {:?}", actual);
        assert!((expected.0 - actual.0).abs() < TEST_ACCURACY);
        assert!((expected.1 - actual.1).abs() < TEST_ACCURACY);
        assert!((expected.2 - actual.2).abs() < TEST_ACCURACY);
    }

    #[test]
    fn fourtythousand_kelvin_is_blue() {
        let color = sRGBColor::from_temperature(Temperature::from_kelvin(40_000.0));
        let expected: (f32, f32, f32) = maximized_color_tuple((0.25, 0.5, 1.0));
        let actual = color.maximized_sRGB_tuple();
        println!("expected: {:?}", expected);
        println!("actual: {:?}", actual);
        assert!((expected.0 - actual.0).abs() < TEST_ACCURACY);
        assert!((expected.1 - actual.1).abs() < TEST_ACCURACY);
        assert!((expected.2 - actual.2).abs() < TEST_ACCURACY);
    }
}
