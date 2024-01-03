use crate::{
    coordinates::{
        declination::Declination, earth_equatorial::EarthEquatorialCoordinates,
        right_ascension::RightAscension,
    },
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
    Luminosity::from_absolute_magnitude(4.83),
    Temperature::from_kelvin(5778.0),
);

pub const SIRIUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Sirius",
    Length::from_meters(1.711 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.063 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(1.43),
    Temperature::from_kelvin(9940.),
);

pub const SIRIUS_RA: RightAscension = RightAscension::new(6, 45, 9);
pub const SIRIUS_DEC: Declination = Declination::new(-16, 42, 58);
pub const SIRIUS_DISTANCE: Length = Length::from_meters(8.6 * METERS_PER_LIGHT_YEAR);

pub const CANOPUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Canopus",
    Length::from_meters(72. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(9. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-5.71),
    Temperature::from_kelvin(7400.),
);

pub const CANOPUS_RA: RightAscension = RightAscension::new(6, 23, 57);
pub const CANOPUS_DEC: Declination = Declination::new(-52, 41, 44);
pub const CANOPUS_DISTANCE: Length = Length::from_meters(310. * METERS_PER_LIGHT_YEAR);

pub const ALPHA_CENTAURI_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alpha Centauri",
    Length::from_meters(1.2175 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.0788 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(4.34),
    Temperature::from_kelvin(5790.),
);

pub const ALPHA_CENTAURI_RA: RightAscension = RightAscension::new(14, 39, 36);
pub const ALPHA_CENTAURI_DEC: Declination = Declination::new(-60, 50, 2);
pub const ALPHA_CENTAURI_DISTANCE: Length = Length::from_meters(4.34 * METERS_PER_LIGHT_YEAR);

pub const ARCTURUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Arcturus",
    Length::from_meters(25.4 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.08 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.3),
    Temperature::from_kelvin(4286.),
);

pub const ARCTURUS_RA: RightAscension = RightAscension::new(14, 15, 40);
pub const ARCTURUS_DEC: Declination = Declination::new(19, 10, 56);
pub const ARCTURUS_DISTANCE: Length = Length::from_meters(37. * METERS_PER_LIGHT_YEAR);

pub const VEGA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Vega",
    Length::from_meters(2.362 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.135 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.582),
    Temperature::from_kelvin(9602.),
);

pub const VEGA_RA: RightAscension = RightAscension::new(18, 36, 56);
pub const VEGA_DEC: Declination = Declination::new(38, 47, 1);
pub const VEGA_DISTANCE: Length = Length::from_meters(25. * METERS_PER_LIGHT_YEAR);

pub const CAPELLA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Capella",
    Length::from_meters(11.98 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.5687 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.296),
    Temperature::from_kelvin(4970.),
);

pub const CAPELLA_RA: RightAscension = RightAscension::new(5, 16, 41);
pub const CAPELLA_DEC: Declination = Declination::new(45, 59, 53);
pub const CAPELLA_DISTANCE: Length = Length::from_meters(43.38 * METERS_PER_LIGHT_YEAR);

pub const RIGEL_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Rigel",
    Length::from_meters(78.9 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(21. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-7.84),
    Temperature::from_kelvin(12_100.),
);

pub const RIGEL_RA: RightAscension = RightAscension::new(5, 14, 32);
pub const RIGEL_DEC: Declination = Declination::new(-8, 12, 6);
pub const RIGEL_DISTANCE: Length = Length::from_meters(860. * METERS_PER_LIGHT_YEAR);

pub const PROCYON_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Procyon",
    Length::from_meters(2.048 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.499 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(2.66),
    Temperature::from_kelvin(6530.),
);

pub const PROCYON_RA: RightAscension = RightAscension::new(7, 39, 18);
pub const PROCYON_DEC: Declination = Declination::new(5, 13, 30);
pub const PROCYON_DISTANCE: Length = Length::from_meters(11.46 * METERS_PER_LIGHT_YEAR);

pub const ACHERNAR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Achernar",
    Length::from_meters(6.78 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(6.0 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.46),
    Temperature::from_kelvin(14_000.),
);

pub const ACHERNAR_RA: RightAscension = RightAscension::new(1, 37, 43);
pub const ACHERNAR_DEC: Declination = Declination::new(-57, 14, 12);
pub const ACHERNAR_DISTANCE: Length = Length::from_meters(139. * METERS_PER_LIGHT_YEAR);

pub const BETELGEUSE_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Betelgeuse",
    Length::from_meters(887. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(16.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-5.85),
    Temperature::from_kelvin(3600.),
);

pub const BETELGEUSE_RA: RightAscension = RightAscension::new(5, 55, 10);
pub const BETELGEUSE_DEC: Declination = Declination::new(7, 24, 25);
pub const BETELGEUSE_DISTANCE: Length = Length::from_meters(548. * METERS_PER_LIGHT_YEAR);

pub const HADAR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Hadar",
    Length::from_meters(9. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(12.02 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-4.9),
    Temperature::from_kelvin(25_000.),
);

pub const HADAR_RA: RightAscension = RightAscension::new(14, 3, 49);
pub const HADAR_DEC: Declination = Declination::new(-60, 22, 23);
pub const HADAR_DISTANCE: Length = Length::from_meters(390. * METERS_PER_LIGHT_YEAR);

pub const ALTAIR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Altair",
    Length::from_meters(1.63 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.86 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(2.22),
    Temperature::from_kelvin(7670.),
);

pub const ALTAIR_RA: RightAscension = RightAscension::new(19, 50, 47);
pub const ALTAIR_DEC: Declination = Declination::new(8, 52, 6);
pub const ALTAIR_DISTANCE: Length = Length::from_meters(16.73 * METERS_PER_LIGHT_YEAR);

pub const ACRUX_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Acrux",
    Length::from_meters(7.8 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(17.8 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.77),
    Temperature::from_kelvin(24_000.),
);

pub const ACRUX_RA: RightAscension = RightAscension::new(12, 26, 36);
pub const ACRUX_DEC: Declination = Declination::new(-63, 5, 57);
pub const ACRUX_DISTANCE: Length = Length::from_meters(320. * METERS_PER_LIGHT_YEAR);

pub const ALDEBARAN_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Aldebaran",
    Length::from_meters(45.1 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.16 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.641),
    Temperature::from_kelvin(3900.),
);

pub const ALDEBARAN_RA: RightAscension = RightAscension::new(4, 35, 55);
pub const ALDEBARAN_DEC: Declination = Declination::new(16, 30, 33);
pub const ALDEBARAN_DISTANCE: Length = Length::from_meters(65.3 * METERS_PER_LIGHT_YEAR);

pub const ANTARES_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Antares",
    Length::from_meters(680. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(13.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-5.28),
    Temperature::from_kelvin(3660.),
);

pub const ANTARES_RA: RightAscension = RightAscension::new(16, 29, 24);
pub const ANTARES_DEC: Declination = Declination::new(-26, 25, 55);
pub const ANTARES_DISTANCE: Length = Length::from_meters(550. * METERS_PER_LIGHT_YEAR);

pub const SPICA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Spica",
    Length::from_meters(7.47 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(11.43 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.55),
    Temperature::from_kelvin(22_300.),
);

pub const SPICA_RA: RightAscension = RightAscension::new(13, 25, 12);
pub const SPICA_DEC: Declination = Declination::new(-11, 9, 41);
pub const SPICA_DISTANCE: Length = Length::from_meters(260. * METERS_PER_LIGHT_YEAR);

pub const POLLUX_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Pollux",
    Length::from_meters(9.06 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.91 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(1.08),
    Temperature::from_kelvin(4586.),
);

pub const POLLUX_RA: RightAscension = RightAscension::new(7, 45, 19);
pub const POLLUX_DEC: Declination = Declination::new(28, 1, 34);
pub const POLLUX_DISTANCE: Length = Length::from_meters(33.78 * METERS_PER_LIGHT_YEAR);

pub const FORMALHAUT_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Formalhaut",
    Length::from_meters(1.842 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.92 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(1.72),
    Temperature::from_kelvin(8590.),
);

pub const FORMALHAUT_RA: RightAscension = RightAscension::new(22, 57, 39);
pub const FORMALHAUT_DEC: Declination = Declination::new(-29, 37, 20);
pub const FORMALHAUT_DISTANCE: Length = Length::from_meters(25.13 * METERS_PER_LIGHT_YEAR);

pub const DENEB_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Deneb",
    Length::from_meters(203. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(19. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-8.38),
    Temperature::from_kelvin(8515.),
);

pub const DENEB_RA: RightAscension = RightAscension::new(20, 41, 26);
pub const DENEB_DEC: Declination = Declination::new(45, 16, 49);
pub const DENEB_DISTANCE: Length = Length::from_meters(2615. * METERS_PER_LIGHT_YEAR);

pub const MIMOSA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Mimosa",
    Length::from_meters(8.4 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(16. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.92),
    Temperature::from_kelvin(27_000.),
);

pub const MIMOSA_RA: RightAscension = RightAscension::new(12, 47, 43);
pub const MIMOSA_DEC: Declination = Declination::new(-59, 41, 20);
pub const MIMOSA_DISTANCE: Length = Length::from_meters(280. * METERS_PER_LIGHT_YEAR);

pub const REGULUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Regulus",
    Length::from_meters(4.35 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(3.8 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.57),
    Temperature::from_kelvin(11_668.),
);

pub const REGULUS_RA: RightAscension = RightAscension::new(10, 8, 22);
pub const REGULUS_DEC: Declination = Declination::new(11, 58, 2);
pub const REGULUS_DISTANCE: Length = Length::from_meters(79.3 * METERS_PER_LIGHT_YEAR);

pub const ADHARA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Adhara",
    Length::from_meters(13.9 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(12.6 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-4.8),
    Temperature::from_kelvin(22_900.),
);

pub const ADHARA_RA: RightAscension = RightAscension::new(6, 58, 38);
pub const ADHARA_DEC: Declination = Declination::new(-28, 58, 19);
pub const ADHARA_DISTANCE: Length = Length::from_meters(430. * METERS_PER_LIGHT_YEAR);

pub const SHAULA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Shaula",
    Length::from_meters(8.8 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(10.4 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.7),
    Temperature::from_kelvin(25_000.),
);

pub const SHAULA_RA: RightAscension = RightAscension::new(17, 33, 37);
pub const SHAULA_DEC: Declination = Declination::new(-37, 6, 14);
pub const SHAULA_DISTANCE: Length = Length::from_meters(570. * METERS_PER_LIGHT_YEAR);

pub const CASTOR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Castor",
    Length::from_meters(2.9 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.37 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.986),
    Temperature::from_kelvin(10_286.),
);

pub const CASTOR_RA: RightAscension = RightAscension::new(7, 34, 36);
pub const CASTOR_DEC: Declination = Declination::new(31, 53, 18);
pub const CASTOR_DISTANCE: Length = Length::from_meters(51. * METERS_PER_LIGHT_YEAR);

pub const GACRUX_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Gacrux",
    Length::from_meters(120. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.52),
    Temperature::from_kelvin(3689.),
);

pub const GACRUX_RA: RightAscension = RightAscension::new(12, 31, 10);
pub const GACRUX_DEC: Declination = Declination::new(-57, 6, 48);
pub const GACRUX_DISTANCE: Length = Length::from_meters(88.6 * METERS_PER_LIGHT_YEAR);

pub const BELLATRIX_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Bellatrix",
    Length::from_meters(5.75 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(7.7 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-2.78),
    Temperature::from_kelvin(21_800.),
);

pub const BELLATRIX_RA: RightAscension = RightAscension::new(5, 25, 8);
pub const BELLATRIX_DEC: Declination = Declination::new(6, 20, 59);
pub const BELLATRIX_DISTANCE: Length = Length::from_meters(250. * METERS_PER_LIGHT_YEAR);

pub const ELNATH_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Elnath",
    Length::from_meters(4.2 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(5.0 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.42),
    Temperature::from_kelvin(13_824.),
);

pub const ELNATH_RA: RightAscension = RightAscension::new(5, 26, 18);
pub const ELNATH_DEC: Declination = Declination::new(28, 36, 27);
pub const ELNATH_DISTANCE: Length = Length::from_meters(134. * METERS_PER_LIGHT_YEAR);

pub const MIAPLACIDUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Miaplacidus",
    Length::from_meters(6.8 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(3.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.03),
    Temperature::from_kelvin(8866.),
);

pub const MIAPLACIDUS_RA: RightAscension = RightAscension::new(9, 13, 12);
pub const MIAPLACIDUS_DEC: Declination = Declination::new(-69, 43, 2);
pub const MIAPLACIDUS_DISTANCE: Length = Length::from_meters(113.2 * METERS_PER_LIGHT_YEAR);

pub const ALNILAM_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alnilam",
    Length::from_meters(42. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(64.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-6.89),
    Temperature::from_kelvin(27_000.),
);

pub const ALNILAM_RA: RightAscension = RightAscension::new(5, 36, 13);
pub const ALNILAM_DEC: Declination = Declination::new(-1, 12, 7);
pub const ALNILAM_DISTANCE: Length = Length::from_meters(2000. * METERS_PER_LIGHT_YEAR);

pub const GAMMA_VELORUM_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Gamma Velorum",
    Length::from_meters(17. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(28.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-5.63),
    Temperature::from_kelvin(35_000.),
);

pub const GAMMA_VELORUM_RA: RightAscension = RightAscension::new(8, 9, 32);
pub const GAMMA_VELORUM_DEC: Declination = Declination::new(-47, 20, 12);
pub const GAMMA_VELORUM_DISTANCE: Length = Length::from_meters(1236. * METERS_PER_LIGHT_YEAR);

pub const ALNAIR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alnair",
    Length::from_meters(3.4 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(4. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.72),
    Temperature::from_kelvin(13_920.),
);

pub const ALNAIR_RA: RightAscension = RightAscension::new(22, 8, 14);
pub const ALNAIR_DEC: Declination = Declination::new(-46, 57, 40);
pub const ALNAIR_DISTANCE: Length = Length::from_meters(101. * METERS_PER_LIGHT_YEAR);

pub const ALNITAK_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alnitak",
    Length::from_meters(20. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(33.0 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-6.0),
    Temperature::from_kelvin(29_500.),
);

pub const ALNITAK_RA: RightAscension = RightAscension::new(5, 40, 46);
pub const ALNITAK_DEC: Declination = Declination::new(-1, 56, 34);
pub const ALNITAK_DISTANCE: Length = Length::from_meters(1260. * METERS_PER_LIGHT_YEAR);

pub const ALIOTH_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alioth",
    Length::from_meters(4.14 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.91 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.2),
    Temperature::from_kelvin(9_020.),
);

pub const ALIOTH_RA: RightAscension = RightAscension::new(12, 54, 2);
pub const ALIOTH_DEC: Declination = Declination::new(55, 57, 36);
pub const ALIOTH_DISTANCE: Length = Length::from_meters(82.6 * METERS_PER_LIGHT_YEAR);

pub const DUBHE_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Dubhe",
    Length::from_meters(17.03 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(3.44 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.1),
    Temperature::from_kelvin(5012.),
);

pub const DUBHE_RA: RightAscension = RightAscension::new(11, 3, 44);
pub const DUBHE_DEC: Declination = Declination::new(61, 45, 4);
pub const DUBHE_DISTANCE: Length = Length::from_meters(123. * METERS_PER_LIGHT_YEAR);

pub const MIRFAK_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Mirfak",
    Length::from_meters(68. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(8.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-5.1),
    Temperature::from_kelvin(6350.),
);

pub const MIRFAK_RA: RightAscension = RightAscension::new(3, 24, 19);
pub const MIRFAK_DEC: Declination = Declination::new(49, 51, 40);
pub const MIRFAK_DISTANCE: Length = Length::from_meters(510. * METERS_PER_LIGHT_YEAR);

pub const WEZEN_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Wezen",
    Length::from_meters(215. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(16.9 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-6.86),
    Temperature::from_kelvin(6390.),
);

pub const WEZEN_RA: RightAscension = RightAscension::new(7, 8, 23);
pub const WEZEN_DEC: Declination = Declination::new(-26, 23, 36);
pub const WEZEN_DISTANCE: Length = Length::from_meters(1600. * METERS_PER_LIGHT_YEAR);

pub const SARGAS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Sargas",
    Length::from_meters(26.3 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(3.1 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-2.71),
    Temperature::from_kelvin(6294.),
);

pub const SARGAS_RA: RightAscension = RightAscension::new(17, 37, 19);
pub const SARGAS_DEC: Declination = Declination::new(-42, 59, 52);
pub const SARGAS_DISTANCE: Length = Length::from_meters(329. * METERS_PER_LIGHT_YEAR);

pub const KAUS_AUSTRALIS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Kaus Australis",
    Length::from_meters(6.8 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(3.515 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.41),
    Temperature::from_kelvin(9960.),
);

pub const KAUS_AUSTRALIS_RA: RightAscension = RightAscension::new(18, 24, 10);
pub const KAUS_AUSTRALIS_DEC: Declination = Declination::new(-34, 23, 5);
pub const KAUS_AUSTRALIS_DISTANCE: Length = Length::from_meters(143. * METERS_PER_LIGHT_YEAR);

pub const AVIOR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Avior",
    Length::from_meters(0. * METERS_PER_SUN_RADIUS), //Unknown
    Mass::from_kilograms(10.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-4.47),
    Temperature::from_kelvin(3.523),
);

pub const AVIOR_RA: RightAscension = RightAscension::new(8, 22, 31);
pub const AVIOR_DEC: Declination = Declination::new(-59, 30, 34);
pub const AVIOR_DISTANCE: Length = Length::from_meters(610. * METERS_PER_LIGHT_YEAR);

pub const ALKAID_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alkaid",
    Length::from_meters(3.4 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(6.1 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.67),
    Temperature::from_kelvin(15_540.),
);

pub const ALKAID_RA: RightAscension = RightAscension::new(13, 47, 32);
pub const ALKAID_DEC: Declination = Declination::new(49, 18, 48);
pub const ALKAID_DISTANCE: Length = Length::from_meters(103.9 * METERS_PER_LIGHT_YEAR);

pub const MENKALINAN_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Menkalinan",
    Length::from_meters(2.77 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.389 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.55),
    Temperature::from_kelvin(9350.),
);

pub const MENKALINAN_RA: RightAscension = RightAscension::new(5, 59, 32);
pub const MENKALINAN_DEC: Declination = Declination::new(44, 56, 51);
pub const MENKALINAN_DISTANCE: Length = Length::from_meters(81.1 * METERS_PER_LIGHT_YEAR);

pub const ATRIA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Atria",
    Length::from_meters(143. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(7. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.68),
    Temperature::from_kelvin(4150.),
);

pub const ATRIA_RA: RightAscension = RightAscension::new(16, 48, 40);
pub const ATRIA_DEC: Declination = Declination::new(-69, 1, 40);
pub const ATRIA_DISTANCE: Length = Length::from_meters(391. * METERS_PER_LIGHT_YEAR);

pub const ALHENA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alhena",
    Length::from_meters(3.3 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.81 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.68),
    Temperature::from_kelvin(9260.),
);

pub const ALHENA_RA: RightAscension = RightAscension::new(6, 37, 43);
pub const ALHENA_DEC: Declination = Declination::new(16, 23, 57);
pub const ALHENA_DISTANCE: Length = Length::from_meters(109. * METERS_PER_LIGHT_YEAR);

pub const PEACOCK_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Peacock",
    Length::from_meters(4.83 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(5.91 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.762),
    Temperature::from_kelvin(17_711.),
);

pub const PEACOCK_RA: RightAscension = RightAscension::new(20, 25, 39);
pub const PEACOCK_DEC: Declination = Declination::new(-56, 44, 6);
pub const PEACOCK_DISTANCE: Length = Length::from_meters(179. * METERS_PER_LIGHT_YEAR);

pub const ALSEPHINA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alsephina",
    Length::from_meters(2.9 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.43 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.02),
    Temperature::from_kelvin(9440.),
);

pub const ALSEPHINA_RA: RightAscension = RightAscension::new(8, 44, 42);
pub const ALSEPHINA_DEC: Declination = Declination::new(-54, 42, 32);
pub const ALSEPHINA_DISTANCE: Length = Length::from_meters(80.6 * METERS_PER_LIGHT_YEAR);

pub const MIRZAM_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Mirzam",
    Length::from_meters(9.7 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(13.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-4.1),
    Temperature::from_kelvin(25_000.),
);

pub const MIRZAM_RA: RightAscension = RightAscension::new(6, 22, 42);
pub const MIRZAM_DEC: Declination = Declination::new(-17, 57, 21);
pub const MIRZAM_DISTANCE: Length = Length::from_meters(490. * METERS_PER_LIGHT_YEAR);

pub const ALPHARD_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alphard",
    Length::from_meters(50.5 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(3.03 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.69),
    Temperature::from_kelvin(4120.),
);

pub const ALPHARD_RA: RightAscension = RightAscension::new(9, 27, 35);
pub const ALPHARD_DEC: Declination = Declination::new(-8, 39, 30);
pub const ALPHARD_DISTANCE: Length = Length::from_meters(177. * METERS_PER_LIGHT_YEAR);

pub const POLARIS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Polaris",
    Length::from_meters(37.5 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(5.4 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.6),
    Temperature::from_kelvin(6015.),
);

pub const POLARIS_RA: RightAscension = RightAscension::new(2, 31, 49);
pub const POLARIS_DEC: Declination = Declination::new(89, 15, 51);
pub const POLARIS_DISTANCE: Length = Length::from_meters(380. * METERS_PER_LIGHT_YEAR);

pub const HAMAL_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Hamal",
    Length::from_meters(14.9 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.47),
    Temperature::from_kelvin(4480.),
);

pub const HAMAL_RA: RightAscension = RightAscension::new(2, 7, 10);
pub const HAMAL_DEC: Declination = Declination::new(23, 27, 45);
pub const HAMAL_DISTANCE: Length = Length::from_meters(65.8 * METERS_PER_LIGHT_YEAR);

pub const ALGIEBA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Algieba",
    Length::from_meters(31.88 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.23 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.27),
    Temperature::from_kelvin(4470.),
);

pub const ALGIEBA_RA: RightAscension = RightAscension::new(10, 19, 58);
pub const ALGIEBA_DEC: Declination = Declination::new(19, 50, 29);
pub const ALGIEBA_DISTANCE: Length = Length::from_meters(130. * METERS_PER_LIGHT_YEAR);

pub const DIPHDA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Diphda",
    Length::from_meters(16.78 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.8 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.13),
    Temperature::from_kelvin(4797.),
);

pub const DIPHDA_RA: RightAscension = RightAscension::new(0, 43, 35);
pub const DIPHDA_DEC: Declination = Declination::new(-17, 59, 12);
pub const DIPHDA_DISTANCE: Length = Length::from_meters(96.3 * METERS_PER_LIGHT_YEAR);

pub const MIZAR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Mizar",
    Length::from_meters(2.4 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.2 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.32),
    Temperature::from_kelvin(9000.),
);

pub const MIZAR_RA: RightAscension = RightAscension::new(13, 23, 56);
pub const MIZAR_DEC: Declination = Declination::new(54, 55, 31);
pub const MIZAR_DISTANCE: Length = Length::from_meters(82.9 * METERS_PER_LIGHT_YEAR);

pub const NUNKI_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Nunki",
    Length::from_meters(4.5 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(7.8 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-2.17),
    Temperature::from_kelvin(18_890.),
);

pub const NUNKI_RA: RightAscension = RightAscension::new(18, 55, 16);
pub const NUNKI_DEC: Declination = Declination::new(-26, 17, 49);
pub const NUNKI_DISTANCE: Length = Length::from_meters(228. * METERS_PER_LIGHT_YEAR);

pub const MENKENT_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Menkent",
    Length::from_meters(10.6 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.27 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.87),
    Temperature::from_kelvin(4980.),
);

pub const MENKENT_RA: RightAscension = RightAscension::new(14, 6, 41);
pub const MENKENT_DEC: Declination = Declination::new(-36, 22, 11);
pub const MENKENT_DISTANCE: Length = Length::from_meters(28.8 * METERS_PER_LIGHT_YEAR);

pub const MIRACH_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Mirach",
    Length::from_meters(100 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.49 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.76),
    Temperature::from_kelvin(3842.),
);

pub const MIRACH_RA: RightAscension = RightAscension::new(1, 9, 44);
pub const MIRACH_DEC: Declination = Declination::new(35, 37, 14);
pub const MIRACH_DISTANCE: Length = Length::from_meters(197. * METERS_PER_LIGHT_YEAR);

pub const ALPHERATZ_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alpheratz",
    Length::from_meters(2.7 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(3.8 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(2.00),
    Temperature::from_kelvin(13_800.),
);

pub const ALPHERATZ_RA: RightAscension = RightAscension::new(0, 8, 23);
pub const ALPHERATZ_DEC: Declination = Declination::new(29, 5, 26);
pub const ALPHERATZ_DISTANCE: Length = Length::from_meters(97.0 * METERS_PER_LIGHT_YEAR);

pub const RASALHAGUE_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Rasalhague",
    Length::from_meters(2.6 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.4 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(1.248),
    Temperature::from_kelvin(8000.),
);

pub const RASALHAGUE_RA: RightAscension = RightAscension::new(17, 34, 56);
pub const RASALHAGUE_DEC: Declination = Declination::new(12, 33, 37);
pub const RASALHAGUE_DISTANCE: Length = Length::from_meters(48.6 * METERS_PER_LIGHT_YEAR);

pub const KOCHAB_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Kochab",
    Length::from_meters(42.06 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.2 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.83),
    Temperature::from_kelvin(4030.),
);

pub const KOCHAB_RA: RightAscension = RightAscension::new(14, 50, 42);
pub const KOCHAB_DEC: Declination = Declination::new(74, 9, 20);
pub const KOCHAB_DISTANCE: Length = Length::from_meters(130.9 * METERS_PER_LIGHT_YEAR);

pub const SAIPH_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Saiph",
    Length::from_meters(22.2 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(15.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-6.1),
    Temperature::from_kelvin(26_500.),
);

pub const SAIPH_RA: RightAscension = RightAscension::new(5, 47, 45);
pub const SAIPH_DEC: Declination = Declination::new(-9, 40, 11);
pub const SAIPH_DISTANCE: Length = Length::from_meters(650. * METERS_PER_LIGHT_YEAR);

pub const DENEBOLA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Denebola",
    Length::from_meters(1.728 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.78 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(2.14),
    Temperature::from_kelvin(8500.),
);

pub const DENEBOLA_RA: RightAscension = RightAscension::new(11, 49, 3);
pub const DENEBOLA_DEC: Declination = Declination::new(14, 34, 19);
pub const DENEBOLA_DISTANCE: Length = Length::from_meters(35.9 * METERS_PER_LIGHT_YEAR);

pub const ALGOL_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Algol",
    Length::from_meters(2.73 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(3.17 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.07),
    Temperature::from_kelvin(13_000.),
);

pub const ALGOL_RA: RightAscension = RightAscension::new(3, 8, 10);
pub const ALGOL_DEC: Declination = Declination::new(40, 57, 20);
pub const ALGOL_DISTANCE: Length = Length::from_meters(90. * METERS_PER_LIGHT_YEAR);

pub const TIAKI_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Tiaki",
    Length::from_meters(180. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.4 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.61),
    Temperature::from_kelvin(3480.),
);

pub const TIAKI_RA: RightAscension = RightAscension::new(22, 42, 40);
pub const TIAKI_DEC: Declination = Declination::new(-46, 53, 4);
pub const TIAKI_DISTANCE: Length = Length::from_meters(177. * METERS_PER_LIGHT_YEAR);

pub const MUHLIFAIN_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Muhlifain",
    Length::from_meters(0. * METERS_PER_SUN_RADIUS), //Unknown
    Mass::from_kilograms(2.91 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.81),
    Temperature::from_kelvin(9082.),
);

pub const MUHLIFAIN_RA: RightAscension = RightAscension::new(12, 41, 31);
pub const MUHLIFAIN_DEC: Declination = Declination::new(-48, 57, 35);
pub const MUHLIFAIN_DISTANCE: Length = Length::from_meters(130. * METERS_PER_LIGHT_YEAR);

pub const ASPIDISKE_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Aspidiske",
    Length::from_meters(43. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(7.4 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-5.1),
    Temperature::from_kelvin(7500.),
);

pub const ASPIDISKE_RA: RightAscension = RightAscension::new(9, 17, 5);
pub const ASPIDISKE_DEC: Declination = Declination::new(-59, 16, 30);
pub const ASPIDISKE_DISTANCE: Length = Length::from_meters(690. * METERS_PER_LIGHT_YEAR);

pub const SUHAIL_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Suhail",
    Length::from_meters(210. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(7. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.99),
    Temperature::from_kelvin(3900.),
);

pub const SUHAIL_RA: RightAscension = RightAscension::new(9, 7, 60);
pub const SUHAIL_DEC: Declination = Declination::new(-43, 25, 57);
pub const SUHAIL_DISTANCE: Length = Length::from_meters(545. * METERS_PER_LIGHT_YEAR);

pub const ALPHECCA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alphecca",
    Length::from_meters(3. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.58 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.16),
    Temperature::from_kelvin(9700.),
);

pub const ALPHECCA_RA: RightAscension = RightAscension::new(15, 34, 41);
pub const ALPHECCA_DEC: Declination = Declination::new(26, 42, 53);
pub const ALPHECCA_DISTANCE: Length = Length::from_meters(75. * METERS_PER_LIGHT_YEAR);

pub const MINTAKA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Mintaka",
    Length::from_meters(16.5 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(24. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-5.8),
    Temperature::from_kelvin(29_500.),
);

pub const MINTAKA_RA: RightAscension = RightAscension::new(5, 32, 0);
pub const MINTAKA_DEC: Declination = Declination::new(0, 17, 57);
pub const MINTAKA_DISTANCE: Length = Length::from_meters(1200. * METERS_PER_LIGHT_YEAR);

pub const SADR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Sadr",
    Length::from_meters(150. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(12.11 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-4.54),
    Temperature::from_kelvin(5790.),
);

pub const SADR_RA: RightAscension = RightAscension::new(20, 22, 14);
pub const SADR_DEC: Declination = Declination::new(40, 15, 24);
pub const SADR_DISTANCE: Length = Length::from_meters(1800. * METERS_PER_LIGHT_YEAR);

pub const ELTANIN_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Eltanin",
    Length::from_meters(48.15 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.72 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.93),
    Temperature::from_kelvin(3930.),
);

pub const ELTANIN_RA: RightAscension = RightAscension::new(17, 56, 36);
pub const ELTANIN_DEC: Declination = Declination::new(51, 29, 20);
pub const ELTANIN_DISTANCE: Length = Length::from_meters(154.3 * METERS_PER_LIGHT_YEAR);

pub const SCHEDAR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Schedar",
    Length::from_meters(45.39 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(3.98 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-2.01),
    Temperature::from_kelvin(4552.),
);

pub const SCHEDAR_RA: RightAscension = RightAscension::new(0, 40, 30);
pub const SCHEDAR_DEC: Declination = Declination::new(56, 32, 14);
pub const SCHEDAR_DISTANCE: Length = Length::from_meters(228. * METERS_PER_LIGHT_YEAR);

pub const NAOS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Naos",
    Length::from_meters(20. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(56.1 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-6.23),
    Temperature::from_kelvin(40_000.),
);

pub const NAOS_RA: RightAscension = RightAscension::new(8, 3, 35);
pub const NAOS_DEC: Declination = Declination::new(-40, 0, 12);
pub const NAOS_DISTANCE: Length = Length::from_meters(1080. * METERS_PER_LIGHT_YEAR);

pub const ALMACH_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Almach",
    Length::from_meters(80. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(23.7 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.08),
    Temperature::from_kelvin(4250.),
);

pub const ALMACH_RA: RightAscension = RightAscension::new(2, 3, 54);
pub const ALMACH_DEC: Declination = Declination::new(42, 19, 47);
pub const ALMACH_DISTANCE: Length = Length::from_meters(390. * METERS_PER_LIGHT_YEAR);

pub const CAPH_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Caph",
    Length::from_meters(3.5 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.91 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(1.3),
    Temperature::from_kelvin(7079.),
);

pub const CAPH_RA: RightAscension = RightAscension::new(0, 9, 11);
pub const CAPH_DEC: Declination = Declination::new(59, 8, 59);
pub const CAPH_DISTANCE: Length = Length::from_meters(54.7 * METERS_PER_LIGHT_YEAR);

pub const IZAR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Izar",
    Length::from_meters(33. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(4.6 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.61),
    Temperature::from_kelvin(4550.),
);

pub const IZAR_RA: RightAscension = RightAscension::new(14, 44, 59);
pub const IZAR_DEC: Declination = Declination::new(27, 4, 27);
pub const IZAR_DISTANCE: Length = Length::from_meters(236. * METERS_PER_LIGHT_YEAR);

pub const ALPHA_LUPI_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alpha Lupi",
    Length::from_meters(0. * METERS_PER_SUN_RADIUS), //unknown
    Mass::from_kilograms(10.1 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-4.3),
    Temperature::from_kelvin(21_820.),
);

pub const ALPHA_LUPI_RA: RightAscension = RightAscension::new(14, 41, 56);
pub const ALPHA_LUPI_DEC: Declination = Declination::new(-47, 23, 18);
pub const ALPHA_LUPI_DISTANCE: Length = Length::from_meters(460. * METERS_PER_LIGHT_YEAR);

pub const EPSILON_CENTAURI_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Epsilon Centauri",
    Length::from_meters(0. * METERS_PER_SUN_RADIUS), //unknown
    Mass::from_kilograms(11.6 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.9),
    Temperature::from_kelvin(24_000.),
);

pub const EPSILON_CENTAURI_RA: RightAscension = RightAscension::new(13, 39, 53);
pub const EPSILON_CENTAURI_DEC: Declination = Declination::new(-53, 27, 59);
pub const EPSILON_CENTAURI_DISTANCE: Length = Length::from_meters(430. * METERS_PER_LIGHT_YEAR);

pub const DSCHUBBA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Dschubba",
    Length::from_meters(6.7 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(13. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.8),
    Temperature::from_kelvin(27_400.),
);

pub const DSCHUBBA_RA: RightAscension = RightAscension::new(16, 0, 20);
pub const DSCHUBBA_DEC: Declination = Declination::new(-22, 37, 18);
pub const DSCHUBBA_DISTANCE: Length = Length::from_meters(136. * METERS_PER_LIGHT_YEAR);

pub const LARAWAG_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Larawag",
    Length::from_meters(12.6 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.24 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.78),
    Temperature::from_kelvin(4560.),
);

pub const LARAWAG_RA: RightAscension = RightAscension::new(16, 50, 10);
pub const LARAWAG_DEC: Declination = Declination::new(-34, 17, 36);
pub const LARAWAG_DISTANCE: Length = Length::from_meters(63.7 * METERS_PER_LIGHT_YEAR);

pub const ETA_CENTAURI_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Eta Centauri",
    Length::from_meters(6.1 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(12.0 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-2.53),
    Temperature::from_kelvin(25_700.),
);

pub const ETA_CENTAURI_RA: RightAscension = RightAscension::new(14, 35, 30);
pub const ETA_CENTAURI_DEC: Declination = Declination::new(-42, 9, 28);
pub const ETA_CENTAURI_DISTANCE: Length = Length::from_meters(206. * METERS_PER_LIGHT_YEAR);

pub const MERAK_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Merak",
    Length::from_meters(3.021 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.7 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.61),
    Temperature::from_kelvin(9377.),
);

pub const MERAK_RA: RightAscension = RightAscension::new(11, 1, 50);
pub const MERAK_DEC: Declination = Declination::new(56, 22, 57);
pub const MERAK_DISTANCE: Length = Length::from_meters(79.7 * METERS_PER_LIGHT_YEAR);

pub const ANKAA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Ankaa",
    Length::from_meters(15. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(1.57 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.52),
    Temperature::from_kelvin(4436.),
);

pub const ANKAA_RA: RightAscension = RightAscension::new(0, 26, 17);
pub const ANKAA_DEC: Declination = Declination::new(-42, 18, 21);
pub const ANKAA_DISTANCE: Length = Length::from_meters(82. * METERS_PER_LIGHT_YEAR);

pub const GIRTAB_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Girtab",
    Length::from_meters(6.8 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(17. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.46),
    Temperature::from_kelvin(23_400.),
);

pub const GIRTAB_RA: RightAscension = RightAscension::new(17, 42, 29);
pub const GIRTAB_DEC: Declination = Declination::new(-39, 1, 48);
pub const GIRTAB_DISTANCE: Length = Length::from_meters(480. * METERS_PER_LIGHT_YEAR);

pub const ENIF_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Enif",
    Length::from_meters(211. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(7.07 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-4.142),
    Temperature::from_kelvin(3963.),
);

pub const ENIF_RA: RightAscension = RightAscension::new(21, 44, 11);
pub const ENIF_DEC: Declination = Declination::new(9, 52, 30);
pub const ENIF_DISTANCE: Length = Length::from_meters(690. * METERS_PER_LIGHT_YEAR);

pub const SCHEAT_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Scheat",
    Length::from_meters(95. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.1 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.41),
    Temperature::from_kelvin(3689.),
);

pub const SCHEAT_RA: RightAscension = RightAscension::new(23, 3, 46);
pub const SCHEAT_DEC: Declination = Declination::new(28, 4, 58);
pub const SCHEAT_DISTANCE: Length = Length::from_meters(196. * METERS_PER_LIGHT_YEAR);

pub const SABIK_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Sabik",
    Length::from_meters(0. * METERS_PER_SUN_RADIUS), //unknown
    Mass::from_kilograms(2.966 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.372),
    Temperature::from_kelvin(8900.),
);

pub const SABIK_RA: RightAscension = RightAscension::new(17, 10, 23);
pub const SABIK_DEC: Declination = Declination::new(-15, 43, 30);
pub const SABIK_DISTANCE: Length = Length::from_meters(88. * METERS_PER_LIGHT_YEAR);

pub const PHECDA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Phecda",
    Length::from_meters(3.04 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2.94 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.4),
    Temperature::from_kelvin(9355.),
);

pub const PHECDA_RA: RightAscension = RightAscension::new(11, 53, 50);
pub const PHECDA_DEC: Declination = Declination::new(53, 41, 41);
pub const PHECDA_DISTANCE: Length = Length::from_meters(83.2 * METERS_PER_LIGHT_YEAR);

pub const ALUDRA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Aludra",
    Length::from_meters(54. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(18.19 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-7.0),
    Temperature::from_kelvin(15_500.),
);

pub const ALUDRA_RA: RightAscension = RightAscension::new(7, 24, 6);
pub const ALUDRA_DEC: Declination = Declination::new(-29, 18, 11);
pub const ALUDRA_DISTANCE: Length = Length::from_meters(2000. * METERS_PER_LIGHT_YEAR);

pub const MARKEB_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Markeb",
    Length::from_meters(9.1 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(10.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.74),
    Temperature::from_kelvin(23_000.),
);

pub const MARKEB_RA: RightAscension = RightAscension::new(9, 22, 7);
pub const MARKEB_DEC: Declination = Declination::new(-55, 0, 38);
pub const MARKEB_DISTANCE: Length = Length::from_meters(570. * METERS_PER_LIGHT_YEAR);

pub const NAVI_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Navi",
    Length::from_meters(10. * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(13. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.98),
    Temperature::from_kelvin(25_000.),
);

pub const NAVI_RA: RightAscension = RightAscension::new(0, 56, 43);
pub const NAVI_DEC: Declination = Declination::new(60, 43, 0);
pub const NAVI_DISTANCE: Length = Length::from_meters(550. * METERS_PER_LIGHT_YEAR);

pub const MARKAB_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Markab",
    Length::from_meters(4.62 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(3.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.718),
    Temperature::from_kelvin(10_100.),
);

pub const MARKAB_RA: RightAscension = RightAscension::new(23, 4, 46);
pub const MARKAB_DEC: Declination = Declination::new(15, 12, 19);
pub const MARKAB_DISTANCE: Length = Length::from_meters(133. * METERS_PER_LIGHT_YEAR);

pub const ALJANAH_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Aljanah",
    Length::from_meters(10.82 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(2. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.74),
    Temperature::from_kelvin(4710.),
);

pub const ALJANAH_RA: RightAscension = RightAscension::new(20, 46, 13);
pub const ALJANAH_DEC: Declination = Declination::new(33, 58, 13);
pub const ALJANAH_DISTANCE: Length = Length::from_meters(72.7 * METERS_PER_LIGHT_YEAR);

pub const ACRAB_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Acrab",
    Length::from_meters(6.3 * METERS_PER_SUN_RADIUS),
    Mass::from_kilograms(15.0 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.92),
    Temperature::from_kelvin(28_000.),
);

pub const ACRAB_RA: RightAscension = RightAscension::new(16, 5, 26);
pub const ACRAB_DEC: Declination = Declination::new(-19, 48, 20);
pub const ACRAB_DISTANCE: Length = Length::from_meters(400. * METERS_PER_LIGHT_YEAR);
