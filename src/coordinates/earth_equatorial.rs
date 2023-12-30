use crate::units::angle::Angle;

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
}
