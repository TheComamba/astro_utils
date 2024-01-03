use super::{direction::Direction, ecliptic::EclipticCoordinates, spherical::SphericalCoordinates};
use crate::{
    data::planets::EARTH_AXIS_TILT,
    units::{angle::Angle, length::Length},
};

pub struct EarthEquatorialCoordinates {
    right_ascension: Angle,
    declination: Angle,
}

impl EarthEquatorialCoordinates {
    pub const fn new(right_ascension: Angle, declination: Angle) -> EarthEquatorialCoordinates {
        EarthEquatorialCoordinates {
            right_ascension,
            declination,
        }
    }

    pub fn to_direction(&self) -> Direction {
        let direction_in_equatorial =
            SphericalCoordinates::new(self.right_ascension, self.declination).to_direction();
        let direction_in_ecliptic =
            direction_in_equatorial.rotated(-EARTH_AXIS_TILT, &Direction::X);
        direction_in_ecliptic
    }

    pub fn to_ecliptic(&self) -> EclipticCoordinates {
        let dir = self.to_direction();
        let vec = dir.to_cartesian(Length::from_meters(1.));
        vec.to_ecliptic()
    }
}

#[cfg(test)]
pub(super) const EARTH_NORTH_POLE_IN_ECLIPTIC_COORDINATES: EclipticCoordinates =
    EclipticCoordinates::new(SphericalCoordinates::new(
        Angle::from_radians(crate::PI / 2.),
        Angle::from_radians(crate::PI / 2. - EARTH_AXIS_TILT.as_radians()),
    ));

#[cfg(test)]
mod tests {
    use super::EarthEquatorialCoordinates;
    use crate::{
        coordinates::{
            earth_equatorial::EARTH_NORTH_POLE_IN_ECLIPTIC_COORDINATES,
            ecliptic::EclipticCoordinates, spherical::SphericalCoordinates,
        },
        data::planets::*,
        tests::TEST_ANGLE_ACCURACY,
        units::angle::Angle,
    };

    const TILT_TEST_ACCURACY: Angle = Angle::from_radians(2e-3);

    /*
     * https://ned.ipac.caltech.edu/coordinate_calculator?in_csys=Equatorial&in_equinox=J2000.0&obs_epoch=2000.0&ra=0&dec=90&pa=0.0&out_csys=Ecliptic&out_equinox=J2000.0
     */
    #[test]
    fn ra_zero_dec_ninty_is_north_pole() {
        let equatorial =
            EarthEquatorialCoordinates::new(Angle::from_degrees(0.), Angle::from_degrees(90.));
        let expected = EARTH_NORTH_POLE_IN_ECLIPTIC_COORDINATES;
        let actual = equatorial.to_ecliptic();
        println!("expected: {},\n  actual: {}", expected, actual);
        assert!(actual.eq_within(
            &EARTH_NORTH_POLE_IN_ECLIPTIC_COORDINATES,
            TEST_ANGLE_ACCURACY
        ));
    }

    /*
     * https://ned.ipac.caltech.edu/coordinate_calculator?in_csys=Equatorial&in_equinox=J2000.0&obs_epoch=2000.0&ra=0&dec=0&pa=0.0&out_csys=Ecliptic&out_equinox=J2000.0
     */
    #[test]
    fn ra_zero_dec_zer_is_x_axis() {
        let equatorial =
            EarthEquatorialCoordinates::new(Angle::from_degrees(0.), Angle::from_degrees(0.));
        let expected = EclipticCoordinates::X_DIRECTION;
        let actual = equatorial.to_ecliptic();
        println!("expected: {},\n  actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));
    }

    /*
     * https://ned.ipac.caltech.edu/coordinate_calculator?in_csys=Equatorial&in_equinox=J2000.0&obs_epoch=2000.0&ra=6&dec=0&pa=0.0&out_csys=Ecliptic&out_equinox=J2000.0
     */
    #[test]
    fn ra_ninty_dec_zero_is_equator_zenith() {
        let equatorial =
            EarthEquatorialCoordinates::new(Angle::from_degrees(90.), Angle::from_degrees(0.));
        let expected = EclipticCoordinates::new(SphericalCoordinates::new(
            Angle::from_degrees(90.),
            -EARTH_AXIS_TILT,
        ));
        let actual = equatorial.to_ecliptic();
        println!("expected: {},\n  actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));
    }

    /*
     * https://ned.ipac.caltech.edu/coordinate_calculator?in_csys=Equatorial&in_equinox=J2000.0&obs_epoch=2000.0&ra=12&dec=0&pa=0.0&out_csys=Ecliptic&out_equinox=J2000.0
     */
    #[test]
    fn ra_oneeighty_dec_zero_is_minus_x_axis() {
        let equatorial =
            EarthEquatorialCoordinates::new(Angle::from_degrees(180.), Angle::from_degrees(0.));
        let expected = -EclipticCoordinates::X_DIRECTION;
        let actual = equatorial.to_ecliptic();
        println!("expected: {},\n  actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));
    }

    /*
     * https://ned.ipac.caltech.edu/coordinate_calculator?in_csys=Equatorial&in_equinox=J2000.0&obs_epoch=2000.0&ra=18&dec=0&pa=0.0&out_csys=Ecliptic&out_equinox=J2000.0
     */
    #[test]
    fn ra_twoseventy_dec_zero_is_equator_midnight() {
        let equatorial =
            EarthEquatorialCoordinates::new(Angle::from_degrees(270.), Angle::from_degrees(0.));
        let expected = EclipticCoordinates::new(SphericalCoordinates::new(
            Angle::from_degrees(270.),
            EARTH_AXIS_TILT,
        ));
        let actual = equatorial.to_ecliptic();
        println!("expected: {},\n  actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));
    }

    /*
     * https://ned.ipac.caltech.edu/coordinate_calculator?in_csys=Equatorial&in_equinox=J2000.0&obs_epoch=2000.0&ra=6&dec=23%2027&pa=0.0&out_csys=Ecliptic&out_equinox=J2000.0
     */
    #[test]
    fn ra_ninty_dec_tilt_is_y_axis() {
        let equatorial = EarthEquatorialCoordinates::new(Angle::from_degrees(90.), EARTH_AXIS_TILT);
        let expected = EclipticCoordinates::Y_DIRECTION;
        let actual = equatorial.to_ecliptic();
        println!("expected: {},\n  actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));
    }

    /*
     * https://ned.ipac.caltech.edu/coordinate_calculator?in_csys=Equatorial&in_equinox=J2000.0&obs_epoch=2000.0&ra=18&dec=66%2033&pa=0.0&out_csys=Ecliptic&out_equinox=J2000.0
     */
    #[test]
    fn ra_twoseventy_dec_ninty_minus_tilt_is_z_axis() {
        let equatorial = EarthEquatorialCoordinates::new(
            Angle::from_degrees(270.),
            Angle::from_degrees(90.) - EARTH_AXIS_TILT,
        );
        let expected = EclipticCoordinates::Z_DIRECTION;
        let actual = equatorial.to_ecliptic();
        println!("expected: {},\n  actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));
    }

    /*
     * Calculated using https://frostydrew.org/utilities.dc/convert/tool-eq_coordinates/
     */
    #[test]
    fn specific_testcase() {
        let equatorial =
            EarthEquatorialCoordinates::new(Angle::from_degrees(234.), Angle::from_degrees(56.));
        let expected = EclipticCoordinates::new(SphericalCoordinates::new(
            Angle::from_degrees(194.547656),
            Angle::from_degrees(70.149178),
        ));
        let actual = equatorial.to_ecliptic();
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ANGLE_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_mercury() {
        let orbit_normal = MERCURY_ORBIT_ORIENTATION.normal();
        let north = MERCURY_NORTH.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = MERCURY_AXIS_TILT;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(expected, TILT_TEST_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_venus() {
        let orbit_normal = VENUS_ORBIT_ORIENTATION.normal();
        let north = VENUS_NORTH.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = VENUS_AXIS_TILT;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(expected, TILT_TEST_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_earth() {
        let orbit_normal = EARTH_ORBIT_ORIENTATION.normal();
        let north = EARTH_NORTH.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = EARTH_AXIS_TILT;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(expected, TILT_TEST_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_mars() {
        let orbit_normal = MARS_ORBIT_ORIENTATION.normal();
        let north = MARS_NORTH.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = MARS_AXIS_TILT;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(expected, TILT_TEST_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_ceres() {
        let orbit_normal = CERES_ORBIT_ORIENTATION.normal();
        let north = CERES_NORTH.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = CERES_AXIS_TILT;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(expected, TILT_TEST_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_jupiter() {
        let orbit_normal = JUPITER_ORBIT_ORIENTATION.normal();
        let north = JUPITER_NORTH.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = JUPITER_AXIS_TILT;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(expected, TILT_TEST_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_saturn() {
        let orbit_normal = SATURN_ORBIT_ORIENTATION.normal();
        let north = SATURN_NORTH.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = SATURN_AXIS_TILT;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(expected, TILT_TEST_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_uranus() {
        let orbit_normal = URANUS_ORBIT_ORIENTATION.normal();
        let north = URANUS_NORTH.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = URANUS_AXIS_TILT;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(expected, TILT_TEST_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_neptune() {
        let orbit_normal = NEPTUNE_ORBIT_ORIENTATION.normal();
        let north = NEPTUNE_NORTH.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = NEPTUNE_AXIS_TILT;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(expected, TILT_TEST_ACCURACY));
    }

    #[test]
    fn axis_tilt_of_pluto() {
        let orbit_normal = PLUTO_ORBIT_ORIENTATION.normal();
        let north = PLUTO_NORTH.to_direction();
        println!("orbit_normal: {}", orbit_normal);
        println!("north: {}", north);
        let expected = PLUTO_AXIS_TILT;
        let actual = orbit_normal.angle_to(&north);
        println!("expected: {}, actual: {}", expected, actual);
        assert!(actual.eq_within(expected, TILT_TEST_ACCURACY));
    }
}
