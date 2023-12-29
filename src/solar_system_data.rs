use crate::units::angle::{Angle, RADIANS_PER_DEGREE};
use crate::units::length::{
    Length, METERS_PER_EARTH_RADIUS, METERS_PER_JUPITER_RADIUS, METERS_PER_SUN_RADIUS,
};
use crate::units::mass::{
    Mass, KILOGRAMS_PER_EARTH_MASS, KILOGRAMS_PER_JUPITER_MASS, KILOGRAMS_PER_MOON_MASS,
    KILOGRAMS_PER_SOLAR_MASS,
};
use crate::Float;

pub const SUN_RADIUS: Length = Length::from_meters(METERS_PER_SUN_RADIUS);
pub const SUN_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_SOLAR_MASS);

pub const MERCURY_SEMI_MAJOR_AXIS: Length = Length::from_meters(57_909_050_000.);
pub const MERCURY_RADIUS: Length = Length::from_meters(2_439_700.);
pub const MERCURY_MASS: Mass = Mass::from_kilograms(3.3011e23);
pub const MERCURY_ECCENTRICITY: Float = 0.205_630_69;
pub const MERCURY_INCLINATION: Angle = Angle::from_radians(7.00487 * RADIANS_PER_DEGREE);
pub const MERCURY_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(48.33167 * RADIANS_PER_DEGREE);
pub const MERCURY_ARGUMENT_OF_PERIAPSIS: Angle =
    Angle::from_radians(29.124279 * RADIANS_PER_DEGREE);
pub const MERCURY_ALBEDO: Float = 0.106;

pub const VENUS_SEMI_MAJOR_AXIS: Length = Length::from_meters(108_208_000_000.);
pub const VENUS_RADIUS: Length = Length::from_meters(6_051_800.);
pub const VENUS_MASS: Mass = Mass::from_kilograms(4.8675e24);
pub const VENUS_ECCENTRICITY: Float = 0.006_773_23;
pub const VENUS_INCLINATION: Angle = Angle::from_radians(3.39471 * RADIANS_PER_DEGREE);
pub const VENUS_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(76.68069 * RADIANS_PER_DEGREE);
pub const VENUS_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(54.85229 * RADIANS_PER_DEGREE);
pub const VENUS_ALBEDO: Float = 0.65;

pub const EARTH_SEMI_MAJOR_AXIS: Length = Length::from_meters(149_598_023_000.);
pub const EARTH_RADIUS: Length = Length::from_meters(METERS_PER_EARTH_RADIUS);
pub const EARTH_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_EARTH_MASS);
pub const EARTH_ECCENTRICITY: Float = 0.016_708_6;
pub const EARTH_INCLINATION: Angle = Angle::from_radians(0.);
pub const EARTH_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(-11.26064 * RADIANS_PER_DEGREE);
pub const EARTH_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(114.20783 * RADIANS_PER_DEGREE);
pub const EARTH_ALBEDO: Float = 0.306;

pub const MARS_SEMI_MAJOR_AXIS: Length = Length::from_meters(227_939_200_000.);
pub const MARS_RADIUS: Length = Length::from_meters(3_389_500.);
pub const MARS_MASS: Mass = Mass::from_kilograms(6.4171e23);
pub const MARS_ECCENTRICITY: Float = 0.093_394_1;
pub const MARS_INCLINATION: Angle = Angle::from_radians(1.85061 * RADIANS_PER_DEGREE);
pub const MARS_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(49.57854 * RADIANS_PER_DEGREE);
pub const MARS_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(286.4623 * RADIANS_PER_DEGREE);
pub const MARS_ALBEDO: Float = 0.25;

pub const JUPITER_SEMI_MAJOR_AXIS: Length = Length::from_meters(778_547_200_000.);
pub const JUPITER_RADIUS: Length = Length::from_meters(METERS_PER_JUPITER_RADIUS);
pub const JUPITER_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_JUPITER_MASS);
pub const JUPITER_ECCENTRICITY: Float = 0.048_386_24;
pub const JUPITER_INCLINATION: Angle = Angle::from_radians(1.30530 * RADIANS_PER_DEGREE);
pub const JUPITER_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(100.55615 * RADIANS_PER_DEGREE);
pub const JUPITER_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(273.865 * RADIANS_PER_DEGREE);
pub const JUPITER_ALBEDO: Float = 0.343;

pub const SATURN_SEMI_MAJOR_AXIS: Length = Length::from_meters(1_433_449_370_000.);
pub const SATURN_RADIUS: Length = Length::from_meters(58_232_000.);
pub const SATURN_MASS: Mass = Mass::from_kilograms(5.6834e26);
pub const SATURN_ECCENTRICITY: Float = 0.054_150_60;
pub const SATURN_INCLINATION: Angle = Angle::from_radians(2.48446 * RADIANS_PER_DEGREE);
pub const SATURN_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(113.71504 * RADIANS_PER_DEGREE);
pub const SATURN_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(339.39153 * RADIANS_PER_DEGREE);
pub const SATURN_ALBEDO: Float = 0.342;

pub const URANUS_SEMI_MAJOR_AXIS: Length = Length::from_meters(2_872_463_710_000.);
pub const URANUS_RADIUS: Length = Length::from_meters(25_362_000.);
pub const URANUS_MASS: Mass = Mass::from_kilograms(8.6810e25);
pub const URANUS_ECCENTRICITY: Float = 0.047_167_71;
pub const URANUS_INCLINATION: Angle = Angle::from_radians(0.76986 * RADIANS_PER_DEGREE);
pub const URANUS_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(74.22988 * RADIANS_PER_DEGREE);
pub const URANUS_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(96.734 * RADIANS_PER_DEGREE);
pub const URANUS_ALBEDO: Float = 0.300;

pub const NEPTUNE_SEMI_MAJOR_AXIS: Length = Length::from_meters(4_495_060_000_000.);
pub const NEPTUNE_RADIUS: Length = Length::from_meters(24_622_000.);
pub const NEPTUNE_MASS: Mass = Mass::from_kilograms(1.0243e26);
pub const NEPTUNE_ECCENTRICITY: Float = 0.008_585_87;
pub const NEPTUNE_INCLINATION: Angle = Angle::from_radians(1.76917 * RADIANS_PER_DEGREE);
pub const NEPTUNE_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(131.72169 * RADIANS_PER_DEGREE);
pub const NEPTUNE_ARGUMENT_OF_PERIAPSIS: Angle =
    Angle::from_radians(265.64685 * RADIANS_PER_DEGREE);
pub const NEPTUNE_ALBEDO: Float = 0.290;

pub const PLUTO_SEMI_MAJOR_AXIS: Length = Length::from_meters(5_906_380_000_000.);
pub const PLUTO_RADIUS: Length = Length::from_meters(1_188_300.);
pub const PLUTO_MASS: Mass = Mass::from_kilograms(1.303e22);
pub const PLUTO_ECCENTRICITY: Float = 0.248_807_66;
pub const PLUTO_INCLINATION: Angle = Angle::from_radians(17.14175 * RADIANS_PER_DEGREE);
pub const PLUTO_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(110.30347 * RADIANS_PER_DEGREE);
pub const PLUTO_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(113.76329 * RADIANS_PER_DEGREE);
pub const PLUTO_ALBEDO: Float = 0.3;

pub const CERES_SEMI_MAJOR_AXIS: Length = Length::from_meters(413_690_250_000.);
pub const CERES_RADIUS: Length = Length::from_meters(476_200.);
pub const CERES_MASS: Mass = Mass::from_kilograms(9.393e20);
pub const CERES_ECCENTRICITY: Float = 0.075_823_9;
pub const CERES_INCLINATION: Angle = Angle::from_radians(10.593 * RADIANS_PER_DEGREE);
pub const CERES_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(80.393 * RADIANS_PER_DEGREE);
pub const CERES_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(73.597 * RADIANS_PER_DEGREE);
pub const CERES_ALBEDO: Float = 0.09;

pub const MOON_SEMI_MAJOR_AXIS: Length = Length::from_meters(384_399_000.);
pub const MOON_RADIUS: Length = Length::from_meters(1_737_400.);
pub const MOON_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_MOON_MASS);
