use super::spherical::SphericalCoordinates;
use crate::Float;
use serde::{Deserialize, Serialize};
use simple_si_units::geometry::Angle;
use std::fmt::{Display, Formatter};
use std::ops::Neg;

/*  The "absolute" reference we use for polar coordiantes is heliocentric ecliptic coordinates:
 * Longitude denotes the angle between the vernal equinox and the body, measured in the ecliptic plane.
 * Latitude denotes the angle between the ecliptic plane and the body.
 * https://en.wikipedia.org/wiki/Ecliptic_coordinate_system
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EclipticCoordinates {
    pub(super) spherical: SphericalCoordinates,
}

impl EclipticCoordinates {
    pub const X_DIRECTION: EclipticCoordinates = EclipticCoordinates {
        spherical: SphericalCoordinates::X_DIRECTION,
    };

    pub const Y_DIRECTION: EclipticCoordinates = EclipticCoordinates {
        spherical: SphericalCoordinates::Y_DIRECTION,
    };

    pub const Z_DIRECTION: EclipticCoordinates = EclipticCoordinates {
        spherical: SphericalCoordinates::Z_DIRECTION,
    };

    pub const fn new(spherical: SphericalCoordinates) -> EclipticCoordinates {
        EclipticCoordinates { spherical }
    }

    pub fn normalize(&mut self) {
        self.spherical.normalize();
    }

    #[cfg(test)]
    pub(crate) fn eq_within(&self, other: &EclipticCoordinates, accuracy: Angle<Float>) -> bool {
        self.spherical.eq_within(&other.spherical, accuracy)
    }

    pub fn get_longitude(&self) -> Angle<Float> {
        self.spherical.get_longitude()
    }

    pub fn get_latitude(&self) -> Angle<Float> {
        self.spherical.get_latitude()
    }

    pub fn get_spherical(&self) -> &SphericalCoordinates {
        &self.spherical
    }

    pub fn set_longitude(&mut self, longitude: Angle<Float>) {
        self.spherical.set_longitude(longitude);
    }

    pub fn set_latitude(&mut self, latitude: Angle<Float>) {
        self.spherical.set_latitude(latitude);
    }
}

impl Neg for &EclipticCoordinates {
    type Output = EclipticCoordinates;

    fn neg(self) -> EclipticCoordinates {
        EclipticCoordinates {
            spherical: -self.spherical,
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
        write!(f, "{} to ecliptic plane", self.spherical)
    }
}
