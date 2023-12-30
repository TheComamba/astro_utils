use crate::{
    solar_system_data::EARTH_AXIS_TILT,
    units::{angle::Angle, length::Length},
    PI,
};

use super::{
    direction::{Direction, X, Y, Z},
    ecliptic::EclipticCoordinates,
};

const EARTH_NORTH_POLE_IN_ECLIPTIC_COORDINATES: EclipticCoordinates = EclipticCoordinates::new(
    Angle::from_radians(PI / 2.),
    Angle::from_radians(PI / 2. - EARTH_AXIS_TILT.as_radians()),
);

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

    // /*
    //  * https://aas.aanda.org/articles/aas/full/1998/01/ds1449/node3.html
    //  */
    // pub fn to_ecliptic(&self) -> EclipticCoordinates {
    //     let obliquity = EARTH_AXIS_TILT;
    //     let alpha = self.right_ascension;
    //     let delta = self.declination;
    //     let sinbeta = delta.sin() * obliquity.cos() + delta.cos() * obliquity.sin() * alpha.sin();
    //     let beta = sinbeta.asin();
    //     let coslambda = delta.cos() * alpha.cos() / beta.cos();
    //     let lambda = coslambda.acos();

    //     EclipticCoordinates::new(Angle::from_radians(lambda), Angle::from_radians(beta))
    // }

    pub fn to_direction(&self) -> Direction {
        let direction_in_equatorial = Direction::from_ecliptic(&EclipticCoordinates::new(
            self.right_ascension,
            self.declination,
        ));
        let direction_in_ecliptic = direction_in_equatorial.rotated(-EARTH_AXIS_TILT, &X);
        direction_in_ecliptic
    }

    pub fn to_ecliptic(&self) -> EclipticCoordinates {
        let dir = self.to_direction();
        let vec = dir.to_cartesian(Length::from_meters(1.));
        EclipticCoordinates::from_cartesian(&vec)
    }
}

#[cfg(test)]
mod tests {
    use super::EarthEquatorialCoordinates;
    use crate::{
        coordinates::{
            earth_equatorial::EARTH_NORTH_POLE_IN_ECLIPTIC_COORDINATES,
            ecliptic::EclipticCoordinates,
        },
        solar_system_data::*,
        units::angle::Angle,
    };

    const TEST_ACCURACY: Angle = Angle::from_radians(1e-5);

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
        assert!(actual.eq_within(&EARTH_NORTH_POLE_IN_ECLIPTIC_COORDINATES, TEST_ACCURACY));
    }

    /*
     * https://ned.ipac.caltech.edu/coordinate_calculator?in_csys=Equatorial&in_equinox=J2000.0&obs_epoch=2000.0&ra=0&dec=0&pa=0.0&out_csys=Ecliptic&out_equinox=J2000.0
     */
    #[test]
    fn ra_zero_dec_zer_is_x_axis() {
        let equatorial =
            EarthEquatorialCoordinates::new(Angle::from_degrees(0.), Angle::from_degrees(0.));
        let expected = EclipticCoordinates::new(Angle::from_degrees(0.), Angle::from_degrees(0.));
        let actual = equatorial.to_ecliptic();
        println!("expected: {},\n  actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    /*
     * https://ned.ipac.caltech.edu/coordinate_calculator?in_csys=Equatorial&in_equinox=J2000.0&obs_epoch=2000.0&ra=6&dec=0&pa=0.0&out_csys=Ecliptic&out_equinox=J2000.0
     */
    #[test]
    fn ra_ninty_dec_zero_is_equator_zenith() {
        let equatorial =
            EarthEquatorialCoordinates::new(Angle::from_degrees(90.), Angle::from_degrees(0.));
        let expected = EclipticCoordinates::new(Angle::from_degrees(90.), -EARTH_AXIS_TILT);
        let actual = equatorial.to_ecliptic();
        println!("expected: {},\n  actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    /*
     * https://ned.ipac.caltech.edu/coordinate_calculator?in_csys=Equatorial&in_equinox=J2000.0&obs_epoch=2000.0&ra=12&dec=0&pa=0.0&out_csys=Ecliptic&out_equinox=J2000.0
     */
    #[test]
    fn ra_oneeighty_dec_zero_is_minus_x_axis() {
        let equatorial =
            EarthEquatorialCoordinates::new(Angle::from_degrees(180.), Angle::from_degrees(0.));
        let expected = EclipticCoordinates::new(Angle::from_degrees(180.), Angle::from_degrees(0.));
        let actual = equatorial.to_ecliptic();
        println!("expected: {},\n  actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    /*
     * https://ned.ipac.caltech.edu/coordinate_calculator?in_csys=Equatorial&in_equinox=J2000.0&obs_epoch=2000.0&ra=18&dec=0&pa=0.0&out_csys=Ecliptic&out_equinox=J2000.0
     */
    #[test]
    fn ra_twoseventy_dec_zero_is_equator_midnight() {
        let equatorial =
            EarthEquatorialCoordinates::new(Angle::from_degrees(270.), Angle::from_degrees(0.));
        let expected = EclipticCoordinates::new(Angle::from_degrees(270.), EARTH_AXIS_TILT);
        let actual = equatorial.to_ecliptic();
        println!("expected: {},\n  actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    /*
     * https://ned.ipac.caltech.edu/coordinate_calculator?in_csys=Equatorial&in_equinox=J2000.0&obs_epoch=2000.0&ra=6&dec=23%2027&pa=0.0&out_csys=Ecliptic&out_equinox=J2000.0
     */
    #[test]
    fn ra_ninty_dec_tilt_is_y_axis() {
        let equatorial = EarthEquatorialCoordinates::new(Angle::from_degrees(90.), EARTH_AXIS_TILT);
        let expected = EclipticCoordinates::new(Angle::from_degrees(90.), Angle::from_degrees(0.));
        let actual = equatorial.to_ecliptic();
        println!("expected: {},\n  actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
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
        let expected = EclipticCoordinates::new(Angle::from_degrees(0.), Angle::from_degrees(90.));
        let actual = equatorial.to_ecliptic();
        println!("expected: {},\n  actual: {}", expected, actual);
        assert!(actual.eq_within(&expected, TEST_ACCURACY));
    }

    // /*
    //  * Calculated using https://frostydrew.org/utilities.dc/convert/tool-eq_coordinates/
    //  */
    // #[test]
    // fn specific_testcase() {
    //     let equatorial =
    //         EarthEquatorialCoordinates::new(Angle::from_degrees(234.), Angle::from_degrees(56.));
    //     let expected = EclipticCoordinates::new(
    //         Angle::from_degrees(194.547656),
    //         Angle::from_degrees(70.149178),
    //     );
    //     let actual = equatorial.to_ecliptic();
    //     println!("expected: {}, actual: {}", expected, actual);
    //     assert!(actual.eq_within(&expected, TEST_ACCURACY));
    // }

    // #[test]
    // fn axis_tilt_of_mercury() {
    //     let orbit_normal = MERCURY_ORBIT_ORIENTATION.normal();
    //     let north = MERCURY_NORTH.to_direction();
    //     println!("orbit_normal: {}", orbit_normal);
    //     println!("north: {}", north);
    //     let expected = MERCURY_AXIS_TILT;
    //     let actual = orbit_normal.angle_to(&north);
    //     println!("expected: {}, actual: {}", expected, actual);
    //     assert!(actual.eq_within(expected, TEST_ACCURACY));
    // }

    // #[test]
    // fn axis_tilt_of_venus() {
    //     let orbit_normal = VENUS_ORBIT_ORIENTATION.normal();
    //     let north = VENUS_NORTH.to_direction();
    //     println!("orbit_normal: {}", orbit_normal);
    //     println!("north: {}", north);
    //     let expected = VENUS_AXIS_TILT;
    //     let actual = orbit_normal.angle_to(&north);
    //     println!("expected: {}, actual: {}", expected, actual);
    //     assert!(actual.eq_within(expected, TEST_ACCURACY));
    // }

    // #[test]
    // fn axis_tilt_of_earth() {
    //     let orbit_normal = EARTH_ORBIT_ORIENTATION.normal();
    //     let north = EARTH_NORTH.to_direction();
    //     println!("orbit_normal: {}", orbit_normal);
    //     println!("north: {}", north);
    //     let expected = EARTH_AXIS_TILT;
    //     let actual = orbit_normal.angle_to(&north);
    //     println!("expected: {}, actual: {}", expected, actual);
    //     assert!(actual.eq_within(expected, TEST_ACCURACY));
    // }
}
