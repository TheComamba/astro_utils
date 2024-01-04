use crate::{
    coordinates::{declination::Declination, right_ascension::RightAscension},
    stellar_properties::StellarProperties,
    units::{
        length::{Length, AU_PER_LIGHT_YEARS, AU_PER_SUN_RADII},
        luminosity::Luminosity,
        mass::{Mass, KILOGRAMS_PER_SOLAR_MASS},
        temperature::Temperature,
    },
};

pub const SUN_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Sun",
    Length::from_astronomical_units(AU_PER_SUN_RADII),
    Mass::from_kilograms(KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(4.83),
    Temperature::from_kelvin(5778.0),
);

const SIRIUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Sirius",
    Length::from_astronomical_units(1.711 * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.063 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(1.43),
    Temperature::from_kelvin(9940.),
);

const SIRIUS_RA: RightAscension = RightAscension::new(6, 45, 9);
const SIRIUS_DEC: Declination = Declination::new(-16, 42, 58);
const SIRIUS_DISTANCE: Length = Length::from_astronomical_units(8.6 * AU_PER_LIGHT_YEARS);

const CANOPUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Canopus",
    Length::from_astronomical_units(72. * AU_PER_SUN_RADII),
    Mass::from_kilograms(9. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-5.71),
    Temperature::from_kelvin(7400.),
);

const CANOPUS_RA: RightAscension = RightAscension::new(6, 23, 57);
const CANOPUS_DEC: Declination = Declination::new(-52, 41, 44);
const CANOPUS_DISTANCE: Length = Length::from_astronomical_units(310. * AU_PER_LIGHT_YEARS);

const ALPHA_CENTAURI_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alpha Centauri",
    Length::from_astronomical_units(1.2175 * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.0788 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(4.34),
    Temperature::from_kelvin(5790.),
);

const ALPHA_CENTAURI_RA: RightAscension = RightAscension::new(14, 39, 36);
const ALPHA_CENTAURI_DEC: Declination = Declination::new(-60, 50, 2);
const ALPHA_CENTAURI_DISTANCE: Length = Length::from_astronomical_units(4.34 * AU_PER_LIGHT_YEARS);

const ARCTURUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Arcturus",
    Length::from_astronomical_units(25.4 * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.08 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.3),
    Temperature::from_kelvin(4286.),
);

const ARCTURUS_RA: RightAscension = RightAscension::new(14, 15, 40);
const ARCTURUS_DEC: Declination = Declination::new(19, 10, 56);
const ARCTURUS_DISTANCE: Length = Length::from_astronomical_units(37. * AU_PER_LIGHT_YEARS);

const VEGA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Vega",
    Length::from_astronomical_units(2.362 * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.135 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.582),
    Temperature::from_kelvin(9602.),
);

const VEGA_RA: RightAscension = RightAscension::new(18, 36, 56);
const VEGA_DEC: Declination = Declination::new(38, 47, 1);
const VEGA_DISTANCE: Length = Length::from_astronomical_units(25. * AU_PER_LIGHT_YEARS);

const CAPELLA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Capella",
    Length::from_astronomical_units(11.98 * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.5687 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.296),
    Temperature::from_kelvin(4970.),
);

const CAPELLA_RA: RightAscension = RightAscension::new(5, 16, 41);
const CAPELLA_DEC: Declination = Declination::new(45, 59, 53);
const CAPELLA_DISTANCE: Length = Length::from_astronomical_units(43.38 * AU_PER_LIGHT_YEARS);

const RIGEL_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Rigel",
    Length::from_astronomical_units(78.9 * AU_PER_SUN_RADII),
    Mass::from_kilograms(21. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-7.84),
    Temperature::from_kelvin(12_100.),
);

const RIGEL_RA: RightAscension = RightAscension::new(5, 14, 32);
const RIGEL_DEC: Declination = Declination::new(-8, 12, 6);
const RIGEL_DISTANCE: Length = Length::from_astronomical_units(860. * AU_PER_LIGHT_YEARS);

const PROCYON_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Procyon",
    Length::from_astronomical_units(2.048 * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.499 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(2.66),
    Temperature::from_kelvin(6530.),
);

const PROCYON_RA: RightAscension = RightAscension::new(7, 39, 18);
const PROCYON_DEC: Declination = Declination::new(5, 13, 30);
const PROCYON_DISTANCE: Length = Length::from_astronomical_units(11.46 * AU_PER_LIGHT_YEARS);

const ACHERNAR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Achernar",
    Length::from_astronomical_units(6.78 * AU_PER_SUN_RADII),
    Mass::from_kilograms(6.0 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.46),
    Temperature::from_kelvin(14_000.),
);

const ACHERNAR_RA: RightAscension = RightAscension::new(1, 37, 43);
const ACHERNAR_DEC: Declination = Declination::new(-57, 14, 12);
const ACHERNAR_DISTANCE: Length = Length::from_astronomical_units(139. * AU_PER_LIGHT_YEARS);

const BETELGEUSE_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Betelgeuse",
    Length::from_astronomical_units(887. * AU_PER_SUN_RADII),
    Mass::from_kilograms(16.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-5.85),
    Temperature::from_kelvin(3600.),
);

const BETELGEUSE_RA: RightAscension = RightAscension::new(5, 55, 10);
const BETELGEUSE_DEC: Declination = Declination::new(7, 24, 25);
const BETELGEUSE_DISTANCE: Length = Length::from_astronomical_units(548. * AU_PER_LIGHT_YEARS);

const HADAR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Hadar",
    Length::from_astronomical_units(9. * AU_PER_SUN_RADII),
    Mass::from_kilograms(12.02 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-4.9),
    Temperature::from_kelvin(25_000.),
);

const HADAR_RA: RightAscension = RightAscension::new(14, 3, 49);
const HADAR_DEC: Declination = Declination::new(-60, 22, 23);
const HADAR_DISTANCE: Length = Length::from_astronomical_units(390. * AU_PER_LIGHT_YEARS);

const ALTAIR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Altair",
    Length::from_astronomical_units(1.63 * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.86 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(2.22),
    Temperature::from_kelvin(7670.),
);

const ALTAIR_RA: RightAscension = RightAscension::new(19, 50, 47);
const ALTAIR_DEC: Declination = Declination::new(8, 52, 6);
const ALTAIR_DISTANCE: Length = Length::from_astronomical_units(16.73 * AU_PER_LIGHT_YEARS);

const ACRUX_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Acrux",
    Length::from_astronomical_units(7.8 * AU_PER_SUN_RADII),
    Mass::from_kilograms(17.8 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.77),
    Temperature::from_kelvin(24_000.),
);

const ACRUX_RA: RightAscension = RightAscension::new(12, 26, 36);
const ACRUX_DEC: Declination = Declination::new(-63, 5, 57);
const ACRUX_DISTANCE: Length = Length::from_astronomical_units(320. * AU_PER_LIGHT_YEARS);

const ALDEBARAN_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Aldebaran",
    Length::from_astronomical_units(45.1 * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.16 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.641),
    Temperature::from_kelvin(3900.),
);

const ALDEBARAN_RA: RightAscension = RightAscension::new(4, 35, 55);
const ALDEBARAN_DEC: Declination = Declination::new(16, 30, 33);
const ALDEBARAN_DISTANCE: Length = Length::from_astronomical_units(65.3 * AU_PER_LIGHT_YEARS);

const ANTARES_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Antares",
    Length::from_astronomical_units(680. * AU_PER_SUN_RADII),
    Mass::from_kilograms(13.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-5.28),
    Temperature::from_kelvin(3660.),
);

const ANTARES_RA: RightAscension = RightAscension::new(16, 29, 24);
const ANTARES_DEC: Declination = Declination::new(-26, 25, 55);
const ANTARES_DISTANCE: Length = Length::from_astronomical_units(550. * AU_PER_LIGHT_YEARS);

const SPICA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Spica",
    Length::from_astronomical_units(7.47 * AU_PER_SUN_RADII),
    Mass::from_kilograms(11.43 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.55),
    Temperature::from_kelvin(22_300.),
);

const SPICA_RA: RightAscension = RightAscension::new(13, 25, 12);
const SPICA_DEC: Declination = Declination::new(-11, 9, 41);
const SPICA_DISTANCE: Length = Length::from_astronomical_units(260. * AU_PER_LIGHT_YEARS);

const POLLUX_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Pollux",
    Length::from_astronomical_units(9.06 * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.91 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(1.08),
    Temperature::from_kelvin(4586.),
);

const POLLUX_RA: RightAscension = RightAscension::new(7, 45, 19);
const POLLUX_DEC: Declination = Declination::new(28, 1, 34);
const POLLUX_DISTANCE: Length = Length::from_astronomical_units(33.78 * AU_PER_LIGHT_YEARS);

const FORMALHAUT_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Formalhaut",
    Length::from_astronomical_units(1.842 * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.92 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(1.72),
    Temperature::from_kelvin(8590.),
);

const FORMALHAUT_RA: RightAscension = RightAscension::new(22, 57, 39);
const FORMALHAUT_DEC: Declination = Declination::new(-29, 37, 20);
const FORMALHAUT_DISTANCE: Length = Length::from_astronomical_units(25.13 * AU_PER_LIGHT_YEARS);

const DENEB_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Deneb",
    Length::from_astronomical_units(203. * AU_PER_SUN_RADII),
    Mass::from_kilograms(19. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-8.38),
    Temperature::from_kelvin(8515.),
);

const DENEB_RA: RightAscension = RightAscension::new(20, 41, 26);
const DENEB_DEC: Declination = Declination::new(45, 16, 49);
const DENEB_DISTANCE: Length = Length::from_astronomical_units(2615. * AU_PER_LIGHT_YEARS);

const MIMOSA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Mimosa",
    Length::from_astronomical_units(8.4 * AU_PER_SUN_RADII),
    Mass::from_kilograms(16. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.92),
    Temperature::from_kelvin(27_000.),
);

const MIMOSA_RA: RightAscension = RightAscension::new(12, 47, 43);
const MIMOSA_DEC: Declination = Declination::new(-59, 41, 20);
const MIMOSA_DISTANCE: Length = Length::from_astronomical_units(280. * AU_PER_LIGHT_YEARS);

const REGULUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Regulus",
    Length::from_astronomical_units(4.35 * AU_PER_SUN_RADII),
    Mass::from_kilograms(3.8 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.57),
    Temperature::from_kelvin(11_668.),
);

const REGULUS_RA: RightAscension = RightAscension::new(10, 8, 22);
const REGULUS_DEC: Declination = Declination::new(11, 58, 2);
const REGULUS_DISTANCE: Length = Length::from_astronomical_units(79.3 * AU_PER_LIGHT_YEARS);

const ADHARA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Adhara",
    Length::from_astronomical_units(13.9 * AU_PER_SUN_RADII),
    Mass::from_kilograms(12.6 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-4.8),
    Temperature::from_kelvin(22_900.),
);

const ADHARA_RA: RightAscension = RightAscension::new(6, 58, 38);
const ADHARA_DEC: Declination = Declination::new(-28, 58, 19);
const ADHARA_DISTANCE: Length = Length::from_astronomical_units(430. * AU_PER_LIGHT_YEARS);

const SHAULA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Shaula",
    Length::from_astronomical_units(8.8 * AU_PER_SUN_RADII),
    Mass::from_kilograms(10.4 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.7),
    Temperature::from_kelvin(25_000.),
);

const SHAULA_RA: RightAscension = RightAscension::new(17, 33, 37);
const SHAULA_DEC: Declination = Declination::new(-37, 6, 14);
const SHAULA_DISTANCE: Length = Length::from_astronomical_units(570. * AU_PER_LIGHT_YEARS);

const CASTOR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Castor",
    Length::from_astronomical_units(2.9 * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.37 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.986),
    Temperature::from_kelvin(10_286.),
);

const CASTOR_RA: RightAscension = RightAscension::new(7, 34, 36);
const CASTOR_DEC: Declination = Declination::new(31, 53, 18);
const CASTOR_DISTANCE: Length = Length::from_astronomical_units(51. * AU_PER_LIGHT_YEARS);

const GACRUX_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Gacrux",
    Length::from_astronomical_units(120. * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.52),
    Temperature::from_kelvin(3689.),
);

const GACRUX_RA: RightAscension = RightAscension::new(12, 31, 10);
const GACRUX_DEC: Declination = Declination::new(-57, 6, 48);
const GACRUX_DISTANCE: Length = Length::from_astronomical_units(88.6 * AU_PER_LIGHT_YEARS);

const BELLATRIX_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Bellatrix",
    Length::from_astronomical_units(5.75 * AU_PER_SUN_RADII),
    Mass::from_kilograms(7.7 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-2.78),
    Temperature::from_kelvin(21_800.),
);

const BELLATRIX_RA: RightAscension = RightAscension::new(5, 25, 8);
const BELLATRIX_DEC: Declination = Declination::new(6, 20, 59);
const BELLATRIX_DISTANCE: Length = Length::from_astronomical_units(250. * AU_PER_LIGHT_YEARS);

const ELNATH_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Elnath",
    Length::from_astronomical_units(4.2 * AU_PER_SUN_RADII),
    Mass::from_kilograms(5.0 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.42),
    Temperature::from_kelvin(13_824.),
);

const ELNATH_RA: RightAscension = RightAscension::new(5, 26, 18);
const ELNATH_DEC: Declination = Declination::new(28, 36, 27);
const ELNATH_DISTANCE: Length = Length::from_astronomical_units(134. * AU_PER_LIGHT_YEARS);

const MIAPLACIDUS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Miaplacidus",
    Length::from_astronomical_units(6.8 * AU_PER_SUN_RADII),
    Mass::from_kilograms(3.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.03),
    Temperature::from_kelvin(8866.),
);

const MIAPLACIDUS_RA: RightAscension = RightAscension::new(9, 13, 12);
const MIAPLACIDUS_DEC: Declination = Declination::new(-69, 43, 2);
const MIAPLACIDUS_DISTANCE: Length = Length::from_astronomical_units(113.2 * AU_PER_LIGHT_YEARS);

const ALNILAM_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alnilam",
    Length::from_astronomical_units(42. * AU_PER_SUN_RADII),
    Mass::from_kilograms(64.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-6.89),
    Temperature::from_kelvin(27_000.),
);

const ALNILAM_RA: RightAscension = RightAscension::new(5, 36, 13);
const ALNILAM_DEC: Declination = Declination::new(-1, 12, 7);
const ALNILAM_DISTANCE: Length = Length::from_astronomical_units(2000. * AU_PER_LIGHT_YEARS);

const GAMMA_VELORUM_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Gamma Velorum",
    Length::from_astronomical_units(17. * AU_PER_SUN_RADII),
    Mass::from_kilograms(28.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-5.63),
    Temperature::from_kelvin(35_000.),
);

const GAMMA_VELORUM_RA: RightAscension = RightAscension::new(8, 9, 32);
const GAMMA_VELORUM_DEC: Declination = Declination::new(-47, 20, 12);
const GAMMA_VELORUM_DISTANCE: Length = Length::from_astronomical_units(1236. * AU_PER_LIGHT_YEARS);

const ALNAIR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alnair",
    Length::from_astronomical_units(3.4 * AU_PER_SUN_RADII),
    Mass::from_kilograms(4. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.72),
    Temperature::from_kelvin(13_920.),
);

const ALNAIR_RA: RightAscension = RightAscension::new(22, 8, 14);
const ALNAIR_DEC: Declination = Declination::new(-46, 57, 40);
const ALNAIR_DISTANCE: Length = Length::from_astronomical_units(101. * AU_PER_LIGHT_YEARS);

const ALNITAK_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alnitak",
    Length::from_astronomical_units(20. * AU_PER_SUN_RADII),
    Mass::from_kilograms(33.0 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-6.0),
    Temperature::from_kelvin(29_500.),
);

const ALNITAK_RA: RightAscension = RightAscension::new(5, 40, 46);
const ALNITAK_DEC: Declination = Declination::new(-1, 56, 34);
const ALNITAK_DISTANCE: Length = Length::from_astronomical_units(1260. * AU_PER_LIGHT_YEARS);

const ALIOTH_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alioth",
    Length::from_astronomical_units(4.14 * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.91 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.2),
    Temperature::from_kelvin(9_020.),
);

const ALIOTH_RA: RightAscension = RightAscension::new(12, 54, 2);
const ALIOTH_DEC: Declination = Declination::new(55, 57, 36);
const ALIOTH_DISTANCE: Length = Length::from_astronomical_units(82.6 * AU_PER_LIGHT_YEARS);

const DUBHE_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Dubhe",
    Length::from_astronomical_units(17.03 * AU_PER_SUN_RADII),
    Mass::from_kilograms(3.44 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.1),
    Temperature::from_kelvin(5012.),
);

const DUBHE_RA: RightAscension = RightAscension::new(11, 3, 44);
const DUBHE_DEC: Declination = Declination::new(61, 45, 4);
const DUBHE_DISTANCE: Length = Length::from_astronomical_units(123. * AU_PER_LIGHT_YEARS);

const MIRFAK_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Mirfak",
    Length::from_astronomical_units(68. * AU_PER_SUN_RADII),
    Mass::from_kilograms(8.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-5.1),
    Temperature::from_kelvin(6350.),
);

const MIRFAK_RA: RightAscension = RightAscension::new(3, 24, 19);
const MIRFAK_DEC: Declination = Declination::new(49, 51, 40);
const MIRFAK_DISTANCE: Length = Length::from_astronomical_units(510. * AU_PER_LIGHT_YEARS);

const WEZEN_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Wezen",
    Length::from_astronomical_units(215. * AU_PER_SUN_RADII),
    Mass::from_kilograms(16.9 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-6.86),
    Temperature::from_kelvin(6390.),
);

const WEZEN_RA: RightAscension = RightAscension::new(7, 8, 23);
const WEZEN_DEC: Declination = Declination::new(-26, 23, 36);
const WEZEN_DISTANCE: Length = Length::from_astronomical_units(1600. * AU_PER_LIGHT_YEARS);

const SARGAS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Sargas",
    Length::from_astronomical_units(26.3 * AU_PER_SUN_RADII),
    Mass::from_kilograms(3.1 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-2.71),
    Temperature::from_kelvin(6294.),
);

const SARGAS_RA: RightAscension = RightAscension::new(17, 37, 19);
const SARGAS_DEC: Declination = Declination::new(-42, 59, 52);
const SARGAS_DISTANCE: Length = Length::from_astronomical_units(329. * AU_PER_LIGHT_YEARS);

const KAUS_AUSTRALIS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Kaus Australis",
    Length::from_astronomical_units(6.8 * AU_PER_SUN_RADII),
    Mass::from_kilograms(3.515 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.41),
    Temperature::from_kelvin(9960.),
);

const KAUS_AUSTRALIS_RA: RightAscension = RightAscension::new(18, 24, 10);
const KAUS_AUSTRALIS_DEC: Declination = Declination::new(-34, 23, 5);
const KAUS_AUSTRALIS_DISTANCE: Length = Length::from_astronomical_units(143. * AU_PER_LIGHT_YEARS);

const AVIOR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Avior",
    Length::from_astronomical_units(0. * AU_PER_SUN_RADII), //Unknown
    Mass::from_kilograms(10.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-4.47),
    Temperature::from_kelvin(3.523),
);

const AVIOR_RA: RightAscension = RightAscension::new(8, 22, 31);
const AVIOR_DEC: Declination = Declination::new(-59, 30, 34);
const AVIOR_DISTANCE: Length = Length::from_astronomical_units(610. * AU_PER_LIGHT_YEARS);

const ALKAID_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alkaid",
    Length::from_astronomical_units(3.4 * AU_PER_SUN_RADII),
    Mass::from_kilograms(6.1 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.67),
    Temperature::from_kelvin(15_540.),
);

const ALKAID_RA: RightAscension = RightAscension::new(13, 47, 32);
const ALKAID_DEC: Declination = Declination::new(49, 18, 48);
const ALKAID_DISTANCE: Length = Length::from_astronomical_units(103.9 * AU_PER_LIGHT_YEARS);

const MENKALINAN_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Menkalinan",
    Length::from_astronomical_units(2.77 * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.389 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.55),
    Temperature::from_kelvin(9350.),
);

const MENKALINAN_RA: RightAscension = RightAscension::new(5, 59, 32);
const MENKALINAN_DEC: Declination = Declination::new(44, 56, 51);
const MENKALINAN_DISTANCE: Length = Length::from_astronomical_units(81.1 * AU_PER_LIGHT_YEARS);

const ATRIA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Atria",
    Length::from_astronomical_units(143. * AU_PER_SUN_RADII),
    Mass::from_kilograms(7. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.68),
    Temperature::from_kelvin(4150.),
);

const ATRIA_RA: RightAscension = RightAscension::new(16, 48, 40);
const ATRIA_DEC: Declination = Declination::new(-69, 1, 40);
const ATRIA_DISTANCE: Length = Length::from_astronomical_units(391. * AU_PER_LIGHT_YEARS);

const ALHENA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alhena",
    Length::from_astronomical_units(3.3 * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.81 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.68),
    Temperature::from_kelvin(9260.),
);

const ALHENA_RA: RightAscension = RightAscension::new(6, 37, 43);
const ALHENA_DEC: Declination = Declination::new(16, 23, 57);
const ALHENA_DISTANCE: Length = Length::from_astronomical_units(109. * AU_PER_LIGHT_YEARS);

const PEACOCK_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Peacock",
    Length::from_astronomical_units(4.83 * AU_PER_SUN_RADII),
    Mass::from_kilograms(5.91 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.762),
    Temperature::from_kelvin(17_711.),
);

const PEACOCK_RA: RightAscension = RightAscension::new(20, 25, 39);
const PEACOCK_DEC: Declination = Declination::new(-56, 44, 6);
const PEACOCK_DISTANCE: Length = Length::from_astronomical_units(179. * AU_PER_LIGHT_YEARS);

const ALSEPHINA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alsephina",
    Length::from_astronomical_units(2.9 * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.43 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.02),
    Temperature::from_kelvin(9440.),
);

const ALSEPHINA_RA: RightAscension = RightAscension::new(8, 44, 42);
const ALSEPHINA_DEC: Declination = Declination::new(-54, 42, 32);
const ALSEPHINA_DISTANCE: Length = Length::from_astronomical_units(80.6 * AU_PER_LIGHT_YEARS);

const MIRZAM_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Mirzam",
    Length::from_astronomical_units(9.7 * AU_PER_SUN_RADII),
    Mass::from_kilograms(13.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-4.1),
    Temperature::from_kelvin(25_000.),
);

const MIRZAM_RA: RightAscension = RightAscension::new(6, 22, 42);
const MIRZAM_DEC: Declination = Declination::new(-17, 57, 21);
const MIRZAM_DISTANCE: Length = Length::from_astronomical_units(490. * AU_PER_LIGHT_YEARS);

const ALPHARD_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alphard",
    Length::from_astronomical_units(50.5 * AU_PER_SUN_RADII),
    Mass::from_kilograms(3.03 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.69),
    Temperature::from_kelvin(4120.),
);

const ALPHARD_RA: RightAscension = RightAscension::new(9, 27, 35);
const ALPHARD_DEC: Declination = Declination::new(-8, 39, 30);
const ALPHARD_DISTANCE: Length = Length::from_astronomical_units(177. * AU_PER_LIGHT_YEARS);

const POLARIS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Polaris",
    Length::from_astronomical_units(37.5 * AU_PER_SUN_RADII),
    Mass::from_kilograms(5.4 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.6),
    Temperature::from_kelvin(6015.),
);

const POLARIS_RA: RightAscension = RightAscension::new(2, 31, 49);
const POLARIS_DEC: Declination = Declination::new(89, 15, 51);
const POLARIS_DISTANCE: Length = Length::from_astronomical_units(380. * AU_PER_LIGHT_YEARS);

const HAMAL_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Hamal",
    Length::from_astronomical_units(14.9 * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.47),
    Temperature::from_kelvin(4480.),
);

const HAMAL_RA: RightAscension = RightAscension::new(2, 7, 10);
const HAMAL_DEC: Declination = Declination::new(23, 27, 45);
const HAMAL_DISTANCE: Length = Length::from_astronomical_units(65.8 * AU_PER_LIGHT_YEARS);

const ALGIEBA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Algieba",
    Length::from_astronomical_units(31.88 * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.23 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.27),
    Temperature::from_kelvin(4470.),
);

const ALGIEBA_RA: RightAscension = RightAscension::new(10, 19, 58);
const ALGIEBA_DEC: Declination = Declination::new(19, 50, 29);
const ALGIEBA_DISTANCE: Length = Length::from_astronomical_units(130. * AU_PER_LIGHT_YEARS);

const DIPHDA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Diphda",
    Length::from_astronomical_units(16.78 * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.8 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.13),
    Temperature::from_kelvin(4797.),
);

const DIPHDA_RA: RightAscension = RightAscension::new(0, 43, 35);
const DIPHDA_DEC: Declination = Declination::new(-17, 59, 12);
const DIPHDA_DISTANCE: Length = Length::from_astronomical_units(96.3 * AU_PER_LIGHT_YEARS);

const MIZAR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Mizar",
    Length::from_astronomical_units(2.4 * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.2 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.32),
    Temperature::from_kelvin(9000.),
);

const MIZAR_RA: RightAscension = RightAscension::new(13, 23, 56);
const MIZAR_DEC: Declination = Declination::new(54, 55, 31);
const MIZAR_DISTANCE: Length = Length::from_astronomical_units(82.9 * AU_PER_LIGHT_YEARS);

const NUNKI_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Nunki",
    Length::from_astronomical_units(4.5 * AU_PER_SUN_RADII),
    Mass::from_kilograms(7.8 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-2.17),
    Temperature::from_kelvin(18_890.),
);

const NUNKI_RA: RightAscension = RightAscension::new(18, 55, 16);
const NUNKI_DEC: Declination = Declination::new(-26, 17, 49);
const NUNKI_DISTANCE: Length = Length::from_astronomical_units(228. * AU_PER_LIGHT_YEARS);

const MENKENT_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Menkent",
    Length::from_astronomical_units(10.6 * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.27 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.87),
    Temperature::from_kelvin(4980.),
);

const MENKENT_RA: RightAscension = RightAscension::new(14, 6, 41);
const MENKENT_DEC: Declination = Declination::new(-36, 22, 11);
const MENKENT_DISTANCE: Length = Length::from_astronomical_units(28.8 * AU_PER_LIGHT_YEARS);

const MIRACH_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Mirach",
    Length::from_astronomical_units(100. * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.49 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.76),
    Temperature::from_kelvin(3842.),
);

const MIRACH_RA: RightAscension = RightAscension::new(1, 9, 44);
const MIRACH_DEC: Declination = Declination::new(35, 37, 14);
const MIRACH_DISTANCE: Length = Length::from_astronomical_units(197. * AU_PER_LIGHT_YEARS);

const ALPHERATZ_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alpheratz",
    Length::from_astronomical_units(2.7 * AU_PER_SUN_RADII),
    Mass::from_kilograms(3.8 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(2.00),
    Temperature::from_kelvin(13_800.),
);

const ALPHERATZ_RA: RightAscension = RightAscension::new(0, 8, 23);
const ALPHERATZ_DEC: Declination = Declination::new(29, 5, 26);
const ALPHERATZ_DISTANCE: Length = Length::from_astronomical_units(97.0 * AU_PER_LIGHT_YEARS);

const RASALHAGUE_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Rasalhague",
    Length::from_astronomical_units(2.6 * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.4 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(1.248),
    Temperature::from_kelvin(8000.),
);

const RASALHAGUE_RA: RightAscension = RightAscension::new(17, 34, 56);
const RASALHAGUE_DEC: Declination = Declination::new(12, 33, 37);
const RASALHAGUE_DISTANCE: Length = Length::from_astronomical_units(48.6 * AU_PER_LIGHT_YEARS);

const KOCHAB_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Kochab",
    Length::from_astronomical_units(42.06 * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.2 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.83),
    Temperature::from_kelvin(4030.),
);

const KOCHAB_RA: RightAscension = RightAscension::new(14, 50, 42);
const KOCHAB_DEC: Declination = Declination::new(74, 9, 20);
const KOCHAB_DISTANCE: Length = Length::from_astronomical_units(130.9 * AU_PER_LIGHT_YEARS);

const SAIPH_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Saiph",
    Length::from_astronomical_units(22.2 * AU_PER_SUN_RADII),
    Mass::from_kilograms(15.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-6.1),
    Temperature::from_kelvin(26_500.),
);

const SAIPH_RA: RightAscension = RightAscension::new(5, 47, 45);
const SAIPH_DEC: Declination = Declination::new(-9, 40, 11);
const SAIPH_DISTANCE: Length = Length::from_astronomical_units(650. * AU_PER_LIGHT_YEARS);

const DENEBOLA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Denebola",
    Length::from_astronomical_units(1.728 * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.78 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(2.14),
    Temperature::from_kelvin(8500.),
);

const DENEBOLA_RA: RightAscension = RightAscension::new(11, 49, 3);
const DENEBOLA_DEC: Declination = Declination::new(14, 34, 19);
const DENEBOLA_DISTANCE: Length = Length::from_astronomical_units(35.9 * AU_PER_LIGHT_YEARS);

const ALGOL_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Algol",
    Length::from_astronomical_units(2.73 * AU_PER_SUN_RADII),
    Mass::from_kilograms(3.17 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.07),
    Temperature::from_kelvin(13_000.),
);

const ALGOL_RA: RightAscension = RightAscension::new(3, 8, 10);
const ALGOL_DEC: Declination = Declination::new(40, 57, 20);
const ALGOL_DISTANCE: Length = Length::from_astronomical_units(90. * AU_PER_LIGHT_YEARS);

const TIAKI_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Tiaki",
    Length::from_astronomical_units(180. * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.4 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.61),
    Temperature::from_kelvin(3480.),
);

const TIAKI_RA: RightAscension = RightAscension::new(22, 42, 40);
const TIAKI_DEC: Declination = Declination::new(-46, 53, 4);
const TIAKI_DISTANCE: Length = Length::from_astronomical_units(177. * AU_PER_LIGHT_YEARS);

const MUHLIFAIN_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Muhlifain",
    Length::from_astronomical_units(0. * AU_PER_SUN_RADII), //Unknown
    Mass::from_kilograms(2.91 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.81),
    Temperature::from_kelvin(9082.),
);

const MUHLIFAIN_RA: RightAscension = RightAscension::new(12, 41, 31);
const MUHLIFAIN_DEC: Declination = Declination::new(-48, 57, 35);
const MUHLIFAIN_DISTANCE: Length = Length::from_astronomical_units(130. * AU_PER_LIGHT_YEARS);

const ASPIDISKE_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Aspidiske",
    Length::from_astronomical_units(43. * AU_PER_SUN_RADII),
    Mass::from_kilograms(7.4 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-5.1),
    Temperature::from_kelvin(7500.),
);

const ASPIDISKE_RA: RightAscension = RightAscension::new(9, 17, 5);
const ASPIDISKE_DEC: Declination = Declination::new(-59, 16, 30);
const ASPIDISKE_DISTANCE: Length = Length::from_astronomical_units(690. * AU_PER_LIGHT_YEARS);

const SUHAIL_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Suhail",
    Length::from_astronomical_units(210. * AU_PER_SUN_RADII),
    Mass::from_kilograms(7. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.99),
    Temperature::from_kelvin(3900.),
);

const SUHAIL_RA: RightAscension = RightAscension::new(9, 7, 60);
const SUHAIL_DEC: Declination = Declination::new(-43, 25, 57);
const SUHAIL_DISTANCE: Length = Length::from_astronomical_units(545. * AU_PER_LIGHT_YEARS);

const ALPHECCA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alphecca",
    Length::from_astronomical_units(3. * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.58 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.16),
    Temperature::from_kelvin(9700.),
);

const ALPHECCA_RA: RightAscension = RightAscension::new(15, 34, 41);
const ALPHECCA_DEC: Declination = Declination::new(26, 42, 53);
const ALPHECCA_DISTANCE: Length = Length::from_astronomical_units(75. * AU_PER_LIGHT_YEARS);

const MINTAKA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Mintaka",
    Length::from_astronomical_units(16.5 * AU_PER_SUN_RADII),
    Mass::from_kilograms(24. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-5.8),
    Temperature::from_kelvin(29_500.),
);

const MINTAKA_RA: RightAscension = RightAscension::new(5, 32, 0);
const MINTAKA_DEC: Declination = Declination::new(0, 17, 57);
const MINTAKA_DISTANCE: Length = Length::from_astronomical_units(1200. * AU_PER_LIGHT_YEARS);

const SADR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Sadr",
    Length::from_astronomical_units(150. * AU_PER_SUN_RADII),
    Mass::from_kilograms(12.11 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-4.54),
    Temperature::from_kelvin(5790.),
);

const SADR_RA: RightAscension = RightAscension::new(20, 22, 14);
const SADR_DEC: Declination = Declination::new(40, 15, 24);
const SADR_DISTANCE: Length = Length::from_astronomical_units(1800. * AU_PER_LIGHT_YEARS);

const ELTANIN_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Eltanin",
    Length::from_astronomical_units(48.15 * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.72 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.93),
    Temperature::from_kelvin(3930.),
);

const ELTANIN_RA: RightAscension = RightAscension::new(17, 56, 36);
const ELTANIN_DEC: Declination = Declination::new(51, 29, 20);
const ELTANIN_DISTANCE: Length = Length::from_astronomical_units(154.3 * AU_PER_LIGHT_YEARS);

const SCHEDAR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Schedar",
    Length::from_astronomical_units(45.39 * AU_PER_SUN_RADII),
    Mass::from_kilograms(3.98 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-2.01),
    Temperature::from_kelvin(4552.),
);

const SCHEDAR_RA: RightAscension = RightAscension::new(0, 40, 30);
const SCHEDAR_DEC: Declination = Declination::new(56, 32, 14);
const SCHEDAR_DISTANCE: Length = Length::from_astronomical_units(228. * AU_PER_LIGHT_YEARS);

const NAOS_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Naos",
    Length::from_astronomical_units(20. * AU_PER_SUN_RADII),
    Mass::from_kilograms(56.1 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-6.23),
    Temperature::from_kelvin(40_000.),
);

const NAOS_RA: RightAscension = RightAscension::new(8, 3, 35);
const NAOS_DEC: Declination = Declination::new(-40, 0, 12);
const NAOS_DISTANCE: Length = Length::from_astronomical_units(1080. * AU_PER_LIGHT_YEARS);

const ALMACH_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Almach",
    Length::from_astronomical_units(80. * AU_PER_SUN_RADII),
    Mass::from_kilograms(23.7 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.08),
    Temperature::from_kelvin(4250.),
);

const ALMACH_RA: RightAscension = RightAscension::new(2, 3, 54);
const ALMACH_DEC: Declination = Declination::new(42, 19, 47);
const ALMACH_DISTANCE: Length = Length::from_astronomical_units(390. * AU_PER_LIGHT_YEARS);

const CAPH_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Caph",
    Length::from_astronomical_units(3.5 * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.91 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(1.3),
    Temperature::from_kelvin(7079.),
);

const CAPH_RA: RightAscension = RightAscension::new(0, 9, 11);
const CAPH_DEC: Declination = Declination::new(59, 8, 59);
const CAPH_DISTANCE: Length = Length::from_astronomical_units(54.7 * AU_PER_LIGHT_YEARS);

const IZAR_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Izar",
    Length::from_astronomical_units(33. * AU_PER_SUN_RADII),
    Mass::from_kilograms(4.6 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.61),
    Temperature::from_kelvin(4550.),
);

const IZAR_RA: RightAscension = RightAscension::new(14, 44, 59);
const IZAR_DEC: Declination = Declination::new(27, 4, 27);
const IZAR_DISTANCE: Length = Length::from_astronomical_units(236. * AU_PER_LIGHT_YEARS);

const ALPHA_LUPI_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Alpha Lupi",
    Length::from_astronomical_units(0. * AU_PER_SUN_RADII), //unknown
    Mass::from_kilograms(10.1 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-4.3),
    Temperature::from_kelvin(21_820.),
);

const ALPHA_LUPI_RA: RightAscension = RightAscension::new(14, 41, 56);
const ALPHA_LUPI_DEC: Declination = Declination::new(-47, 23, 18);
const ALPHA_LUPI_DISTANCE: Length = Length::from_astronomical_units(460. * AU_PER_LIGHT_YEARS);

const EPSILON_CENTAURI_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Epsilon Centauri",
    Length::from_astronomical_units(0. * AU_PER_SUN_RADII), //unknown
    Mass::from_kilograms(11.6 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.9),
    Temperature::from_kelvin(24_000.),
);

const EPSILON_CENTAURI_RA: RightAscension = RightAscension::new(13, 39, 53);
const EPSILON_CENTAURI_DEC: Declination = Declination::new(-53, 27, 59);
const EPSILON_CENTAURI_DISTANCE: Length =
    Length::from_astronomical_units(430. * AU_PER_LIGHT_YEARS);

const DSCHUBBA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Dschubba",
    Length::from_astronomical_units(6.7 * AU_PER_SUN_RADII),
    Mass::from_kilograms(13. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.8),
    Temperature::from_kelvin(27_400.),
);

const DSCHUBBA_RA: RightAscension = RightAscension::new(16, 0, 20);
const DSCHUBBA_DEC: Declination = Declination::new(-22, 37, 18);
const DSCHUBBA_DISTANCE: Length = Length::from_astronomical_units(136. * AU_PER_LIGHT_YEARS);

const LARAWAG_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Larawag",
    Length::from_astronomical_units(12.6 * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.24 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.78),
    Temperature::from_kelvin(4560.),
);

const LARAWAG_RA: RightAscension = RightAscension::new(16, 50, 10);
const LARAWAG_DEC: Declination = Declination::new(-34, 17, 36);
const LARAWAG_DISTANCE: Length = Length::from_astronomical_units(63.7 * AU_PER_LIGHT_YEARS);

const ETA_CENTAURI_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Eta Centauri",
    Length::from_astronomical_units(6.1 * AU_PER_SUN_RADII),
    Mass::from_kilograms(12.0 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-2.53),
    Temperature::from_kelvin(25_700.),
);

const ETA_CENTAURI_RA: RightAscension = RightAscension::new(14, 35, 30);
const ETA_CENTAURI_DEC: Declination = Declination::new(-42, 9, 28);
const ETA_CENTAURI_DISTANCE: Length = Length::from_astronomical_units(206. * AU_PER_LIGHT_YEARS);

const MERAK_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Merak",
    Length::from_astronomical_units(3.021 * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.7 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.61),
    Temperature::from_kelvin(9377.),
);

const MERAK_RA: RightAscension = RightAscension::new(11, 1, 50);
const MERAK_DEC: Declination = Declination::new(56, 22, 57);
const MERAK_DISTANCE: Length = Length::from_astronomical_units(79.7 * AU_PER_LIGHT_YEARS);

const ANKAA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Ankaa",
    Length::from_astronomical_units(15. * AU_PER_SUN_RADII),
    Mass::from_kilograms(1.57 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.52),
    Temperature::from_kelvin(4436.),
);

const ANKAA_RA: RightAscension = RightAscension::new(0, 26, 17);
const ANKAA_DEC: Declination = Declination::new(-42, 18, 21);
const ANKAA_DISTANCE: Length = Length::from_astronomical_units(82. * AU_PER_LIGHT_YEARS);

const GIRTAB_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Girtab",
    Length::from_astronomical_units(6.8 * AU_PER_SUN_RADII),
    Mass::from_kilograms(17. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.46),
    Temperature::from_kelvin(23_400.),
);

const GIRTAB_RA: RightAscension = RightAscension::new(17, 42, 29);
const GIRTAB_DEC: Declination = Declination::new(-39, 1, 48);
const GIRTAB_DISTANCE: Length = Length::from_astronomical_units(480. * AU_PER_LIGHT_YEARS);

const ENIF_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Enif",
    Length::from_astronomical_units(211. * AU_PER_SUN_RADII),
    Mass::from_kilograms(7.07 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-4.142),
    Temperature::from_kelvin(3963.),
);

const ENIF_RA: RightAscension = RightAscension::new(21, 44, 11);
const ENIF_DEC: Declination = Declination::new(9, 52, 30);
const ENIF_DISTANCE: Length = Length::from_astronomical_units(690. * AU_PER_LIGHT_YEARS);

const SCHEAT_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Scheat",
    Length::from_astronomical_units(95. * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.1 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-1.41),
    Temperature::from_kelvin(3689.),
);

const SCHEAT_RA: RightAscension = RightAscension::new(23, 3, 46);
const SCHEAT_DEC: Declination = Declination::new(28, 4, 58);
const SCHEAT_DISTANCE: Length = Length::from_astronomical_units(196. * AU_PER_LIGHT_YEARS);

const SABIK_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Sabik",
    Length::from_astronomical_units(0. * AU_PER_SUN_RADII), //unknown
    Mass::from_kilograms(2.966 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.372),
    Temperature::from_kelvin(8900.),
);

const SABIK_RA: RightAscension = RightAscension::new(17, 10, 23);
const SABIK_DEC: Declination = Declination::new(-15, 43, 30);
const SABIK_DISTANCE: Length = Length::from_astronomical_units(88. * AU_PER_LIGHT_YEARS);

const PHECDA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Phecda",
    Length::from_astronomical_units(3.04 * AU_PER_SUN_RADII),
    Mass::from_kilograms(2.94 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.4),
    Temperature::from_kelvin(9355.),
);

const PHECDA_RA: RightAscension = RightAscension::new(11, 53, 50);
const PHECDA_DEC: Declination = Declination::new(53, 41, 41);
const PHECDA_DISTANCE: Length = Length::from_astronomical_units(83.2 * AU_PER_LIGHT_YEARS);

const ALUDRA_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Aludra",
    Length::from_astronomical_units(54. * AU_PER_SUN_RADII),
    Mass::from_kilograms(18.19 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-7.0),
    Temperature::from_kelvin(15_500.),
);

const ALUDRA_RA: RightAscension = RightAscension::new(7, 24, 6);
const ALUDRA_DEC: Declination = Declination::new(-29, 18, 11);
const ALUDRA_DISTANCE: Length = Length::from_astronomical_units(2000. * AU_PER_LIGHT_YEARS);

const MARKEB_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Markeb",
    Length::from_astronomical_units(9.1 * AU_PER_SUN_RADII),
    Mass::from_kilograms(10.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.74),
    Temperature::from_kelvin(23_000.),
);

const MARKEB_RA: RightAscension = RightAscension::new(9, 22, 7);
const MARKEB_DEC: Declination = Declination::new(-55, 0, 38);
const MARKEB_DISTANCE: Length = Length::from_astronomical_units(570. * AU_PER_LIGHT_YEARS);

const NAVI_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Navi",
    Length::from_astronomical_units(10. * AU_PER_SUN_RADII),
    Mass::from_kilograms(13. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.98),
    Temperature::from_kelvin(25_000.),
);

const NAVI_RA: RightAscension = RightAscension::new(0, 56, 43);
const NAVI_DEC: Declination = Declination::new(60, 43, 0);
const NAVI_DISTANCE: Length = Length::from_astronomical_units(550. * AU_PER_LIGHT_YEARS);

const MARKAB_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Markab",
    Length::from_astronomical_units(4.62 * AU_PER_SUN_RADII),
    Mass::from_kilograms(3.5 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-0.718),
    Temperature::from_kelvin(10_100.),
);

const MARKAB_RA: RightAscension = RightAscension::new(23, 4, 46);
const MARKAB_DEC: Declination = Declination::new(15, 12, 19);
const MARKAB_DISTANCE: Length = Length::from_astronomical_units(133. * AU_PER_LIGHT_YEARS);

const ALJANAH_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Aljanah",
    Length::from_astronomical_units(10.82 * AU_PER_SUN_RADII),
    Mass::from_kilograms(2. * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(0.74),
    Temperature::from_kelvin(4710.),
);

const ALJANAH_RA: RightAscension = RightAscension::new(20, 46, 13);
const ALJANAH_DEC: Declination = Declination::new(33, 58, 13);
const ALJANAH_DISTANCE: Length = Length::from_astronomical_units(72.7 * AU_PER_LIGHT_YEARS);

const ACRAB_PROPERTIES: StellarProperties = StellarProperties::new_const(
    "Acrab",
    Length::from_astronomical_units(6.3 * AU_PER_SUN_RADII),
    Mass::from_kilograms(15.0 * KILOGRAMS_PER_SOLAR_MASS),
    Luminosity::from_absolute_magnitude(-3.92),
    Temperature::from_kelvin(28_000.),
);

const ACRAB_RA: RightAscension = RightAscension::new(16, 5, 26);
const ACRAB_DEC: Declination = Declination::new(-19, 48, 20);
const ACRAB_DISTANCE: Length = Length::from_astronomical_units(400. * AU_PER_LIGHT_YEARS);

pub const STARS_TO_TWO_POINT_FIVE_APPARENT_MAG: [(
    StellarProperties,
    RightAscension,
    Declination,
    Length,
); 92] = [
    (SIRIUS_PROPERTIES, SIRIUS_RA, SIRIUS_DEC, SIRIUS_DISTANCE),
    (
        CANOPUS_PROPERTIES,
        CANOPUS_RA,
        CANOPUS_DEC,
        CANOPUS_DISTANCE,
    ),
    (
        ALPHA_CENTAURI_PROPERTIES,
        ALPHA_CENTAURI_RA,
        ALPHA_CENTAURI_DEC,
        ALPHA_CENTAURI_DISTANCE,
    ),
    (
        ARCTURUS_PROPERTIES,
        ARCTURUS_RA,
        ARCTURUS_DEC,
        ARCTURUS_DISTANCE,
    ),
    (VEGA_PROPERTIES, VEGA_RA, VEGA_DEC, VEGA_DISTANCE),
    (
        CAPELLA_PROPERTIES,
        CAPELLA_RA,
        CAPELLA_DEC,
        CAPELLA_DISTANCE,
    ),
    (RIGEL_PROPERTIES, RIGEL_RA, RIGEL_DEC, RIGEL_DISTANCE),
    (
        PROCYON_PROPERTIES,
        PROCYON_RA,
        PROCYON_DEC,
        PROCYON_DISTANCE,
    ),
    (
        ACHERNAR_PROPERTIES,
        ACHERNAR_RA,
        ACHERNAR_DEC,
        ACHERNAR_DISTANCE,
    ),
    (
        BETELGEUSE_PROPERTIES,
        BETELGEUSE_RA,
        BETELGEUSE_DEC,
        BETELGEUSE_DISTANCE,
    ),
    (HADAR_PROPERTIES, HADAR_RA, HADAR_DEC, HADAR_DISTANCE),
    (ALTAIR_PROPERTIES, ALTAIR_RA, ALTAIR_DEC, ALTAIR_DISTANCE),
    (ACRUX_PROPERTIES, ACRUX_RA, ACRUX_DEC, ACRUX_DISTANCE),
    (
        ALDEBARAN_PROPERTIES,
        ALDEBARAN_RA,
        ALDEBARAN_DEC,
        ALDEBARAN_DISTANCE,
    ),
    (
        ANTARES_PROPERTIES,
        ANTARES_RA,
        ANTARES_DEC,
        ANTARES_DISTANCE,
    ),
    (SPICA_PROPERTIES, SPICA_RA, SPICA_DEC, SPICA_DISTANCE),
    (POLLUX_PROPERTIES, POLLUX_RA, POLLUX_DEC, POLLUX_DISTANCE),
    (
        FORMALHAUT_PROPERTIES,
        FORMALHAUT_RA,
        FORMALHAUT_DEC,
        FORMALHAUT_DISTANCE,
    ),
    (DENEB_PROPERTIES, DENEB_RA, DENEB_DEC, DENEB_DISTANCE),
    (MIMOSA_PROPERTIES, MIMOSA_RA, MIMOSA_DEC, MIMOSA_DISTANCE),
    (
        REGULUS_PROPERTIES,
        REGULUS_RA,
        REGULUS_DEC,
        REGULUS_DISTANCE,
    ),
    (ADHARA_PROPERTIES, ADHARA_RA, ADHARA_DEC, ADHARA_DISTANCE),
    (SHAULA_PROPERTIES, SHAULA_RA, SHAULA_DEC, SHAULA_DISTANCE),
    (CASTOR_PROPERTIES, CASTOR_RA, CASTOR_DEC, CASTOR_DISTANCE),
    (GACRUX_PROPERTIES, GACRUX_RA, GACRUX_DEC, GACRUX_DISTANCE),
    (
        BELLATRIX_PROPERTIES,
        BELLATRIX_RA,
        BELLATRIX_DEC,
        BELLATRIX_DISTANCE,
    ),
    (ELNATH_PROPERTIES, ELNATH_RA, ELNATH_DEC, ELNATH_DISTANCE),
    (
        MIAPLACIDUS_PROPERTIES,
        MIAPLACIDUS_RA,
        MIAPLACIDUS_DEC,
        MIAPLACIDUS_DISTANCE,
    ),
    (
        ALNILAM_PROPERTIES,
        ALNILAM_RA,
        ALNILAM_DEC,
        ALNILAM_DISTANCE,
    ),
    (
        GAMMA_VELORUM_PROPERTIES,
        GAMMA_VELORUM_RA,
        GAMMA_VELORUM_DEC,
        GAMMA_VELORUM_DISTANCE,
    ),
    (ALNAIR_PROPERTIES, ALNAIR_RA, ALNAIR_DEC, ALNAIR_DISTANCE),
    (
        ALNITAK_PROPERTIES,
        ALNITAK_RA,
        ALNITAK_DEC,
        ALNITAK_DISTANCE,
    ),
    (ALIOTH_PROPERTIES, ALIOTH_RA, ALIOTH_DEC, ALIOTH_DISTANCE),
    (DUBHE_PROPERTIES, DUBHE_RA, DUBHE_DEC, DUBHE_DISTANCE),
    (MIRFAK_PROPERTIES, MIRFAK_RA, MIRFAK_DEC, MIRFAK_DISTANCE),
    (WEZEN_PROPERTIES, WEZEN_RA, WEZEN_DEC, WEZEN_DISTANCE),
    (SARGAS_PROPERTIES, SARGAS_RA, SARGAS_DEC, SARGAS_DISTANCE),
    (
        KAUS_AUSTRALIS_PROPERTIES,
        KAUS_AUSTRALIS_RA,
        KAUS_AUSTRALIS_DEC,
        KAUS_AUSTRALIS_DISTANCE,
    ),
    (AVIOR_PROPERTIES, AVIOR_RA, AVIOR_DEC, AVIOR_DISTANCE),
    (ALKAID_PROPERTIES, ALKAID_RA, ALKAID_DEC, ALKAID_DISTANCE),
    (
        MENKALINAN_PROPERTIES,
        MENKALINAN_RA,
        MENKALINAN_DEC,
        MENKALINAN_DISTANCE,
    ),
    (ATRIA_PROPERTIES, ATRIA_RA, ATRIA_DEC, ATRIA_DISTANCE),
    (ALHENA_PROPERTIES, ALHENA_RA, ALHENA_DEC, ALHENA_DISTANCE),
    (
        PEACOCK_PROPERTIES,
        PEACOCK_RA,
        PEACOCK_DEC,
        PEACOCK_DISTANCE,
    ),
    (
        ALSEPHINA_PROPERTIES,
        ALSEPHINA_RA,
        ALSEPHINA_DEC,
        ALSEPHINA_DISTANCE,
    ),
    (MIRZAM_PROPERTIES, MIRZAM_RA, MIRZAM_DEC, MIRZAM_DISTANCE),
    (
        ALPHARD_PROPERTIES,
        ALPHARD_RA,
        ALPHARD_DEC,
        ALPHARD_DISTANCE,
    ),
    (
        POLARIS_PROPERTIES,
        POLARIS_RA,
        POLARIS_DEC,
        POLARIS_DISTANCE,
    ),
    (HAMAL_PROPERTIES, HAMAL_RA, HAMAL_DEC, HAMAL_DISTANCE),
    (
        ALGIEBA_PROPERTIES,
        ALGIEBA_RA,
        ALGIEBA_DEC,
        ALGIEBA_DISTANCE,
    ),
    (DIPHDA_PROPERTIES, DIPHDA_RA, DIPHDA_DEC, DIPHDA_DISTANCE),
    (MIZAR_PROPERTIES, MIZAR_RA, MIZAR_DEC, MIZAR_DISTANCE),
    (NUNKI_PROPERTIES, NUNKI_RA, NUNKI_DEC, NUNKI_DISTANCE),
    (
        MENKENT_PROPERTIES,
        MENKENT_RA,
        MENKENT_DEC,
        MENKENT_DISTANCE,
    ),
    (MIRACH_PROPERTIES, MIRACH_RA, MIRACH_DEC, MIRACH_DISTANCE),
    (
        ALPHERATZ_PROPERTIES,
        ALPHERATZ_RA,
        ALPHERATZ_DEC,
        ALPHERATZ_DISTANCE,
    ),
    (
        RASALHAGUE_PROPERTIES,
        RASALHAGUE_RA,
        RASALHAGUE_DEC,
        RASALHAGUE_DISTANCE,
    ),
    (KOCHAB_PROPERTIES, KOCHAB_RA, KOCHAB_DEC, KOCHAB_DISTANCE),
    (SAIPH_PROPERTIES, SAIPH_RA, SAIPH_DEC, SAIPH_DISTANCE),
    (
        DENEBOLA_PROPERTIES,
        DENEBOLA_RA,
        DENEBOLA_DEC,
        DENEBOLA_DISTANCE,
    ),
    (ALGOL_PROPERTIES, ALGOL_RA, ALGOL_DEC, ALGOL_DISTANCE),
    (TIAKI_PROPERTIES, TIAKI_RA, TIAKI_DEC, TIAKI_DISTANCE),
    (
        MUHLIFAIN_PROPERTIES,
        MUHLIFAIN_RA,
        MUHLIFAIN_DEC,
        MUHLIFAIN_DISTANCE,
    ),
    (
        ASPIDISKE_PROPERTIES,
        ASPIDISKE_RA,
        ASPIDISKE_DEC,
        ASPIDISKE_DISTANCE,
    ),
    (SUHAIL_PROPERTIES, SUHAIL_RA, SUHAIL_DEC, SUHAIL_DISTANCE),
    (
        ALPHECCA_PROPERTIES,
        ALPHECCA_RA,
        ALPHECCA_DEC,
        ALPHECCA_DISTANCE,
    ),
    (
        MINTAKA_PROPERTIES,
        MINTAKA_RA,
        MINTAKA_DEC,
        MINTAKA_DISTANCE,
    ),
    (SADR_PROPERTIES, SADR_RA, SADR_DEC, SADR_DISTANCE),
    (
        ELTANIN_PROPERTIES,
        ELTANIN_RA,
        ELTANIN_DEC,
        ELTANIN_DISTANCE,
    ),
    (
        SCHEDAR_PROPERTIES,
        SCHEDAR_RA,
        SCHEDAR_DEC,
        SCHEDAR_DISTANCE,
    ),
    (NAOS_PROPERTIES, NAOS_RA, NAOS_DEC, NAOS_DISTANCE),
    (ALMACH_PROPERTIES, ALMACH_RA, ALMACH_DEC, ALMACH_DISTANCE),
    (CAPH_PROPERTIES, CAPH_RA, CAPH_DEC, CAPH_DISTANCE),
    (IZAR_PROPERTIES, IZAR_RA, IZAR_DEC, IZAR_DISTANCE),
    (
        ALPHA_LUPI_PROPERTIES,
        ALPHA_LUPI_RA,
        ALPHA_LUPI_DEC,
        ALPHA_LUPI_DISTANCE,
    ),
    (
        EPSILON_CENTAURI_PROPERTIES,
        EPSILON_CENTAURI_RA,
        EPSILON_CENTAURI_DEC,
        EPSILON_CENTAURI_DISTANCE,
    ),
    (
        DSCHUBBA_PROPERTIES,
        DSCHUBBA_RA,
        DSCHUBBA_DEC,
        DSCHUBBA_DISTANCE,
    ),
    (
        LARAWAG_PROPERTIES,
        LARAWAG_RA,
        LARAWAG_DEC,
        LARAWAG_DISTANCE,
    ),
    (
        ETA_CENTAURI_PROPERTIES,
        ETA_CENTAURI_RA,
        ETA_CENTAURI_DEC,
        ETA_CENTAURI_DISTANCE,
    ),
    (MERAK_PROPERTIES, MERAK_RA, MERAK_DEC, MERAK_DISTANCE),
    (ANKAA_PROPERTIES, ANKAA_RA, ANKAA_DEC, ANKAA_DISTANCE),
    (GIRTAB_PROPERTIES, GIRTAB_RA, GIRTAB_DEC, GIRTAB_DISTANCE),
    (ENIF_PROPERTIES, ENIF_RA, ENIF_DEC, ENIF_DISTANCE),
    (SCHEAT_PROPERTIES, SCHEAT_RA, SCHEAT_DEC, SCHEAT_DISTANCE),
    (SABIK_PROPERTIES, SABIK_RA, SABIK_DEC, SABIK_DISTANCE),
    (PHECDA_PROPERTIES, PHECDA_RA, PHECDA_DEC, PHECDA_DISTANCE),
    (ALUDRA_PROPERTIES, ALUDRA_RA, ALUDRA_DEC, ALUDRA_DISTANCE),
    (MARKEB_PROPERTIES, MARKEB_RA, MARKEB_DEC, MARKEB_DISTANCE),
    (NAVI_PROPERTIES, NAVI_RA, NAVI_DEC, NAVI_DISTANCE),
    (MARKAB_PROPERTIES, MARKAB_RA, MARKAB_DEC, MARKAB_DISTANCE),
    (
        ALJANAH_PROPERTIES,
        ALJANAH_RA,
        ALJANAH_DEC,
        ALJANAH_DISTANCE,
    ),
    (ACRAB_PROPERTIES, ACRAB_RA, ACRAB_DEC, ACRAB_DISTANCE),
];
