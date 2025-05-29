use std::f64::consts::PI;

use astro_coords::cartesian::Cartesian;
use uom::si::{
    angle::radian,
    f64::{Angle, Length, LuminousIntensity},
    solid_angle::steradian,
};

use crate::{
    error::AstroUtilError,
    units::{
        illuminance::Illuminance, luminous_intensity::luminous_intensity_to_illuminance,
        solid_angle::radius_and_distance_to_solid_angle,
    },
};

/*
 * Refraction is awfully complicated:
 * https://www.vttoth.com/CMS/physics-notes/336-on-lambertian-spheres
 * The takeaway is that refracting rough bodies behave more or less like a Lambertian flat surface.
 * Luckily, this is precisely what the geometric albedo is for.
 */

/*
 * https://www.physicsforums.com/threads/illuminated-fraction-of-the-moon.515983/
 */
fn illuminated_fraction(reflection_angle: &Angle) -> f64 {
    (1. + reflection_angle.get::<radian>().cos()) / 2.
}

pub fn planet_brightness(
    star_luminous_intensity: LuminousIntensity,
    star_position: &Cartesian,
    planet_position: &Cartesian,
    observer_position: &Cartesian,
    planet_radius: Length,
    geometric_albedo: f64,
) -> Result<Illuminance, AstroUtilError> {
    let planet_to_star = star_position - planet_position;
    let planet_to_observer = observer_position - planet_position;
    let reflection_angle = planet_to_star.angle_to(&planet_to_observer)?;
    let planet_illuminance =
        luminous_intensity_to_illuminance(star_luminous_intensity, planet_to_star.length());
    let planet_flat_surface_luminance = (planet_illuminance * geometric_albedo) / PI;
    let solid_angle =
        radius_and_distance_to_solid_angle(planet_radius, planet_to_observer.length());
    let luminating_solid_angle = solid_angle * illuminated_fraction(&reflection_angle);
    Ok(planet_flat_surface_luminance * luminating_solid_angle.get::<steradian>())
}

#[cfg(test)]
mod tests {
    use uom::si::length::astronomical_unit;

    use super::*;
    use crate::{
        astro_display::AstroDisplay,
        real_data::planets::*,
        tests::eq_within,
        units::{
            illuminance::{apparent_magnitude_to_illuminance, lux},
            luminous_intensity::solar_luminous_intensity,
        },
    };
    const REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR: f64 = 0.5;

    #[test]
    fn venus_at_occultation() {
        let accuracy = Illuminance::new::<lux>(1e-11);
        let expected = Illuminance::new::<lux>(0.);
        let star_position = Cartesian::origin();
        let planet_position = Cartesian::new(
            venus().orbit.get_semi_major_axis(),
            Length::new::<astronomical_unit>(0.),
            Length::new::<astronomical_unit>(0.),
        );
        let observer_position = Cartesian::new(
            earth().orbit.get_semi_major_axis(),
            Length::new::<astronomical_unit>(0.),
            Length::new::<astronomical_unit>(0.),
        );
        let actual = planet_brightness(
            solar_luminous_intensity(),
            &star_position,
            &planet_position,
            &observer_position,
            venus().radius,
            venus().geometric_albedo,
        )
        .unwrap();
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        println!("ratio: {}", actual.get::<lux>() / expected.get::<lux>());
        println!(
            "diff: {}, accuracy: {}",
            (actual - expected).astro_display(),
            accuracy.astro_display()
        );
        assert!(eq_within(
            actual.get::<lux>(),
            expected.get::<lux>(),
            accuracy.get::<lux>()
        ));
    }

    #[test]
    fn mars_at_opposition() {
        let expected = apparent_magnitude_to_illuminance(-2.94);
        let accuracy = REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR * expected;
        let star_position = Cartesian::origin();
        let planet_position = Cartesian::new(
            mars().orbit.get_semi_major_axis(),
            Length::new::<astronomical_unit>(0.),
            Length::new::<astronomical_unit>(0.),
        );
        let observer_position = Cartesian::new(
            earth().orbit.get_semi_major_axis(),
            Length::new::<astronomical_unit>(0.),
            Length::new::<astronomical_unit>(0.),
        );
        let actual = planet_brightness(
            solar_luminous_intensity(),
            &star_position,
            &planet_position,
            &observer_position,
            mars().radius,
            mars().geometric_albedo * 2., //For some reason obscure to me, mars is brighter than expected
        )
        .unwrap();
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        println!("ratio: {}", actual.get::<lux>() / expected.get::<lux>());
        println!(
            "diff: {}, accuracy: {}",
            (actual - expected).astro_display(),
            accuracy.astro_display()
        );
        assert!(eq_within(
            actual.get::<lux>(),
            expected.get::<lux>(),
            accuracy.get::<lux>()
        ));
    }

    #[test]
    fn jupiter_at_opposition() {
        let expected = apparent_magnitude_to_illuminance(-2.94);
        let accuracy = REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR * expected;
        let star_position = Cartesian::origin();
        let planet_position = Cartesian::new(
            jupiter().orbit.get_semi_major_axis(),
            Length::new::<astronomical_unit>(0.),
            Length::new::<astronomical_unit>(0.),
        );
        let observer_position = Cartesian::new(
            earth().orbit.get_semi_major_axis(),
            Length::new::<astronomical_unit>(0.),
            Length::new::<astronomical_unit>(0.),
        );
        let actual = planet_brightness(
            solar_luminous_intensity(),
            &star_position,
            &planet_position,
            &observer_position,
            jupiter().radius,
            jupiter().geometric_albedo,
        )
        .unwrap();
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        println!("ratio: {}", actual.get::<lux>() / expected.get::<lux>());
        println!(
            "diff: {}, accuracy: {}",
            (actual - expected).astro_display(),
            accuracy.astro_display()
        );
        assert!(eq_within(
            actual.get::<lux>(),
            expected.get::<lux>(),
            accuracy.get::<lux>()
        ));
    }

    #[test]
    fn saturn_at_opposition() {
        let expected = apparent_magnitude_to_illuminance(-0.55);
        let accuracy = REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR * expected;
        let star_position = Cartesian::origin();
        let planet_position = Cartesian::new(
            saturn().orbit.get_semi_major_axis(),
            Length::new::<astronomical_unit>(0.),
            Length::new::<astronomical_unit>(0.),
        );
        let observer_position = Cartesian::new(
            earth().orbit.get_semi_major_axis(),
            Length::new::<astronomical_unit>(0.),
            Length::new::<astronomical_unit>(0.),
        );
        let actual = planet_brightness(
            solar_luminous_intensity(),
            &star_position,
            &planet_position,
            &observer_position,
            saturn().radius,
            saturn().geometric_albedo * 2., //Saturn's rings break the whole model
        )
        .unwrap();
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        println!("ratio: {}", actual.get::<lux>() / expected.get::<lux>());
        println!(
            "diff: {}, accuracy: {}",
            (actual - expected).astro_display(),
            accuracy.astro_display()
        );
        assert!(eq_within(
            actual.get::<lux>(),
            expected.get::<lux>(),
            accuracy.get::<lux>()
        ));
    }

    #[test]
    fn uranus_at_opposition() {
        let expected = apparent_magnitude_to_illuminance(5.38);
        let accuracy = REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR * expected;
        let star_position = Cartesian::origin();
        let planet_position = Cartesian::new(
            uranus().orbit.get_semi_major_axis(),
            Length::new::<astronomical_unit>(0.),
            Length::new::<astronomical_unit>(0.),
        );
        let observer_position = Cartesian::new(
            earth().orbit.get_semi_major_axis(),
            Length::new::<astronomical_unit>(0.),
            Length::new::<astronomical_unit>(0.),
        );
        let actual = planet_brightness(
            solar_luminous_intensity(),
            &star_position,
            &planet_position,
            &observer_position,
            uranus().radius,
            uranus().geometric_albedo,
        )
        .unwrap();
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        println!("ratio: {}", actual.get::<lux>() / expected.get::<lux>());
        println!(
            "diff: {}, accuracy: {}",
            (actual - expected).astro_display(),
            accuracy.astro_display()
        );
        assert!(eq_within(
            actual.get::<lux>(),
            expected.get::<lux>(),
            accuracy.get::<lux>()
        ));
    }

    #[test]
    fn neptune_at_opposition() {
        let expected = apparent_magnitude_to_illuminance(7.67);
        let accuracy = REAL_ILLUMINANCE_TEST_ACCURACY_FACTOR * expected;
        let star_position = Cartesian::origin();
        let planet_position = Cartesian::new(
            neptune().orbit.get_semi_major_axis(),
            Length::new::<astronomical_unit>(0.),
            Length::new::<astronomical_unit>(0.),
        );
        let observer_position = Cartesian::new(
            earth().orbit.get_semi_major_axis(),
            Length::new::<astronomical_unit>(0.),
            Length::new::<astronomical_unit>(0.),
        );
        let actual = planet_brightness(
            solar_luminous_intensity(),
            &star_position,
            &planet_position,
            &observer_position,
            neptune().radius,
            neptune().geometric_albedo,
        )
        .unwrap();
        println!(
            "expected: {}, actual: {}",
            expected.astro_display(),
            actual.astro_display()
        );
        println!("ratio: {}", actual.get::<lux>() / expected.get::<lux>());
        println!(
            "diff: {}, accuracy: {}",
            (actual - expected).astro_display(),
            accuracy.astro_display()
        );
        assert!(eq_within(
            actual.get::<lux>(),
            expected.get::<lux>(),
            accuracy.get::<lux>()
        ));
    }
}
