use crate::{
    coordinates::{declination::Declination, right_ascension::RightAscension},
    stars::real_data::RealData,
    units::{
        length::{Length, AU_PER_LIGHT_YEARS, AU_PER_SUN_RADII},
        luminosity::Luminosity,
        mass::{Mass, KILOGRAMS_PER_SOLAR_MASS},
        temperature::Temperature,
        time::{Time, SECONDS_PER_BILLION_YEARS},
    },
};

//https://web.pa.msu.edu/people/horvatin/Astronomy_Facts/brightest_stars.html

pub const SUN_DATA: RealData = RealData {
    common_name: "Sun",
    astronomical_name: "Sol",
    mass: Some(Mass::from_kilograms(KILOGRAMS_PER_SOLAR_MASS)),
    radius: Some(Length::from_astronomical_units(AU_PER_SUN_RADII)),
    luminosity: Luminosity::from_absolute_magnitude(4.83),
    apparent_magnitude: -26.74, //seen from earth
    temperature: Some(Temperature::from_kelvin(5778.0)),
    age: Some(Time::from_seconds(4.6 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(0, 0, 0),
    declination: Declination::new(0, 0, 0),
    distance: Length::from_astronomical_units(0.0),
};

//1
const SIRIUS_DATA: RealData = RealData {
    common_name: "Sirius",
    astronomical_name: "Alpha Canis Majoris",
    radius: Some(Length::from_astronomical_units(1.711 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.063 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(1.45),
    apparent_magnitude: -1.44,
    temperature: Some(Temperature::from_kelvin(9940.)),
    age: Some(Time::from_seconds(0.242 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(6, 45, 9),
    declination: Declination::new(-16, 42, 58),
    distance: Length::from_astronomical_units(9. * AU_PER_LIGHT_YEARS),
};

//2
const CANOPUS_DATA: RealData = RealData {
    common_name: "Canopus",
    astronomical_name: "Alpha Carinae",
    radius: Some(Length::from_astronomical_units(72. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(9. * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-5.53),
    apparent_magnitude: -0.62,
    temperature: Some(Temperature::from_kelvin(7400.)),
    age: Some(Time::from_seconds(0.0251 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(6, 23, 57),
    declination: Declination::new(-52, 41, 44),
    distance: Length::from_astronomical_units(313. * AU_PER_LIGHT_YEARS),
};

//3
const ARCTURUS_DATA: RealData = RealData {
    common_name: "Arcturus",
    astronomical_name: "Alpha Bootis",
    radius: Some(Length::from_astronomical_units(25.4 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.08 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.31),
    apparent_magnitude: -0.05,
    temperature: Some(Temperature::from_kelvin(4286.)),
    age: Some(Time::from_seconds(7.1 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(14, 15, 40),
    declination: Declination::new(19, 10, 56),
    distance: Length::from_astronomical_units(37. * AU_PER_LIGHT_YEARS),
};

//4
const RIGEL_KENTAURUS_DATA: RealData = RealData {
    common_name: "Rigel Kentaurus",
    astronomical_name: "Alpha Centauri",
    radius: Some(Length::from_astronomical_units(1.2175 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.0788 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(4.34),
    apparent_magnitude: -0.27,
    temperature: Some(Temperature::from_kelvin(5790.)),
    age: Some(Time::from_seconds(4.85 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(14, 39, 36),
    declination: Declination::new(-60, 50, 2),
    distance: Length::from_astronomical_units(4. * AU_PER_LIGHT_YEARS),
};

//5
const VEGA_DATA: RealData = RealData {
    common_name: "Vega",
    astronomical_name: "Alpha Lyrae",
    radius: Some(Length::from_astronomical_units(2.362 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.135 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(0.58),
    apparent_magnitude: 0.03,
    temperature: Some(Temperature::from_kelvin(9602.)),
    age: Some(Time::from_seconds(0.455 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(18, 36, 56),
    declination: Declination::new(38, 47, 1),
    distance: Length::from_astronomical_units(25. * AU_PER_LIGHT_YEARS),
};

//6
const CAPELLA_DATA: RealData = RealData {
    common_name: "Capella",
    astronomical_name: "Alpha Aurigae",
    radius: Some(Length::from_astronomical_units(11.98 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.5687 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.48),
    apparent_magnitude: 0.08,
    temperature: Some(Temperature::from_kelvin(4970.)),
    age: Some(Time::from_seconds(0.620 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(5, 16, 41),
    declination: Declination::new(45, 59, 53),
    distance: Length::from_astronomical_units(42. * AU_PER_LIGHT_YEARS),
};

//7
const RIGEL_DATA: RealData = RealData {
    common_name: "Rigel",
    astronomical_name: "Beta Orionis",
    radius: Some(Length::from_astronomical_units(78.9 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(21. * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-6.69),
    apparent_magnitude: 0.18,
    temperature: Some(Temperature::from_kelvin(12_100.)),
    age: Some(Time::from_seconds(0.008 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(5, 14, 32),
    declination: Declination::new(-8, 12, 6),
    distance: Length::from_astronomical_units(773. * AU_PER_LIGHT_YEARS),
};

//8
const PROCYON_DATA: RealData = RealData {
    common_name: "Procyon",
    astronomical_name: "Alpha Canis Minoris",
    radius: Some(Length::from_astronomical_units(2.048 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.499 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(2.68),
    apparent_magnitude: 0.40,
    temperature: Some(Temperature::from_kelvin(6530.)),
    age: Some(Time::from_seconds(1.37 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(7, 39, 18),
    declination: Declination::new(5, 13, 30),
    distance: Length::from_astronomical_units(11. * AU_PER_LIGHT_YEARS),
};

//9
const BETELGEUSE_DATA: RealData = RealData {
    common_name: "Betelgeuse",
    astronomical_name: "Alpha Orionis",
    radius: Some(Length::from_astronomical_units(887. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(16.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-5.14),
    apparent_magnitude: 0.9,
    temperature: Some(Temperature::from_kelvin(3600.)),
    age: Some(Time::from_seconds(0.008 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(5, 55, 10),
    declination: Declination::new(7, 24, 25),
    distance: Length::from_astronomical_units(522. * AU_PER_LIGHT_YEARS),
};

//10
const ACHERNAR_DATA: RealData = RealData {
    common_name: "Achernar",
    astronomical_name: "Alpha Eridani",
    radius: Some(Length::from_astronomical_units(6.78 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(6.0 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-2.77),
    apparent_magnitude: 0.45,
    temperature: Some(Temperature::from_kelvin(14_000.)),
    age: Some(Time::from_seconds(0.063 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(1, 37, 43),
    declination: Declination::new(-57, 14, 12),
    distance: Length::from_astronomical_units(144. * AU_PER_LIGHT_YEARS),
};

//11
const HADAR_DATA: RealData = RealData {
    common_name: "Hadar",
    astronomical_name: "Beta Centauri",
    radius: Some(Length::from_astronomical_units(9. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(12.02 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-5.42),
    apparent_magnitude: 0.61,
    temperature: Some(Temperature::from_kelvin(25_000.)),
    age: Some(Time::from_seconds(0.0141 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(14, 3, 49),
    declination: Declination::new(-60, 22, 23),
    distance: Length::from_astronomical_units(526. * AU_PER_LIGHT_YEARS),
};

//12
const ALTAIR_DATA: RealData = RealData {
    common_name: "Altair",
    astronomical_name: "Alpha Aquilae",
    radius: Some(Length::from_astronomical_units(1.63 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.86 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(2.20),
    apparent_magnitude: 0.76,
    temperature: Some(Temperature::from_kelvin(7670.)),
    age: Some(Time::from_seconds(0.100 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(19, 50, 47),
    declination: Declination::new(8, 52, 6),
    distance: Length::from_astronomical_units(17. * AU_PER_LIGHT_YEARS),
};

//13
const ACRUX_DATA: RealData = RealData {
    common_name: "Acrux",
    astronomical_name: "Alpha Crucis",
    radius: Some(Length::from_astronomical_units(7.8 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(17.8 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-4.19),
    apparent_magnitude: 0.77,
    temperature: Some(Temperature::from_kelvin(24_000.)),
    age: Some(Time::from_seconds(0.0108 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(12, 26, 36),
    declination: Declination::new(-63, 5, 57),
    distance: Length::from_astronomical_units(321. * AU_PER_LIGHT_YEARS),
};

//14
const ALDEBARAN_DATA: RealData = RealData {
    common_name: "Aldebaran",
    astronomical_name: "Alpha Tauri",
    radius: Some(Length::from_astronomical_units(45.1 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.16 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.63),
    apparent_magnitude: 0.87,
    temperature: Some(Temperature::from_kelvin(3900.)),
    age: Some(Time::from_seconds(6.4 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(4, 35, 55),
    declination: Declination::new(16, 30, 33),
    distance: Length::from_astronomical_units(65. * AU_PER_LIGHT_YEARS),
};

//15
const SPICA_DATA: RealData = RealData {
    common_name: "Spica",
    astronomical_name: "Alpha Virginis",
    radius: Some(Length::from_astronomical_units(7.47 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(11.43 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.55),
    apparent_magnitude: 0.98,
    temperature: Some(Temperature::from_kelvin(22_300.)),
    age: Some(Time::from_seconds(0.0125 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(13, 25, 12),
    declination: Declination::new(-11, 9, 41),
    distance: Length::from_astronomical_units(262. * AU_PER_LIGHT_YEARS),
};

//16
const ANTARES_DATA: RealData = RealData {
    common_name: "Antares",
    astronomical_name: "Alpha Scorpii",
    radius: Some(Length::from_astronomical_units(680. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(13.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-5.28),
    apparent_magnitude: 1.06,
    temperature: Some(Temperature::from_kelvin(3660.)),
    age: Some(Time::from_seconds(0.015 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(16, 29, 24),
    declination: Declination::new(-26, 25, 55),
    distance: Length::from_astronomical_units(604. * AU_PER_LIGHT_YEARS),
};

//17
const POLLUX_DATA: RealData = RealData {
    common_name: "Pollux",
    astronomical_name: "Beta Geminorum",
    radius: Some(Length::from_astronomical_units(9.06 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.91 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(1.09),
    apparent_magnitude: 1.16,
    temperature: Some(Temperature::from_kelvin(4586.)),
    age: Some(Time::from_seconds(0.724 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(7, 45, 19),
    declination: Declination::new(28, 1, 34),
    distance: Length::from_astronomical_units(34. * AU_PER_LIGHT_YEARS),
};

//18
const FORMALHAUT_DATA: RealData = RealData {
    common_name: "Formalhaut",
    astronomical_name: "Alpha Piscis Austrini",
    radius: Some(Length::from_astronomical_units(1.842 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.92 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(1.74),
    apparent_magnitude: 1.17,
    temperature: Some(Temperature::from_kelvin(8590.)),
    age: Some(Time::from_seconds(0.44 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(22, 57, 39),
    declination: Declination::new(-29, 37, 20),
    distance: Length::from_astronomical_units(25. * AU_PER_LIGHT_YEARS),
};

//19
const DENEB_DATA: RealData = RealData {
    common_name: "Deneb",
    astronomical_name: "Alpha Cygni",
    radius: Some(Length::from_astronomical_units(203. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(19. * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-7.13),
    apparent_magnitude: 1.25,
    temperature: Some(Temperature::from_kelvin(8515.)),
    age: None,
    right_ascension: RightAscension::new(20, 41, 26),
    declination: Declination::new(45, 16, 49),
    distance: Length::from_astronomical_units(1548. * AU_PER_LIGHT_YEARS),
};

//20
const MIMOSA_DATA: RealData = RealData {
    common_name: "Mimosa",
    astronomical_name: "Beta Crucis",
    radius: Some(Length::from_astronomical_units(8.4 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(16. * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.92),
    apparent_magnitude: 1.25,
    temperature: Some(Temperature::from_kelvin(27_000.)),
    age: Some(Time::from_seconds(0.010 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(12, 47, 43),
    declination: Declination::new(-59, 41, 20),
    distance: Length::from_astronomical_units(352. * AU_PER_LIGHT_YEARS),
};

//21
const REGULUS_DATA: RealData = RealData {
    common_name: "Regulus",
    astronomical_name: "Alpha Leonis",
    radius: Some(Length::from_astronomical_units(4.35 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(3.8 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.52),
    apparent_magnitude: 1.36,
    temperature: Some(Temperature::from_kelvin(11_668.)),
    age: Some(Time::from_seconds(1. * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(10, 8, 22),
    declination: Declination::new(11, 58, 2),
    distance: Length::from_astronomical_units(77. * AU_PER_LIGHT_YEARS),
};

//22
const ADHARA_DATA: RealData = RealData {
    common_name: "Adhara",
    astronomical_name: "Epsilon Canis Majoris",
    radius: Some(Length::from_astronomical_units(13.9 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(12.6 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-4.10),
    apparent_magnitude: 1.5,
    temperature: Some(Temperature::from_kelvin(22_900.)),
    age: Some(Time::from_seconds(0.0225 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(6, 58, 38),
    declination: Declination::new(-28, 58, 19),
    distance: Length::from_astronomical_units(431. * AU_PER_LIGHT_YEARS),
};

//23
const CASTOR_DATA: RealData = RealData {
    common_name: "Castor",
    astronomical_name: "Alpha Geminorum",
    radius: Some(Length::from_astronomical_units(2.089 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.37 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(0.59),
    apparent_magnitude: 1.58,
    temperature: Some(Temperature::from_kelvin(10_286.)),
    age: Some(Time::from_seconds(0.290 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(7, 34, 36),
    declination: Declination::new(31, 53, 18),
    distance: Length::from_astronomical_units(52. * AU_PER_LIGHT_YEARS),
};

//24
const GACRUX_DATA: RealData = RealData {
    common_name: "Gacrux",
    astronomical_name: "Gamma Crucis",
    radius: Some(Length::from_astronomical_units(120. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.56),
    apparent_magnitude: 1.59,
    temperature: Some(Temperature::from_kelvin(3689.)),
    age: None,
    right_ascension: RightAscension::new(12, 31, 10),
    declination: Declination::new(-57, 6, 48),
    distance: Length::from_astronomical_units(88. * AU_PER_LIGHT_YEARS),
};

//25
const SHAULA_DATA: RealData = RealData {
    common_name: "Shaula",
    astronomical_name: "Lambda Scorpii",
    radius: Some(Length::from_astronomical_units(8.8 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(10.4 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-4.8),
    apparent_magnitude: 1.62,
    temperature: Some(Temperature::from_kelvin(25_000.)),
    age: None,
    right_ascension: RightAscension::new(17, 33, 37),
    declination: Declination::new(-37, 6, 14),
    distance: Length::from_astronomical_units(600. * AU_PER_LIGHT_YEARS),
};

//26
const BELLATRIX_DATA: RealData = RealData {
    common_name: "Bellatrix",
    astronomical_name: "Gamma Orionis",
    radius: Some(Length::from_astronomical_units(5.75 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(7.7 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-2.72),
    apparent_magnitude: 1.64,
    temperature: Some(Temperature::from_kelvin(21_800.)),
    age: Some(Time::from_seconds(0.0252 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(5, 25, 8),
    declination: Declination::new(6, 20, 59),
    distance: Length::from_astronomical_units(243. * AU_PER_LIGHT_YEARS),
};

//27
const ALNATH_DATA: RealData = RealData {
    common_name: "Alnath",
    astronomical_name: "Beta Tauri",
    radius: Some(Length::from_astronomical_units(4.2 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(5.0 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.37),
    apparent_magnitude: 1.65,
    temperature: Some(Temperature::from_kelvin(13_824.)),
    age: Some(Time::from_seconds(0.1 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(5, 26, 18),
    declination: Declination::new(28, 36, 27),
    distance: Length::from_astronomical_units(131. * AU_PER_LIGHT_YEARS),
};

//28
const MIAPLACIDUS_DATA: RealData = RealData {
    common_name: "Miaplacidus",
    astronomical_name: "Beta Carinae",
    radius: Some(Length::from_astronomical_units(6.8 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(3.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.99),
    apparent_magnitude: 1.67,
    temperature: Some(Temperature::from_kelvin(8866.)),
    age: Some(Time::from_seconds(0.260 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(9, 13, 12),
    declination: Declination::new(-69, 43, 2),
    distance: Length::from_astronomical_units(111. * AU_PER_LIGHT_YEARS),
};

//29
const ALNILAM_DATA: RealData = RealData {
    common_name: "Alnilam",
    astronomical_name: "Epsilon Orionis",
    radius: Some(Length::from_astronomical_units(42. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(64.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-6.38),
    apparent_magnitude: 1.69,
    temperature: Some(Temperature::from_kelvin(27_000.)),
    age: Some(Time::from_seconds(0.0057 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(5, 36, 13),
    declination: Declination::new(-1, 12, 7),
    distance: Length::from_astronomical_units(1342. * AU_PER_LIGHT_YEARS),
};

//30
const ALNAIR_DATA: RealData = RealData {
    common_name: "Alnair",
    astronomical_name: "Alpha Gruis",
    radius: Some(Length::from_astronomical_units(3.4 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(4. * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.73),
    apparent_magnitude: 1.73,
    temperature: Some(Temperature::from_kelvin(13_920.)),
    age: Some(Time::from_seconds(0.1 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(22, 8, 14),
    declination: Declination::new(-46, 57, 40),
    distance: Length::from_astronomical_units(101. * AU_PER_LIGHT_YEARS),
};

//31
const ALNITAK_DATA: RealData = RealData {
    common_name: "Alnitak",
    astronomical_name: "Zeta Orionis",
    radius: Some(Length::from_astronomical_units(20. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(33.0 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-5.26),
    apparent_magnitude: 1.74,
    temperature: Some(Temperature::from_kelvin(29_500.)),
    age: Some(Time::from_seconds(0.0064 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(5, 40, 46),
    declination: Declination::new(-1, 56, 34),
    distance: Length::from_astronomical_units(817. * AU_PER_LIGHT_YEARS),
};

//32
const REGOR_DATA: RealData = RealData {
    common_name: "Regor",
    astronomical_name: "Gamma Velorum",
    radius: Some(Length::from_astronomical_units(17. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(28.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-5.31),
    apparent_magnitude: 1.75,
    temperature: Some(Temperature::from_kelvin(35_000.)),
    age: Some(Time::from_seconds(0.0045 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(8, 9, 32),
    declination: Declination::new(-47, 20, 12),
    distance: Length::from_astronomical_units(840. * AU_PER_LIGHT_YEARS),
};

//33
const ALIOTH_DATA: RealData = RealData {
    common_name: "Alioth",
    astronomical_name: "Epsilon Ursae Majoris",
    radius: Some(Length::from_astronomical_units(4.14 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.91 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.21),
    apparent_magnitude: 1.76,
    temperature: Some(Temperature::from_kelvin(9_020.)),
    age: Some(Time::from_seconds(0.3 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(12, 54, 2),
    declination: Declination::new(55, 57, 36),
    distance: Length::from_astronomical_units(81. * AU_PER_LIGHT_YEARS),
};

//34
const KAUS_AUSTRALIS_DATA: RealData = RealData {
    common_name: "Kaus Australis",
    astronomical_name: "Epsilon Sagittarii",
    radius: Some(Length::from_astronomical_units(6.8 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(3.515 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.44),
    apparent_magnitude: 1.79,
    temperature: Some(Temperature::from_kelvin(9960.)),
    age: Some(Time::from_seconds(0.232 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(18, 24, 10),
    declination: Declination::new(-34, 23, 5),
    distance: Length::from_astronomical_units(145. * AU_PER_LIGHT_YEARS),
};

//35
const MIRPHAK_DATA: RealData = RealData {
    common_name: "Mirphak",
    astronomical_name: "Alpha Persei",
    radius: Some(Length::from_astronomical_units(68. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(8.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-4.50),
    apparent_magnitude: 1.79,
    temperature: Some(Temperature::from_kelvin(6350.)),
    age: Some(Time::from_seconds(0.041 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(3, 24, 19),
    declination: Declination::new(49, 51, 40),
    distance: Length::from_astronomical_units(592. * AU_PER_LIGHT_YEARS),
};

//36
const DUBHE_DATA: RealData = RealData {
    common_name: "Dubhe",
    astronomical_name: "Alpha Ursae Majoris",
    radius: Some(Length::from_astronomical_units(17.03 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(3.44 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.08),
    apparent_magnitude: 1.81,
    temperature: Some(Temperature::from_kelvin(5012.)),
    age: Some(Time::from_seconds(0.28 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(11, 3, 44),
    declination: Declination::new(61, 45, 4),
    distance: Length::from_astronomical_units(124. * AU_PER_LIGHT_YEARS),
};

//37
const WEZEN_DATA: RealData = RealData {
    common_name: "Wezen",
    astronomical_name: "Delta Canis Majoris",
    radius: Some(Length::from_astronomical_units(215. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(16.9 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-6.87),
    apparent_magnitude: 1.83,
    temperature: Some(Temperature::from_kelvin(6390.)),
    age: Some(Time::from_seconds(0.012 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(7, 8, 23),
    declination: Declination::new(-26, 23, 36),
    distance: Length::from_astronomical_units(1791. * AU_PER_LIGHT_YEARS),
};

//38
const ALKAID_DATA: RealData = RealData {
    common_name: "Alkaid",
    astronomical_name: "Eta Ursae Majoris",
    radius: Some(Length::from_astronomical_units(3.4 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(6.1 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.60),
    apparent_magnitude: 1.85,
    temperature: Some(Temperature::from_kelvin(15_540.)),
    age: Some(Time::from_seconds(0.01 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(13, 47, 32),
    declination: Declination::new(49, 18, 48),
    distance: Length::from_astronomical_units(101. * AU_PER_LIGHT_YEARS),
};

//39
const SARGAS_DATA: RealData = RealData {
    common_name: "Sargas",
    astronomical_name: "Theta Scorpii",
    radius: Some(Length::from_astronomical_units(26.3 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(3.1 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-2.75),
    apparent_magnitude: 1.86,
    temperature: Some(Temperature::from_kelvin(6294.)),
    age: None,
    right_ascension: RightAscension::new(17, 37, 19),
    declination: Declination::new(-42, 59, 52),
    distance: Length::from_astronomical_units(272. * AU_PER_LIGHT_YEARS),
};

//40
const AVIOR_DATA: RealData = RealData {
    common_name: "Avior",
    astronomical_name: "Epsilon Carinae",
    radius: None,
    mass: Some(Mass::from_kilograms(10.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-4.58),
    apparent_magnitude: 1.86,
    temperature: Some(Temperature::from_kelvin(3523.)),
    age: Some(Time::from_seconds(0.020 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(8, 22, 31),
    declination: Declination::new(-59, 30, 34),
    distance: Length::from_astronomical_units(632. * AU_PER_LIGHT_YEARS),
};

//41
const MENKALINAN_DATA: RealData = RealData {
    common_name: "Menkalinan",
    astronomical_name: "Beta Aurigae",
    radius: Some(Length::from_astronomical_units(2.77 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.389 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.10),
    apparent_magnitude: 1.9,
    temperature: Some(Temperature::from_kelvin(9350.)),
    age: Some(Time::from_seconds(0.570 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(5, 59, 32),
    declination: Declination::new(44, 56, 51),
    distance: Length::from_astronomical_units(82. * AU_PER_LIGHT_YEARS),
};

//42
const ATRIA_DATA: RealData = RealData {
    common_name: "Atria",
    astronomical_name: "Alpha Trianguli Australis",
    radius: Some(Length::from_astronomical_units(143. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(7. * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.62),
    apparent_magnitude: 1.91,
    temperature: Some(Temperature::from_kelvin(4150.)),
    age: Some(Time::from_seconds(0.048 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(16, 48, 40),
    declination: Declination::new(-69, 1, 40),
    distance: Length::from_astronomical_units(415. * AU_PER_LIGHT_YEARS),
};

//43
const ALSEPHINA_DATA: RealData = RealData {
    common_name: "Alsephina",
    astronomical_name: "Delta Velorum",
    radius: Some(Length::from_astronomical_units(2.4 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.27 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.01),
    apparent_magnitude: 1.93,
    temperature: Some(Temperature::from_kelvin(9440.)),
    age: Some(Time::from_seconds(0.4 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(8, 44, 42),
    declination: Declination::new(-54, 42, 32),
    distance: Length::from_astronomical_units(80. * AU_PER_LIGHT_YEARS),
};

//44
const ALHENA_DATA: RealData = RealData {
    common_name: "Alhena",
    astronomical_name: "Gamma Geminorum",
    radius: Some(Length::from_astronomical_units(3.3 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.81 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.60),
    apparent_magnitude: 1.93,
    temperature: Some(Temperature::from_kelvin(9260.)),
    age: None,
    right_ascension: RightAscension::new(6, 37, 43),
    declination: Declination::new(16, 23, 57),
    distance: Length::from_astronomical_units(105. * AU_PER_LIGHT_YEARS),
};

//45
const PEACOCK_DATA: RealData = RealData {
    common_name: "Peacock",
    astronomical_name: "Alpha Pavonis",
    radius: Some(Length::from_astronomical_units(4.83 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(5.91 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.81),
    apparent_magnitude: 1.94,
    temperature: Some(Temperature::from_kelvin(17_711.)),
    age: Some(Time::from_seconds(0.048 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(20, 25, 39),
    declination: Declination::new(-56, 44, 6),
    distance: Length::from_astronomical_units(183. * AU_PER_LIGHT_YEARS),
};

//46
const POLARIS_DATA: RealData = RealData {
    common_name: "Polaris",
    astronomical_name: "Alpha Ursae Minoris",
    radius: Some(Length::from_astronomical_units(37.5 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(5.4 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.64),
    apparent_magnitude: 1.97,
    temperature: Some(Temperature::from_kelvin(6015.)),
    age: Some(Time::from_seconds(0.05 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(2, 31, 49),
    declination: Declination::new(89, 15, 51),
    distance: Length::from_astronomical_units(431. * AU_PER_LIGHT_YEARS),
};

//47
const MIRZAM_DATA: RealData = RealData {
    common_name: "Mirzam",
    astronomical_name: "Beta Canis Majoris",
    radius: Some(Length::from_astronomical_units(9.7 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(13.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.95),
    apparent_magnitude: 1.98,
    temperature: Some(Temperature::from_kelvin(25_000.)),
    age: Some(Time::from_seconds(0.0124 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(6, 22, 42),
    declination: Declination::new(-17, 57, 21),
    distance: Length::from_astronomical_units(499. * AU_PER_LIGHT_YEARS),
};

//48
const ALPHARD_DATA: RealData = RealData {
    common_name: "Alphard",
    astronomical_name: "Alpha Hydrae",
    radius: Some(Length::from_astronomical_units(50.5 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(3.03 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.69),
    apparent_magnitude: 1.99,
    temperature: Some(Temperature::from_kelvin(4120.)),
    age: Some(Time::from_seconds(0.42 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(9, 27, 35),
    declination: Declination::new(-8, 39, 30),
    distance: Length::from_astronomical_units(177. * AU_PER_LIGHT_YEARS),
};

//49
const ALGIEBA_DATA: RealData = RealData {
    common_name: "Algieba",
    astronomical_name: "Gamma Leonis",
    radius: Some(Length::from_astronomical_units(31.88 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.23 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.92),
    apparent_magnitude: 2.01,
    temperature: Some(Temperature::from_kelvin(4470.)),
    age: None,
    right_ascension: RightAscension::new(10, 19, 58),
    declination: Declination::new(19, 50, 29),
    distance: Length::from_astronomical_units(126. * AU_PER_LIGHT_YEARS),
};

//50
const HAMAL_DATA: RealData = RealData {
    common_name: "Hamal",
    astronomical_name: "Alpha Arietis",
    radius: Some(Length::from_astronomical_units(14.9 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(0.48),
    apparent_magnitude: 2.01,
    temperature: Some(Temperature::from_kelvin(4480.)),
    age: Some(Time::from_seconds(3.4 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(2, 7, 10),
    declination: Declination::new(23, 27, 45),
    distance: Length::from_astronomical_units(66. * AU_PER_LIGHT_YEARS),
};

//51
const DIPHDA_DATA: RealData = RealData {
    common_name: "Diphda",
    astronomical_name: "Beta Ceti",
    radius: Some(Length::from_astronomical_units(16.78 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.8 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.30),
    apparent_magnitude: 2.04,
    temperature: Some(Temperature::from_kelvin(4797.)),
    age: Some(Time::from_seconds(1. * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(0, 43, 35),
    declination: Declination::new(-17, 59, 12),
    distance: Length::from_astronomical_units(96. * AU_PER_LIGHT_YEARS),
};

//52
const NUNKI_DATA: RealData = RealData {
    common_name: "Nunki",
    astronomical_name: "Sigma Sagittarii",
    radius: Some(Length::from_astronomical_units(4.5 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(7.8 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-2.14),
    apparent_magnitude: 2.05,
    temperature: Some(Temperature::from_kelvin(18_890.)),
    age: Some(Time::from_seconds(0.0314 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(18, 55, 16),
    declination: Declination::new(-26, 17, 49),
    distance: Length::from_astronomical_units(224. * AU_PER_LIGHT_YEARS),
};

//53
const MENKENT_DATA: RealData = RealData {
    common_name: "Menkent",
    astronomical_name: "Theta Centauri",
    radius: Some(Length::from_astronomical_units(10.6 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.27 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(0.70),
    apparent_magnitude: 2.06,
    temperature: Some(Temperature::from_kelvin(4980.)),
    age: None,
    right_ascension: RightAscension::new(14, 6, 41),
    declination: Declination::new(-36, 22, 11),
    distance: Length::from_astronomical_units(61. * AU_PER_LIGHT_YEARS),
};

//54
const SAIPH_DATA: RealData = RealData {
    common_name: "Saiph",
    astronomical_name: "Kappa Orionis",
    radius: Some(Length::from_astronomical_units(22.2 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(15.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-4.65),
    apparent_magnitude: 2.07,
    temperature: Some(Temperature::from_kelvin(26_500.)),
    age: Some(Time::from_seconds(0.0111 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(5, 47, 45),
    declination: Declination::new(-9, 40, 11),
    distance: Length::from_astronomical_units(721.2 * AU_PER_LIGHT_YEARS),
};

//55
const ALPHERATZ_DATA: RealData = RealData {
    common_name: "Alpheratz",
    astronomical_name: "Alpha Andromedae",
    radius: Some(Length::from_astronomical_units(2.7 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(3.8 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.30),
    apparent_magnitude: 2.07,
    temperature: Some(Temperature::from_kelvin(13_800.)),
    age: Some(Time::from_seconds(0.06 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(0, 8, 23),
    declination: Declination::new(29, 5, 26),
    distance: Length::from_astronomical_units(97.0 * AU_PER_LIGHT_YEARS),
};

//56
const TIAKI_DATA: RealData = RealData {
    common_name: "Tiaki",
    astronomical_name: "Beta Gruis",
    radius: Some(Length::from_astronomical_units(180. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.4 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.52),
    apparent_magnitude: 2.07,
    temperature: Some(Temperature::from_kelvin(3480.)),
    age: None,
    right_ascension: RightAscension::new(22, 42, 40),
    declination: Declination::new(-46, 53, 4),
    distance: Length::from_astronomical_units(170. * AU_PER_LIGHT_YEARS),
};

//57
const MIRACH_DATA: RealData = RealData {
    common_name: "Mirach",
    astronomical_name: "Beta Andromedae",
    radius: Some(Length::from_astronomical_units(100. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.49 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.86),
    apparent_magnitude: 2.07,
    temperature: Some(Temperature::from_kelvin(3842.)),
    age: None,
    right_ascension: RightAscension::new(1, 9, 44),
    declination: Declination::new(35, 37, 14),
    distance: Length::from_astronomical_units(199. * AU_PER_LIGHT_YEARS),
};

//58
const KOCHAB_DATA: RealData = RealData {
    common_name: "Kochab",
    astronomical_name: "Beta Ursae Minoris",
    radius: Some(Length::from_astronomical_units(42.06 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.2 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.87),
    apparent_magnitude: 2.07,
    temperature: Some(Temperature::from_kelvin(4030.)),
    age: None,
    right_ascension: RightAscension::new(14, 50, 42),
    declination: Declination::new(74, 9, 20),
    distance: Length::from_astronomical_units(126. * AU_PER_LIGHT_YEARS),
};

//59
const RASALHAGUE_DATA: RealData = RealData {
    common_name: "Rasalhague",
    astronomical_name: "Alpha Ophiuchi",
    radius: Some(Length::from_astronomical_units(2.6 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.4 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(1.30),
    apparent_magnitude: 2.08,
    temperature: Some(Temperature::from_kelvin(8000.)),
    age: Some(Time::from_seconds(0.77 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(17, 34, 56),
    declination: Declination::new(12, 33, 37),
    distance: Length::from_astronomical_units(47. * AU_PER_LIGHT_YEARS),
};

//60
const ALGOL_DATA: RealData = RealData {
    common_name: "Algol",
    astronomical_name: "Beta Persei",
    radius: Some(Length::from_astronomical_units(2.73 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(3.17 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.18),
    apparent_magnitude: 2.09,
    temperature: Some(Temperature::from_kelvin(13_000.)),
    age: Some(Time::from_seconds(0.57 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(3, 8, 10),
    declination: Declination::new(40, 57, 20),
    distance: Length::from_astronomical_units(93. * AU_PER_LIGHT_YEARS),
};

//61
const ALMACH_DATA: RealData = RealData {
    common_name: "Almach",
    astronomical_name: "Gamma Andromedae",
    radius: Some(Length::from_astronomical_units(80. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(23.7 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.08),
    apparent_magnitude: 2.1,
    temperature: Some(Temperature::from_kelvin(4250.)),
    age: Some(Time::from_seconds(0.0065 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(2, 3, 54),
    declination: Declination::new(42, 19, 47),
    distance: Length::from_astronomical_units(355. * AU_PER_LIGHT_YEARS),
};

//62
const DENEBOLA_DATA: RealData = RealData {
    common_name: "Denebola",
    astronomical_name: "Beta Leonis",
    radius: Some(Length::from_astronomical_units(1.728 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.78 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(1.92),
    apparent_magnitude: 2.14,
    temperature: Some(Temperature::from_kelvin(8500.)),
    age: Some(Time::from_seconds(0.25 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(11, 49, 3),
    declination: Declination::new(14, 34, 19),
    distance: Length::from_astronomical_units(36. * AU_PER_LIGHT_YEARS),
};

//63
const NAVI_DATA: RealData = RealData {
    common_name: "Navi",
    astronomical_name: "Gamma Cassiopeiae",
    radius: Some(Length::from_astronomical_units(10. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(13. * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-4.22),
    apparent_magnitude: 2.20,
    temperature: Some(Temperature::from_kelvin(25_000.)),
    age: Some(Time::from_seconds(0.008 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(0, 56, 43),
    declination: Declination::new(60, 43, 0),
    distance: Length::from_astronomical_units(613. * AU_PER_LIGHT_YEARS),
};

//64
const MUHLIFAIN_DATA: RealData = RealData {
    common_name: "Muhlifain",
    astronomical_name: "Gamma Centauri",
    radius: None,
    mass: Some(Mass::from_kilograms(2.91 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.81),
    apparent_magnitude: 2.20,
    temperature: Some(Temperature::from_kelvin(9082.)),
    age: None,
    right_ascension: RightAscension::new(12, 41, 31),
    declination: Declination::new(-48, 57, 35),
    distance: Length::from_astronomical_units(130. * AU_PER_LIGHT_YEARS),
};

//65
const NAOS_DATA: RealData = RealData {
    common_name: "Naos",
    astronomical_name: "Zeta Puppis",
    radius: Some(Length::from_astronomical_units(20. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(56.1 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-5.95),
    apparent_magnitude: 2.21,
    temperature: Some(Temperature::from_kelvin(40_000.)),
    age: Some(Time::from_seconds(0.0032 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(8, 3, 35),
    declination: Declination::new(-40, 0, 12),
    distance: Length::from_astronomical_units(1399. * AU_PER_LIGHT_YEARS),
};

//66
const ASPIDISKE_DATA: RealData = RealData {
    common_name: "Aspidiske",
    astronomical_name: "Iota Carinae",
    radius: Some(Length::from_astronomical_units(43. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(7.4 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-4.42),
    apparent_magnitude: 2.21,
    temperature: Some(Temperature::from_kelvin(7500.)),
    age: Some(Time::from_seconds(0.0374 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(9, 17, 5),
    declination: Declination::new(-59, 16, 30),
    distance: Length::from_astronomical_units(694. * AU_PER_LIGHT_YEARS),
};

//67
const ALPHECCA_DATA: RealData = RealData {
    common_name: "Alphecca",
    astronomical_name: "Alpha Coronae Borealis",
    radius: Some(Length::from_astronomical_units(3. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.58 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(0.42),
    apparent_magnitude: 2.22,
    temperature: Some(Temperature::from_kelvin(9700.)),
    age: Some(Time::from_seconds(0.314 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(15, 34, 41),
    declination: Declination::new(26, 42, 53),
    distance: Length::from_astronomical_units(75. * AU_PER_LIGHT_YEARS),
};

//68
const SUHAIL_DATA: RealData = RealData {
    common_name: "Suhail",
    astronomical_name: "Lambda Velorum",
    radius: Some(Length::from_astronomical_units(210. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(7. * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.99),
    apparent_magnitude: 2.23,
    temperature: Some(Temperature::from_kelvin(3900.)),
    age: Some(Time::from_seconds(0.0316 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(9, 7, 60),
    declination: Declination::new(-43, 25, 57),
    distance: Length::from_astronomical_units(573. * AU_PER_LIGHT_YEARS),
};

//69
const SADIR_DATA: RealData = RealData {
    common_name: "Sadir",
    astronomical_name: "Gamma Cygni",
    radius: Some(Length::from_astronomical_units(150. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(12.11 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-6.12),
    apparent_magnitude: 2.23,
    temperature: Some(Temperature::from_kelvin(5790.)),
    age: Some(Time::from_seconds(0.012 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(20, 22, 14),
    declination: Declination::new(40, 15, 24),
    distance: Length::from_astronomical_units(1522. * AU_PER_LIGHT_YEARS),
};

//70
const MIZAR_DATA: RealData = RealData {
    common_name: "Mizar",
    astronomical_name: "Zeta Ursae Majoris",
    radius: Some(Length::from_astronomical_units(2.4 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.2 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(0.33),
    apparent_magnitude: 2.23,
    temperature: Some(Temperature::from_kelvin(9000.)),
    age: Some(Time::from_seconds(0.37 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(13, 23, 56),
    declination: Declination::new(54, 55, 31),
    distance: Length::from_astronomical_units(78. * AU_PER_LIGHT_YEARS),
};

//71
const SCHEDAR_DATA: RealData = RealData {
    common_name: "Schedar",
    astronomical_name: "Alpha Cassiopeiae",
    radius: Some(Length::from_astronomical_units(45.39 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(3.98 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.99),
    apparent_magnitude: 2.24,
    temperature: Some(Temperature::from_kelvin(4552.)),
    age: Some(Time::from_seconds(0.22 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(0, 40, 30),
    declination: Declination::new(56, 32, 14),
    distance: Length::from_astronomical_units(228. * AU_PER_LIGHT_YEARS),
};

//72
const ELTANIN_DATA: RealData = RealData {
    common_name: "Eltanin",
    astronomical_name: "Gamma Draconis",
    radius: Some(Length::from_astronomical_units(48.15 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.72 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.04),
    apparent_magnitude: 2.24,
    temperature: Some(Temperature::from_kelvin(3930.)),
    age: None,
    right_ascension: RightAscension::new(17, 56, 36),
    declination: Declination::new(51, 29, 20),
    distance: Length::from_astronomical_units(148. * AU_PER_LIGHT_YEARS),
};

//73
const MINTAKA_DATA: RealData = RealData {
    common_name: "Mintaka",
    astronomical_name: "Delta Orionis",
    radius: Some(Length::from_astronomical_units(16.5 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(24. * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-4.99),
    apparent_magnitude: 2.25,
    temperature: Some(Temperature::from_kelvin(29_500.)),
    age: None,
    right_ascension: RightAscension::new(5, 32, 0),
    declination: Declination::new(-0, 17, 57),
    distance: Length::from_astronomical_units(916. * AU_PER_LIGHT_YEARS),
};

//74
const CAPH_DATA: RealData = RealData {
    common_name: "Caph",
    astronomical_name: "Beta Cassiopeiae",
    radius: Some(Length::from_astronomical_units(3.5 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.91 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(1.17),
    apparent_magnitude: 2.28,
    temperature: Some(Temperature::from_kelvin(7079.)),
    age: Some(Time::from_seconds(1.1 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(0, 9, 11),
    declination: Declination::new(59, 8, 59),
    distance: Length::from_astronomical_units(54. * AU_PER_LIGHT_YEARS),
};

//75
const DSCHUBBA_DATA: RealData = RealData {
    common_name: "Dschubba",
    astronomical_name: "Delta Scorpii",
    radius: Some(Length::from_astronomical_units(6.7 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(13. * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.16),
    apparent_magnitude: 2.29,
    temperature: Some(Temperature::from_kelvin(27_400.)),
    age: Some(Time::from_seconds(0.0095 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(16, 0, 20),
    declination: Declination::new(-22, 37, 18),
    distance: Length::from_astronomical_units(401.5 * AU_PER_LIGHT_YEARS),
};

//76
const LARAWAG_DATA: RealData = RealData {
    common_name: "Larawag",
    astronomical_name: "Epsilon Scorpii",
    radius: Some(Length::from_astronomical_units(12.6 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.24 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(0.78),
    apparent_magnitude: 2.29,
    temperature: Some(Temperature::from_kelvin(4560.)),
    age: None,
    right_ascension: RightAscension::new(16, 50, 10),
    declination: Declination::new(-34, 17, 36),
    distance: Length::from_astronomical_units(65. * AU_PER_LIGHT_YEARS),
};

//77
const EPSILON_CENTAURI_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "Epsilon Centauri",
    radius: None,
    mass: Some(Mass::from_kilograms(11.6 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.02),
    apparent_magnitude: 2.29,
    temperature: Some(Temperature::from_kelvin(24_000.)),
    age: Some(Time::from_seconds(0.0158 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(13, 39, 53),
    declination: Declination::new(-53, 27, 59),
    distance: Length::from_astronomical_units(376. * AU_PER_LIGHT_YEARS),
};

//78
const ALPHA_LUPI_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Lupi",
    radius: None,
    mass: Some(Mass::from_kilograms(10.1 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.83),
    apparent_magnitude: 2.30,
    temperature: Some(Temperature::from_kelvin(21_820.)),
    age: Some(Time::from_seconds(0.018 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(14, 41, 56),
    declination: Declination::new(-47, 23, 18),
    distance: Length::from_astronomical_units(548. * AU_PER_LIGHT_YEARS),
};

//79
const ETA_CENTAURI_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "Eta Centauri",
    radius: Some(Length::from_astronomical_units(6.1 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(12.0 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-2.55),
    apparent_magnitude: 2.29,
    temperature: Some(Temperature::from_kelvin(25_700.)),
    age: Some(Time::from_seconds(0.0056 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(14, 35, 30),
    declination: Declination::new(-42, 9, 28),
    distance: Length::from_astronomical_units(308. * AU_PER_LIGHT_YEARS),
};

//80
const MERAK_DATA: RealData = RealData {
    common_name: "Merak",
    astronomical_name: "Beta Ursae Majoris",
    radius: Some(Length::from_astronomical_units(3.021 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.7 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(0.41),
    apparent_magnitude: 2.34,
    temperature: Some(Temperature::from_kelvin(9377.)),
    age: Some(Time::from_seconds(0.5 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(11, 1, 50),
    declination: Declination::new(56, 22, 57),
    distance: Length::from_astronomical_units(79. * AU_PER_LIGHT_YEARS),
};

//81
const IZAR_DATA: RealData = RealData {
    common_name: "Izar",
    astronomical_name: "Epsilon Bootis",
    radius: Some(Length::from_astronomical_units(33. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(4.6 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.69),
    apparent_magnitude: 2.35,
    temperature: Some(Temperature::from_kelvin(4550.)),
    age: Some(Time::from_seconds(0.0374 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(14, 44, 59),
    declination: Declination::new(27, 4, 27),
    distance: Length::from_astronomical_units(210. * AU_PER_LIGHT_YEARS),
};

//82
const ENIF_DATA: RealData = RealData {
    common_name: "Enif",
    astronomical_name: "Epsilon Pegasi",
    radius: Some(Length::from_astronomical_units(211. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(7.07 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-4.19),
    apparent_magnitude: 2.38,
    temperature: Some(Temperature::from_kelvin(3963.)),
    age: Some(Time::from_seconds(0.020 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(21, 44, 11),
    declination: Declination::new(9, 52, 30),
    distance: Length::from_astronomical_units(672. * AU_PER_LIGHT_YEARS),
};

//83
const GIRTAB_DATA: RealData = RealData {
    common_name: "Girtab",
    astronomical_name: "Kappa Scorpii",
    radius: Some(Length::from_astronomical_units(6.8 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(17. * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.38),
    apparent_magnitude: 2.39,
    temperature: Some(Temperature::from_kelvin(23_400.)),
    age: Some(Time::from_seconds(0.0251 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(17, 42, 29),
    declination: Declination::new(-39, 1, 48),
    distance: Length::from_astronomical_units(464. * AU_PER_LIGHT_YEARS),
};

//84
const ANKAA_DATA: RealData = RealData {
    common_name: "Ankaa",
    astronomical_name: "Alpha Phoenicis",
    radius: Some(Length::from_astronomical_units(15. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.57 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(0.52),
    apparent_magnitude: 2.4,
    temperature: Some(Temperature::from_kelvin(4436.)),
    age: None,
    right_ascension: RightAscension::new(0, 26, 17),
    declination: Declination::new(-42, 18, 21),
    distance: Length::from_astronomical_units(77. * AU_PER_LIGHT_YEARS),
};

//85
const PHECDA_DATA: RealData = RealData {
    common_name: "Phecda",
    astronomical_name: "Gamma Ursae Majoris",
    radius: Some(Length::from_astronomical_units(3.04 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.94 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(0.36),
    apparent_magnitude: 2.41,
    temperature: Some(Temperature::from_kelvin(9355.)),
    age: Some(Time::from_seconds(0.3 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(11, 53, 50),
    declination: Declination::new(53, 41, 41),
    distance: Length::from_astronomical_units(84. * AU_PER_LIGHT_YEARS),
};

//86
const SABIK_DATA: RealData = RealData {
    common_name: "Sabik",
    astronomical_name: "Eta Ophiuchi",
    radius: None,
    mass: Some(Mass::from_kilograms(2.966 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(0.37),
    apparent_magnitude: 2.43,
    temperature: Some(Temperature::from_kelvin(8900.)),
    age: None,
    right_ascension: RightAscension::new(17, 10, 23),
    declination: Declination::new(-15, 43, 30),
    distance: Length::from_astronomical_units(84. * AU_PER_LIGHT_YEARS),
};

//87
const SCHEAT_DATA: RealData = RealData {
    common_name: "Scheat",
    astronomical_name: "Beta Pegasi",
    radius: Some(Length::from_astronomical_units(95. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.1 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.49),
    apparent_magnitude: 2.44,
    temperature: Some(Temperature::from_kelvin(3689.)),
    age: None,
    right_ascension: RightAscension::new(23, 3, 46),
    declination: Declination::new(28, 4, 58),
    distance: Length::from_astronomical_units(199. * AU_PER_LIGHT_YEARS),
};

//88
const ALDERAMIN_DATA: RealData = RealData {
    common_name: "Alderamin",
    astronomical_name: "Alpha Cephei",
    radius: Some(Length::from_astronomical_units(2.4 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2. * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(1.58),
    apparent_magnitude: 2.45,
    temperature: Some(Temperature::from_kelvin(7700.)),
    age: Some(Time::from_seconds(0.82 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(21, 18, 35),
    declination: Declination::new(62, 35, 8),
    distance: Length::from_astronomical_units(49. * AU_PER_LIGHT_YEARS),
};

//89
const ALUDRA_DATA: RealData = RealData {
    common_name: "Aludra",
    astronomical_name: "Eta Canis Majoris",
    radius: Some(Length::from_astronomical_units(54. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(18.19 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-7.51),
    apparent_magnitude: 2.45,
    temperature: Some(Temperature::from_kelvin(15_500.)),
    age: Some(Time::from_seconds(0.0083 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(7, 24, 6),
    declination: Declination::new(-29, 18, 11),
    distance: Length::from_astronomical_units(3196. * AU_PER_LIGHT_YEARS),
};

//90
const MARKEB_DATA: RealData = RealData {
    common_name: "Markeb",
    astronomical_name: "Kappa Velorum",
    radius: Some(Length::from_astronomical_units(9.1 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(10.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.62),
    apparent_magnitude: 2.47,
    temperature: Some(Temperature::from_kelvin(23_000.)),
    age: Some(Time::from_seconds(0.018 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(9, 22, 7),
    declination: Declination::new(-55, 0, 38),
    distance: Length::from_astronomical_units(539. * AU_PER_LIGHT_YEARS),
};

//91
const ALJANAH_DATA: RealData = RealData {
    common_name: "Aljanah",
    astronomical_name: "Epsilon Cygni",
    radius: Some(Length::from_astronomical_units(10.82 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2. * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(0.76),
    apparent_magnitude: 2.48,
    temperature: Some(Temperature::from_kelvin(4710.)),
    age: Some(Time::from_seconds(1.5 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(20, 46, 13),
    declination: Declination::new(33, 58, 13),
    distance: Length::from_astronomical_units(72. * AU_PER_LIGHT_YEARS),
};

//92
const MARKAB_DATA: RealData = RealData {
    common_name: "Markab",
    astronomical_name: "Alpha Pegasi",
    radius: Some(Length::from_astronomical_units(4.62 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(3.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.67),
    apparent_magnitude: 2.49,
    temperature: Some(Temperature::from_kelvin(10_100.)),
    age: Some(Time::from_seconds(0.2 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(23, 4, 46),
    declination: Declination::new(15, 12, 19),
    distance: Length::from_astronomical_units(140. * AU_PER_LIGHT_YEARS),
};

//93
const HAN_DATA: RealData = RealData {
    common_name: "Han",
    astronomical_name: "Zeta Ophiuchi",
    radius: Some(Length::from_astronomical_units(8.5 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(20.2 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.20),
    apparent_magnitude: 2.54,
    temperature: Some(Temperature::from_kelvin(34_300.)),
    age: Some(Time::from_seconds(3. * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(16, 37, 10),
    declination: Declination::new(-10, 34, 2),
    distance: Length::from_astronomical_units(458. * AU_PER_LIGHT_YEARS),
};

//94
const MENKAR_DATA: RealData = RealData {
    common_name: "Menkar",
    astronomical_name: "Alpha Ceti",
    radius: Some(Length::from_astronomical_units(89. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.3 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.61),
    apparent_magnitude: 2.54,
    temperature: Some(Temperature::from_kelvin(3795.)),
    age: None,
    right_ascension: RightAscension::new(3, 2, 17),
    declination: Declination::new(4, 5, 23),
    distance: Length::from_astronomical_units(220. * AU_PER_LIGHT_YEARS),
};

//95
const ZETA_CENTAURI_DATA: RealData = RealData {
    common_name: "Alnair",
    astronomical_name: "Zeta Centauri",
    radius: Some(Length::from_astronomical_units(5.8 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(7.8 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-2.81),
    apparent_magnitude: 2.55,
    temperature: Some(Temperature::from_kelvin(23_561.)),
    age: Some(Time::from_seconds(0.04 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(13, 55, 33),
    declination: Declination::new(-47, 17, 18),
    distance: Length::from_astronomical_units(384. * AU_PER_LIGHT_YEARS),
};

//96
const ACRAB_DATA: RealData = RealData {
    common_name: "Acrab",
    astronomical_name: "Beta Scorpii",
    radius: Some(Length::from_astronomical_units(6.3 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(15.0 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.50),
    apparent_magnitude: 2.56,
    temperature: Some(Temperature::from_kelvin(28_000.)),
    age: None,
    right_ascension: RightAscension::new(16, 5, 26),
    declination: Declination::new(-19, 48, 20),
    distance: Length::from_astronomical_units(530. * AU_PER_LIGHT_YEARS),
};

//97
const ZOSMA_DATA: RealData = RealData {
    common_name: "Zosma",
    astronomical_name: "Delta Leonis",
    radius: Some(Length::from_astronomical_units(2.14 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.2 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(1.32),
    apparent_magnitude: 2.56,
    temperature: Some(Temperature::from_kelvin(8_296.)),
    age: Some(Time::from_seconds(0.65 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(11, 14, 7),
    declination: Declination::new(20, 31, 25),
    distance: Length::from_astronomical_units(58. * AU_PER_LIGHT_YEARS),
};

//98
const MA_WEI_DATA: RealData = RealData {
    common_name: "Ma Wei",
    astronomical_name: "Delta Centauri",
    radius: Some(Length::from_astronomical_units(6.5 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(8.7 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-2.84),
    apparent_magnitude: 2.58,
    temperature: Some(Temperature::from_kelvin(22_360.)),
    age: Some(Time::from_seconds(0.02 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(12, 8, 21),
    declination: Declination::new(-50, 43, 21),
    distance: Length::from_astronomical_units(395. * AU_PER_LIGHT_YEARS),
};

//99
const ARNEB_DATA: RealData = RealData {
    common_name: "Arneb",
    astronomical_name: "Alpha Leporis",
    radius: Some(Length::from_astronomical_units(75. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(13.9 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-5.40),
    apparent_magnitude: 2.58,
    temperature: Some(Temperature::from_kelvin(6_850.)),
    age: Some(Time::from_seconds(0.013 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(5, 32, 44),
    declination: Declination::new(-17, 49, 20),
    distance: Length::from_astronomical_units(1283. * AU_PER_LIGHT_YEARS),
};

//100
const GHURAB_DATA: RealData = RealData {
    common_name: "Ghurab",
    astronomical_name: "Gamma Corvi",
    radius: None,
    mass: Some(Mass::from_kilograms(4.2 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.94),
    apparent_magnitude: 2.58,
    temperature: Some(Temperature::from_kelvin(12_000.)),
    age: Some(Time::from_seconds(0.160 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(12, 15, 48),
    declination: Declination::new(-17, 32, 31),
    distance: Length::from_astronomical_units(165. * AU_PER_LIGHT_YEARS),
};

// Gaia data that was not found in the list of 100 brightest stars
// Designations can be checked under http://simbad.cds.unistra.fr/simbad/sim-fid

const TEJAT_DATA: RealData = RealData {
    common_name: "Tejat",
    astronomical_name: "Mu Geminorum",
    radius: Some(Length::from_astronomical_units(90. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.1 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.42),
    apparent_magnitude: 2.75,
    temperature: Some(Temperature::from_kelvin(3460.)),
    age: None,
    right_ascension: RightAscension::new(6, 22, 58),
    declination: Declination::new(22, 30, 49),
    distance: Length::from_astronomical_units(230. * AU_PER_LIGHT_YEARS),
};

const R_DORADUS_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "R Doradus",
    radius: Some(Length::from_astronomical_units(298. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(0.8 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(1.61),
    apparent_magnitude: 5.59,
    temperature: Some(Temperature::from_kelvin(2710.)),
    age: Some(Time::from_seconds(10. * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(4, 36, 46),
    declination: Declination::new(-62, 4, 38),
    distance: Length::from_astronomical_units(203.5 * AU_PER_LIGHT_YEARS),
};

const YED_PRIOR: RealData = RealData {
    common_name: "Yed Prior",
    astronomical_name: "Delta Ophiuchi",
    radius: Some(Length::from_astronomical_units(59. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.90),
    apparent_magnitude: 2.73,
    temperature: Some(Temperature::from_kelvin(3679.)),
    age: None,
    right_ascension: RightAscension::new(16, 14, 21),
    declination: Declination::new(-3, 41, 40),
    distance: Length::from_astronomical_units(171. * AU_PER_LIGHT_YEARS),
};

const GORGONEA_TERTIA_DATA: RealData = RealData {
    common_name: "Gorgonea Tertia",
    astronomical_name: "Rho Persei",
    radius: Some(Length::from_astronomical_units(143. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.9 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.67),
    apparent_magnitude: 3.32,
    temperature: Some(Temperature::from_kelvin(3479.)),
    age: Some(Time::from_seconds(0.440 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(3, 5, 11),
    declination: Declination::new(38, 50, 25),
    distance: Length::from_astronomical_units(325. * AU_PER_LIGHT_YEARS),
};

const NAMALWARID: RealData = RealData {
    common_name: "Namalwarid",
    astronomical_name: "Eta Sagittarii",
    radius: None,
    mass: None,
    luminosity: Luminosity::from_absolute_magnitude(-0.201),
    apparent_magnitude: 3.1,
    temperature: None,
    age: None,
    right_ascension: RightAscension::new(18, 17, 38),
    declination: Declination::new(-36, 45, 42),
    distance: Length::from_astronomical_units(149.1 * AU_PER_LIGHT_YEARS),
};

const HASSALEH: RealData = RealData {
    common_name: "Hassaleh",
    astronomical_name: "Iota Aurigae",
    radius: Some(Length::from_astronomical_units(127. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(7.1 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.20),
    apparent_magnitude: 2.69,
    temperature: Some(Temperature::from_kelvin(4160.)),
    age: Some(Time::from_seconds(0.04 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(4, 56, 60),
    declination: Declination::new(33, 9, 58),
    distance: Length::from_astronomical_units(490. * AU_PER_LIGHT_YEARS),
};

const PROPUS_DATA: RealData = RealData {
    common_name: "Propus",
    astronomical_name: "Eta Geminorum",
    radius: Some(Length::from_astronomical_units(275. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.84),
    apparent_magnitude: 3.31,
    temperature: Some(Temperature::from_kelvin(3502.)),
    age: Some(Time::from_seconds(0.81 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(6, 14, 53),
    declination: Declination::new(22, 30, 24),
    distance: Length::from_astronomical_units(349. * AU_PER_LIGHT_YEARS),
};

const ZAURAK_DATA: RealData = RealData {
    common_name: "Zaurak",
    astronomical_name: "Gamma Eridani",
    radius: Some(Length::from_astronomical_units(80. * AU_PER_SUN_RADII)),
    mass: None,
    luminosity: Luminosity::from_absolute_magnitude(-1.19),
    apparent_magnitude: 2.97,
    temperature: Some(Temperature::from_kelvin(3811.)),
    age: None,
    right_ascension: RightAscension::new(3, 58, 2),
    declination: Declination::new(-13, 30, 31),
    distance: Length::from_astronomical_units(221. * AU_PER_LIGHT_YEARS),
};

const KAUS_MEDIA_DATA: RealData = RealData {
    common_name: "Kaus Media",
    astronomical_name: "Delta Sagittarii",
    radius: Some(Length::from_astronomical_units(16. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(3.21 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-2.14),
    apparent_magnitude: 2.72,
    temperature: Some(Temperature::from_kelvin(4203.)),
    age: Some(Time::from_seconds(0.26 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(18, 20, 60),
    declination: Declination::new(-29, 49, 41),
    distance: Length::from_astronomical_units(305.5 * AU_PER_LIGHT_YEARS),
};

const BRACHIUM_DATA: RealData = RealData {
    common_name: "Brachium",
    astronomical_name: "Sigma Librae",
    radius: Some(Length::from_astronomical_units(108. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.2 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.5),
    apparent_magnitude: 3.21,
    temperature: Some(Temperature::from_kelvin(3596.)),
    age: None,
    right_ascension: RightAscension::new(15, 4, 4),
    declination: Declination::new(-25, 16, 55),
    distance: Length::from_astronomical_units(288. * AU_PER_LIGHT_YEARS),
};

const TANIA_AUSTRALIS_DATA: RealData = RealData {
    common_name: "Tania Australis",
    astronomical_name: "Mu Ursae Majoris",
    radius: Some(Length::from_astronomical_units(75. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(6.3 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.2),
    apparent_magnitude: 3.06,
    temperature: Some(Temperature::from_kelvin(3899.)),
    age: None,
    right_ascension: RightAscension::new(10, 22, 20),
    declination: Declination::new(41, 29, 58),
    distance: Length::from_astronomical_units(230.0 * AU_PER_LIGHT_YEARS),
};

const UNUKALHAI_DATA: RealData = RealData {
    common_name: "Unukalhai",
    astronomical_name: "Alpha Serpentis",
    radius: Some(Length::from_astronomical_units(13.48 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.66 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(0.88),
    apparent_magnitude: 2.63,
    temperature: Some(Temperature::from_kelvin(4498.)),
    age: None,
    right_ascension: RightAscension::new(15, 44, 16),
    declination: Declination::new(6, 25, 32),
    distance: Length::from_astronomical_units(74. * AU_PER_LIGHT_YEARS),
};

const R_LYRAE_DATE: RealData = RealData {
    common_name: "",
    astronomical_name: "R Lyrae",
    radius: None,
    mass: Some(Mass::from_kilograms(1.8 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.07),
    apparent_magnitude: 4.08,
    temperature: Some(Temperature::from_kelvin(3313.)),
    age: None,
    right_ascension: RightAscension::new(18, 55, 20),
    declination: Declination::new(43, 56, 46),
    distance: Length::from_astronomical_units(349.4 * AU_PER_LIGHT_YEARS),
};

const BETA_ARAE_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Arae",
    radius: Some(Length::from_astronomical_units(142. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(8.21 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.49),
    apparent_magnitude: 2.84,
    temperature: Some(Temperature::from_kelvin(4197.)),
    age: Some(Time::from_seconds(0.05 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(17, 25, 18),
    declination: Declination::new(-55, 31, 48),
    distance: Length::from_astronomical_units(602.6 * AU_PER_LIGHT_YEARS),
};

const APLHA_TUCANAE_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Tucanae",
    radius: Some(Length::from_astronomical_units(37. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.5 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.05),
    apparent_magnitude: 2.87,
    temperature: Some(Temperature::from_kelvin(4300.)),
    age: None,
    right_ascension: RightAscension::new(22, 18, 30),
    declination: Declination::new(-60, 15, 35),
    distance: Length::from_astronomical_units(198.5 * AU_PER_LIGHT_YEARS),
};

const MINELAUVA_DATA: RealData = RealData {
    common_name: "Minelauva",
    astronomical_name: "Delta Virginis",
    radius: Some(Length::from_astronomical_units(48. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.4 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.575),
    apparent_magnitude: 3.39,
    temperature: Some(Temperature::from_kelvin(3999.)),
    age: None,
    right_ascension: RightAscension::new(12, 55, 36),
    declination: Declination::new(3, 23, 51),
    distance: Length::from_astronomical_units(202.4 * AU_PER_LIGHT_YEARS),
};

const CEBALRAI_DATA: RealData = RealData {
    common_name: "Cebalrai",
    astronomical_name: "Beta Ophiuchi",
    radius: Some(Length::from_astronomical_units(12.42 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1.13 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(0.77),
    apparent_magnitude: 2.76,
    temperature: Some(Temperature::from_kelvin(4467.)),
    age: Some(Time::from_seconds(3.82 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(17, 43, 28),
    declination: Declination::new(4, 34, 2),
    distance: Length::from_astronomical_units(81.8 * AU_PER_LIGHT_YEARS),
};

const KRAZ_DATA: RealData = RealData {
    common_name: "Kraz",
    astronomical_name: "Beta Corvi",
    radius: Some(Length::from_astronomical_units(16. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(3.7 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.61),
    apparent_magnitude: 2.65,
    temperature: Some(Temperature::from_kelvin(5100.)),
    age: Some(Time::from_seconds(0.206 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(12, 34, 23),
    declination: Declination::new(-23, 23, 48),
    distance: Length::from_astronomical_units(146. * AU_PER_LIGHT_YEARS),
};

const ERAKIS_DATA: RealData = RealData {
    common_name: "Erakis",
    astronomical_name: "Mu Cephei",
    radius: Some(Length::from_astronomical_units(972. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(19.2 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-6.5),
    apparent_magnitude: 3.43,
    temperature: Some(Temperature::from_kelvin(3551.)),
    age: Some(Time::from_seconds(0.01 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(21, 43, 30),
    declination: Declination::new(58, 46, 48),
    distance: Length::from_astronomical_units(3066. * AU_PER_LIGHT_YEARS),
};

const GAMMA_HYDRI_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Hydri",
    radius: Some(Length::from_astronomical_units(62. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(1. * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-0.83),
    apparent_magnitude: 3.26,
    temperature: Some(Temperature::from_kelvin(3499.)),
    age: None,
    right_ascension: RightAscension::new(3, 47, 14),
    declination: Declination::new(-74, 14, 20),
    distance: Length::from_astronomical_units(214. * AU_PER_LIGHT_YEARS),
};

const ALPHA_LYNCIS_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Lyncis",
    radius: Some(Length::from_astronomical_units(54.5 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2. * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-1.02),
    apparent_magnitude: 3.14,
    temperature: Some(Temperature::from_kelvin(3882.)),
    age: Some(Time::from_seconds(1.4 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(9, 21, 3),
    declination: Declination::new(34, 23, 33),
    distance: Length::from_astronomical_units(221.9 * AU_PER_LIGHT_YEARS),
};

const ATHEBYNE_DATA: RealData = RealData {
    common_name: "Athebyne",
    astronomical_name: "Eta Draconis",
    radius: Some(Length::from_astronomical_units(11. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(2.55 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(0.58),
    apparent_magnitude: 2.73,
    temperature: Some(Temperature::from_kelvin(5055.)),
    age: Some(Time::from_seconds(0.55 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(16, 23, 59),
    declination: Declination::new(61, 30, 51),
    distance: Length::from_astronomical_units(87.68 * AU_PER_LIGHT_YEARS),
};

// http://www.avastronomyclub.org/skymap/d/skymap.php

const AHADI_DATA: RealData = RealData {
    common_name: "Ahadi",
    astronomical_name: "Pi Puppis",
    radius: Some(Length::from_astronomical_units(235. * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(11.7 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-4.92),
    apparent_magnitude: 2.71,
    temperature: Some(Temperature::from_kelvin(4000.)),
    age: Some(Time::from_seconds(0.02 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(7, 17, 9),
    declination: Declination::new(-37, 5, 51),
    distance: Length::from_astronomical_units(1094. * AU_PER_LIGHT_YEARS),
};

const TARAZED_DATA: RealData = RealData {
    common_name: "Tarazed",
    astronomical_name: "Gamma Aquilae",
    radius: Some(Length::from_astronomical_units(91.82 * AU_PER_SUN_RADII)),
    mass: Some(Mass::from_kilograms(3.51 * KILOGRAMS_PER_SOLAR_MASS)),
    luminosity: Luminosity::from_absolute_magnitude(-3.03),
    apparent_magnitude: 2.72,
    temperature: Some(Temperature::from_kelvin(4098.)),
    age: Some(Time::from_seconds(0.270 * SECONDS_PER_BILLION_YEARS)),
    right_ascension: RightAscension::new(19, 46, 16),
    declination: Declination::new(10, 36, 48),
    distance: Length::from_astronomical_units(460.5 * AU_PER_LIGHT_YEARS),
};

// RealData = RealData {
//     common_name: "",
//     astronomical_name: "",
//     radius: Some(Length::from_astronomical_units(. * AU_PER_SUN_RADII)),
//     mass: Some(Mass::from_kilograms( * KILOGRAMS_PER_SOLAR_MASS)),
//     luminosity: Luminosity::from_absolute_magnitude(),
//     apparent_magnitude: ,
//     temperature: Some(Temperature::from_kelvin()),
//     age: Some(Time::from_seconds(* SECONDS_PER_BILLION_YEARS)),
//     right_ascension: RightAscension::new(),
//     declination: Declination::new(),
//     distance: Length::from_astronomical_units(. * AU_PER_LIGHT_YEARS),
// };

pub const BRIGHTEST_STARS: [RealData; 124] = [
    SIRIUS_DATA,
    CANOPUS_DATA,
    ARCTURUS_DATA,
    RIGEL_KENTAURUS_DATA,
    VEGA_DATA,
    CAPELLA_DATA,
    RIGEL_DATA,
    PROCYON_DATA,
    BETELGEUSE_DATA,
    ACHERNAR_DATA,
    HADAR_DATA,
    ALTAIR_DATA,
    ACRUX_DATA,
    ALDEBARAN_DATA,
    SPICA_DATA,
    ANTARES_DATA,
    POLLUX_DATA,
    FORMALHAUT_DATA,
    DENEB_DATA,
    MIMOSA_DATA,
    REGULUS_DATA,
    ADHARA_DATA,
    CASTOR_DATA,
    GACRUX_DATA,
    SHAULA_DATA,
    BELLATRIX_DATA,
    ALNATH_DATA,
    MIAPLACIDUS_DATA,
    ALNILAM_DATA,
    ALNAIR_DATA,
    ALNITAK_DATA,
    REGOR_DATA,
    ALIOTH_DATA,
    KAUS_AUSTRALIS_DATA,
    MIRPHAK_DATA,
    DUBHE_DATA,
    WEZEN_DATA,
    ALKAID_DATA,
    SARGAS_DATA,
    AVIOR_DATA,
    MENKALINAN_DATA,
    ATRIA_DATA,
    ALSEPHINA_DATA,
    ALHENA_DATA,
    PEACOCK_DATA,
    POLARIS_DATA,
    MIRZAM_DATA,
    ALPHARD_DATA,
    ALGIEBA_DATA,
    HAMAL_DATA,
    DIPHDA_DATA,
    NUNKI_DATA,
    MENKENT_DATA,
    SAIPH_DATA,
    ALPHERATZ_DATA,
    TIAKI_DATA,
    MIRACH_DATA,
    KOCHAB_DATA,
    RASALHAGUE_DATA,
    ALGOL_DATA,
    ALMACH_DATA,
    DENEBOLA_DATA,
    NAVI_DATA,
    MUHLIFAIN_DATA,
    NAOS_DATA,
    ASPIDISKE_DATA,
    ALPHECCA_DATA,
    SUHAIL_DATA,
    SADIR_DATA,
    MIZAR_DATA,
    SCHEDAR_DATA,
    ELTANIN_DATA,
    MINTAKA_DATA,
    CAPH_DATA,
    DSCHUBBA_DATA,
    LARAWAG_DATA,
    EPSILON_CENTAURI_DATA,
    ALPHA_LUPI_DATA,
    ETA_CENTAURI_DATA,
    MERAK_DATA,
    IZAR_DATA,
    ENIF_DATA,
    GIRTAB_DATA,
    ANKAA_DATA,
    PHECDA_DATA,
    SABIK_DATA,
    SCHEAT_DATA,
    ALDERAMIN_DATA,
    ALUDRA_DATA,
    MARKEB_DATA,
    ALJANAH_DATA,
    MARKAB_DATA,
    HAN_DATA,
    MENKAR_DATA,
    ZETA_CENTAURI_DATA,
    ACRAB_DATA,
    ZOSMA_DATA,
    MA_WEI_DATA,
    ARNEB_DATA,
    GHURAB_DATA,
    TEJAT_DATA,
    R_DORADUS_DATA,
    YED_PRIOR,
    GORGONEA_TERTIA_DATA,
    NAMALWARID,
    HASSALEH,
    PROPUS_DATA,
    ZAURAK_DATA,
    KAUS_MEDIA_DATA,
    BRACHIUM_DATA,
    TANIA_AUSTRALIS_DATA,
    UNUKALHAI_DATA,
    R_LYRAE_DATE,
    BETA_ARAE_DATA,
    APLHA_TUCANAE_DATA,
    MINELAUVA_DATA,
    CEBALRAI_DATA,
    KRAZ_DATA,
    ERAKIS_DATA,
    GAMMA_HYDRI_DATA,
    ALPHA_LYNCIS_DATA,
    ATHEBYNE_DATA,
    AHADI_DATA,
    TARAZED_DATA,
];
