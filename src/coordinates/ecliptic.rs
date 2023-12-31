use super::cartesian::CartesianCoordinates;
use crate::{units::angle::Angle, Float, PI};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Neg;

pub const X_DIRECTION: EclipticCoordinates = EclipticCoordinates {
    longitude: Angle::from_radians(0.),
    latitude: Angle::from_radians(0.),
};

pub const Y_DIRECTION: EclipticCoordinates = EclipticCoordinates {
    longitude: Angle::from_radians(PI / 2.),
    latitude: Angle::from_radians(0.),
};

pub const Z_DIRECTION: EclipticCoordinates = EclipticCoordinates {
    longitude: Angle::from_radians(0.),
    latitude: Angle::from_radians(PI / 2.),
};

const PI_HALF: Float = PI / 2.;

/*  The "absolute" reference we use for polar coordiantes is heliocentric ecliptic coordinates:
 * Longitude denotes the angle between the vernal equinox and the body, measured in the ecliptic plane.
 * Latitude denotes the angle between the ecliptic plane and the body.
 * https://en.wikipedia.org/wiki/Ecliptic_coordinate_system
 */
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct EclipticCoordinates {
    longitude: Angle,
    latitude: Angle,
}

impl EclipticCoordinates {
    pub(crate) const fn new(longitude: Angle, latitude: Angle) -> EclipticCoordinates {
        EclipticCoordinates {
            longitude,
            latitude,
        }
    }

    pub fn from_cartesian(cart: &CartesianCoordinates) -> EclipticCoordinates {
        let x = cart.x().as_meters();
        let y = cart.y().as_meters();
        let z = cart.z().as_meters();
        let longitude = Angle::from_radians(y.atan2(x));
        let latitude = Angle::from_radians(z.atan2((x * x + y * y).sqrt()));
        EclipticCoordinates {
            longitude,
            latitude,
        }
    }

    pub(crate) fn horizontal(longitude: Angle) -> EclipticCoordinates {
        EclipticCoordinates {
            longitude,
            latitude: Angle::from_radians(0.),
        }
    }

    pub fn normalize(&mut self) {
        self.longitude.normalize();
        self.latitude.normalize();
        if self.latitude.as_radians() > PI_HALF {
            self.longitude = self.longitude + Angle::from_radians(PI);
            self.longitude.normalize();
            self.latitude = Angle::from_radians(PI) - self.latitude;
        } else if self.latitude.as_radians() < -PI_HALF {
            self.longitude = self.longitude + Angle::from_radians(PI);
            self.longitude.normalize();
            self.latitude = Angle::from_radians(-PI) - self.latitude;
        }
    }

    pub fn eq_within(&self, other: &EclipticCoordinates, accuracy: Angle) -> bool {
        const NORTHPOLE_LATITUDE: Angle = Angle::from_radians(PI_HALF);
        const SOUTHPOLE_LATITUDE: Angle = Angle::from_radians(-PI_HALF);
        let mut clone = self.clone();
        let mut other_clone = other.clone();
        clone.normalize();
        other_clone.normalize();
        let latitudes_equal = clone.latitude.eq_within(other_clone.latitude, accuracy);
        let is_pole = clone.latitude.eq_within(NORTHPOLE_LATITUDE, accuracy)
            || clone.latitude.eq_within(SOUTHPOLE_LATITUDE, accuracy);
        let longitudes_equal = if is_pole {
            true
        } else {
            clone.longitude.eq_within(other_clone.longitude, accuracy)
        };
        latitudes_equal && longitudes_equal
    }

    pub fn get_longitude(&self) -> Angle {
        self.longitude
    }

    pub fn get_latitude(&self) -> Angle {
        self.latitude
    }

    pub fn set_longitude(&mut self, longitude: Angle) {
        self.longitude = longitude;
    }

    pub fn set_latitude(&mut self, latitude: Angle) {
        self.latitude = latitude;
    }
}

impl Neg for &EclipticCoordinates {
    type Output = EclipticCoordinates;

    fn neg(self) -> EclipticCoordinates {
        let mut longitude = self.longitude + Angle::from_radians(PI);
        longitude.normalize();
        let latitude = -self.latitude;
        EclipticCoordinates {
            longitude,
            latitude,
        }
    }
}

impl Neg for EclipticCoordinates {
    type Output = EclipticCoordinates;

    fn neg(self) -> EclipticCoordinates {
        -&self
    }
}

impl Display for EclipticCoordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} long, {} lat)", self.longitude, self.latitude)
    }
}
