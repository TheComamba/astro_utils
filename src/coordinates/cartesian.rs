use crate::{units::length::Length, Float};

pub static ORIGIN: CartesianCoordinates = CartesianCoordinates {
    x: Length::from_meters(0.0),
    y: Length::from_meters(0.0),
    z: Length::from_meters(0.0),
};

pub struct CartesianCoordinates {
    x: Length,
    y: Length,
    z: Length,
}

impl CartesianCoordinates {
    pub fn eq_within(&self, other: &CartesianCoordinates, accuracy: Float) -> bool {
        self.x.eq_within(other.x, accuracy)
            && self.y.eq_within(other.y, accuracy)
            && self.z.eq_within(other.z, accuracy)
    }
}
