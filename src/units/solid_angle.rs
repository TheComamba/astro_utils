use crate::astro_display::AstroDisplay;
use simple_si_units::geometry::SolidAngle;

pub const SOLID_ANGLE_ZERO: SolidAngle<f64> = SolidAngle { sr: 0.0 };

impl AstroDisplay for SolidAngle<f64> {
    fn astro_display(&self) -> String {
        format!("{:.2} sr", self.sr)
    }
}
