use crate::{solar_system_data::EARTH_AXIS_TILT, units::angle::Angle};

use super::ecliptic::EclipticCoordinates;

pub struct EarthEquatorialCoordinates {
    right_ascension: Angle,
    declination: Angle,
}

impl EarthEquatorialCoordinates {
    pub const fn new(right_ascension: Angle, declination: Angle) -> EarthEquatorialCoordinates {
        EarthEquatorialCoordinates {
            right_ascension,
            declination,
        }
    }

    pub fn to_ecliptic(&self) -> EclipticCoordinates {
        let obliquity = EARTH_AXIS_TILT;
        let right_ascension = self.right_ascension;
        let declination = self.declination;
        let ecliptic_longitude = right_ascension.cos() * declination.cos();
        let ecliptic_latitude = obliquity.sin() * right_ascension.cos() * declination.sin()
            + obliquity.cos() * declination.cos();

        EclipticCoordinates::new(
            Angle::from_radians(ecliptic_longitude),
            Angle::from_radians(ecliptic_latitude),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
