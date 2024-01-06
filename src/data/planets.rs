use crate::color::sRGBColor;
use crate::coordinates::earth_equatorial::EarthEquatorialCoordinates;
use crate::planets::orbit_parameters::OrbitParameters;
use crate::planets::real_data::RealData;
use crate::units::angle::{Angle, RADIANS_PER_DEGREE};
use crate::units::length::{Length, AU_PER_EARTH_RADII, AU_PER_JUPITER_RADII, AU_PER_KILOMETERS};
use crate::units::mass::{
    Mass, KILOGRAMS_PER_EARTH_MASS, KILOGRAMS_PER_JUPITER_MASS, KILOGRAMS_PER_MOON_MASS,
};
use crate::units::time::{Time, SECONDS_PER_DAY, SECONDS_PER_HOUR};

pub const MERCURY: RealData = RealData {
    name: "Mercury",
    // orbit: OrbitParameters,
    // bond_albedo: Float,
    // geometric_albedo: Float,
    // color: sRGBColor,
    // radius: Length,
    // mass: Mass,
    // siderial_rotation_period: Time,
    // axis_tilt: Angle,
    // rotation_axis: EarthEquatorialCoordinates,
    orbit: OrbitParameters {
        semi_major_axis: Length::from_astronomical_units(57_909_050. * AU_PER_KILOMETERS),
        eccentricity: 0.205_630_69,
        inclination: Angle::from_radians(7.00487 * RADIANS_PER_DEGREE),
        longitude_of_ascending_node: Angle::from_radians(48.33167 * RADIANS_PER_DEGREE),
        argument_of_periapsis: Angle::from_radians(29.124279 * RADIANS_PER_DEGREE),
    },
    geometric_albedo: 0.142,
    bond_albedo: Some(0.088),
    color: sRGBColor::from_sRGB(0.6, 0.58, 0.58),
    radius: Length::from_astronomical_units(2_439.700 * AU_PER_KILOMETERS),
    mass: Mass::from_kilograms(3.3011e23),
    siderial_rotation_period: Time::from_seconds(58.646_2 * SECONDS_PER_DAY),
    axis_tilt: Angle::from_radians(0.034 * RADIANS_PER_DEGREE),
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle::from_radians(281.01 * RADIANS_PER_DEGREE),
        Angle::from_radians(61.45 * RADIANS_PER_DEGREE),
    ),
};

pub const VENUS: RealData = RealData {
    name: "Venus",
    orbit: OrbitParameters {
        semi_major_axis: Length::from_astronomical_units(108_208_000. * AU_PER_KILOMETERS),
        eccentricity: 0.006_773_23,
        inclination: Angle::from_radians(3.39471 * RADIANS_PER_DEGREE),
        longitude_of_ascending_node: Angle::from_radians(76.68069 * RADIANS_PER_DEGREE),
        argument_of_periapsis: Angle::from_radians(54.85229 * RADIANS_PER_DEGREE),
    },
    geometric_albedo: 0.689,
    bond_albedo: Some(0.76),
    color: sRGBColor::from_sRGB(0.75, 0.74, 0.71),
    radius: Length::from_astronomical_units(6_051.8 * AU_PER_KILOMETERS),
    mass: Mass::from_kilograms(4.8675e24),
    siderial_rotation_period: Time::from_seconds(-243.022_6 * SECONDS_PER_DAY),
    axis_tilt: Angle::from_radians(2.64 * RADIANS_PER_DEGREE),
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle::from_radians(272.76 * RADIANS_PER_DEGREE),
        Angle::from_radians(67.16 * RADIANS_PER_DEGREE),
    ),
};

pub const EARTH: RealData = RealData {
    name: "Earth",
    orbit: OrbitParameters {
        semi_major_axis: Length::from_astronomical_units(149_598_023. * AU_PER_KILOMETERS),
        eccentricity: 0.016_708_6,
        inclination: Angle::from_radians(0.00005 * RADIANS_PER_DEGREE),
        longitude_of_ascending_node: Angle::from_radians(-11.26064 * RADIANS_PER_DEGREE),
        argument_of_periapsis: Angle::from_radians(114.20783 * RADIANS_PER_DEGREE),
    },
    geometric_albedo: 0.367,
    bond_albedo: Some(0.306),
    color: sRGBColor::from_sRGB(0.38, 0.39, 0.48),
    radius: Length::from_astronomical_units(AU_PER_EARTH_RADII),
    mass: Mass::from_kilograms(KILOGRAMS_PER_EARTH_MASS),
    siderial_rotation_period: Time::from_seconds(0.997_269_68 * SECONDS_PER_DAY),
    axis_tilt: Angle::from_radians(23.439_2811 * RADIANS_PER_DEGREE),
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle::from_radians(0. * RADIANS_PER_DEGREE),
        Angle::from_radians(90. * RADIANS_PER_DEGREE),
    ),
};

pub const MARS: RealData = RealData {
    name: "Mars",
    orbit: OrbitParameters {
        semi_major_axis: Length::from_astronomical_units(227_939_200. * AU_PER_KILOMETERS),
        eccentricity: 0.093_394_1,
        inclination: Angle::from_radians(1.85061 * RADIANS_PER_DEGREE),
        longitude_of_ascending_node: Angle::from_radians(49.57854 * RADIANS_PER_DEGREE),
        argument_of_periapsis: Angle::from_radians(286.4623 * RADIANS_PER_DEGREE),
    },
    geometric_albedo: 0.17,
    bond_albedo: Some(0.25),
    color: sRGBColor::from_sRGB(0.59, 0.38, 0.19),
    radius: Length::from_astronomical_units(3_389.5 * AU_PER_KILOMETERS),
    mass: Mass::from_kilograms(6.4171e23),
    siderial_rotation_period: Time::from_seconds(1.025_956_75 * SECONDS_PER_DAY),
    axis_tilt: Angle::from_radians(25.19 * RADIANS_PER_DEGREE),
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle::from_radians(317.681 * RADIANS_PER_DEGREE),
        Angle::from_radians(52.88650 * RADIANS_PER_DEGREE),
    ),
};

pub const CERES: RealData = RealData {
    name: "Ceres",
    orbit: OrbitParameters {
        semi_major_axis: Length::from_astronomical_units(413_690_250. * AU_PER_KILOMETERS),
        eccentricity: 0.075_823_9,
        inclination: Angle::from_radians(10.593 * RADIANS_PER_DEGREE),
        longitude_of_ascending_node: Angle::from_radians(80.393 * RADIANS_PER_DEGREE),
        argument_of_periapsis: Angle::from_radians(73.597 * RADIANS_PER_DEGREE),
    },
    geometric_albedo: 0.09,
    bond_albedo: None,
    color: sRGBColor::from_sRGB(1., 1., 1.), //Color unknown, taking grey
    radius: Length::from_astronomical_units(476.2 * AU_PER_KILOMETERS),
    mass: Mass::from_kilograms(9.393e20),
    siderial_rotation_period: Time::from_seconds(9.074_170 * SECONDS_PER_HOUR),
    axis_tilt: Angle::from_radians(4. * RADIANS_PER_DEGREE),
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle::from_radians(291.42744 * RADIANS_PER_DEGREE),
        Angle::from_radians(66.76033 * RADIANS_PER_DEGREE),
    ),
};

pub const JUPITER: RealData = RealData {
    name: "Jupiter",
    orbit: OrbitParameters {
        semi_major_axis: Length::from_astronomical_units(778_547_200. * AU_PER_KILOMETERS),
        eccentricity: 0.048_386_24,
        inclination: Angle::from_radians(1.30530 * RADIANS_PER_DEGREE),
        longitude_of_ascending_node: Angle::from_radians(100.55615 * RADIANS_PER_DEGREE),
        argument_of_periapsis: Angle::from_radians(273.865 * RADIANS_PER_DEGREE),
    },
    geometric_albedo: 0.538,
    bond_albedo: Some(0.503),
    color: sRGBColor::from_sRGB(0.76, 0.7, 0.67),
    radius: Length::from_astronomical_units(AU_PER_JUPITER_RADII),
    mass: Mass::from_kilograms(KILOGRAMS_PER_JUPITER_MASS),
    siderial_rotation_period: Time::from_seconds(9.925_8 * SECONDS_PER_HOUR),
    axis_tilt: Angle::from_radians(3.13 * RADIANS_PER_DEGREE),
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle::from_radians(268.057 * RADIANS_PER_DEGREE),
        Angle::from_radians(64.495 * RADIANS_PER_DEGREE),
    ),
};

pub const SATURN: RealData = RealData {
    name: "Saturn",
    orbit: OrbitParameters {
        semi_major_axis: Length::from_astronomical_units(1_433_449_370. * AU_PER_KILOMETERS),
        eccentricity: 0.054_150_60,
        inclination: Angle::from_radians(2.48446 * RADIANS_PER_DEGREE),
        longitude_of_ascending_node: Angle::from_radians(113.71504 * RADIANS_PER_DEGREE),
        argument_of_periapsis: Angle::from_radians(339.39153 * RADIANS_PER_DEGREE),
    },
    geometric_albedo: 0.499,
    bond_albedo: Some(0.342),
    color: sRGBColor::from_sRGB(0.77, 0.7, 0.56),
    radius: Length::from_astronomical_units(58_232. * AU_PER_KILOMETERS),
    mass: Mass::from_kilograms(5.6834e26),
    siderial_rotation_period: Time::from_seconds(10.656 * SECONDS_PER_HOUR),
    axis_tilt: Angle::from_radians(26.73 * RADIANS_PER_DEGREE),
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle::from_radians(40.589 * RADIANS_PER_DEGREE),
        Angle::from_radians(83.537 * RADIANS_PER_DEGREE),
    ),
};

pub const URANUS: RealData = RealData {
    name: "Uranus",
    orbit: OrbitParameters {
        semi_major_axis: Length::from_astronomical_units(2_872_463_710. * AU_PER_KILOMETERS),
        eccentricity: 0.047_167_71,
        inclination: Angle::from_radians(0.76986 * RADIANS_PER_DEGREE),
        longitude_of_ascending_node: Angle::from_radians(74.22988 * RADIANS_PER_DEGREE),
        argument_of_periapsis: Angle::from_radians(96.734 * RADIANS_PER_DEGREE),
    },
    geometric_albedo: 0.488,
    bond_albedo: Some(0.300),
    color: sRGBColor::from_sRGB(0.57, 0.75, 0.83),
    radius: Length::from_astronomical_units(25_362. * AU_PER_KILOMETERS),
    mass: Mass::from_kilograms(8.6810e25),
    siderial_rotation_period: Time::from_seconds(-17.24 * SECONDS_PER_HOUR),
    axis_tilt: Angle::from_radians(82.23 * RADIANS_PER_DEGREE),
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle::from_radians(257.311 * RADIANS_PER_DEGREE),
        Angle::from_radians(-15.175 * RADIANS_PER_DEGREE),
    ),
};

pub const NEPTUNE: RealData = RealData {
    name: "Neptune",
    orbit: OrbitParameters {
        semi_major_axis: Length::from_astronomical_units(4_495_060_000. * AU_PER_KILOMETERS),
        eccentricity: 0.008_585_87,
        inclination: Angle::from_radians(1.76917 * RADIANS_PER_DEGREE),
        longitude_of_ascending_node: Angle::from_radians(131.72169 * RADIANS_PER_DEGREE),
        argument_of_periapsis: Angle::from_radians(265.64685 * RADIANS_PER_DEGREE),
    },
    geometric_albedo: 0.442,
    bond_albedo: Some(0.290),
    color: sRGBColor::from_sRGB(0.56, 0.75, 0.88),
    radius: Length::from_astronomical_units(24_622. * AU_PER_KILOMETERS),
    mass: Mass::from_kilograms(1.0243e26),
    siderial_rotation_period: Time::from_seconds(16.11 * SECONDS_PER_HOUR),
    axis_tilt: Angle::from_radians(28.32 * RADIANS_PER_DEGREE),
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle::from_radians(299.3 * RADIANS_PER_DEGREE),
        Angle::from_radians(42.950 * RADIANS_PER_DEGREE),
    ),
};

pub const PLUTO: RealData = RealData {
    name: "Pluto",
    orbit: OrbitParameters {
        semi_major_axis: Length::from_astronomical_units(5_906_380_000. * AU_PER_KILOMETERS),
        eccentricity: 0.248_807_66,
        inclination: Angle::from_radians(17.14175 * RADIANS_PER_DEGREE),
        longitude_of_ascending_node: Angle::from_radians(110.30347 * RADIANS_PER_DEGREE),
        argument_of_periapsis: Angle::from_radians(113.76329 * RADIANS_PER_DEGREE),
    },
    geometric_albedo: 0.52,
    bond_albedo: Some(0.72),
    color: sRGBColor::from_sRGB(0.63, 0.48, 0.37),
    radius: Length::from_astronomical_units(1_188.3 * AU_PER_KILOMETERS),
    mass: Mass::from_kilograms(1.303e22),
    siderial_rotation_period: Time::from_seconds(-6.387_230 * SECONDS_PER_DAY),
    axis_tilt: Angle::from_radians(119.51 * RADIANS_PER_DEGREE),
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle::from_radians(132.99 * RADIANS_PER_DEGREE),
        Angle::from_radians(-6.16 * RADIANS_PER_DEGREE),
    ),
};

pub const MOON: RealData = RealData {
    name: "Moon",
    orbit: OrbitParameters {
        semi_major_axis: Length::from_astronomical_units(384_399. * AU_PER_KILOMETERS),
        eccentricity: 0.054_9,
        inclination: Angle::from_radians(5.145 * RADIANS_PER_DEGREE),
        longitude_of_ascending_node: Angle::from_radians(125.08 * RADIANS_PER_DEGREE),
        argument_of_periapsis: Angle::from_radians(318.15 * RADIANS_PER_DEGREE),
    },
    geometric_albedo: 0.120,
    bond_albedo: Some(0.110),
    color: sRGBColor::from_sRGB(0.59, 0.53, 0.52),
    radius: Length::from_astronomical_units(1_737.4 * AU_PER_KILOMETERS),
    mass: Mass::from_kilograms(KILOGRAMS_PER_MOON_MASS),
    siderial_rotation_period: Time::from_seconds(27.321_661 * SECONDS_PER_DAY),
    axis_tilt: Angle::from_radians(6.68 * RADIANS_PER_DEGREE),
    rotation_axis: EarthEquatorialCoordinates::new(
        Angle::from_radians(266.86 * RADIANS_PER_DEGREE),
        Angle::from_radians(65.64 * RADIANS_PER_DEGREE),
    ),
};
