use super::illuminance::Illuminance;
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

    pub fn to_illuminance(&self, solid_angle: Float) -> Illuminance {
        let lux = self.nit * solid_angle;
        Illuminance::from_lux(lux)
    }
}
