use astro_coords::ecliptic::EclipticCoordinates;
use serde::{Deserialize, Serialize};
use simple_si_units::{base::Time, electromagnetic::Illuminance, geometry::Angle};

use crate::{astro_display::AstroDisplay, color::srgb::sRGBColor};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarAppearance {
    pub(crate) name: String,
    pub(crate) illuminance: Illuminance<f64>,
    pub(crate) color: sRGBColor,
    pub(crate) pos: EclipticCoordinates,
    pub(crate) time_since_epoch: Time<f64>,
}

impl StarAppearance {
    pub fn new(
        name: String,
        illuminance: Illuminance<f64>,
        color: sRGBColor,
        pos: EclipticCoordinates,
        time_since_epoch: Time<f64>,
    ) -> Self {
        Self {
            name,
            illuminance,
            color,
            pos,
            time_since_epoch,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub const fn get_illuminance(&self) -> &Illuminance<f64> {
        &self.illuminance
    }

    pub const fn get_color(&self) -> &sRGBColor {
        &self.color
    }

    pub const fn get_pos(&self) -> &EclipticCoordinates {
        &self.pos
    }

    pub const fn get_time_since_epoch(&self) -> &Time<f64> {
        &self.time_since_epoch
    }

    pub fn set_pos(&mut self, direction: EclipticCoordinates) {
        self.pos = direction;
    }

    pub fn apparently_the_same(&self, other: &Self) -> bool {
        let angle_accuracy = Angle::from_degrees(0.03); //Rather high due to accos inaccuracy

        if !self.pos.eq_within(&other.pos, angle_accuracy) {
            return false;
        }
        let illuminance_ratio = self.illuminance.to_lux() / other.illuminance.to_lux();
        if !(0.1..=10.0).contains(&illuminance_ratio) {
            return false;
        }
        true
    }
}

impl AstroDisplay for StarAppearance {
    fn astro_display(&self) -> String {
        format!(
            "Star: {}\nIlluminance: {}\nColor: {}\nDirection: {}",
            self.name.astro_display(),
            self.illuminance.astro_display(),
            self.color.astro_display(),
            self.pos
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::units::time::TIME_ZERO;

    use super::*;

    #[test]
    fn star_is_apparently_the_same_with_itself() {
        let star = StarAppearance::new(
            "Schnuffelpuff".to_string(),
            Illuminance::from_lux(1.0),
            sRGBColor::from_sRGB(1.0, 1.0, 1.0),
            EclipticCoordinates::X_DIRECTION,
            TIME_ZERO,
        );

        assert!(star.apparently_the_same(&star));
    }

    #[test]
    fn star_is_not_the_same_if_direction_is_too_different() {
        let star = StarAppearance::new(
            "Schnuffelpuff".to_string(),
            Illuminance::from_lux(1.0),
            sRGBColor::from_sRGB(1.0, 1.0, 1.0),
            EclipticCoordinates::X_DIRECTION,
            TIME_ZERO,
        );
        let mut other = star.clone();
        other.pos = EclipticCoordinates::Y_DIRECTION;

        assert!(!star.apparently_the_same(&other));
    }

    #[test]
    fn star_is_not_the_same_if_brightness_is_too_different() {
        let star = StarAppearance::new(
            "Schnuffelpuff".to_string(),
            Illuminance::from_lux(1.0),
            sRGBColor::from_sRGB(1.0, 1.0, 1.0),
            EclipticCoordinates::X_DIRECTION,
            TIME_ZERO,
        );
        let mut other = star.clone();
        other.illuminance = Illuminance::from_lux(100.0);

        assert!(!star.apparently_the_same(&other));
    }
}
