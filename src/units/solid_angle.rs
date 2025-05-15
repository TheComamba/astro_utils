use std::f64::consts::PI;

use uom::si::{
    f64::{Area, Length, SolidAngle},
    solid_angle::steradian,
};

use crate::astro_display::AstroDisplay;

impl AstroDisplay for SolidAngle {
    fn astro_display(&self) -> String {
        format!("{:.2} sr", self.get::<steradian>())
    }
}

pub fn area_and_distance_to_solid_angle(surface_area: Area, distance: Length) -> SolidAngle {
    (surface_area / (distance * distance)).into()
}

pub fn radius_and_distance_to_solid_angle(radius: Length, distance: Length) -> SolidAngle {
    area_and_distance_to_solid_angle(PI * radius * radius, distance)
}

pub fn solid_angle_to_area_at_distance(solid_angle: SolidAngle, distance: Length) -> Area {
    (solid_angle * distance * distance).into()
}

#[cfg(test)]
mod tests {
    use uom::si::{area::square_meter, length::meter};

    use super::*;
    use crate::{
        real_data::planets::{earth, luna},
        tests::eq,
        units::length::solar_radii,
    };

    #[test]
    fn test_area_and_distance_to_solid_angle() {
        let surface_area = Area::new::<square_meter>(1.0);
        let distance = Length::new::<meter>(1.0);
        let solid_angle = area_and_distance_to_solid_angle(surface_area, distance);
        assert!(eq(solid_angle.get::<steradian>(), 1.0));
    }

    #[test]
    fn test_solid_angle_to_area_at_distance() {
        let solid_angle = SolidAngle::new::<steradian>(1.);
        let distance = Length::new::<meter>(1.0);
        let surface_area = solid_angle_to_area_at_distance(solid_angle, distance);
        assert!(eq(surface_area.get::<square_meter>(), 1.0));
    }

    #[test]
    fn solid_angle_of_sun() {
        let expected = SolidAngle::new::<steradian>(7e-5);
        let actual = radius_and_distance_to_solid_angle(
            Length::new::<solar_radii>(1.),
            earth().orbit.get_semi_major_axis(),
        );
        assert!(eq(actual.value, expected.value));
    }

    #[test]
    fn solid_angle_of_full_moon() {
        let expected = SolidAngle::new::<steradian>(6.4e-5);
        let actual =
            radius_and_distance_to_solid_angle(luna().radius, luna().orbit.get_semi_major_axis());
        assert!(eq(actual.value, expected.value));
    }
}
