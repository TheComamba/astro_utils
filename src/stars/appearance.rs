use astro_coords::ecliptic::Ecliptic;
use serde::{Deserialize, Serialize};
use uom::si::{
    angle::degree,
    f64::{Angle, Time},
};

use crate::{
    astro_display::AstroDisplay,
    color::srgb::sRGBColor,
    units::illuminance::{lux, Illuminance},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarAppearance {
    pub(crate) name: String,
    pub(crate) illuminance: Illuminance,
    pub(crate) color: sRGBColor,
    pub(crate) pos: Ecliptic,
    pub(crate) time_since_epoch: Time,
}

impl StarAppearance {
    pub fn new(
        name: String,
        illuminance: Illuminance,
        color: sRGBColor,
        pos: Ecliptic,
        time_since_epoch: Time,
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

    pub const fn get_illuminance(&self) -> &Illuminance {
        &self.illuminance
    }

    pub const fn get_color(&self) -> &sRGBColor {
        &self.color
    }

    pub const fn get_pos(&self) -> &Ecliptic {
        &self.pos
    }

    pub const fn get_time_since_epoch(&self) -> &Time {
        &self.time_since_epoch
    }

    pub fn set_pos(&mut self, direction: Ecliptic) {
        self.pos = direction;
    }

    pub fn apparently_the_same(&self, other: &Self) -> bool {
        let angle_accuracy = Angle::new::<degree>(0.03); //Rather high due to accos inaccuracy

        if !self.pos.eq_within(&other.pos, angle_accuracy) {
            return false;
        }
        let illuminance_ratio = self.illuminance.get::<lux>() / other.illuminance.get::<lux>();
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

    use uom::si::time::year;

    use super::*;

    #[test]
    fn star_is_apparently_the_same_with_itself() {
        let star = StarAppearance::new(
            "Schnuffelpuff".to_string(),
            Illuminance::new::<lux>(1.0),
            sRGBColor::from_sRGB(1.0, 1.0, 1.0),
            Ecliptic::x_direction(),
            Time::new::<year>(0.),
        );

        assert!(star.apparently_the_same(&star));
    }

    #[test]
    fn star_is_not_the_same_if_direction_is_too_different() {
        let star = StarAppearance::new(
            "Schnuffelpuff".to_string(),
            Illuminance::new::<lux>(1.0),
            sRGBColor::from_sRGB(1.0, 1.0, 1.0),
            Ecliptic::x_direction(),
            Time::new::<year>(0.),
        );
        let mut other = star.clone();
        other.pos = Ecliptic::y_direction();

        assert!(!star.apparently_the_same(&other));
    }

    #[test]
    fn star_is_not_the_same_if_brightness_is_too_different() {
        let star = StarAppearance::new(
            "Schnuffelpuff".to_string(),
            Illuminance::new::<lux>(1.0),
            sRGBColor::from_sRGB(1.0, 1.0, 1.0),
            Ecliptic::x_direction(),
            Time::new::<year>(0.),
        );
        let mut other = star.clone();
        other.illuminance = Illuminance::new::<lux>(100.0);

        assert!(!star.apparently_the_same(&other));
    }
}
