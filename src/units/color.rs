use crate::Float;

use super::temperature::Temperature;

pub struct Color {
    red: Float,
    green: Float,
    blue: Float,
}

impl Color {
    pub const fn from_rgb(red: Float, green: Float, blue: Float) -> Color {
        Color { red, green, blue }
    }

    pub fn from_temperature(t: Temperature) -> Color {
        todo!()
    }

    pub const fn as_rgb(&self) -> (Float, Float, Float) {
        (self.red, self.green, self.blue)
    }
}
