use crate::units::angle::{Angle, RADIANS_PER_DEGREE};
use crate::units::length::{
    Length, METERS_PER_EARTH_RADIUS, METERS_PER_JUPITER_RADIUS, METERS_PER_SUN_RADIUS,
};
use crate::units::mass::{
    Mass, KILOGRAMS_PER_EARTH_MASS, KILOGRAMS_PER_JUPITER_MASS, KILOGRAMS_PER_MOON_MASS,
    KILOGRAMS_PER_SOLAR_MASS,
};
use crate::units::time::{Time, SECONDS_PER_DAY, SECONDS_PER_HOUR};
use crate::Float;

pub const SUN_RADIUS: Length = Length::from_meters(METERS_PER_SUN_RADIUS);
pub const SUN_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_SOLAR_MASS);

pub const MERCURY_SEMI_MAJOR_AXIS: Length = Length::from_meters(57_909_050_000.);
pub const MERCURY_ECCENTRICITY: Float = 0.205_630_69;
pub const MERCURY_INCLINATION: Angle = Angle::from_radians(7.00487 * RADIANS_PER_DEGREE);
pub const MERCURY_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(48.33167 * RADIANS_PER_DEGREE);
pub const MERCURY_ARGUMENT_OF_PERIAPSIS: Angle =
    Angle::from_radians(29.124279 * RADIANS_PER_DEGREE);
pub const MERCURY_BOND_ALBEDO: Float = 0.088;

pub const MERCURY_RADIUS: Length = Length::from_meters(2_439_700.);
pub const MERCURY_MASS: Mass = Mass::from_kilograms(3.3011e23);
pub const MERCURY_SIDERIAL_ROTATION_PERIOD: Time = Time::from_seconds(58.646_2 * SECONDS_PER_DAY);
pub const MERCURY_AXIS_TILT: Angle = Angle::from_radians(0.034 * RADIANS_PER_DEGREE);

pub const VENUS_SEMI_MAJOR_AXIS: Length = Length::from_meters(108_208_000_000.);
pub const VENUS_ECCENTRICITY: Float = 0.006_773_23;
pub const VENUS_INCLINATION: Angle = Angle::from_radians(3.39471 * RADIANS_PER_DEGREE);
pub const VENUS_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(76.68069 * RADIANS_PER_DEGREE);
pub const VENUS_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(54.85229 * RADIANS_PER_DEGREE);
pub const VENUS_BOND_ALBEDO: Float = 0.76;

pub const VENUS_RADIUS: Length = Length::from_meters(6_051_800.);
pub const VENUS_MASS: Mass = Mass::from_kilograms(4.8675e24);
pub const VENUS_SIDERIAL_ROTATION_PERIOD: Time = Time::from_seconds(-243.022_6 * SECONDS_PER_DAY);
pub const VENUS_AXIS_TILT: Angle = Angle::from_radians(177.36 * RADIANS_PER_DEGREE);

pub const EARTH_SEMI_MAJOR_AXIS: Length = Length::from_meters(149_598_023_000.);
pub const EARTH_ECCENTRICITY: Float = 0.016_708_6;
pub const EARTH_INCLINATION: Angle = Angle::from_radians(0.);
pub const EARTH_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(-11.26064 * RADIANS_PER_DEGREE);
pub const EARTH_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(114.20783 * RADIANS_PER_DEGREE);
pub const EARTH_BOND_ALBEDO: Float = 0.306;

pub const EARTH_RADIUS: Length = Length::from_meters(METERS_PER_EARTH_RADIUS);
pub const EARTH_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_EARTH_MASS);
pub const EARTH_SIDERIAL_ROTATION_PERIOD: Time = Time::from_seconds(0.997_269_68 * SECONDS_PER_DAY);
pub const EARTH_AXIS_TILT: Angle = Angle::from_radians(23.439_2811 * RADIANS_PER_DEGREE);

pub const MARS_SEMI_MAJOR_AXIS: Length = Length::from_meters(227_939_200_000.);
pub const MARS_ECCENTRICITY: Float = 0.093_394_1;
pub const MARS_INCLINATION: Angle = Angle::from_radians(1.85061 * RADIANS_PER_DEGREE);
pub const MARS_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(49.57854 * RADIANS_PER_DEGREE);
pub const MARS_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(286.4623 * RADIANS_PER_DEGREE);
pub const MARS_BOND_ALBEDO: Float = 0.25;

pub const MARS_RADIUS: Length = Length::from_meters(3_389_500.);
pub const MARS_MASS: Mass = Mass::from_kilograms(6.4171e23);
pub const MARS_SIDERIAL_ROTATION_PERIOD: Time = Time::from_seconds(1.025_956_75 * SECONDS_PER_DAY);
pub const MARS_AXIS_TILT: Angle = Angle::from_radians(25.19 * RADIANS_PER_DEGREE);

pub const CERES_SEMI_MAJOR_AXIS: Length = Length::from_meters(413_690_250_000.);
pub const CERES_ECCENTRICITY: Float = 0.075_823_9;
pub const CERES_INCLINATION: Angle = Angle::from_radians(10.593 * RADIANS_PER_DEGREE);
pub const CERES_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(80.393 * RADIANS_PER_DEGREE);
pub const CERES_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(73.597 * RADIANS_PER_DEGREE);
pub const CERES_BOND_ALBEDO: Float = 0.09; //Bond albedo is apparentyl unknown, taking geometric albedo

pub const CERES_RADIUS: Length = Length::from_meters(476_200.);
pub const CERES_MASS: Mass = Mass::from_kilograms(9.393e20);
pub const CERES_SIDERIAL_ROTATION_PERIOD: Time = Time::from_seconds(9.074_170 * SECONDS_PER_HOUR);
pub const CERES_AXIS_TILT: Angle = Angle::from_radians(4. * RADIANS_PER_DEGREE);

pub const JUPITER_SEMI_MAJOR_AXIS: Length = Length::from_meters(778_547_200_000.);
pub const JUPITER_ECCENTRICITY: Float = 0.048_386_24;
pub const JUPITER_INCLINATION: Angle = Angle::from_radians(1.30530 * RADIANS_PER_DEGREE);
pub const JUPITER_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(100.55615 * RADIANS_PER_DEGREE);
pub const JUPITER_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(273.865 * RADIANS_PER_DEGREE);
pub const JUPITER_BOND_ALBEDO: Float = 0.503;

pub const JUPITER_RADIUS: Length = Length::from_meters(METERS_PER_JUPITER_RADIUS);
pub const JUPITER_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_JUPITER_MASS);
pub const JUPITER_SIDERIAL_ROTATION_PERIOD: Time = Time::from_seconds(9.925_8 * SECONDS_PER_HOUR);
pub const JUPITER_AXIS_TILT: Angle = Angle::from_radians(3.13 * RADIANS_PER_DEGREE);

pub const SATURN_SEMI_MAJOR_AXIS: Length = Length::from_meters(1_433_449_370_000.);
pub const SATURN_ECCENTRICITY: Float = 0.054_150_60;
pub const SATURN_INCLINATION: Angle = Angle::from_radians(2.48446 * RADIANS_PER_DEGREE);
pub const SATURN_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(113.71504 * RADIANS_PER_DEGREE);
pub const SATURN_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(339.39153 * RADIANS_PER_DEGREE);
pub const SATURN_BOND_ALBEDO: Float = 0.342;

pub const SATURN_RADIUS: Length = Length::from_meters(58_232_000.);
pub const SATURN_MASS: Mass = Mass::from_kilograms(5.6834e26);
pub const SATURN_SIDERIAL_ROTATION_PERIOD: Time = Time::from_seconds(10.656 * SECONDS_PER_HOUR);
pub const SATURN_AXIS_TILT: Angle = Angle::from_radians(26.73 * RADIANS_PER_DEGREE);

pub const URANUS_SEMI_MAJOR_AXIS: Length = Length::from_meters(2_872_463_710_000.);
pub const URANUS_ECCENTRICITY: Float = 0.047_167_71;
pub const URANUS_INCLINATION: Angle = Angle::from_radians(0.76986 * RADIANS_PER_DEGREE);
pub const URANUS_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(74.22988 * RADIANS_PER_DEGREE);
pub const URANUS_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(96.734 * RADIANS_PER_DEGREE);
pub const URANUS_BOND_ALBEDO: Float = 0.3;

pub const URANUS_RADIUS: Length = Length::from_meters(25_362_000.);
pub const URANUS_MASS: Mass = Mass::from_kilograms(8.6810e25);
pub const URANUS_SIDERIAL_ROTATION_PERIOD: Time = Time::from_seconds(-17.24 * SECONDS_PER_HOUR);
pub const URANUS_AXIS_TILT: Angle = Angle::from_radians(82.23 * RADIANS_PER_DEGREE);

pub const NEPTUNE_SEMI_MAJOR_AXIS: Length = Length::from_meters(4_495_060_000_000.);
pub const NEPTUNE_ECCENTRICITY: Float = 0.008_585_87;
pub const NEPTUNE_INCLINATION: Angle = Angle::from_radians(1.76917 * RADIANS_PER_DEGREE);
pub const NEPTUNE_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(131.72169 * RADIANS_PER_DEGREE);
pub const NEPTUNE_ARGUMENT_OF_PERIAPSIS: Angle =
    Angle::from_radians(265.64685 * RADIANS_PER_DEGREE);
pub const NEPTUNE_BOND_ALBEDO: Float = 0.29;

pub const NEPTUNE_RADIUS: Length = Length::from_meters(24_622_000.);
pub const NEPTUNE_MASS: Mass = Mass::from_kilograms(1.0243e26);
pub const NEPTUNE_SIDERIAL_ROTATION_PERIOD: Time = Time::from_seconds(16.11 * SECONDS_PER_HOUR);
pub const NEPTUNE_AXIS_TILT: Angle = Angle::from_radians(28.32 * RADIANS_PER_DEGREE);

pub const PLUTO_SEMI_MAJOR_AXIS: Length = Length::from_meters(5_906_380_000_000.);
pub const PLUTO_ECCENTRICITY: Float = 0.248_807_66;
pub const PLUTO_INCLINATION: Angle = Angle::from_radians(17.16 * RADIANS_PER_DEGREE);
pub const PLUTO_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(110.299 * RADIANS_PER_DEGREE);
pub const PLUTO_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(113.76329 * RADIANS_PER_DEGREE);
pub const PLUTO_BOND_ALBEDO: Float = 0.41;

pub const PLUTO_RADIUS: Length = Length::from_meters(1_188_300.);
pub const PLUTO_MASS: Mass = Mass::from_kilograms(1.303e22);
pub const PLUTO_SIDERIAL_ROTATION_PERIOD: Time = Time::from_seconds(-6.387_230 * SECONDS_PER_DAY);
pub const PLUTO_AXIS_TILT: Angle = Angle::from_radians(122.53 * RADIANS_PER_DEGREE);

pub const MOON_SEMI_MAJOR_AXIS: Length = Length::from_meters(384_399_000.);
pub const MOON_ECCENTRICITY: Float = 0.054_9;
pub const MOON_INCLINATION: Angle = Angle::from_radians(5.145 * RADIANS_PER_DEGREE);
pub const MOON_LONGITUDE_OF_ASCENDING_NODE: Angle =
    Angle::from_radians(125.08 * RADIANS_PER_DEGREE);
pub const MOON_ARGUMENT_OF_PERIAPSIS: Angle = Angle::from_radians(318.15 * RADIANS_PER_DEGREE);
pub const MOON_BOND_ALBEDO: Float = 0.11;

pub const MOON_RADIUS: Length = Length::from_meters(1_737_400.);
pub const MOON_MASS: Mass = Mass::from_kilograms(KILOGRAMS_PER_MOON_MASS);
pub const MOON_SIDERIAL_ROTATION_PERIOD: Time = Time::from_seconds(27.321_661 * SECONDS_PER_DAY);
pub const MOON_AXIS_TILT: Angle = Angle::from_radians(6.68 * RADIANS_PER_DEGREE);
