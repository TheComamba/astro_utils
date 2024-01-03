use super::{angle::Angle, illuminance::Illuminance};
use crate::Float;

pub struct Luminance {
    pub(super) nit: Float,
}

impl Luminance {
    pub const fn from_nit(nit: Float) -> Luminance {
        Luminance { nit }
    }

    pub fn as_nit(&self) -> Float {
        self.nit
    }

    pub fn to_illuminance(&self, solid_angle: &Angle) -> Illuminance {
        let lux = self.nit * solid_angle.as_radians();
        Illuminance::from_lux(lux)
    }
}
