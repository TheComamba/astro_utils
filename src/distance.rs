use crate::Float;
use std::{
    fmt::Display,
    ops::{Add, Sub},
};

pub static METERS_PER_KILOMETER: Float = 1000.0;
pub static METERS_PER_EARTH_RADIUS: Float = 6_371_000.0;
pub static METERS_PER_JUPITER_RADIUS: Float = 69_911_000.0;
pub static METERS_PER_SUN_RADIUS: Float = 695_700_000.0;
pub static METERS_PER_ASTRONOMICAL_UNIT: Float = 149_597_870_700.0;
pub static METERS_PER_LIGHT_YEAR: Float = 9_460_730_472_580_800.0;

pub struct Distance {
    meters: Float,
}

impl Distance {
    pub fn from_meters(meters: Float) -> Distance {
        Distance { meters }
    }

    pub fn from_kilometers(kilometers: Float) -> Distance {
        Distance {
            meters: kilometers * METERS_PER_KILOMETER,
        }
    }

    pub fn from_earth_radii(earth_radii: Float) -> Distance {
        Distance {
            meters: earth_radii * METERS_PER_EARTH_RADIUS,
        }
    }

    pub fn from_jupiter_radii(jupiter_radii: Float) -> Distance {
        Distance {
            meters: jupiter_radii * METERS_PER_JUPITER_RADIUS,
        }
    }

    pub fn from_sun_radii(sun_radii: Float) -> Distance {
        Distance {
            meters: sun_radii * METERS_PER_SUN_RADIUS,
        }
    }

    pub fn from_astronomical_units(astronomical_units: Float) -> Distance {
        Distance {
            meters: astronomical_units * METERS_PER_ASTRONOMICAL_UNIT,
        }
    }

    pub fn from_light_years(light_years: Float) -> Distance {
        Distance {
            meters: light_years * METERS_PER_LIGHT_YEAR,
        }
    }

    pub fn as_meters(&self) -> Float {
        self.meters
    }

    pub fn as_kilometers(&self) -> Float {
        self.meters / METERS_PER_KILOMETER
    }

    pub fn as_earth_radii(&self) -> Float {
        self.meters / METERS_PER_EARTH_RADIUS
    }

    pub fn as_jupiter_radii(&self) -> Float {
        self.meters / METERS_PER_JUPITER_RADIUS
    }

    pub fn as_sun_radii(&self) -> Float {
        self.meters / METERS_PER_SUN_RADIUS
    }

    pub fn as_astronomical_units(&self) -> Float {
        self.meters / METERS_PER_ASTRONOMICAL_UNIT
    }

    pub fn as_light_years(&self) -> Float {
        self.meters / METERS_PER_LIGHT_YEAR
    }
}

impl Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.meters > METERS_PER_LIGHT_YEAR {
            write!(f, "{} light years", self.as_light_years())
        } else if self.meters > METERS_PER_ASTRONOMICAL_UNIT {
            write!(f, "{} AU", self.as_astronomical_units())
        } else if self.meters > METERS_PER_SUN_RADIUS {
            write!(f, "{} Sun radii", self.as_sun_radii())
        } else if self.meters > METERS_PER_JUPITER_RADIUS {
            write!(f, "{} Jupiter radii", self.as_jupiter_radii())
        } else if self.meters > METERS_PER_EARTH_RADIUS {
            write!(f, "{} Earth radii", self.as_earth_radii())
        } else if self.meters > METERS_PER_KILOMETER {
            write!(f, "{} km", self.as_kilometers())
        } else {
            write!(f, "{} m", self.as_meters())
        }
    }
}

impl Add for Distance {
    type Output = Distance;

    fn add(self, other: Distance) -> Distance {
        Distance {
            meters: self.meters + other.meters,
        }
    }
}

impl Sub for Distance {
    type Output = Distance;

    fn sub(self, other: Distance) -> Distance {
        Distance {
            meters: self.meters - other.meters,
        }
    }
}
