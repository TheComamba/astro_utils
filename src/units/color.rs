use crate::{color_matching_functions::*, Float};

use super::{length::Length, temperature::Temperature};

pub struct Color {
    red: Float,
    green: Float,
    blue: Float,
}

pub(crate) struct CIEXYZColor {
    x: Float,
    y: Float,
    z: Float,
}

impl Color {
    pub const fn from_rgb(red: Float, green: Float, blue: Float) -> Color {
        Color { red, green, blue }
    }

    pub fn from_temperature(temperature: Temperature) -> Color {
        let x_fun = Box::new(|lambda: Length| x_color_matching(lambda));
        let y_fun = Box::new(|lambda: Length| y_color_matching(lambda));
        let z_fun = Box::new(|lambda: Length| z_color_matching(lambda));
        let x = convolute_with_black_body(x_fun, temperature);
        let y = convolute_with_black_body(y_fun, temperature);
        let z = convolute_with_black_body(z_fun, temperature);
        let xyz = CIEXYZColor::from_xyz(x, y, z);
        xyz.to_rgb()
    }

    pub const fn as_rgb(&self) -> (Float, Float, Float) {
        (self.red, self.green, self.blue)
    }
}

/*
 * https://engineering.purdue.edu/~bouman/ece637/notes/pdf/Tristimulus.pdf
 * Page 15 dfines the transformation matrix from RGB to XYZ
 */
impl CIEXYZColor {
    const M: [[Float; 3]; 3] = [
        [0.490, 0.310, 0.200],
        [0.177, 0.813, 0.010],
        [0.000, 0.010, 0.990],
    ];

    const M_INV: [[Float; 3]; 3] = [
        [2.36, -0.896, -0.469],
        [-0.515, 1.425, 0.090],
        [0.005, -0.014, 1.009],
    ];

    fn from_xyz(x: Float, y: Float, z: Float) -> CIEXYZColor {
        CIEXYZColor { x, y, z }
    }

    fn to_rgb(&self) -> Color {
        let r = Self::M[0][0] * self.x + Self::M[0][1] * self.y + Self::M[0][2] * self.z;
        let g = Self::M[1][0] * self.x + Self::M[1][1] * self.y + Self::M[1][2] * self.z;
        let b = Self::M[2][0] * self.x + Self::M[2][1] * self.y + Self::M[2][2] * self.z;
        Color::from_rgb(r, g, b)
    }

    pub const fn as_xyz(&self) -> (Float, Float, Float) {
        (self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::TEST_ACCURACY;

    #[test]
    fn thousand_degrees_is_red() {
        let color = Color::from_temperature(Temperature::from_kelvin(5000.0));
        let (r, g, b) = color.as_rgb();
        println!("r: {}, g: {}, b: {}", r, g, b);
        assert!((r - 0.999).abs() < TEST_ACCURACY);
        assert!((g - 0.999).abs() < TEST_ACCURACY);
        assert!((b - 0.999).abs() < TEST_ACCURACY);
    }

    #[test]
    fn six_thousand_degrees_is_white() {
        let color = Color::from_temperature(Temperature::from_kelvin(6000.0));
        let (r, g, b) = color.as_rgb();
        println!("r: {}, g: {}, b: {}", r, g, b);
        assert!((r - 1.0).abs() < TEST_ACCURACY);
        assert!((g - 1.0).abs() < TEST_ACCURACY);
        assert!((b - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn twelve_thousand_degrees_is_blue() {
        let color = Color::from_temperature(Temperature::from_kelvin(10000.0));
        let (r, g, b) = color.as_rgb();
        println!("r: {}, g: {}, b: {}", r, g, b);
        assert!((r - 0.999).abs() < TEST_ACCURACY);
        assert!((g - 0.999).abs() < TEST_ACCURACY);
        assert!((b - 1.0).abs() < TEST_ACCURACY);
    }
}
