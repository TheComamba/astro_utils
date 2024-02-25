use crate::{
    astro_display::AstroDisplay, color::srgb::sRGBColor, coordinates::ecliptic::EclipticCoordinates,
};
use serde::{Deserialize, Serialize};
use simple_si_units::{base::Time, electromagnetic::Illuminance, geometry::Angle};

use super::appearance_evolution::StarAppearanceEvolution;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarAppearance {
    pub(crate) name: String,
    pub(crate) illuminance: Illuminance<f64>,
    pub(crate) color: sRGBColor,
    pub(crate) pos: EclipticCoordinates,
    pub(crate) evolution: StarAppearanceEvolution,
}

impl StarAppearance {
    pub fn new(
        name: String,
        illuminance: Illuminance<f64>,
        color: sRGBColor,
        pos: EclipticCoordinates,
        evolution: StarAppearanceEvolution,
    ) -> Self {
        Self {
            name,
            illuminance,
            color,
            pos,
            evolution,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub const fn get_illuminance_at_epoch(&self) -> &Illuminance<f64> {
        &self.illuminance
    }

    pub fn get_illuminance(&self, time: Time<f64>) -> Illuminance<f64> {
        self.evolution.apply_to_illuminance(self.illuminance, time)
    }

    pub const fn get_color_at_epoch(&self) -> &sRGBColor {
        &self.color
    }

    pub fn get_color(&self, time: Time<f64>) -> sRGBColor {
        self.evolution.apply_to_color(self.color, time)
    }

    pub const fn get_pos_at_epoch(&self) -> &EclipticCoordinates {
        &self.pos
    }

    pub fn get_pos(&self, _time: Time<f64>) -> EclipticCoordinates {
        self.pos.clone()
    }

    pub fn set_pos_at_epoch(&mut self, direction: EclipticCoordinates) {
        self.pos = direction;
    }

    pub fn get_evolution(&self) -> &StarAppearanceEvolution {
        &self.evolution
    }

    pub(super) fn apparently_the_same(&self, other: &Self) -> bool {
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
            self.pos.astro_display()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        real_data::stars::all::get_many_stars,
        tests::{eq_within, TEST_ACCURACY},
        units::{tests::ANGLE_TEST_ACCURACY, time::TIME_ZERO},
    };

    use super::*;

    #[test]
    fn star_is_apparently_the_same_with_itself() {
        let star = StarAppearance::new(
            "Schnuffelpuff".to_string(),
            Illuminance::from_lux(1.0),
            sRGBColor::from_sRGB(1.0, 1.0, 1.0),
            EclipticCoordinates::X_DIRECTION,
            StarAppearanceEvolution::NONE,
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
            StarAppearanceEvolution::NONE,
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
            StarAppearanceEvolution::NONE,
        );
        let mut other = star.clone();
        other.illuminance = Illuminance::from_lux(100.0);

        assert!(!star.apparently_the_same(&other));
    }

    #[test]
    fn comparing_getting_stuff_at_epoch() {
        let star_data: Vec<StarAppearance> = get_many_stars()
            .iter()
            .map(|s| s.to_star_appearance())
            .collect();
        for star in star_data {
            assert!(eq_within(
                star.get_illuminance_at_epoch().lux,
                star.get_illuminance(TIME_ZERO).lux,
                TEST_ACCURACY
            ));
            let color1 = star.get_color_at_epoch().maximized_sRGB_tuple();
            let color2 = star.get_color(TIME_ZERO).maximized_sRGB_tuple();
            assert!(eq_within(color1.0, color2.0, TEST_ACCURACY));
            assert!(eq_within(color1.1, color2.1, TEST_ACCURACY));
            assert!(eq_within(color1.2, color2.2, TEST_ACCURACY));
            assert!(star
                .get_pos_at_epoch()
                .eq_within(&star.get_pos(TIME_ZERO), ANGLE_TEST_ACCURACY));
        }
    }
}
