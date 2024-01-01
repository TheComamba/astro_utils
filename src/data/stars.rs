use crate::{
    coordinates::earth_equatorial::EarthEquatorialCoordinates,
    stellar_properties::StellarProperties,
    units::{
        angle::{Angle, RADIANS_PER_DEGREE},
        length::{Length, METERS_PER_LIGHT_YEAR, METERS_PER_SUN_RADIUS},
        luminosity::Luminosity,
        mass::{Mass, KILOGRAMS_PER_SOLAR_MASS},
        temperature::Temperature,
    },
};

pub const SUN_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Sun",
    Length::from_meters(6.957e8),
    Mass::from_kilograms(1.98847e30),
    Luminosity::from_magnitude(4.83),
    Temperature::from_kelvin(5778.0),
);

pub const SIRIUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Sirius",
    Length::from_meters(1.711 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.063 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(1.43),
    Temperature::from_kelvin(9940.),
);

pub const SIRIUS_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(101.28715533 * RADIANS_PER_DEGREE),
    Angle::from_radians(-16.71611587 * RADIANS_PER_DEGREE),
);

pub const SIRIUS_DISTANCE: Length = Length::from_meters(8.6 * METERS_PER_LIGHT_YEAR);

pub const CANOPUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Canopus",
    Length::from_meters(71.2 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(8.02 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-5.71),
    Temperature::from_kelvin(7350.),
);

pub const CANOPUS_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(94.008 * RADIANS_PER_DEGREE),
    Angle::from_radians(-52.695 * RADIANS_PER_DEGREE),
);

pub const CANOPUS_DISTANCE: Length = Length::from_meters(310. * METERS_PER_LIGHT_YEAR);

pub const RIGIL_KENTAURUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Rigil Kentaurus",
    Length::from_meters(1.227 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.33 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(4.38),
    Temperature::from_kelvin(5790.),
);

pub const RIGIL_KENTAURUS_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(219.902 * RADIANS_PER_DEGREE),
    Angle::from_radians(-60.833 * RADIANS_PER_DEGREE),
);

pub const RIGIL_KENTAURUS_DISTANCE: Length = Length::from_meters(4.4 * METERS_PER_LIGHT_YEAR);

pub const ARCTURUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Arcturus",
    Length::from_meters(25.4 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.08 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-0.04),
    Temperature::from_kelvin(4286.),
);

pub const ARCTURUS_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(213.915 * RADIANS_PER_DEGREE),
    Angle::from_radians(19.182 * RADIANS_PER_DEGREE),
);

pub const ARCTURUS_DISTANCE: Length = Length::from_meters(37. * METERS_PER_LIGHT_YEAR);

pub const VEGA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Vega",
    Length::from_meters(2.362 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.135 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(0.58),
    Temperature::from_kelvin(9602.),
);

pub const VEGA_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(279.234 * RADIANS_PER_DEGREE),
    Angle::from_radians(38.783 * RADIANS_PER_DEGREE),
);

pub const VEGA_DISTANCE: Length = Length::from_meters(25. * METERS_PER_LIGHT_YEAR);

pub const CAPELLA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Capella",
    Length::from_meters(11.98 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.56 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-0.51),
    Temperature::from_kelvin(4900.),
);

pub const CAPELLA_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(79.172 * RADIANS_PER_DEGREE),
    Angle::from_radians(45.998 * RADIANS_PER_DEGREE),
);

pub const CAPELLA_DISTANCE: Length = Length::from_meters(42. * METERS_PER_LIGHT_YEAR);

pub const RIGEL_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Rigel",
    Length::from_meters(0.296 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(0.00048 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(10.4),
    Temperature::from_kelvin(3060.),
);

pub const RIGEL_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(79.172 * RADIANS_PER_DEGREE),
    Angle::from_radians(45.998 * RADIANS_PER_DEGREE),
);

pub const RIGEL_DISTANCE: Length = Length::from_meters(4.4 * METERS_PER_LIGHT_YEAR);

pub const PROCYON_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Procyon",
    Length::from_meters(1.911 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.499 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(2.66),
    Temperature::from_kelvin(6530.),
);

pub const PROCYON_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(114.825 * RADIANS_PER_DEGREE),
    Angle::from_radians(5.225 * RADIANS_PER_DEGREE),
);

pub const PROCYON_DISTANCE: Length = Length::from_meters(11.4 * METERS_PER_LIGHT_YEAR);

pub const ACHERNAR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Achernar",
    Length::from_meters(7.144 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(6.7 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-1.57),
    Temperature::from_kelvin(15000.),
);

pub const ACHERNAR_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(24.428 * RADIANS_PER_DEGREE),
    Angle::from_radians(-57.236 * RADIANS_PER_DEGREE),
);

pub const ACHERNAR_DISTANCE: Length = Length::from_meters(139. * METERS_PER_LIGHT_YEAR);

pub const BETELGEUSE_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Betelgeuse",
    Length::from_meters(887. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(11.6 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-5.85),
    Temperature::from_kelvin(3600.),
);

pub const BETELGEUSE_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(88.792 * RADIANS_PER_DEGREE),
    Angle::from_radians(7.407 * RADIANS_PER_DEGREE),
);

pub const BETELGEUSE_DISTANCE: Length = Length::from_meters(640. * METERS_PER_LIGHT_YEAR);

pub const HADAR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Hadar",
    Length::from_meters(13.2 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(9.7 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-3.77),
    Temperature::from_kelvin(25000.),
);

pub const HADAR_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(210.955 * RADIANS_PER_DEGREE),
    Angle::from_radians(-60.373 * RADIANS_PER_DEGREE),
);

pub const HADAR_DISTANCE: Length = Length::from_meters(390. * METERS_PER_LIGHT_YEAR);

pub const ALTAIR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Altair",
    Length::from_meters(1.63 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.79 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(2.21),
    Temperature::from_kelvin(7670.),
);

pub const ALTAIR_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(297.695 * RADIANS_PER_DEGREE),
    Angle::from_radians(8.868 * RADIANS_PER_DEGREE),
);

pub const ALTAIR_DISTANCE: Length = Length::from_meters(17. * METERS_PER_LIGHT_YEAR);

pub const ACRUX_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Acrux",
    Length::from_meters(7.8 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(17. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-3.77),
    Temperature::from_kelvin(28_000.),
);

pub const ACRUX_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(186.649 * RADIANS_PER_DEGREE),
    Angle::from_radians(-63.099 * RADIANS_PER_DEGREE),
);

pub const ACRUX_DISTANCE: Length = Length::from_meters(320. * METERS_PER_LIGHT_YEAR);

pub const ALDEBARAN_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Aldebaran",
    Length::from_meters(44.2 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.65 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-0.63),
    Temperature::from_kelvin(3910.),
);

pub const ALDEBARAN_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(68.980 * RADIANS_PER_DEGREE),
    Angle::from_radians(16.509 * RADIANS_PER_DEGREE),
);

pub const ALDEBARAN_DISTANCE: Length = Length::from_meters(65. * METERS_PER_LIGHT_YEAR);

pub const ANTARES_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Antares",
    Length::from_meters(883. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(12.4 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-5.28),
    Temperature::from_kelvin(3600.),
);

pub const ANTARES_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(247.351 * RADIANS_PER_DEGREE),
    Angle::from_radians(-26.432 * RADIANS_PER_DEGREE),
);

pub const ANTARES_DISTANCE: Length = Length::from_meters(550. * METERS_PER_LIGHT_YEAR);

pub const SPICA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Spica",
    Length::from_meters(7.04 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(11. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-3.55),
    Temperature::from_kelvin(22500.),
);

pub const SPICA_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(201.298 * RADIANS_PER_DEGREE),
    Angle::from_radians(-11.16 * RADIANS_PER_DEGREE),
);

pub const SPICA_DISTANCE: Length = Length::from_meters(260. * METERS_PER_LIGHT_YEAR);

pub const POLLUX_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Pollux",
    Length::from_meters(9.3 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.8 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(1.15),
    Temperature::from_kelvin(4690.),
);

pub const POLLUX_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(116.328 * RADIANS_PER_DEGREE),
    Angle::from_radians(28.026 * RADIANS_PER_DEGREE),
);

pub const POLLUX_DISTANCE: Length = Length::from_meters(34. * METERS_PER_LIGHT_YEAR);

pub const FORMALHAUT_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Formalhaut",
    Length::from_meters(1.84 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.92 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(1.72),
    Temperature::from_kelvin(8590.),
);

pub const FORMALHAUT_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(344.412 * RADIANS_PER_DEGREE),
    Angle::from_radians(-29.622 * RADIANS_PER_DEGREE),
);

pub const FORMALHAUT_DISTANCE: Length = Length::from_meters(25. * METERS_PER_LIGHT_YEAR);

pub const DENEB_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Deneb",
    Length::from_meters(203. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(19. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-7.17),
    Temperature::from_kelvin(8500.),
);

pub const DENEB_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(310.357 * RADIANS_PER_DEGREE),
    Angle::from_radians(45.281 * RADIANS_PER_DEGREE),
);

pub const DENEB_DISTANCE: Length = Length::from_meters(2600. * METERS_PER_LIGHT_YEAR);

pub const MIMOSA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Mimosa",
    Length::from_meters(9. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(15. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-4.7),
    Temperature::from_kelvin(22000.),
);

pub const MIMOSA_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(210.955 * RADIANS_PER_DEGREE),
    Angle::from_radians(-60.373 * RADIANS_PER_DEGREE),
);

pub const MIMOSA_DISTANCE: Length = Length::from_meters(350. * METERS_PER_LIGHT_YEAR);

pub const REGULUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Regulus",
    Length::from_meters(3.15 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(3.8 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(1.4),
    Temperature::from_kelvin(12100.),
);

pub const REGULUS_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(152.092 * RADIANS_PER_DEGREE),
    Angle::from_radians(11.967 * RADIANS_PER_DEGREE),
);

pub const REGULUS_DISTANCE: Length = Length::from_meters(79. * METERS_PER_LIGHT_YEAR);

pub const ADHARA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Adhara",
    Length::from_meters(23. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(18. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-4.99),
    Temperature::from_kelvin(22000.),
);

pub const ADHARA_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(104.656 * RADIANS_PER_DEGREE),
    Angle::from_radians(-28.972 * RADIANS_PER_DEGREE),
);

pub const ADHARA_DISTANCE: Length = Length::from_meters(430. * METERS_PER_LIGHT_YEAR);

pub const SHAULA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Shaula",
    Length::from_meters(7. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(18. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-3.3),
    Temperature::from_kelvin(25000.),
);

pub const SHAULA_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(96.346 * RADIANS_PER_DEGREE),
    Angle::from_radians(-37.103 * RADIANS_PER_DEGREE),
);

pub const SHAULA_DISTANCE: Length = Length::from_meters(700. * METERS_PER_LIGHT_YEAR);

pub const CASTOR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Castor",
    Length::from_meters(2.9 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.2 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(0.986),
    Temperature::from_kelvin(8840.),
);

pub const CASTOR_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(113.649 * RADIANS_PER_DEGREE),
    Angle::from_radians(31.888 * RADIANS_PER_DEGREE),
);

pub const CASTOR_DISTANCE: Length = Length::from_meters(51. * METERS_PER_LIGHT_YEAR);

pub const GACRUX_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Gacrux",
    Length::from_meters(120. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-0.52),
    Temperature::from_kelvin(3689.),
);

pub const GACRUX_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(186.649 * RADIANS_PER_DEGREE),
    Angle::from_radians(-63.099 * RADIANS_PER_DEGREE),
);

pub const GACRUX_DISTANCE: Length = Length::from_meters(88.6 * METERS_PER_LIGHT_YEAR);

pub const BELLATRIX_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Bellatrix",
    Length::from_meters(5.7 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(7.7 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-2.78),
    Temperature::from_kelvin(21_800.),
);

pub const BELLATRIX_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(81.282 * RADIANS_PER_DEGREE),
    Angle::from_radians(6.349 * RADIANS_PER_DEGREE),
);

pub const BELLATRIX_DISTANCE: Length = Length::from_meters(250. * METERS_PER_LIGHT_YEAR);

pub const ELNATH_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Elnath",
    Length::from_meters(4.2 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(5.0 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-1.42),
    Temperature::from_kelvin(13_824.),
);

pub const ELNATH_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(1.439),
    Angle::from_radians(28. * RADIANS_PER_DEGREE),
);

pub const ELNATH_DISTANCE: Length = Length::from_meters(134. * METERS_PER_LIGHT_YEAR);

pub const MIAPLACIDUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Miaplacidus",
    Length::from_meters(6.8 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(3.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-1.03),
    Temperature::from_kelvin(8866.),
);

pub const MIAPLACIDUS_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(2.42),
    Angle::from_radians(-69. * RADIANS_PER_DEGREE),
);

pub const MIAPLACIDUS_DISTANCE: Length = Length::from_meters(113. * METERS_PER_LIGHT_YEAR);

pub const ALNILAM_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alnilam",
    Length::from_meters(42. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(64.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-6.89),
    Temperature::from_kelvin(27_000.),
);

pub const ALNILAM_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(1.44),
    Angle::from_radians(-1. * RADIANS_PER_DEGREE),
);

pub const ALNILAM_DISTANCE: Length = Length::from_meters(2000. * METERS_PER_LIGHT_YEAR);

pub const GAMMA_VELORUM_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Gamma Velorum",
    Length::from_meters(6. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(9. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-4.23),
    Temperature::from_kelvin(57000.),
);

pub const GAMMA_VELORUM_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(2.10),
    Angle::from_radians(-47. * RADIANS_PER_DEGREE),
);

pub const GAMMA_VELORUM_DISTANCE: Length = Length::from_meters(1236. * METERS_PER_LIGHT_YEAR);

pub const ALNAIR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alnair",
    Length::from_meters(3.4 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(4. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_magnitude(-0.72),
    Temperature::from_kelvin(13_920.),
);

pub const ALNAIR_POSITION: EarthEquatorialCoordinates = EarthEquatorialCoordinates::new(
    Angle::from_radians(5.8),
    Angle::from_radians(-46. * RADIANS_PER_DEGREE),
);

pub const ALNAIR_DISTANCE: Length = Length::from_meters(100. * METERS_PER_LIGHT_YEAR);
