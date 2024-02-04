use crate::color::sRGBColor;
use crate::coordinates::earth_equatorial::EarthEquatorialCoordinates;
use crate::planets::orbit_parameters::OrbitParameters;
use crate::planets::real_data::RealData;
use crate::units::time::{SECONDS_PER_DAY, SECONDS_PER_HOUR};
use simple_si_units::base::{Distance, Mass, Time};
use simple_si_units::geometry::Angle;

pub const MERCURY: RealData = RealData {
    name: "Mercury",
    orbit: OrbitParameters {
        semi_major_axis: Distance { m: 57_909_050_000. },
        eccentricity: 0.205_630_69,
        inclination: Angle {
            rad: 7.00487 * RADIANS_PER_DEGREE,
        },
        longitude_of_ascending_node: Angle {
            rad: 48.33167 * RADIANS_PER_DEGREE,
        },
        argument_of_periapsis: Angle {
            rad: 29.124279 * RADIANS_PER_DEGREE,
        },
    },
    geometric_albedo: 0.142,
    bond_albedo: Some(0.088),
    color: sRGBColor::from_sRGB(0.6, 0.58, 0.58),
    radius: Distance { m: 2_439_700. },
    mass: Mass { kg: 3.3011e23 },
    siderial_rotation_period: Time {
        s: 58.646_2 * SECONDS_PER_DAY,
    },
    axis_tilt: Angle {
        rad: 0.034 * RADIANS_PER_DEGREE,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 281.01 * RADIANS_PER_DEGREE,
        },
        Angle {
            rad: 61.45 * RADIANS_PER_DEGREE,
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
            rad: 3.39471 * RADIANS_PER_DEGREE,
        },
        longitude_of_ascending_node: Angle {
            rad: 76.68069 * RADIANS_PER_DEGREE,
        },
        argument_of_periapsis: Angle {
            rad: 54.85229 * RADIANS_PER_DEGREE,
        },
    },
    geometric_albedo: 0.689,
    bond_albedo: Some(0.76),
    color: sRGBColor::from_sRGB(0.75, 0.74, 0.71),
    radius: Distance { m: 6_051_800. },
    mass: Mass { kg: 4.8675e24 },
    siderial_rotation_period: Time {
        s: -243.022_6 * SECONDS_PER_DAY,
    },
    axis_tilt: Angle {
        rad: 2.64 * RADIANS_PER_DEGREE,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 272.76 * RADIANS_PER_DEGREE,
        },
        Angle {
            rad: 67.16 * RADIANS_PER_DEGREE,
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
            rad: 0.00005 * RADIANS_PER_DEGREE,
        },
        longitude_of_ascending_node: Angle {
            rad: -11.26064 * RADIANS_PER_DEGREE,
        },
        argument_of_periapsis: Angle {
            rad: 114.20783 * RADIANS_PER_DEGREE,
        },
    },
    geometric_albedo: 0.367,
    bond_albedo: Some(0.306),
    color: sRGBColor::from_sRGB(0.38, 0.39, 0.48),
    radius: Distance { m: 6_371_009. },
    mass: Mass { kg: 5.97e24 },
    siderial_rotation_period: Time {
        s: 0.997_269_68 * SECONDS_PER_DAY,
    },
    axis_tilt: Angle {
        rad: 23.439_2811 * RADIANS_PER_DEGREE,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 0. * RADIANS_PER_DEGREE,
        },
        Angle {
            rad: 90. * RADIANS_PER_DEGREE,
        },
    ),
};

pub const MARS: RealData = RealData {
    name: "Mars",
    orbit: OrbitParameters {
        semi_major_axis: Distance {
            m: 227_939_200_000.,
        },
        eccentricity: 0.093_394_1,
        inclination: Angle {
            rad: 1.85061 * RADIANS_PER_DEGREE,
        },
        longitude_of_ascending_node: Angle {
            rad: 49.57854 * RADIANS_PER_DEGREE,
        },
        argument_of_periapsis: Angle {
            rad: 286.4623 * RADIANS_PER_DEGREE,
        },
    },
    geometric_albedo: 0.17,
    bond_albedo: Some(0.25),
    color: sRGBColor::from_sRGB(0.59, 0.38, 0.19),
    radius: Distance { m: 3_389_500. },
    mass: Mass { kg: 6.4171e23 },
    siderial_rotation_period: Time {
        s: 1.025_956_75 * SECONDS_PER_DAY,
    },
    axis_tilt: Angle {
        rad: 25.19 * RADIANS_PER_DEGREE,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 317.681 * RADIANS_PER_DEGREE,
        },
        Angle {
            rad: 52.88650 * RADIANS_PER_DEGREE,
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
            rad: 10.593 * RADIANS_PER_DEGREE,
        },
        longitude_of_ascending_node: Angle {
            rad: 80.393 * RADIANS_PER_DEGREE,
        },
        argument_of_periapsis: Angle {
            rad: 73.597 * RADIANS_PER_DEGREE,
        },
    },
    geometric_albedo: 0.09,
    bond_albedo: None,
    color: sRGBColor::from_sRGB(1., 1., 1.), //Color unknown, taking grey
    radius: Distance { m: 476_200. },
    mass: Mass { kg: 9.393e20 },
    siderial_rotation_period: Time {
        s: 9.074_170 * SECONDS_PER_HOUR,
    },
    axis_tilt: Angle {
        rad: 4. * RADIANS_PER_DEGREE,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 291.42744 * RADIANS_PER_DEGREE,
        },
        Angle {
            rad: 66.76033 * RADIANS_PER_DEGREE,
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
            rad: 1.30530 * RADIANS_PER_DEGREE,
        },
        longitude_of_ascending_node: Angle {
            rad: 100.55615 * RADIANS_PER_DEGREE,
        },
        argument_of_periapsis: Angle {
            rad: 273.865 * RADIANS_PER_DEGREE,
        },
    },
    geometric_albedo: 0.538,
    bond_albedo: Some(0.503),
    color: sRGBColor::from_sRGB(0.76, 0.7, 0.67),
    radius: Distance { m: 69_950_000. },
    mass: Mass { kg: 1.898e27 },
    siderial_rotation_period: Time {
        s: 9.925_8 * SECONDS_PER_HOUR,
    },
    axis_tilt: Angle {
        rad: 3.13 * RADIANS_PER_DEGREE,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 268.057 * RADIANS_PER_DEGREE,
        },
        Angle {
            rad: 64.495 * RADIANS_PER_DEGREE,
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
            rad: 2.48446 * RADIANS_PER_DEGREE,
        },
        longitude_of_ascending_node: Angle {
            rad: 113.71504 * RADIANS_PER_DEGREE,
        },
        argument_of_periapsis: Angle {
            rad: 339.39153 * RADIANS_PER_DEGREE,
        },
    },
    geometric_albedo: 0.499,
    bond_albedo: Some(0.342),
    color: sRGBColor::from_sRGB(0.77, 0.7, 0.56),
    radius: Distance { m: 58_232_000. },
    mass: Mass { kg: 5.6834e26 },
    siderial_rotation_period: Time {
        s: 10.656 * SECONDS_PER_HOUR,
    },
    axis_tilt: Angle {
        rad: 26.73 * RADIANS_PER_DEGREE,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 40.589 * RADIANS_PER_DEGREE,
        },
        Angle {
            rad: 83.537 * RADIANS_PER_DEGREE,
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
            rad: 0.76986 * RADIANS_PER_DEGREE,
        },
        longitude_of_ascending_node: Angle {
            rad: 74.22988 * RADIANS_PER_DEGREE,
        },
        argument_of_periapsis: Angle {
            rad: 96.734 * RADIANS_PER_DEGREE,
        },
    },
    geometric_albedo: 0.488,
    bond_albedo: Some(0.300),
    color: sRGBColor::from_sRGB(0.57, 0.75, 0.83),
    radius: Distance { m: 25_362_000. },
    mass: Mass { kg: 8.6810e25 },
    siderial_rotation_period: Time {
        s: -17.24 * SECONDS_PER_HOUR,
    },
    axis_tilt: Angle {
        rad: 82.23 * RADIANS_PER_DEGREE,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 257.311 * RADIANS_PER_DEGREE,
        },
        Angle {
            rad: -15.175 * RADIANS_PER_DEGREE,
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
            rad: 1.76917 * RADIANS_PER_DEGREE,
        },
        longitude_of_ascending_node: Angle {
            rad: 131.72169 * RADIANS_PER_DEGREE,
        },
        argument_of_periapsis: Angle {
            rad: 265.64685 * RADIANS_PER_DEGREE,
        },
    },
    geometric_albedo: 0.442,
    bond_albedo: Some(0.290),
    color: sRGBColor::from_sRGB(0.56, 0.75, 0.88),
    radius: Distance { m: 24_622_000. },
    mass: Mass { kg: 1.0243e26 },
    siderial_rotation_period: Time {
        s: 16.11 * SECONDS_PER_HOUR,
    },
    axis_tilt: Angle {
        rad: 28.32 * RADIANS_PER_DEGREE,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 299.3 * RADIANS_PER_DEGREE,
        },
        Angle {
            rad: 42.950 * RADIANS_PER_DEGREE,
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
            rad: 17.14175 * RADIANS_PER_DEGREE,
        },
        longitude_of_ascending_node: Angle {
            rad: 110.30347 * RADIANS_PER_DEGREE,
        },
        argument_of_periapsis: Angle {
            rad: 113.76329 * RADIANS_PER_DEGREE,
        },
    },
    geometric_albedo: 0.52,
    bond_albedo: Some(0.72),
    color: sRGBColor::from_sRGB(0.63, 0.48, 0.37),
    radius: Distance { m: 1_188_300. },
    mass: Mass { kg: 1.303e22 },
    siderial_rotation_period: Time {
        s: -6.387_230 * SECONDS_PER_DAY,
    },
    axis_tilt: Angle {
        rad: 119.51 * RADIANS_PER_DEGREE,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 132.99 * RADIANS_PER_DEGREE,
        },
        Angle {
            rad: -6.16 * RADIANS_PER_DEGREE,
        },
    ),
};

pub const MOON: RealData = RealData {
    name: "Moon",
    orbit: OrbitParameters {
        semi_major_axis: Distance { m: 384_399_000. },
        eccentricity: 0.054_9,
        inclination: Angle {
            rad: 5.145 * RADIANS_PER_DEGREE,
        },
        longitude_of_ascending_node: Angle {
            rad: 125.08 * RADIANS_PER_DEGREE,
        },
        argument_of_periapsis: Angle {
            rad: 318.15 * RADIANS_PER_DEGREE,
        },
    },
    geometric_albedo: 0.120,
    bond_albedo: Some(0.110),
    color: sRGBColor::from_sRGB(0.59, 0.53, 0.52),
    radius: Distance { m: 1_737_400. },
    mass: Mass { kg: 7.3459e22 },
    siderial_rotation_period: Time {
        s: 27.321_661 * SECONDS_PER_DAY,
    },
    axis_tilt: Angle {
        rad: 6.68 * RADIANS_PER_DEGREE,
    },
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle {
            rad: 266.86 * RADIANS_PER_DEGREE,
        },
        Angle {
            rad: 65.64 * RADIANS_PER_DEGREE,
        },
    ),
};
