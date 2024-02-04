use crate::{
    coordinates::{
        declination::{Declination, Sgn},
        right_ascension::RightAscension,
    },
    stars::real_data::RealData,
    units::{
        distance::{DISTANCE_ZERO, LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};
use simple_si_units::base::{Distance, Mass, Temperature, Time};

//https://web.pa.msu.edu/people/horvatin/Astronomy_Facts/brightest_stars.html

pub const SUN_DATA: RealData = RealData {
    common_name: "Sun",
    astronomical_name: "Sol",
    mass: Some(SOLAR_MASS),
    radius: Some(SOLAR_RADIUS),
    absolute_magnitude: 4.83,
    apparent_magnitude: -26.74, //seen from earth
    temperature: Some(Temperature { K: 5778.0 }),
    age: Some(Time {
        s: 4.6 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(0, 0, 0),
    declination: Declination::new(Sgn::Pos, 0, 0, 0),
    distance: DISTANCE_ZERO,
};

//1
const SIRIUS_DATA: RealData = RealData {
    common_name: "Sirius",
    astronomical_name: "Alpha Canis Majoris",
    radius: Some(Distance {
        m: 1.711 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.063 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.45,
    apparent_magnitude: -1.44,
    temperature: Some(Temperature { K: 9940. }),
    age: Some(Time {
        s: 0.242 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(6, 45, 9),
    declination: Declination::new(Sgn::Neg, 16, 42, 58),
    distance: Distance {
        m: 9. * LIGHT_YEAR.m,
    },
};

//2
const CANOPUS_DATA: RealData = RealData {
    common_name: "Canopus",
    astronomical_name: "Alpha Carinae",
    radius: Some(Distance {
        m: 72. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 9. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -5.53,
    apparent_magnitude: -0.62,
    temperature: Some(Temperature { K: 7400. }),
    age: Some(Time {
        s: 0.0251 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(6, 23, 57),
    declination: Declination::new(Sgn::Neg, 52, 41, 44),
    distance: Distance {
        m: 313. * LIGHT_YEAR.m,
    },
};

//3
const ARCTURUS_DATA: RealData = RealData {
    common_name: "Arcturus",
    astronomical_name: "Alpha Bootis",
    radius: Some(Distance {
        m: 25.4 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.08 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.31,
    apparent_magnitude: -0.05,
    temperature: Some(Temperature { K: 4286. }),
    age: Some(Time {
        s: 7.1 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(14, 15, 40),
    declination: Declination::new(Sgn::Pos, 19, 10, 56),
    distance: Distance {
        m: 37. * LIGHT_YEAR.m,
    },
};

//4
const RIGEL_KENTAURUS_DATA: RealData = RealData {
    common_name: "Rigel Kentaurus",
    astronomical_name: "Alpha Centauri",
    radius: Some(Distance {
        m: 1.2175 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.0788 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 4.34,
    apparent_magnitude: -0.27,
    temperature: Some(Temperature { K: 5790. }),
    age: Some(Time {
        s: 4.85 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(14, 39, 36),
    declination: Declination::new(Sgn::Neg, 60, 50, 2),
    distance: Distance {
        m: 4. * LIGHT_YEAR.m,
    },
};

//5
const VEGA_DATA: RealData = RealData {
    common_name: "Vega",
    astronomical_name: "Alpha Lyrae",
    radius: Some(Distance {
        m: 2.362 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.135 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.58,
    apparent_magnitude: 0.03,
    temperature: Some(Temperature { K: 9602. }),
    age: Some(Time {
        s: 0.455 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(18, 36, 56),
    declination: Declination::new(Sgn::Pos, 38, 47, 1),
    distance: Distance {
        m: 25. * LIGHT_YEAR.m,
    },
};

//6
const CAPELLA_DATA: RealData = RealData {
    common_name: "Capella",
    astronomical_name: "Alpha Aurigae",
    radius: Some(Distance {
        m: 11.98 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.5687 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.48,
    apparent_magnitude: 0.08,
    temperature: Some(Temperature { K: 4970. }),
    age: Some(Time {
        s: 0.620 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 16, 41),
    declination: Declination::new(Sgn::Pos, 45, 59, 53),
    distance: Distance {
        m: 42. * LIGHT_YEAR.m,
    },
};

//7
const RIGEL_DATA: RealData = RealData {
    common_name: "Rigel",
    astronomical_name: "Beta Orionis",
    radius: Some(Distance {
        m: 78.9 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 21. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -6.69,
    apparent_magnitude: 0.18,
    temperature: Some(Temperature { K: 12_100. }),
    age: Some(Time {
        s: 0.008 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 14, 32),
    declination: Declination::new(Sgn::Neg, 8, 12, 6),
    distance: Distance {
        m: 773. * LIGHT_YEAR.m,
    },
};

//8
const PROCYON_DATA: RealData = RealData {
    common_name: "Procyon",
    astronomical_name: "Alpha Canis Minoris",
    radius: Some(Distance {
        m: 2.048 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.499 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 2.68,
    apparent_magnitude: 0.40,
    temperature: Some(Temperature { K: 6530. }),
    age: Some(Time {
        s: 1.37 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(7, 39, 18),
    declination: Declination::new(Sgn::Pos, 5, 13, 30),
    distance: Distance {
        m: 11. * LIGHT_YEAR.m,
    },
};

//9
const BETELGEUSE_DATA: RealData = RealData {
    common_name: "Betelgeuse",
    astronomical_name: "Alpha Orionis",
    radius: Some(Distance {
        m: 887. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 16.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -5.14,
    apparent_magnitude: 0.9,
    temperature: Some(Temperature { K: 3600. }),
    age: Some(Time {
        s: 0.008 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 55, 10),
    declination: Declination::new(Sgn::Pos, 7, 24, 25),
    distance: Distance {
        m: 522. * LIGHT_YEAR.m,
    },
};

//10
const ACHERNAR_DATA: RealData = RealData {
    common_name: "Achernar",
    astronomical_name: "Alpha Eridani",
    radius: Some(Distance {
        m: 6.78 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 6.0 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -2.77,
    apparent_magnitude: 0.45,
    temperature: Some(Temperature { K: 14_000. }),
    age: Some(Time {
        s: 0.063 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(1, 37, 43),
    declination: Declination::new(Sgn::Neg, 57, 14, 12),
    distance: Distance {
        m: 144. * LIGHT_YEAR.m,
    },
};

//11
const HADAR_DATA: RealData = RealData {
    common_name: "Hadar",
    astronomical_name: "Beta Centauri",
    radius: Some(Distance {
        m: 9. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 12.02 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -5.42,
    apparent_magnitude: 0.61,
    temperature: Some(Temperature { K: 25_000. }),
    age: Some(Time {
        s: 0.0141 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(14, 3, 49),
    declination: Declination::new(Sgn::Neg, 60, 22, 23),
    distance: Distance {
        m: 526. * LIGHT_YEAR.m,
    },
};

//12
const ALTAIR_DATA: RealData = RealData {
    common_name: "Altair",
    astronomical_name: "Alpha Aquilae",
    radius: Some(Distance {
        m: 1.63 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.86 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 2.20,
    apparent_magnitude: 0.76,
    temperature: Some(Temperature { K: 7670. }),
    age: Some(Time {
        s: 0.100 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(19, 50, 47),
    declination: Declination::new(Sgn::Pos, 8, 52, 6),
    distance: Distance {
        m: 17. * LIGHT_YEAR.m,
    },
};

//13
const ACRUX_DATA: RealData = RealData {
    common_name: "Acrux",
    astronomical_name: "Alpha Crucis",
    radius: Some(Distance {
        m: 7.8 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 17.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.19,
    apparent_magnitude: 0.77,
    temperature: Some(Temperature { K: 24_000. }),
    age: Some(Time {
        s: 0.0108 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 26, 36),
    declination: Declination::new(Sgn::Neg, 63, 5, 57),
    distance: Distance {
        m: 321. * LIGHT_YEAR.m,
    },
};

//14
const ALDEBARAN_DATA: RealData = RealData {
    common_name: "Aldebaran",
    astronomical_name: "Alpha Tauri",
    radius: Some(Distance {
        m: 45.1 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.16 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.63,
    apparent_magnitude: 0.87,
    temperature: Some(Temperature { K: 3900. }),
    age: Some(Time {
        s: 6.4 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(4, 35, 55),
    declination: Declination::new(Sgn::Pos, 16, 30, 33),
    distance: Distance {
        m: 65. * LIGHT_YEAR.m,
    },
};

//15
const SPICA_DATA: RealData = RealData {
    common_name: "Spica",
    astronomical_name: "Alpha Virginis",
    radius: Some(Distance {
        m: 7.47 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 11.43 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.55,
    apparent_magnitude: 0.98,
    temperature: Some(Temperature { K: 22_300. }),
    age: Some(Time {
        s: 0.0125 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(13, 25, 12),
    declination: Declination::new(Sgn::Neg, 11, 9, 41),
    distance: Distance {
        m: 262. * LIGHT_YEAR.m,
    },
};

//16
const ANTARES_DATA: RealData = RealData {
    common_name: "Antares",
    astronomical_name: "Alpha Scorpii",
    radius: Some(Distance {
        m: 680. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 13.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -5.28,
    apparent_magnitude: 1.06,
    temperature: Some(Temperature { K: 3660. }),
    age: Some(Time {
        s: 0.015 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(16, 29, 24),
    declination: Declination::new(Sgn::Neg, 26, 25, 55),
    distance: Distance {
        m: 604. * LIGHT_YEAR.m,
    },
};

//17
const POLLUX_DATA: RealData = RealData {
    common_name: "Pollux",
    astronomical_name: "Beta Geminorum",
    radius: Some(Distance {
        m: 9.06 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.91 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.09,
    apparent_magnitude: 1.16,
    temperature: Some(Temperature { K: 4586. }),
    age: Some(Time {
        s: 0.724 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(7, 45, 19),
    declination: Declination::new(Sgn::Pos, 28, 1, 34),
    distance: Distance {
        m: 34. * LIGHT_YEAR.m,
    },
};

//18
const FORMALHAUT_DATA: RealData = RealData {
    common_name: "Formalhaut",
    astronomical_name: "Alpha Piscis Austrini",
    radius: Some(Distance {
        m: 1.842 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.92 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.74,
    apparent_magnitude: 1.17,
    temperature: Some(Temperature { K: 8590. }),
    age: Some(Time {
        s: 0.44 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(22, 57, 39),
    declination: Declination::new(Sgn::Neg, 29, 37, 20),
    distance: Distance {
        m: 25. * LIGHT_YEAR.m,
    },
};

//19
const DENEB_DATA: RealData = RealData {
    common_name: "Deneb",
    astronomical_name: "Alpha Cygni",
    radius: Some(Distance {
        m: 203. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 19. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -7.13,
    apparent_magnitude: 1.25,
    temperature: Some(Temperature { K: 8515. }),
    age: None,
    right_ascension: RightAscension::new(20, 41, 26),
    declination: Declination::new(Sgn::Pos, 45, 16, 49),
    distance: Distance {
        m: 1548. * LIGHT_YEAR.m,
    },
};

//20
const MIMOSA_DATA: RealData = RealData {
    common_name: "Mimosa",
    astronomical_name: "Beta Crucis",
    radius: Some(Distance {
        m: 8.4 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 16. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.92,
    apparent_magnitude: 1.25,
    temperature: Some(Temperature { K: 27_000. }),
    age: Some(Time {
        s: 0.010 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 47, 43),
    declination: Declination::new(Sgn::Neg, 59, 41, 20),
    distance: Distance {
        m: 352. * LIGHT_YEAR.m,
    },
};

//21
const REGULUS_DATA: RealData = RealData {
    common_name: "Regulus",
    astronomical_name: "Alpha Leonis",
    radius: Some(Distance {
        m: 4.35 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.52,
    apparent_magnitude: 1.36,
    temperature: Some(Temperature { K: 11_668. }),
    age: Some(Time {
        s: 1. * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(10, 8, 22),
    declination: Declination::new(Sgn::Pos, 11, 58, 2),
    distance: Distance {
        m: 77. * LIGHT_YEAR.m,
    },
};

//22
const ADHARA_DATA: RealData = RealData {
    common_name: "Adhara",
    astronomical_name: "Epsilon Canis Majoris",
    radius: Some(Distance {
        m: 13.9 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 12.6 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.10,
    apparent_magnitude: 1.5,
    temperature: Some(Temperature { K: 22_900. }),
    age: Some(Time {
        s: 0.0225 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(6, 58, 38),
    declination: Declination::new(Sgn::Neg, 28, 58, 19),
    distance: Distance {
        m: 431. * LIGHT_YEAR.m,
    },
};

//23
const CASTOR_DATA: RealData = RealData {
    common_name: "Castor",
    astronomical_name: "Alpha Geminorum",
    radius: Some(Distance {
        m: 2.089 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.37 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.59,
    apparent_magnitude: 1.58,
    temperature: Some(Temperature { K: 10_286. }),
    age: Some(Time {
        s: 0.290 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(7, 34, 36),
    declination: Declination::new(Sgn::Pos, 31, 53, 18),
    distance: Distance {
        m: 52. * LIGHT_YEAR.m,
    },
};

//24
const GACRUX_DATA: RealData = RealData {
    common_name: "Gacrux",
    astronomical_name: "Gamma Crucis",
    radius: Some(Distance {
        m: 120. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.56,
    apparent_magnitude: 1.59,
    temperature: Some(Temperature { K: 3689. }),
    age: None,
    right_ascension: RightAscension::new(12, 31, 10),
    declination: Declination::new(Sgn::Neg, 57, 6, 48),
    distance: Distance {
        m: 88. * LIGHT_YEAR.m,
    },
};

//25
const SHAULA_DATA: RealData = RealData {
    common_name: "Shaula",
    astronomical_name: "Lambda Scorpii",
    radius: Some(Distance {
        m: 8.8 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 10.4 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.8,
    apparent_magnitude: 1.62,
    temperature: Some(Temperature { K: 25_000. }),
    age: None,
    right_ascension: RightAscension::new(17, 33, 37),
    declination: Declination::new(Sgn::Neg, 37, 6, 14),
    distance: Distance {
        m: 600. * LIGHT_YEAR.m,
    },
};

//26
const BELLATRIX_DATA: RealData = RealData {
    common_name: "Bellatrix",
    astronomical_name: "Gamma Orionis",
    radius: Some(Distance {
        m: 5.75 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 7.7 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -2.72,
    apparent_magnitude: 1.64,
    temperature: Some(Temperature { K: 21_800. }),
    age: Some(Time {
        s: 0.0252 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 25, 8),
    declination: Declination::new(Sgn::Pos, 6, 20, 59),
    distance: Distance {
        m: 243. * LIGHT_YEAR.m,
    },
};

//27
const ALNATH_DATA: RealData = RealData {
    common_name: "Alnath",
    astronomical_name: "Beta Tauri",
    radius: Some(Distance {
        m: 4.2 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 5.0 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.37,
    apparent_magnitude: 1.65,
    temperature: Some(Temperature { K: 13_824. }),
    age: Some(Time {
        s: 0.1 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 26, 18),
    declination: Declination::new(Sgn::Pos, 28, 36, 27),
    distance: Distance {
        m: 131. * LIGHT_YEAR.m,
    },
};

//28
const MIAPLACIDUS_DATA: RealData = RealData {
    common_name: "Miaplacidus",
    astronomical_name: "Beta Carinae",
    radius: Some(Distance {
        m: 6.8 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.99,
    apparent_magnitude: 1.67,
    temperature: Some(Temperature { K: 8866. }),
    age: Some(Time {
        s: 0.260 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(9, 13, 12),
    declination: Declination::new(Sgn::Neg, 69, 43, 2),
    distance: Distance {
        m: 111. * LIGHT_YEAR.m,
    },
};

//29
const ALNILAM_DATA: RealData = RealData {
    common_name: "Alnilam",
    astronomical_name: "Epsilon Orionis",
    radius: Some(Distance {
        m: 42. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 64.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -6.38,
    apparent_magnitude: 1.69,
    temperature: Some(Temperature { K: 27_000. }),
    age: Some(Time {
        s: 0.0057 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 36, 13),
    declination: Declination::new(Sgn::Neg, 1, 12, 7),
    distance: Distance {
        m: 1342. * LIGHT_YEAR.m,
    },
};

//30
const ALNAIR_DATA: RealData = RealData {
    common_name: "Alnair",
    astronomical_name: "Alpha Gruis",
    radius: Some(Distance {
        m: 3.4 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 4. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.73,
    apparent_magnitude: 1.73,
    temperature: Some(Temperature { K: 13_920. }),
    age: Some(Time {
        s: 0.1 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(22, 8, 14),
    declination: Declination::new(Sgn::Neg, 46, 57, 40),
    distance: Distance {
        m: 101. * LIGHT_YEAR.m,
    },
};

//31
const ALNITAK_DATA: RealData = RealData {
    common_name: "Alnitak",
    astronomical_name: "Zeta Orionis",
    radius: Some(Distance {
        m: 20. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 33.0 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -5.26,
    apparent_magnitude: 1.74,
    temperature: Some(Temperature { K: 29_500. }),
    age: Some(Time {
        s: 0.0064 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 40, 46),
    declination: Declination::new(Sgn::Neg, 1, 56, 34),
    distance: Distance {
        m: 817. * LIGHT_YEAR.m,
    },
};

//32
const REGOR_DATA: RealData = RealData {
    common_name: "Regor",
    astronomical_name: "Gamma Velorum",
    radius: Some(Distance {
        m: 17. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 28.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -5.31,
    apparent_magnitude: 1.75,
    temperature: Some(Temperature { K: 35_000. }),
    age: Some(Time {
        s: 0.0045 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(8, 9, 32),
    declination: Declination::new(Sgn::Neg, 47, 20, 12),
    distance: Distance {
        m: 840. * LIGHT_YEAR.m,
    },
};

//33
const ALIOTH_DATA: RealData = RealData {
    common_name: "Alioth",
    astronomical_name: "Epsilon Ursae Majoris",
    radius: Some(Distance {
        m: 4.14 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.91 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.21,
    apparent_magnitude: 1.76,
    temperature: Some(Temperature { K: 9_020. }),
    age: Some(Time {
        s: 0.3 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 54, 2),
    declination: Declination::new(Sgn::Pos, 55, 57, 36),
    distance: Distance {
        m: 81. * LIGHT_YEAR.m,
    },
};

//34
const KAUS_AUSTRALIS_DATA: RealData = RealData {
    common_name: "Kaus Australis",
    astronomical_name: "Epsilon Sagittarii",
    radius: Some(Distance {
        m: 6.8 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.515 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.44,
    apparent_magnitude: 1.79,
    temperature: Some(Temperature { K: 9960. }),
    age: Some(Time {
        s: 0.232 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(18, 24, 10),
    declination: Declination::new(Sgn::Neg, 34, 23, 5),
    distance: Distance {
        m: 145. * LIGHT_YEAR.m,
    },
};

//35
const MIRPHAK_DATA: RealData = RealData {
    common_name: "Mirphak",
    astronomical_name: "Alpha Persei",
    radius: Some(Distance {
        m: 68. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 8.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.50,
    apparent_magnitude: 1.79,
    temperature: Some(Temperature { K: 6350. }),
    age: Some(Time {
        s: 0.041 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(3, 24, 19),
    declination: Declination::new(Sgn::Pos, 49, 51, 40),
    distance: Distance {
        m: 592. * LIGHT_YEAR.m,
    },
};

//36
const DUBHE_DATA: RealData = RealData {
    common_name: "Dubhe",
    astronomical_name: "Alpha Ursae Majoris",
    radius: Some(Distance {
        m: 17.03 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.44 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.08,
    apparent_magnitude: 1.81,
    temperature: Some(Temperature { K: 5012. }),
    age: Some(Time {
        s: 0.28 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(11, 3, 44),
    declination: Declination::new(Sgn::Pos, 61, 45, 4),
    distance: Distance {
        m: 124. * LIGHT_YEAR.m,
    },
};

//37
const WEZEN_DATA: RealData = RealData {
    common_name: "Wezen",
    astronomical_name: "Delta Canis Majoris",
    radius: Some(Distance {
        m: 215. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 16.9 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -6.87,
    apparent_magnitude: 1.83,
    temperature: Some(Temperature { K: 6390. }),
    age: Some(Time {
        s: 0.012 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(7, 8, 23),
    declination: Declination::new(Sgn::Neg, 26, 23, 36),
    distance: Distance {
        m: 1791. * LIGHT_YEAR.m,
    },
};

//38
const ALKAID_DATA: RealData = RealData {
    common_name: "Alkaid",
    astronomical_name: "Eta Ursae Majoris",
    radius: Some(Distance {
        m: 3.4 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 6.1 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.60,
    apparent_magnitude: 1.85,
    temperature: Some(Temperature { K: 15_540. }),
    age: Some(Time {
        s: 0.01 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(13, 47, 32),
    declination: Declination::new(Sgn::Pos, 49, 18, 48),
    distance: Distance {
        m: 101. * LIGHT_YEAR.m,
    },
};

//39
const SARGAS_DATA: RealData = RealData {
    common_name: "Sargas",
    astronomical_name: "Theta Scorpii",
    radius: Some(Distance {
        m: 26.3 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.1 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -2.75,
    apparent_magnitude: 1.86,
    temperature: Some(Temperature { K: 6294. }),
    age: None,
    right_ascension: RightAscension::new(17, 37, 19),
    declination: Declination::new(Sgn::Neg, 42, 59, 52),
    distance: Distance {
        m: 272. * LIGHT_YEAR.m,
    },
};

//40
const AVIOR_DATA: RealData = RealData {
    common_name: "Avior",
    astronomical_name: "Epsilon Carinae",
    radius: None,
    mass: Some(Mass {
        kg: 10.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.58,
    apparent_magnitude: 1.86,
    temperature: Some(Temperature { K: 3523. }),
    age: Some(Time {
        s: 0.020 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(8, 22, 31),
    declination: Declination::new(Sgn::Neg, 59, 30, 34),
    distance: Distance {
        m: 632. * LIGHT_YEAR.m,
    },
};

//41
const MENKALINAN_DATA: RealData = RealData {
    common_name: "Menkalinan",
    astronomical_name: "Beta Aurigae",
    radius: Some(Distance {
        m: 2.77 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.389 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.10,
    apparent_magnitude: 1.9,
    temperature: Some(Temperature { K: 9350. }),
    age: Some(Time {
        s: 0.570 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 59, 32),
    declination: Declination::new(Sgn::Pos, 44, 56, 51),
    distance: Distance {
        m: 82. * LIGHT_YEAR.m,
    },
};

//42
const ATRIA_DATA: RealData = RealData {
    common_name: "Atria",
    astronomical_name: "Alpha Trianguli Australis",
    radius: Some(Distance {
        m: 143. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 7. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.62,
    apparent_magnitude: 1.91,
    temperature: Some(Temperature { K: 4150. }),
    age: Some(Time {
        s: 0.048 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(16, 48, 40),
    declination: Declination::new(Sgn::Neg, 69, 1, 40),
    distance: Distance {
        m: 415. * LIGHT_YEAR.m,
    },
};

//43
const ALSEPHINA_DATA: RealData = RealData {
    common_name: "Alsephina",
    astronomical_name: "Delta Velorum",
    radius: Some(Distance {
        m: 2.4 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.27 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.01,
    apparent_magnitude: 1.93,
    temperature: Some(Temperature { K: 9440. }),
    age: Some(Time {
        s: 0.4 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(8, 44, 42),
    declination: Declination::new(Sgn::Neg, 54, 42, 32),
    distance: Distance {
        m: 80. * LIGHT_YEAR.m,
    },
};

//44
const ALHENA_DATA: RealData = RealData {
    common_name: "Alhena",
    astronomical_name: "Gamma Geminorum",
    radius: Some(Distance {
        m: 3.3 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.81 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.60,
    apparent_magnitude: 1.93,
    temperature: Some(Temperature { K: 9260. }),
    age: None,
    right_ascension: RightAscension::new(6, 37, 43),
    declination: Declination::new(Sgn::Pos, 16, 23, 57),
    distance: Distance {
        m: 105. * LIGHT_YEAR.m,
    },
};

//45
const PEACOCK_DATA: RealData = RealData {
    common_name: "Peacock",
    astronomical_name: "Alpha Pavonis",
    radius: Some(Distance {
        m: 4.83 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 5.91 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.81,
    apparent_magnitude: 1.94,
    temperature: Some(Temperature { K: 17_711. }),
    age: Some(Time {
        s: 0.048 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(20, 25, 39),
    declination: Declination::new(Sgn::Neg, 56, 44, 6),
    distance: Distance {
        m: 183. * LIGHT_YEAR.m,
    },
};

//46
const POLARIS_DATA: RealData = RealData {
    common_name: "Polaris",
    astronomical_name: "Alpha Ursae Minoris",
    radius: Some(Distance {
        m: 37.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 5.4 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.64,
    apparent_magnitude: 1.97,
    temperature: Some(Temperature { K: 6015. }),
    age: Some(Time {
        s: 0.05 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(2, 31, 49),
    declination: Declination::new(Sgn::Pos, 89, 15, 51),
    distance: Distance {
        m: 431. * LIGHT_YEAR.m,
    },
};

//47
const MIRZAM_DATA: RealData = RealData {
    common_name: "Mirzam",
    astronomical_name: "Beta Canis Majoris",
    radius: Some(Distance {
        m: 9.7 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 13.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.95,
    apparent_magnitude: 1.98,
    temperature: Some(Temperature { K: 25_000. }),
    age: Some(Time {
        s: 0.0124 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(6, 22, 42),
    declination: Declination::new(Sgn::Neg, 17, 57, 21),
    distance: Distance {
        m: 499. * LIGHT_YEAR.m,
    },
};

//48
const ALPHARD_DATA: RealData = RealData {
    common_name: "Alphard",
    astronomical_name: "Alpha Hydrae",
    radius: Some(Distance {
        m: 50.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.03 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.69,
    apparent_magnitude: 1.99,
    temperature: Some(Temperature { K: 4120. }),
    age: Some(Time {
        s: 0.42 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(9, 27, 35),
    declination: Declination::new(Sgn::Neg, 8, 39, 30),
    distance: Distance {
        m: 177. * LIGHT_YEAR.m,
    },
};

//49
const ALGIEBA_DATA: RealData = RealData {
    common_name: "Algieba",
    astronomical_name: "Gamma Leonis",
    radius: Some(Distance {
        m: 31.88 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.23 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.92,
    apparent_magnitude: 2.01,
    temperature: Some(Temperature { K: 4470. }),
    age: None,
    right_ascension: RightAscension::new(10, 19, 58),
    declination: Declination::new(Sgn::Pos, 19, 50, 29),
    distance: Distance {
        m: 126. * LIGHT_YEAR.m,
    },
};

//50
const HAMAL_DATA: RealData = RealData {
    common_name: "Hamal",
    astronomical_name: "Alpha Arietis",
    radius: Some(Distance {
        m: 14.9 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.48,
    apparent_magnitude: 2.01,
    temperature: Some(Temperature { K: 4480. }),
    age: Some(Time {
        s: 3.4 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(2, 7, 10),
    declination: Declination::new(Sgn::Pos, 23, 27, 45),
    distance: Distance {
        m: 66. * LIGHT_YEAR.m,
    },
};

//51
const DIPHDA_DATA: RealData = RealData {
    common_name: "Diphda",
    astronomical_name: "Beta Ceti",
    radius: Some(Distance {
        m: 16.78 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.30,
    apparent_magnitude: 2.04,
    temperature: Some(Temperature { K: 4797. }),
    age: Some(Time {
        s: 1. * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(0, 43, 35),
    declination: Declination::new(Sgn::Neg, 17, 59, 12),
    distance: Distance {
        m: 96. * LIGHT_YEAR.m,
    },
};

//52
const NUNKI_DATA: RealData = RealData {
    common_name: "Nunki",
    astronomical_name: "Sigma Sagittarii",
    radius: Some(Distance {
        m: 4.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 7.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -2.14,
    apparent_magnitude: 2.05,
    temperature: Some(Temperature { K: 18_890. }),
    age: Some(Time {
        s: 0.0314 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(18, 55, 16),
    declination: Declination::new(Sgn::Neg, 26, 17, 49),
    distance: Distance {
        m: 224. * LIGHT_YEAR.m,
    },
};

//53
const MENKENT_DATA: RealData = RealData {
    common_name: "Menkent",
    astronomical_name: "Theta Centauri",
    radius: Some(Distance {
        m: 10.6 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.27 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.70,
    apparent_magnitude: 2.06,
    temperature: Some(Temperature { K: 4980. }),
    age: None,
    right_ascension: RightAscension::new(14, 6, 41),
    declination: Declination::new(Sgn::Neg, 36, 22, 11),
    distance: Distance {
        m: 61. * LIGHT_YEAR.m,
    },
};

//54
const SAIPH_DATA: RealData = RealData {
    common_name: "Saiph",
    astronomical_name: "Kappa Orionis",
    radius: Some(Distance {
        m: 22.2 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 15.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.65,
    apparent_magnitude: 2.07,
    temperature: Some(Temperature { K: 26_500. }),
    age: Some(Time {
        s: 0.0111 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 47, 45),
    declination: Declination::new(Sgn::Neg, 9, 40, 11),
    distance: Distance {
        m: 721.2 * LIGHT_YEAR.m,
    },
};

//55
const ALPHERATZ_DATA: RealData = RealData {
    common_name: "Alpheratz",
    astronomical_name: "Alpha Andromedae",
    radius: Some(Distance {
        m: 2.7 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.30,
    apparent_magnitude: 2.07,
    temperature: Some(Temperature { K: 13_800. }),
    age: Some(Time {
        s: 0.06 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(0, 8, 23),
    declination: Declination::new(Sgn::Pos, 29, 5, 26),
    distance: Distance {
        m: 97.0 * LIGHT_YEAR.m,
    },
};

//56
const TIAKI_DATA: RealData = RealData {
    common_name: "Tiaki",
    astronomical_name: "Beta Gruis",
    radius: Some(Distance {
        m: 180. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.4 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.52,
    apparent_magnitude: 2.07,
    temperature: Some(Temperature { K: 3480. }),
    age: None,
    right_ascension: RightAscension::new(22, 42, 40),
    declination: Declination::new(Sgn::Neg, 46, 53, 4),
    distance: Distance {
        m: 170. * LIGHT_YEAR.m,
    },
};

//57
const MIRACH_DATA: RealData = RealData {
    common_name: "Mirach",
    astronomical_name: "Beta Andromedae",
    radius: Some(Distance {
        m: 100. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.49 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.86,
    apparent_magnitude: 2.07,
    temperature: Some(Temperature { K: 3842. }),
    age: None,
    right_ascension: RightAscension::new(1, 9, 44),
    declination: Declination::new(Sgn::Pos, 35, 37, 14),
    distance: Distance {
        m: 199. * LIGHT_YEAR.m,
    },
};

//58
const KOCHAB_DATA: RealData = RealData {
    common_name: "Kochab",
    astronomical_name: "Beta Ursae Minoris",
    radius: Some(Distance {
        m: 42.06 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.87,
    apparent_magnitude: 2.07,
    temperature: Some(Temperature { K: 4030. }),
    age: None,
    right_ascension: RightAscension::new(14, 50, 42),
    declination: Declination::new(Sgn::Pos, 74, 9, 20),
    distance: Distance {
        m: 126. * LIGHT_YEAR.m,
    },
};

//59
const RASALHAGUE_DATA: RealData = RealData {
    common_name: "Rasalhague",
    astronomical_name: "Alpha Ophiuchi",
    radius: Some(Distance {
        m: 2.6 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.4 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.30,
    apparent_magnitude: 2.08,
    temperature: Some(Temperature { K: 8000. }),
    age: Some(Time {
        s: 0.77 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(17, 34, 56),
    declination: Declination::new(Sgn::Pos, 12, 33, 37),
    distance: Distance {
        m: 47. * LIGHT_YEAR.m,
    },
};

//60
const ALGOL_DATA: RealData = RealData {
    common_name: "Algol",
    astronomical_name: "Beta Persei",
    radius: Some(Distance {
        m: 2.73 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.17 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.18,
    apparent_magnitude: 2.09,
    temperature: Some(Temperature { K: 13_000. }),
    age: Some(Time {
        s: 0.57 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(3, 8, 10),
    declination: Declination::new(Sgn::Pos, 40, 57, 20),
    distance: Distance {
        m: 93. * LIGHT_YEAR.m,
    },
};

//61
const ALMACH_DATA: RealData = RealData {
    common_name: "Almach",
    astronomical_name: "Gamma Andromedae",
    radius: Some(Distance {
        m: 80. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 23.7 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.08,
    apparent_magnitude: 2.1,
    temperature: Some(Temperature { K: 4250. }),
    age: Some(Time {
        s: 0.0065 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(2, 3, 54),
    declination: Declination::new(Sgn::Pos, 42, 19, 47),
    distance: Distance {
        m: 355. * LIGHT_YEAR.m,
    },
};

//62
const DENEBOLA_DATA: RealData = RealData {
    common_name: "Denebola",
    astronomical_name: "Beta Leonis",
    radius: Some(Distance {
        m: 1.728 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.78 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.92,
    apparent_magnitude: 2.14,
    temperature: Some(Temperature { K: 8500. }),
    age: Some(Time {
        s: 0.25 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(11, 49, 3),
    declination: Declination::new(Sgn::Pos, 14, 34, 19),
    distance: Distance {
        m: 36. * LIGHT_YEAR.m,
    },
};

//63
const NAVI_DATA: RealData = RealData {
    common_name: "Navi",
    astronomical_name: "Gamma Cassiopeiae",
    radius: Some(Distance {
        m: 10. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 13. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.22,
    apparent_magnitude: 2.20,
    temperature: Some(Temperature { K: 25_000. }),
    age: Some(Time {
        s: 0.008 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(0, 56, 43),
    declination: Declination::new(Sgn::Pos, 60, 43, 0),
    distance: Distance {
        m: 613. * LIGHT_YEAR.m,
    },
};

//64
const MUHLIFAIN_DATA: RealData = RealData {
    common_name: "Muhlifain",
    astronomical_name: "Gamma Centauri",
    radius: None,
    mass: Some(Mass {
        kg: 2.91 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.81,
    apparent_magnitude: 2.20,
    temperature: Some(Temperature { K: 9082. }),
    age: None,
    right_ascension: RightAscension::new(12, 41, 31),
    declination: Declination::new(Sgn::Neg, 48, 57, 35),
    distance: Distance {
        m: 130. * LIGHT_YEAR.m,
    },
};

//65
const NAOS_DATA: RealData = RealData {
    common_name: "Naos",
    astronomical_name: "Zeta Puppis",
    radius: Some(Distance {
        m: 20. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 56.1 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -5.95,
    apparent_magnitude: 2.21,
    temperature: Some(Temperature { K: 40_000. }),
    age: Some(Time {
        s: 0.0032 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(8, 3, 35),
    declination: Declination::new(Sgn::Neg, 40, 0, 12),
    distance: Distance {
        m: 1399. * LIGHT_YEAR.m,
    },
};

//66
const ASPIDISKE_DATA: RealData = RealData {
    common_name: "Aspidiske",
    astronomical_name: "Iota Carinae",
    radius: Some(Distance {
        m: 43. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 7.4 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.42,
    apparent_magnitude: 2.21,
    temperature: Some(Temperature { K: 7500. }),
    age: Some(Time {
        s: 0.0374 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(9, 17, 5),
    declination: Declination::new(Sgn::Neg, 59, 16, 30),
    distance: Distance {
        m: 694. * LIGHT_YEAR.m,
    },
};

//67
const ALPHECCA_DATA: RealData = RealData {
    common_name: "Alphecca",
    astronomical_name: "Alpha Coronae Borealis",
    radius: Some(Distance {
        m: 3. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.58 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.42,
    apparent_magnitude: 2.22,
    temperature: Some(Temperature { K: 9700. }),
    age: Some(Time {
        s: 0.314 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(15, 34, 41),
    declination: Declination::new(Sgn::Pos, 26, 42, 53),
    distance: Distance {
        m: 75. * LIGHT_YEAR.m,
    },
};

//68
const SUHAIL_DATA: RealData = RealData {
    common_name: "Suhail",
    astronomical_name: "Lambda Velorum",
    radius: Some(Distance {
        m: 210. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 7. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.99,
    apparent_magnitude: 2.23,
    temperature: Some(Temperature { K: 3900. }),
    age: Some(Time {
        s: 0.0316 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(9, 7, 60),
    declination: Declination::new(Sgn::Neg, 43, 25, 57),
    distance: Distance {
        m: 573. * LIGHT_YEAR.m,
    },
};

//69
const SADIR_DATA: RealData = RealData {
    common_name: "Sadir",
    astronomical_name: "Gamma Cygni",
    radius: Some(Distance {
        m: 150. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 12.11 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -6.12,
    apparent_magnitude: 2.23,
    temperature: Some(Temperature { K: 5790. }),
    age: Some(Time {
        s: 0.012 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(20, 22, 14),
    declination: Declination::new(Sgn::Pos, 40, 15, 24),
    distance: Distance {
        m: 1522. * LIGHT_YEAR.m,
    },
};

//70
const MIZAR_DATA: RealData = RealData {
    common_name: "Mizar",
    astronomical_name: "Zeta Ursae Majoris",
    radius: Some(Distance {
        m: 2.4 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.33,
    apparent_magnitude: 2.23,
    temperature: Some(Temperature { K: 9000. }),
    age: Some(Time {
        s: 0.37 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(13, 23, 56),
    declination: Declination::new(Sgn::Pos, 54, 55, 31),
    distance: Distance {
        m: 78. * LIGHT_YEAR.m,
    },
};

//71
const SCHEDAR_DATA: RealData = RealData {
    common_name: "Schedar",
    astronomical_name: "Alpha Cassiopeiae",
    radius: Some(Distance {
        m: 45.39 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.98 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.99,
    apparent_magnitude: 2.24,
    temperature: Some(Temperature { K: 4552. }),
    age: Some(Time {
        s: 0.22 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(0, 40, 30),
    declination: Declination::new(Sgn::Pos, 56, 32, 14),
    distance: Distance {
        m: 228. * LIGHT_YEAR.m,
    },
};

//72
const ELTANIN_DATA: RealData = RealData {
    common_name: "Eltanin",
    astronomical_name: "Gamma Draconis",
    radius: Some(Distance {
        m: 48.15 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.72 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.04,
    apparent_magnitude: 2.24,
    temperature: Some(Temperature { K: 3930. }),
    age: None,
    right_ascension: RightAscension::new(17, 56, 36),
    declination: Declination::new(Sgn::Pos, 51, 29, 20),
    distance: Distance {
        m: 148. * LIGHT_YEAR.m,
    },
};

//73
const MINTAKA_DATA: RealData = RealData {
    common_name: "Mintaka",
    astronomical_name: "Delta Orionis",
    radius: Some(Distance {
        m: 16.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 24. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.99,
    apparent_magnitude: 2.25,
    temperature: Some(Temperature { K: 29_500. }),
    age: None,
    right_ascension: RightAscension::new(5, 32, 0),
    declination: Declination::new(Sgn::Neg, 0, 17, 57),
    distance: Distance {
        m: 916. * LIGHT_YEAR.m,
    },
};

//74
const CAPH_DATA: RealData = RealData {
    common_name: "Caph",
    astronomical_name: "Beta Cassiopeiae",
    radius: Some(Distance {
        m: 3.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.91 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.17,
    apparent_magnitude: 2.28,
    temperature: Some(Temperature { K: 7079. }),
    age: Some(Time {
        s: 1.1 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(0, 9, 11),
    declination: Declination::new(Sgn::Pos, 59, 8, 59),
    distance: Distance {
        m: 54. * LIGHT_YEAR.m,
    },
};

//75
const DSCHUBBA_DATA: RealData = RealData {
    common_name: "Dschubba",
    astronomical_name: "Delta Scorpii",
    radius: Some(Distance {
        m: 6.7 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 13. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.16,
    apparent_magnitude: 2.29,
    temperature: Some(Temperature { K: 27_400. }),
    age: Some(Time {
        s: 0.0095 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(16, 0, 20),
    declination: Declination::new(Sgn::Neg, 22, 37, 18),
    distance: Distance {
        m: 401.5 * LIGHT_YEAR.m,
    },
};

//76
const LARAWAG_DATA: RealData = RealData {
    common_name: "Larawag",
    astronomical_name: "Epsilon Scorpii",
    radius: Some(Distance {
        m: 12.6 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.24 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.78,
    apparent_magnitude: 2.29,
    temperature: Some(Temperature { K: 4560. }),
    age: None,
    right_ascension: RightAscension::new(16, 50, 10),
    declination: Declination::new(Sgn::Neg, 34, 17, 36),
    distance: Distance {
        m: 65. * LIGHT_YEAR.m,
    },
};

//77
const EPSILON_CENTAURI_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "Epsilon Centauri",
    radius: None,
    mass: Some(Mass {
        kg: 11.6 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.02,
    apparent_magnitude: 2.29,
    temperature: Some(Temperature { K: 24_000. }),
    age: Some(Time {
        s: 0.0158 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(13, 39, 53),
    declination: Declination::new(Sgn::Neg, 53, 27, 59),
    distance: Distance {
        m: 376. * LIGHT_YEAR.m,
    },
};

//78
const ALPHA_LUPI_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Lupi",
    radius: None,
    mass: Some(Mass {
        kg: 10.1 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.83,
    apparent_magnitude: 2.30,
    temperature: Some(Temperature { K: 21_820. }),
    age: Some(Time {
        s: 0.018 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(14, 41, 56),
    declination: Declination::new(Sgn::Neg, 47, 23, 18),
    distance: Distance {
        m: 548. * LIGHT_YEAR.m,
    },
};

//79
const ETA_CENTAURI_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "Eta Centauri",
    radius: Some(Distance {
        m: 6.1 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 12.0 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -2.55,
    apparent_magnitude: 2.29,
    temperature: Some(Temperature { K: 25_700. }),
    age: Some(Time {
        s: 0.0056 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(14, 35, 30),
    declination: Declination::new(Sgn::Neg, 42, 9, 28),
    distance: Distance {
        m: 308. * LIGHT_YEAR.m,
    },
};

//80
const MERAK_DATA: RealData = RealData {
    common_name: "Merak",
    astronomical_name: "Beta Ursae Majoris",
    radius: Some(Distance {
        m: 3.021 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.7 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.41,
    apparent_magnitude: 2.34,
    temperature: Some(Temperature { K: 9377. }),
    age: Some(Time {
        s: 0.5 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(11, 1, 50),
    declination: Declination::new(Sgn::Pos, 56, 22, 57),
    distance: Distance {
        m: 79. * LIGHT_YEAR.m,
    },
};

//81
const IZAR_DATA: RealData = RealData {
    common_name: "Izar",
    astronomical_name: "Epsilon Bootis",
    radius: Some(Distance {
        m: 33. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 4.6 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.69,
    apparent_magnitude: 2.35,
    temperature: Some(Temperature { K: 4550. }),
    age: Some(Time {
        s: 0.0374 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(14, 44, 59),
    declination: Declination::new(Sgn::Pos, 27, 4, 27),
    distance: Distance {
        m: 210. * LIGHT_YEAR.m,
    },
};

//82
const ENIF_DATA: RealData = RealData {
    common_name: "Enif",
    astronomical_name: "Epsilon Pegasi",
    radius: Some(Distance {
        m: 211. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 7.07 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.19,
    apparent_magnitude: 2.38,
    temperature: Some(Temperature { K: 3963. }),
    age: Some(Time {
        s: 0.020 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(21, 44, 11),
    declination: Declination::new(Sgn::Pos, 9, 52, 30),
    distance: Distance {
        m: 672. * LIGHT_YEAR.m,
    },
};

//83
const GIRTAB_DATA: RealData = RealData {
    common_name: "Girtab",
    astronomical_name: "Kappa Scorpii",
    radius: Some(Distance {
        m: 6.8 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 17. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.38,
    apparent_magnitude: 2.39,
    temperature: Some(Temperature { K: 23_400. }),
    age: Some(Time {
        s: 0.0251 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(17, 42, 29),
    declination: Declination::new(Sgn::Neg, 39, 1, 48),
    distance: Distance {
        m: 464. * LIGHT_YEAR.m,
    },
};

//84
const ANKAA_DATA: RealData = RealData {
    common_name: "Ankaa",
    astronomical_name: "Alpha Phoenicis",
    radius: Some(Distance {
        m: 15. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.57 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.52,
    apparent_magnitude: 2.4,
    temperature: Some(Temperature { K: 4436. }),
    age: None,
    right_ascension: RightAscension::new(0, 26, 17),
    declination: Declination::new(Sgn::Neg, 42, 18, 21),
    distance: Distance {
        m: 77. * LIGHT_YEAR.m,
    },
};

//85
const PHECDA_DATA: RealData = RealData {
    common_name: "Phecda",
    astronomical_name: "Gamma Ursae Majoris",
    radius: Some(Distance {
        m: 3.04 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.94 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.36,
    apparent_magnitude: 2.41,
    temperature: Some(Temperature { K: 9355. }),
    age: Some(Time {
        s: 0.3 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(11, 53, 50),
    declination: Declination::new(Sgn::Pos, 53, 41, 41),
    distance: Distance {
        m: 84. * LIGHT_YEAR.m,
    },
};

//86
const SABIK_DATA: RealData = RealData {
    common_name: "Sabik",
    astronomical_name: "Eta Ophiuchi",
    radius: None,
    mass: Some(Mass {
        kg: 2.966 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.37,
    apparent_magnitude: 2.43,
    temperature: Some(Temperature { K: 8900. }),
    age: None,
    right_ascension: RightAscension::new(17, 10, 23),
    declination: Declination::new(Sgn::Neg, 15, 43, 30),
    distance: Distance {
        m: 84. * LIGHT_YEAR.m,
    },
};

//87
const SCHEAT_DATA: RealData = RealData {
    common_name: "Scheat",
    astronomical_name: "Beta Pegasi",
    radius: Some(Distance {
        m: 95. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.1 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.49,
    apparent_magnitude: 2.44,
    temperature: Some(Temperature { K: 3689. }),
    age: None,
    right_ascension: RightAscension::new(23, 3, 46),
    declination: Declination::new(Sgn::Pos, 28, 4, 58),
    distance: Distance {
        m: 199. * LIGHT_YEAR.m,
    },
};

//88
const ALDERAMIN_DATA: RealData = RealData {
    common_name: "Alderamin",
    astronomical_name: "Alpha Cephei",
    radius: Some(Distance {
        m: 2.4 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.58,
    apparent_magnitude: 2.45,
    temperature: Some(Temperature { K: 7700. }),
    age: Some(Time {
        s: 0.82 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(21, 18, 35),
    declination: Declination::new(Sgn::Pos, 62, 35, 8),
    distance: Distance {
        m: 49. * LIGHT_YEAR.m,
    },
};

//89
const ALUDRA_DATA: RealData = RealData {
    common_name: "Aludra",
    astronomical_name: "Eta Canis Majoris",
    radius: Some(Distance {
        m: 54. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 18.19 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -7.51,
    apparent_magnitude: 2.45,
    temperature: Some(Temperature { K: 15_500. }),
    age: Some(Time {
        s: 0.0083 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(7, 24, 6),
    declination: Declination::new(Sgn::Neg, 29, 18, 11),
    distance: Distance {
        m: 3196. * LIGHT_YEAR.m,
    },
};

//90
const MARKEB_DATA: RealData = RealData {
    common_name: "Markeb",
    astronomical_name: "Kappa Velorum",
    radius: Some(Distance {
        m: 9.1 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 10.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.62,
    apparent_magnitude: 2.47,
    temperature: Some(Temperature { K: 23_000. }),
    age: Some(Time {
        s: 0.018 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(9, 22, 7),
    declination: Declination::new(Sgn::Neg, 55, 0, 38),
    distance: Distance {
        m: 539. * LIGHT_YEAR.m,
    },
};

//91
const ALJANAH_DATA: RealData = RealData {
    common_name: "Aljanah",
    astronomical_name: "Epsilon Cygni",
    radius: Some(Distance {
        m: 10.82 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.76,
    apparent_magnitude: 2.48,
    temperature: Some(Temperature { K: 4710. }),
    age: Some(Time {
        s: 1.5 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(20, 46, 13),
    declination: Declination::new(Sgn::Pos, 33, 58, 13),
    distance: Distance {
        m: 72. * LIGHT_YEAR.m,
    },
};

//92
const MARKAB_DATA: RealData = RealData {
    common_name: "Markab",
    astronomical_name: "Alpha Pegasi",
    radius: Some(Distance {
        m: 4.62 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.67,
    apparent_magnitude: 2.49,
    temperature: Some(Temperature { K: 10_100. }),
    age: Some(Time {
        s: 0.2 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(23, 4, 46),
    declination: Declination::new(Sgn::Pos, 15, 12, 19),
    distance: Distance {
        m: 140. * LIGHT_YEAR.m,
    },
};

//93
const HAN_DATA: RealData = RealData {
    common_name: "Han",
    astronomical_name: "Zeta Ophiuchi",
    radius: Some(Distance {
        m: 8.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 20.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.20,
    apparent_magnitude: 2.54,
    temperature: Some(Temperature { K: 34_300. }),
    age: Some(Time {
        s: 3. * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(16, 37, 10),
    declination: Declination::new(Sgn::Neg, 10, 34, 2),
    distance: Distance {
        m: 458. * LIGHT_YEAR.m,
    },
};

//94
const MENKAR_DATA: RealData = RealData {
    common_name: "Menkar",
    astronomical_name: "Alpha Ceti",
    radius: Some(Distance {
        m: 89. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.3 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.61,
    apparent_magnitude: 2.54,
    temperature: Some(Temperature { K: 3795. }),
    age: None,
    right_ascension: RightAscension::new(3, 2, 17),
    declination: Declination::new(Sgn::Pos, 4, 5, 23),
    distance: Distance {
        m: 220. * LIGHT_YEAR.m,
    },
};

//95
const ZETA_CENTAURI_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "Zeta Centauri",
    radius: Some(Distance {
        m: 5.8 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 7.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -2.81,
    apparent_magnitude: 2.55,
    temperature: Some(Temperature { K: 23_561. }),
    age: Some(Time {
        s: 0.04 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(13, 55, 33),
    declination: Declination::new(Sgn::Neg, 47, 17, 18),
    distance: Distance {
        m: 384. * LIGHT_YEAR.m,
    },
};

//96
const ACRAB_DATA: RealData = RealData {
    common_name: "Acrab",
    astronomical_name: "Beta Scorpii",
    radius: Some(Distance {
        m: 6.3 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 15.0 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.50,
    apparent_magnitude: 2.56,
    temperature: Some(Temperature { K: 28_000. }),
    age: None,
    right_ascension: RightAscension::new(16, 5, 26),
    declination: Declination::new(Sgn::Neg, 19, 48, 20),
    distance: Distance {
        m: 530. * LIGHT_YEAR.m,
    },
};

//97
const ZOSMA_DATA: RealData = RealData {
    common_name: "Zosma",
    astronomical_name: "Delta Leonis",
    radius: Some(Distance {
        m: 2.14 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.32,
    apparent_magnitude: 2.56,
    temperature: Some(Temperature { K: 8_296. }),
    age: Some(Time {
        s: 0.65 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(11, 14, 7),
    declination: Declination::new(Sgn::Pos, 20, 31, 25),
    distance: Distance {
        m: 58. * LIGHT_YEAR.m,
    },
};

//98
const MA_WEI_DATA: RealData = RealData {
    common_name: "Ma Wei",
    astronomical_name: "Delta Centauri",
    radius: Some(Distance {
        m: 6.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 8.7 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -2.84,
    apparent_magnitude: 2.58,
    temperature: Some(Temperature { K: 22_360. }),
    age: Some(Time {
        s: 0.02 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 8, 21),
    declination: Declination::new(Sgn::Neg, 50, 43, 21),
    distance: Distance {
        m: 395. * LIGHT_YEAR.m,
    },
};

//99
const ARNEB_DATA: RealData = RealData {
    common_name: "Arneb",
    astronomical_name: "Alpha Leporis",
    radius: Some(Distance {
        m: 75. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 13.9 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -5.40,
    apparent_magnitude: 2.58,
    temperature: Some(Temperature { K: 6_850. }),
    age: Some(Time {
        s: 0.013 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 32, 44),
    declination: Declination::new(Sgn::Neg, 17, 49, 20),
    distance: Distance {
        m: 1283. * LIGHT_YEAR.m,
    },
};

//100
const GHURAB_DATA: RealData = RealData {
    common_name: "Ghurab",
    astronomical_name: "Gamma Corvi",
    radius: None,
    mass: Some(Mass {
        kg: 4.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.94,
    apparent_magnitude: 2.58,
    temperature: Some(Temperature { K: 12_000. }),
    age: Some(Time {
        s: 0.160 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 15, 48),
    declination: Declination::new(Sgn::Neg, 17, 32, 31),
    distance: Distance {
        m: 165. * LIGHT_YEAR.m,
    },
};

// Gaia data that was not found in the list of 100 brightest stars
// Designations can be checked under http://simbad.cds.unistra.fr/simbad/sim-fid

const TEJAT_DATA: RealData = RealData {
    common_name: "Tejat",
    astronomical_name: "Mu Geminorum",
    radius: Some(Distance {
        m: 90. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.1 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.42,
    apparent_magnitude: 2.75,
    temperature: Some(Temperature { K: 3460. }),
    age: None,
    right_ascension: RightAscension::new(6, 22, 58),
    declination: Declination::new(Sgn::Pos, 22, 30, 49),
    distance: Distance {
        m: 230. * LIGHT_YEAR.m,
    },
};

const R_DORADUS_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "R Doradus",
    radius: Some(Distance {
        m: 298. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 0.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.61,
    apparent_magnitude: 5.59,
    temperature: Some(Temperature { K: 2710. }),
    age: Some(Time {
        s: 10. * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(4, 36, 46),
    declination: Declination::new(Sgn::Neg, 62, 4, 38),
    distance: Distance {
        m: 203.5 * LIGHT_YEAR.m,
    },
};

const YED_PRIOR: RealData = RealData {
    common_name: "Yed Prior",
    astronomical_name: "Delta Ophiuchi",
    radius: Some(Distance {
        m: 59. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.90,
    apparent_magnitude: 2.73,
    temperature: Some(Temperature { K: 3679. }),
    age: None,
    right_ascension: RightAscension::new(16, 14, 21),
    declination: Declination::new(Sgn::Neg, 3, 41, 40),
    distance: Distance {
        m: 171. * LIGHT_YEAR.m,
    },
};

const GORGONEA_TERTIA_DATA: RealData = RealData {
    common_name: "Gorgonea Tertia",
    astronomical_name: "Rho Persei",
    radius: Some(Distance {
        m: 143. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.9 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.67,
    apparent_magnitude: 3.32,
    temperature: Some(Temperature { K: 3479. }),
    age: Some(Time {
        s: 0.440 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(3, 5, 11),
    declination: Declination::new(Sgn::Pos, 38, 50, 25),
    distance: Distance {
        m: 325. * LIGHT_YEAR.m,
    },
};

const NAMALWARID: RealData = RealData {
    common_name: "Namalwarid",
    astronomical_name: "Eta Sagittarii",
    radius: None,
    mass: None,
    absolute_magnitude: -0.201,
    apparent_magnitude: 3.1,
    temperature: None,
    age: None,
    right_ascension: RightAscension::new(18, 17, 38),
    declination: Declination::new(Sgn::Neg, 36, 45, 42),
    distance: Distance {
        m: 149.1 * LIGHT_YEAR.m,
    },
};

const HASSALEH: RealData = RealData {
    common_name: "Hassaleh",
    astronomical_name: "Iota Aurigae",
    radius: Some(Distance {
        m: 127. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 7.1 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.20,
    apparent_magnitude: 2.69,
    temperature: Some(Temperature { K: 4160. }),
    age: Some(Time {
        s: 0.04 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(4, 56, 60),
    declination: Declination::new(Sgn::Pos, 33, 9, 58),
    distance: Distance {
        m: 490. * LIGHT_YEAR.m,
    },
};

const PROPUS_DATA: RealData = RealData {
    common_name: "Propus",
    astronomical_name: "Eta Geminorum",
    radius: Some(Distance {
        m: 275. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.84,
    apparent_magnitude: 3.31,
    temperature: Some(Temperature { K: 3502. }),
    age: Some(Time {
        s: 0.81 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(6, 14, 53),
    declination: Declination::new(Sgn::Pos, 22, 30, 24),
    distance: Distance {
        m: 349. * LIGHT_YEAR.m,
    },
};

const ZAURAK_DATA: RealData = RealData {
    common_name: "Zaurak",
    astronomical_name: "Gamma Eridani",
    radius: Some(Distance {
        m: 80. * SOLAR_RADIUS.m,
    }),
    mass: None,
    absolute_magnitude: -1.19,
    apparent_magnitude: 2.97,
    temperature: Some(Temperature { K: 3811. }),
    age: None,
    right_ascension: RightAscension::new(3, 58, 2),
    declination: Declination::new(Sgn::Neg, 13, 30, 31),
    distance: Distance {
        m: 221. * LIGHT_YEAR.m,
    },
};

const KAUS_MEDIA_DATA: RealData = RealData {
    common_name: "Kaus Media",
    astronomical_name: "Delta Sagittarii",
    radius: Some(Distance {
        m: 16. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.21 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -2.14,
    apparent_magnitude: 2.72,
    temperature: Some(Temperature { K: 4203. }),
    age: Some(Time {
        s: 0.26 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(18, 20, 60),
    declination: Declination::new(Sgn::Neg, 29, 49, 41),
    distance: Distance {
        m: 305.5 * LIGHT_YEAR.m,
    },
};

const BRACHIUM_DATA: RealData = RealData {
    common_name: "Brachium",
    astronomical_name: "Sigma Librae",
    radius: Some(Distance {
        m: 108. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.5,
    apparent_magnitude: 3.21,
    temperature: Some(Temperature { K: 3596. }),
    age: None,
    right_ascension: RightAscension::new(15, 4, 4),
    declination: Declination::new(Sgn::Neg, 25, 16, 55),
    distance: Distance {
        m: 288. * LIGHT_YEAR.m,
    },
};

const TANIA_AUSTRALIS_DATA: RealData = RealData {
    common_name: "Tania Australis",
    astronomical_name: "Mu Ursae Majoris",
    radius: Some(Distance {
        m: 75. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 6.3 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.2,
    apparent_magnitude: 3.06,
    temperature: Some(Temperature { K: 3899. }),
    age: None,
    right_ascension: RightAscension::new(10, 22, 20),
    declination: Declination::new(Sgn::Pos, 41, 29, 58),
    distance: Distance {
        m: 230.0 * LIGHT_YEAR.m,
    },
};

const UNUKALHAI_DATA: RealData = RealData {
    common_name: "Unukalhai",
    astronomical_name: "Alpha Serpentis",
    radius: Some(Distance {
        m: 13.48 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.66 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.88,
    apparent_magnitude: 2.63,
    temperature: Some(Temperature { K: 4498. }),
    age: None,
    right_ascension: RightAscension::new(15, 44, 16),
    declination: Declination::new(Sgn::Pos, 6, 25, 32),
    distance: Distance {
        m: 74. * LIGHT_YEAR.m,
    },
};

const R_LYRAE_DATE: RealData = RealData {
    common_name: "",
    astronomical_name: "R Lyrae",
    radius: None,
    mass: Some(Mass {
        kg: 1.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.07,
    apparent_magnitude: 4.08,
    temperature: Some(Temperature { K: 3313. }),
    age: None,
    right_ascension: RightAscension::new(18, 55, 20),
    declination: Declination::new(Sgn::Pos, 43, 56, 46),
    distance: Distance {
        m: 349.4 * LIGHT_YEAR.m,
    },
};

const BETA_ARAE_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Arae",
    radius: Some(Distance {
        m: 142. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 8.21 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.49,
    apparent_magnitude: 2.84,
    temperature: Some(Temperature { K: 4197. }),
    age: Some(Time {
        s: 0.05 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(17, 25, 18),
    declination: Declination::new(Sgn::Neg, 55, 31, 48),
    distance: Distance {
        m: 602.6 * LIGHT_YEAR.m,
    },
};

const APLHA_TUCANAE_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Tucanae",
    radius: Some(Distance {
        m: 37. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.05,
    apparent_magnitude: 2.87,
    temperature: Some(Temperature { K: 4300. }),
    age: None,
    right_ascension: RightAscension::new(22, 18, 30),
    declination: Declination::new(Sgn::Neg, 60, 15, 35),
    distance: Distance {
        m: 198.5 * LIGHT_YEAR.m,
    },
};

const MINELAUVA_DATA: RealData = RealData {
    common_name: "Minelauva",
    astronomical_name: "Delta Virginis",
    radius: Some(Distance {
        m: 48. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.4 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.575,
    apparent_magnitude: 3.39,
    temperature: Some(Temperature { K: 3999. }),
    age: None,
    right_ascension: RightAscension::new(12, 55, 36),
    declination: Declination::new(Sgn::Pos, 3, 23, 51),
    distance: Distance {
        m: 202.4 * LIGHT_YEAR.m,
    },
};

const CEBALRAI_DATA: RealData = RealData {
    common_name: "Cebalrai",
    astronomical_name: "Beta Ophiuchi",
    radius: Some(Distance {
        m: 12.42 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.13 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.77,
    apparent_magnitude: 2.76,
    temperature: Some(Temperature { K: 4467. }),
    age: Some(Time {
        s: 3.82 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(17, 43, 28),
    declination: Declination::new(Sgn::Pos, 4, 34, 2),
    distance: Distance {
        m: 81.8 * LIGHT_YEAR.m,
    },
};

const KRAZ_DATA: RealData = RealData {
    common_name: "Kraz",
    astronomical_name: "Beta Corvi",
    radius: Some(Distance {
        m: 16. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.7 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.61,
    apparent_magnitude: 2.65,
    temperature: Some(Temperature { K: 5100. }),
    age: Some(Time {
        s: 0.206 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 34, 23),
    declination: Declination::new(Sgn::Neg, 23, 23, 48),
    distance: Distance {
        m: 146. * LIGHT_YEAR.m,
    },
};

const ERAKIS_DATA: RealData = RealData {
    common_name: "Erakis",
    astronomical_name: "Mu Cephei",
    radius: Some(Distance {
        m: 972. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 19.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -6.5,
    apparent_magnitude: 3.43,
    temperature: Some(Temperature { K: 3551. }),
    age: Some(Time {
        s: 0.01 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(21, 43, 30),
    declination: Declination::new(Sgn::Pos, 58, 46, 48),
    distance: Distance {
        m: 3066. * LIGHT_YEAR.m,
    },
};

const GAMMA_HYDRI_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Hydri",
    radius: Some(Distance {
        m: 62. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.83,
    apparent_magnitude: 3.26,
    temperature: Some(Temperature { K: 3499. }),
    age: None,
    right_ascension: RightAscension::new(3, 47, 14),
    declination: Declination::new(Sgn::Neg, 74, 14, 20),
    distance: Distance {
        m: 214. * LIGHT_YEAR.m,
    },
};

const ALPHA_LYNCIS_DATA: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Lyncis",
    radius: Some(Distance {
        m: 54.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.02,
    apparent_magnitude: 3.14,
    temperature: Some(Temperature { K: 3882. }),
    age: Some(Time {
        s: 1.4 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(9, 21, 3),
    declination: Declination::new(Sgn::Pos, 34, 23, 33),
    distance: Distance {
        m: 221.9 * LIGHT_YEAR.m,
    },
};

const ATHEBYNE_DATA: RealData = RealData {
    common_name: "Athebyne",
    astronomical_name: "Eta Draconis",
    radius: Some(Distance {
        m: 11. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.55 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.58,
    apparent_magnitude: 2.73,
    temperature: Some(Temperature { K: 5055. }),
    age: Some(Time {
        s: 0.55 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(16, 23, 59),
    declination: Declination::new(Sgn::Pos, 61, 30, 51),
    distance: Distance {
        m: 87.68 * LIGHT_YEAR.m,
    },
};

// http://www.avastronomyclub.org/skymap/d/skymap.php

const AHADI_DATA: RealData = RealData {
    common_name: "Ahadi",
    astronomical_name: "Pi Puppis",
    radius: Some(Distance {
        m: 235. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 11.7 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.92,
    apparent_magnitude: 2.71,
    temperature: Some(Temperature { K: 4000. }),
    age: Some(Time {
        s: 0.02 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(7, 17, 9),
    declination: Declination::new(Sgn::Neg, 37, 5, 51),
    distance: Distance {
        m: 1094. * LIGHT_YEAR.m,
    },
};

const TARAZED_DATA: RealData = RealData {
    common_name: "Tarazed",
    astronomical_name: "Gamma Aquilae",
    radius: Some(Distance {
        m: 91.82 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.51 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.03,
    apparent_magnitude: 2.72,
    temperature: Some(Temperature { K: 4098. }),
    age: Some(Time {
        s: 0.270 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(19, 46, 16),
    declination: Declination::new(Sgn::Pos, 10, 36, 48),
    distance: Distance {
        m: 460.5 * LIGHT_YEAR.m,
    },
};

// RealData = RealData {
//     common_name: "",
//     astronomical_name: "",
//     radius: Some(Distance{m:Distance{m:. * SOLAR_RADIUS.m}}),
//     mass: Some(Mass{kg: * SOLAR_MASS.kg}}),
//     absolute_magnitude: ),
//     apparent_magnitude: ,
//     temperature: Some(Temperature{K:}),
//     age: Some(Time{s:* BILLION_YEARS.s}}),
//     right_ascension: RightAscension::new(),
//     declination: Declination::new(),
//     distance: Distance{m: Distance{m:. * LIGHT_YEAR.m}},
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
