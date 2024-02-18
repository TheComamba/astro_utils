use crate::color::srgb::sRGBColor;
use crate::coordinates::earth_equatorial::EarthEquatorialCoordinates;
use crate::planets::orbit_parameters::OrbitParameters;
use crate::planets::real_data::RealData;
use crate::units::angle::*;
use crate::units::time::*;
use simple_si_units::base::Time;
use simple_si_units::base::{Distance, Mass};
use simple_si_units::geometry::Angle;

pub const MERCURY: RealData = RealData {
    name: "Mercury",
    orbit: OrbitParameters {
        semi_major_axis: Distance { m: 57_909_050_000. },
        eccentricity: 0.205_630_69,
        inclination: Angle {
            rad: 7.00487 * DEGREE.rad,
        },
        longitude_of_ascending_node: Angle {
            rad: 48.33167 * DEGREE.rad,
        },
        argument_of_periapsis: Angle {
            rad: 29.124279 * DEGREE.rad,
        },
    },
    geometric_albedo: 0.142,
    bond_albedo: Some(0.088),
    color: sRGBColor::from_sRGB(0.6, 0.58, 0.58),
    radius: Distance { m: 2_439_700. },
    mass: Mass { kg: 3.3011e23 },
    siderial_rotation_period: Time {
        s: 58.646_2 * DAY.s,
    },
    axis_tilt: Angle {
        rad: 0.034 * DEGREE.rad,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 281.01 * DEGREE.rad,
        },
        Angle {
            rad: 61.45 * DEGREE.rad,
        },
    ),
};

pub const VENUS: RealData = RealData {
    name: "Venus",
    orbit: OrbitParameters {
        semi_major_axis: Distance {
            m: 108_208_000_000.,
        },
        eccentricity: 0.006_773_23,
        inclination: Angle {
            rad: 3.39471 * DEGREE.rad,
        },
        longitude_of_ascending_node: Angle {
            rad: 76.68069 * DEGREE.rad,
        },
        argument_of_periapsis: Angle {
            rad: 54.85229 * DEGREE.rad,
        },
    },
    geometric_albedo: 0.689,
    bond_albedo: Some(0.76),
    color: sRGBColor::from_sRGB(0.75, 0.74, 0.71),
    radius: Distance { m: 6_051_800. },
    mass: Mass { kg: 4.8675e24 },
    siderial_rotation_period: Time {
        s: -243.022_6 * DAY.s,
    },
    axis_tilt: Angle {
        rad: 2.64 * DEGREE.rad,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 272.76 * DEGREE.rad,
        },
        Angle {
            rad: 67.16 * DEGREE.rad,
        },
    ),
};

pub const EARTH: RealData = RealData {
    name: "Earth",
    orbit: OrbitParameters {
        semi_major_axis: Distance {
            m: 149_598_023_000.,
        },
        eccentricity: 0.016_708_6,
        inclination: Angle {
            rad: 0.00005 * DEGREE.rad,
        },
        longitude_of_ascending_node: Angle {
            rad: -11.26064 * DEGREE.rad,
        },
        argument_of_periapsis: Angle {
            rad: 114.20783 * DEGREE.rad,
        },
    },
    geometric_albedo: 0.367,
    bond_albedo: Some(0.306),
    color: sRGBColor::from_sRGB(0.38, 0.39, 0.48),
    radius: Distance { m: 6_371_009. },
    mass: Mass { kg: 5.97e24 },
    siderial_rotation_period: Time {
        s: 0.997_269_68 * DAY.s,
    },
    axis_tilt: Angle {
        rad: 23.439_281 * DEGREE.rad,
    },
    rotation_axis: EarthEquatorialCoordinates::new(ANGLE_ZERO, QUARTER_CIRC),
};

pub const MARS: RealData = RealData {
    name: "Mars",
    orbit: OrbitParameters {
        semi_major_axis: Distance {
            m: 227_939_200_000.,
        },
        eccentricity: 0.093_394_1,
        inclination: Angle {
            rad: 1.85061 * DEGREE.rad,
        },
        longitude_of_ascending_node: Angle {
            rad: 49.57854 * DEGREE.rad,
        },
        argument_of_periapsis: Angle {
            rad: 286.4623 * DEGREE.rad,
        },
    },
    geometric_albedo: 0.17,
    bond_albedo: Some(0.25),
    color: sRGBColor::from_sRGB(0.59, 0.38, 0.19),
    radius: Distance { m: 3_389_500. },
    mass: Mass { kg: 6.4171e23 },
    siderial_rotation_period: Time {
        s: 1.025_956_75 * DAY.s,
    },
    axis_tilt: Angle {
        rad: 25.19 * DEGREE.rad,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 317.681 * DEGREE.rad,
        },
        Angle {
            rad: 52.88650 * DEGREE.rad,
        },
    ),
};

pub const CERES: RealData = RealData {
    name: "Ceres",
    orbit: OrbitParameters {
        semi_major_axis: Distance {
            m: 413_690_250_000.,
        },
        eccentricity: 0.075_823_9,
        inclination: Angle {
            rad: 10.593 * DEGREE.rad,
        },
        longitude_of_ascending_node: Angle {
            rad: 80.393 * DEGREE.rad,
        },
        argument_of_periapsis: Angle {
            rad: 73.597 * DEGREE.rad,
        },
    },
    geometric_albedo: 0.09,
    bond_albedo: None,
    color: sRGBColor::from_sRGB(1., 1., 1.), //Color unknown, taking grey
    radius: Distance { m: 476_200. },
    mass: Mass { kg: 9.393e20 },
    siderial_rotation_period: Time {
        s: 9.074_170 * HOUR.s,
    },
    axis_tilt: Angle {
        rad: 4. * DEGREE.rad,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 291.42744 * DEGREE.rad,
        },
        Angle {
            rad: 66.76033 * DEGREE.rad,
        },
    ),
};

pub const JUPITER: RealData = RealData {
    name: "Jupiter",
    orbit: OrbitParameters {
        semi_major_axis: Distance {
            m: 778_547_200_000.,
        },
        eccentricity: 0.048_386_24,
        inclination: Angle {
            rad: 1.30530 * DEGREE.rad,
        },
        longitude_of_ascending_node: Angle {
            rad: 100.55615 * DEGREE.rad,
        },
        argument_of_periapsis: Angle {
            rad: 273.865 * DEGREE.rad,
        },
    },
    geometric_albedo: 0.538,
    bond_albedo: Some(0.503),
    color: sRGBColor::from_sRGB(0.76, 0.7, 0.67),
    radius: Distance { m: 69_950_000. },
    mass: Mass { kg: 1.898e27 },
    siderial_rotation_period: Time {
        s: 9.925_8 * HOUR.s,
    },
    axis_tilt: Angle {
        rad: 3.13 * DEGREE.rad,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 268.057 * DEGREE.rad,
        },
        Angle {
            rad: 64.495 * DEGREE.rad,
        },
    ),
};

pub const SATURN: RealData = RealData {
    name: "Saturn",
    orbit: OrbitParameters {
        semi_major_axis: Distance {
            m: 1_433_449_370_000.,
        },
        eccentricity: 0.054_150_60,
        inclination: Angle {
            rad: 2.48446 * DEGREE.rad,
        },
        longitude_of_ascending_node: Angle {
            rad: 113.71504 * DEGREE.rad,
        },
        argument_of_periapsis: Angle {
            rad: 339.39153 * DEGREE.rad,
        },
    },
    geometric_albedo: 0.499,
    bond_albedo: Some(0.342),
    color: sRGBColor::from_sRGB(0.77, 0.7, 0.56),
    radius: Distance { m: 58_232_000. },
    mass: Mass { kg: 5.6834e26 },
    siderial_rotation_period: Time { s: 10.656 * HOUR.s },
    axis_tilt: Angle {
        rad: 26.73 * DEGREE.rad,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 40.589 * DEGREE.rad,
        },
        Angle {
            rad: 83.537 * DEGREE.rad,
        },
    ),
};

pub const URANUS: RealData = RealData {
    name: "Uranus",
    orbit: OrbitParameters {
        semi_major_axis: Distance {
            m: 2_872_463_710_000.,
        },
        eccentricity: 0.047_167_71,
        inclination: Angle {
            rad: 0.76986 * DEGREE.rad,
        },
        longitude_of_ascending_node: Angle {
            rad: 74.22988 * DEGREE.rad,
        },
        argument_of_periapsis: Angle {
            rad: 96.734 * DEGREE.rad,
        },
    },
    geometric_albedo: 0.488,
    bond_albedo: Some(0.300),
    color: sRGBColor::from_sRGB(0.57, 0.75, 0.83),
    radius: Distance { m: 25_362_000. },
    mass: Mass { kg: 8.6810e25 },
    siderial_rotation_period: Time { s: -17.24 * HOUR.s },
    axis_tilt: Angle {
        rad: 82.23 * DEGREE.rad,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 257.311 * DEGREE.rad,
        },
        Angle {
            rad: -15.175 * DEGREE.rad,
        },
    ),
};

pub const NEPTUNE: RealData = RealData {
    name: "Neptune",
    orbit: OrbitParameters {
        semi_major_axis: Distance {
            m: 4_495_060_000_000.,
        },
        eccentricity: 0.008_585_87,
        inclination: Angle {
            rad: 1.76917 * DEGREE.rad,
        },
        longitude_of_ascending_node: Angle {
            rad: 131.72169 * DEGREE.rad,
        },
        argument_of_periapsis: Angle {
            rad: 265.64685 * DEGREE.rad,
        },
    },
    geometric_albedo: 0.442,
    bond_albedo: Some(0.290),
    color: sRGBColor::from_sRGB(0.56, 0.75, 0.88),
    radius: Distance { m: 24_622_000. },
    mass: Mass { kg: 1.0243e26 },
    siderial_rotation_period: Time { s: 16.11 * HOUR.s },
    axis_tilt: Angle {
        rad: 28.32 * DEGREE.rad,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 299.3 * DEGREE.rad,
        },
        Angle {
            rad: 42.950 * DEGREE.rad,
        },
    ),
};

pub const PLUTO: RealData = RealData {
    name: "Pluto",
    orbit: OrbitParameters {
        semi_major_axis: Distance {
            m: 5_906_380_000_000.,
        },
        eccentricity: 0.248_807_66,
        inclination: Angle {
            rad: 17.14175 * DEGREE.rad,
        },
        longitude_of_ascending_node: Angle {
            rad: 110.30347 * DEGREE.rad,
        },
        argument_of_periapsis: Angle {
            rad: 113.76329 * DEGREE.rad,
        },
    },
    geometric_albedo: 0.52,
    bond_albedo: Some(0.72),
    color: sRGBColor::from_sRGB(0.63, 0.48, 0.37),
    radius: Distance { m: 1_188_300. },
    mass: Mass { kg: 1.303e22 },
    siderial_rotation_period: Time {
        s: -6.387_230 * DAY.s,
    },
    axis_tilt: Angle {
        rad: 119.51 * DEGREE.rad,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 132.99 * DEGREE.rad,
        },
        Angle {
            rad: -6.16 * DEGREE.rad,
        },
    ),
};

pub const MOON: RealData = RealData {
    name: "Moon",
    orbit: OrbitParameters {
        semi_major_axis: Distance { m: 384_399_000. },
        eccentricity: 0.054_9,
        inclination: Angle {
            rad: 5.145 * DEGREE.rad,
        },
        longitude_of_ascending_node: Angle {
            rad: 125.08 * DEGREE.rad,
        },
        argument_of_periapsis: Angle {
            rad: 318.15 * DEGREE.rad,
        },
    },
    geometric_albedo: 0.120,
    bond_albedo: Some(0.110),
    color: sRGBColor::from_sRGB(0.59, 0.53, 0.52),
    radius: Distance { m: 1_737_400. },
    mass: Mass { kg: 7.3459e22 },
    siderial_rotation_period: Time {
        s: 27.321_661 * DAY.s,
    },
    axis_tilt: Angle {
        rad: 6.68 * DEGREE.rad,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 266.86 * DEGREE.rad,
        },
        Angle {
            rad: 65.64 * DEGREE.rad,
        },
    ),
};
