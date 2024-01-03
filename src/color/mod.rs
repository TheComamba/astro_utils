use std::fmt::Display;

use self::color_matching_functions::*;
use crate::{
    units::{length::Length, temperature::Temperature},
    Float,
};
use serde::{Deserialize, Serialize};

pub mod black_body;
pub(crate) mod color_matching_functions;

/*
 * https://engineering.purdue.edu/~bouman/ece637/notes/pdf/Tristimulus.pdf
 * Page 15 dfines the transformation matrix from RGB to XYZ
 * Page 19 defines the transformation matrix from XYZ to RGB
 */
#[allow(non_upper_case_globals)]
const sRGB_TO_XYZ: [[Float; 3]; 3] = [
    [0.490, 0.310, 0.200],
    [0.177, 0.813, 0.010],
    [0.000, 0.010, 0.990],
];

#[allow(non_upper_case_globals)]
const XYZ_TO_sRGB: [[Float; 3]; 3] = [
    [2.3644, -0.8958, -0.4686],
    [-0.5148, 1.4252, 0.0896],
    [0.0052, -0.0144, 1.0092],
];

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct sRGBColor {
    R: Float,
    G: Float,
    B: Float,
}

impl Display for sRGBColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (r, g, b) = self.normalized_sRGB_tuple();
        write!(f, "(r: {:.2}, g: {:.2}, b: {:.2})", r, g, b)
    }
}

#[allow(non_snake_case)]
pub struct XYZColor {
    X: Float,
    Y: Float,
    Z: Float,
}

impl sRGBColor {
    #[allow(non_snake_case)]
    pub const fn from_sRGB(R: Float, G: Float, B: Float) -> sRGBColor {
        sRGBColor { R, G, B }
    }

    pub fn from_temperature(temperature: Temperature) -> sRGBColor {
        XYZColor::from_temperature(temperature).to_sRGB()
    }

    #[allow(non_snake_case)]
    pub fn normalized_sRGB_tuple(&self) -> (Float, Float, Float) {
        let sum = self.R + self.G + self.B;
        (self.R / sum, self.G / sum, self.B / sum)
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

impl XYZColor {
    #[allow(non_snake_case)]
    fn from_XYZ(X: Float, Y: Float, Z: Float) -> XYZColor {
        XYZColor { X, Y, Z }
    }

    #[allow(non_snake_case)]
    pub(super) fn from_temperature(temperature: Temperature) -> Self {
        let x_fun = Box::new(|lambda: Length| x_color_matching(lambda));
        let y_fun = Box::new(|lambda: Length| y_color_matching(lambda));
        let z_fun = Box::new(|lambda: Length| z_color_matching(lambda));
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

    const TEST_ACCURACY: Float = 0.05;

    fn normed_color_tuple(color: (Float, Float, Float)) -> (Float, Float, Float) {
        let sum = color.0 + color.1 + color.2;
        (color.0 / sum, color.1 / sum, color.2 / sum)
    }

    #[test]
    fn fivehundred_kelvin_is_red() {
        let color = sRGBColor::from_temperature(Temperature::from_kelvin(500.0));
        let expected = normed_color_tuple((0.9, 0.1, 0.));
        let actual = color.normalized_sRGB_tuple();
        println!("expected: {:?}", expected);
        println!("actual: {:?}", actual);
        assert!((expected.0 - actual.0).abs() < TEST_ACCURACY);
        assert!((expected.1 - actual.1).abs() < TEST_ACCURACY);
        assert!((expected.2 - actual.2).abs() < TEST_ACCURACY);
    }

    #[test]
    fn six_thousand_kelvin_is_white() {
        let color = sRGBColor::from_temperature(Temperature::from_kelvin(6000.0));
        let expected = normed_color_tuple((1., 1., 1.));
        let actual = color.normalized_sRGB_tuple();
        println!("expected: {:?}", expected);
        println!("actual: {:?}", actual);
        assert!((expected.0 - actual.0).abs() < TEST_ACCURACY);
        assert!((expected.1 - actual.1).abs() < TEST_ACCURACY);
        assert!((expected.2 - actual.2).abs() < TEST_ACCURACY);
    }

    #[test]
    fn thirty_thousand_kelvin_is_blue() {
        let color = sRGBColor::from_temperature(Temperature::from_kelvin(30_000.0));
        let expected: (f32, f32, f32) = normed_color_tuple((0.1, 0.3, 0.6));
        let actual = color.normalized_sRGB_tuple();
        println!("expected: {:?}", expected);
        println!("actual: {:?}", actual);
        assert!((expected.0 - actual.0).abs() < TEST_ACCURACY);
        assert!((expected.1 - actual.1).abs() < TEST_ACCURACY);
        assert!((expected.2 - actual.2).abs() < TEST_ACCURACY);
    }
}
