use super::{length::Length, temperature::Temperature};
use crate::{color_matching_functions::*, Float};

/*
 * https://engineering.purdue.edu/~bouman/ece637/notes/pdf/Tristimulus.pdf
 * Page 15 dfines the transformation matrix from RGB to XYZ
 * Page 19 defines the transformation matrix from XYZ to RGB
 */
const RGB_TO_XYZ: [[Float; 3]; 3] = [
    [0.490, 0.310, 0.200],
    [0.177, 0.813, 0.010],
    [0.000, 0.010, 0.990],
];

const XYZ_TO_RGB: [[Float; 3]; 3] = [
    [2.3644, -0.8958, -0.4686],
    [-0.5148, 1.4252, 0.0896],
    [0.0052, -0.0144, 1.0092],
];
pub struct Color {
    red: Float,
    green: Float,
    blue: Float,
}

pub struct CIEXYZColor {
    x: Float,
    y: Float,
    z: Float,
}

impl Color {
    pub const fn from_rgb(red: Float, green: Float, blue: Float) -> Color {
        Color { red, green, blue }
    }

    pub fn from_temperature(temperature: Temperature) -> Color {
        CIEXYZColor::from_temperature(temperature).to_rgb()
    }

    pub fn normalized_rgb(&self) -> (Float, Float, Float) {
        let sum = self.red + self.green + self.blue;
        (self.red / sum, self.green / sum, self.blue / sum)
    }
}

impl CIEXYZColor {
    fn from_xyz(x: Float, y: Float, z: Float) -> CIEXYZColor {
        CIEXYZColor { x, y, z }
    }

    pub(super) fn from_temperature(temperature: Temperature) -> Self {
        let x_fun = Box::new(|lambda: Length| x_color_matching(lambda));
        let y_fun = Box::new(|lambda: Length| y_color_matching(lambda));
        let z_fun = Box::new(|lambda: Length| z_color_matching(lambda));
        let x = convolute_with_black_body(x_fun, temperature);
        let y = convolute_with_black_body(y_fun, temperature);
        let z = convolute_with_black_body(z_fun, temperature);
        CIEXYZColor::from_xyz(x, y, z)
    }

    pub fn from_rgb(red: Float, green: Float, blue: Float) -> CIEXYZColor {
        let x = RGB_TO_XYZ[0][0] * red + RGB_TO_XYZ[0][1] * green + RGB_TO_XYZ[0][2] * blue;
        let y = RGB_TO_XYZ[1][0] * red + RGB_TO_XYZ[1][1] * green + RGB_TO_XYZ[1][2] * blue;
        let z = RGB_TO_XYZ[2][0] * red + RGB_TO_XYZ[2][1] * green + RGB_TO_XYZ[2][2] * blue;
        CIEXYZColor { x, y, z }
    }

    pub fn to_rgb(&self) -> Color {
        let r = XYZ_TO_RGB[0][0] * self.x + XYZ_TO_RGB[0][1] * self.y + XYZ_TO_RGB[0][2] * self.z;
        let g = XYZ_TO_RGB[1][0] * self.x + XYZ_TO_RGB[1][1] * self.y + XYZ_TO_RGB[1][2] * self.z;
        let b = XYZ_TO_RGB[2][0] * self.x + XYZ_TO_RGB[2][1] * self.y + XYZ_TO_RGB[2][2] * self.z;
        Color::from_rgb(r, g, b)
    }

    pub const fn as_xyz(&self) -> (Float, Float, Float) {
        (self.x, self.y, self.z)
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
        let color = Color::from_temperature(Temperature::from_kelvin(500.0));
        let expected = normed_color_tuple((0.9, 0.1, 0.));
        let actual = color.normalized_rgb();
        println!("expected: {:?}", expected);
        println!("actual: {:?}", actual);
        assert!((expected.0 - actual.0).abs() < TEST_ACCURACY);
        assert!((expected.1 - actual.1).abs() < TEST_ACCURACY);
        assert!((expected.2 - actual.2).abs() < TEST_ACCURACY);
    }

    #[test]
    fn six_thousand_kelvin_is_white() {
        let color = Color::from_temperature(Temperature::from_kelvin(6000.0));
        let expected = normed_color_tuple((1., 1., 1.));
        let actual = color.normalized_rgb();
        println!("expected: {:?}", expected);
        println!("actual: {:?}", actual);
        assert!((expected.0 - actual.0).abs() < TEST_ACCURACY);
        assert!((expected.1 - actual.1).abs() < TEST_ACCURACY);
        assert!((expected.2 - actual.2).abs() < TEST_ACCURACY);
    }

    #[test]
    fn thirty_thousand_kelvin_is_blue() {
        let color = Color::from_temperature(Temperature::from_kelvin(30_000.0));
        let expected: (f32, f32, f32) = normed_color_tuple((0.1, 0.3, 0.6));
        let actual = color.normalized_rgb();
        println!("expected: {:?}", expected);
        println!("actual: {:?}", actual);
        assert!((expected.0 - actual.0).abs() < TEST_ACCURACY);
        assert!((expected.1 - actual.1).abs() < TEST_ACCURACY);
        assert!((expected.2 - actual.2).abs() < TEST_ACCURACY);
    }
}
