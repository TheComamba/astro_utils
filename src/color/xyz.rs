use super::color_matching_functions::*;
use super::srgb::sRGBColor;
use simple_si_units::base::{Distance, Temperature};

#[allow(non_snake_case)]
pub(super) struct XYZColor {
    pub(super) X: f64,
    pub(super) Y: f64,
    pub(super) Z: f64,
}

/*
 * https://engineering.purdue.edu/~bouman/ece637/notes/pdf/Tristimulus.pdf
 * Page 15 dfines the transformation matrix from RGB to XYZ
 * Page 19 defines the transformation matrix from XYZ to RGB
 */
#[allow(non_upper_case_globals)]
pub(super) const sRGB_TO_XYZ: [[f64; 3]; 3] = [
    [0.490, 0.310, 0.200],
    [0.177, 0.813, 0.010],
    [0.000, 0.010, 0.990],
];

#[allow(non_upper_case_globals)]
pub(super) const XYZ_TO_sRGB: [[f64; 3]; 3] = [
    [2.3644, -0.8958, -0.4686],
    [-0.5148, 1.4252, 0.0896],
    [0.0052, -0.0144, 1.0092],
];

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
