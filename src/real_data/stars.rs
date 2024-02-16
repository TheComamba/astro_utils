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

pub const SUN: RealData = RealData {
    common_name: "Sun",
    astronomical_name: "Sol",
    constellation: "",
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
const SIRIUS: RealData = RealData {
    common_name: "Sirius",
    astronomical_name: "Alpha Canis Majoris",
    constellation: "Canis Major",
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
const CANOPUS: RealData = RealData {
    common_name: "Canopus",
    astronomical_name: "Alpha Carinae",
    constellation: "Carina",
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
const ARCTURUS: RealData = RealData {
    common_name: "Arcturus",
    astronomical_name: "Alpha Bootis",
    constellation: "Boötes",
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
const RIGEL_KENTAURUS: RealData = RealData {
    common_name: "Rigel Kentaurus",
    astronomical_name: "Alpha Centauri",
    constellation: "Centaurus",
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
const VEGA: RealData = RealData {
    common_name: "Vega",
    astronomical_name: "Alpha Lyrae",
    constellation: "Lyra",
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
const CAPELLA: RealData = RealData {
    common_name: "Capella",
    astronomical_name: "Alpha Aurigae",
    constellation: "Auriga",
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
const RIGEL: RealData = RealData {
    common_name: "Rigel",
    astronomical_name: "Beta Orionis",
    constellation: "Orion",
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
const PROCYON: RealData = RealData {
    common_name: "Procyon",
    astronomical_name: "Alpha Canis Minoris",
    constellation: "Canis Minor",
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
const BETELGEUSE: RealData = RealData {
    common_name: "Betelgeuse",
    astronomical_name: "Alpha Orionis",
    constellation: "Orion",
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
const ACHERNAR: RealData = RealData {
    common_name: "Achernar",
    astronomical_name: "Alpha Eridani",
    constellation: "Eridanus",
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
const HADAR: RealData = RealData {
    common_name: "Hadar",
    astronomical_name: "Beta Centauri",
    constellation: "Centaurus",
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
const ALTAIR: RealData = RealData {
    common_name: "Altair",
    astronomical_name: "Alpha Aquilae",
    constellation: "Aquila",
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
const ACRUX: RealData = RealData {
    common_name: "Acrux",
    astronomical_name: "Alpha Crucis",
    constellation: "Crux",
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
const ALDEBARAN: RealData = RealData {
    common_name: "Aldebaran",
    astronomical_name: "Alpha Tauri",
    constellation: "Taurus",
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
const SPICA: RealData = RealData {
    common_name: "Spica",
    astronomical_name: "Alpha Virginis",
    constellation: "Virgo",
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
const ANTARES: RealData = RealData {
    common_name: "Antares",
    astronomical_name: "Alpha Scorpii",
    constellation: "Scorpius",
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
const POLLUX: RealData = RealData {
    common_name: "Pollux",
    astronomical_name: "Beta Geminorum",
    constellation: "Gemini",
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
const FORMALHAUT: RealData = RealData {
    common_name: "Formalhaut",
    astronomical_name: "Alpha Piscis Austrini",
    constellation: "Piscis Austrinus",
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
const DENEB: RealData = RealData {
    common_name: "Deneb",
    astronomical_name: "Alpha Cygni",
    constellation: "Cygnus",
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
const MIMOSA: RealData = RealData {
    common_name: "Mimosa",
    astronomical_name: "Beta Crucis",
    constellation: "Crux",
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
const REGULUS: RealData = RealData {
    common_name: "Regulus",
    astronomical_name: "Alpha Leonis",
    constellation: "Leo",
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
const ADHARA: RealData = RealData {
    common_name: "Adhara",
    astronomical_name: "Epsilon Canis Majoris",
    constellation: "Canis Major",
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
const CASTOR: RealData = RealData {
    common_name: "Castor",
    astronomical_name: "Alpha Geminorum",
    constellation: "Gemini",
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
const GACRUX: RealData = RealData {
    common_name: "Gacrux",
    astronomical_name: "Gamma Crucis",
    constellation: "Crux",
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
const SHAULA: RealData = RealData {
    common_name: "Shaula",
    astronomical_name: "Lambda Scorpii",
    constellation: "Scorpius",
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
const BELLATRIX: RealData = RealData {
    common_name: "Bellatrix",
    astronomical_name: "Gamma Orionis",
    constellation: "Orion",
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
const ALNATH: RealData = RealData {
    common_name: "Alnath",
    astronomical_name: "Beta Tauri",
    constellation: "Taurus",
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
const MIAPLACIDUS: RealData = RealData {
    common_name: "Miaplacidus",
    astronomical_name: "Beta Carinae",
    constellation: "Carina",
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
const ALNILAM: RealData = RealData {
    common_name: "Alnilam",
    astronomical_name: "Epsilon Orionis",
    constellation: "Orion",
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
const ALNAIR: RealData = RealData {
    common_name: "Alnair",
    astronomical_name: "Alpha Gruis",
    constellation: "Grus",
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
const ALNITAK: RealData = RealData {
    common_name: "Alnitak",
    astronomical_name: "Zeta Orionis",
    constellation: "Orion",
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
const REGOR: RealData = RealData {
    common_name: "Regor",
    astronomical_name: "Gamma Velorum",
    constellation: "Vela",
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
const ALIOTH: RealData = RealData {
    common_name: "Alioth",
    astronomical_name: "Epsilon Ursae Majoris",
    constellation: "Ursa Major",
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
const KAUS_AUSTRALIS: RealData = RealData {
    common_name: "Kaus Australis",
    astronomical_name: "Epsilon Sagittarii",
    constellation: "Sagittarius",
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
const MIRPHAK: RealData = RealData {
    common_name: "Mirphak",
    astronomical_name: "Alpha Persei",
    constellation: "Perseus",
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
const DUBHE: RealData = RealData {
    common_name: "Dubhe",
    astronomical_name: "Alpha Ursae Majoris",
    constellation: "Ursa Major",
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
const WEZEN: RealData = RealData {
    common_name: "Wezen",
    astronomical_name: "Delta Canis Majoris",
    constellation: "Canis Major",
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
const ALKAID: RealData = RealData {
    common_name: "Alkaid",
    astronomical_name: "Eta Ursae Majoris",
    constellation: "Ursa Major",
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
const SARGAS: RealData = RealData {
    common_name: "Sargas",
    astronomical_name: "Theta Scorpii",
    constellation: "Scorpius",
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
const AVIOR: RealData = RealData {
    common_name: "Avior",
    astronomical_name: "Epsilon Carinae",
    constellation: "Carina",
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
const MENKALINAN: RealData = RealData {
    common_name: "Menkalinan",
    astronomical_name: "Beta Aurigae",
    constellation: "Auriga",
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
const ATRIA: RealData = RealData {
    common_name: "Atria",
    astronomical_name: "Alpha Trianguli Australis",
    constellation: "Triangulum Australe",
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
const ALSEPHINA: RealData = RealData {
    common_name: "Alsephina",
    astronomical_name: "Delta Velorum",
    constellation: "Vela",
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
const ALHENA: RealData = RealData {
    common_name: "Alhena",
    astronomical_name: "Gamma Geminorum",
    constellation: "Gemini",
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
const PEACOCK: RealData = RealData {
    common_name: "Peacock",
    astronomical_name: "Alpha Pavonis",
    constellation: "Pavo",
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
const POLARIS: RealData = RealData {
    common_name: "Polaris",
    astronomical_name: "Alpha Ursae Minoris",
    constellation: "Ursa Minor",
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
const MIRZAM: RealData = RealData {
    common_name: "Mirzam",
    astronomical_name: "Beta Canis Majoris",
    constellation: "Canis Major",
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
const ALPHARD: RealData = RealData {
    common_name: "Alphard",
    astronomical_name: "Alpha Hydrae",
    constellation: "Hydra",
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
const ALGIEBA: RealData = RealData {
    common_name: "Algieba",
    astronomical_name: "Gamma Leonis",
    constellation: "Leo",
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
const HAMAL: RealData = RealData {
    common_name: "Hamal",
    astronomical_name: "Alpha Arietis",
    constellation: "Aries",
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
const DIPHDA: RealData = RealData {
    common_name: "Diphda",
    astronomical_name: "Beta Ceti",
    constellation: "Cetus",
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
const NUNKI: RealData = RealData {
    common_name: "Nunki",
    astronomical_name: "Sigma Sagittarii",
    constellation: "Sagittarius",
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
const MENKENT: RealData = RealData {
    common_name: "Menkent",
    astronomical_name: "Theta Centauri",
    constellation: "Centaurus",
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
const SAIPH: RealData = RealData {
    common_name: "Saiph",
    astronomical_name: "Kappa Orionis",
    constellation: "Orion",
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
const ALPHERATZ: RealData = RealData {
    common_name: "Alpheratz",
    astronomical_name: "Alpha Andromedae",
    constellation: "Andromeda",
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
const TIAKI: RealData = RealData {
    common_name: "Tiaki",
    astronomical_name: "Beta Gruis",
    constellation: "Grus",
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
const MIRACH: RealData = RealData {
    common_name: "Mirach",
    astronomical_name: "Beta Andromedae",
    constellation: "Andromeda",
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
const KOCHAB: RealData = RealData {
    common_name: "Kochab",
    astronomical_name: "Beta Ursae Minoris",
    constellation: "Ursa Minor",
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
const RASALHAGUE: RealData = RealData {
    common_name: "Rasalhague",
    astronomical_name: "Alpha Ophiuchi",
    constellation: "Ophiuchus",
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
const ALGOL: RealData = RealData {
    common_name: "Algol",
    astronomical_name: "Beta Persei",
    constellation: "Perseus",
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
const ALMACH: RealData = RealData {
    common_name: "Almach",
    astronomical_name: "Gamma Andromedae",
    constellation: "Andromeda",
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
const DENEBOLA: RealData = RealData {
    common_name: "Denebola",
    astronomical_name: "Beta Leonis",
    constellation: "Leo",
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
const NAVI: RealData = RealData {
    common_name: "Navi",
    astronomical_name: "Gamma Cassiopeiae",
    constellation: "Cassiopeia",
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
const MUHLIFAIN: RealData = RealData {
    common_name: "Muhlifain",
    astronomical_name: "Gamma Centauri",
    constellation: "Centaurus",
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
const NAOS: RealData = RealData {
    common_name: "Naos",
    astronomical_name: "Zeta Puppis",
    constellation: "Puppis",
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
const ASPIDISKE: RealData = RealData {
    common_name: "Aspidiske",
    astronomical_name: "Iota Carinae",
    constellation: "Carina",
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
const ALPHECCA: RealData = RealData {
    common_name: "Alphecca",
    astronomical_name: "Alpha Coronae Borealis",
    constellation: "Corona Borealis",
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
const SUHAIL: RealData = RealData {
    common_name: "Suhail",
    astronomical_name: "Lambda Velorum",
    constellation: "Vela",
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
const SADIR: RealData = RealData {
    common_name: "Sadir",
    astronomical_name: "Gamma Cygni",
    constellation: "Cygnus",
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
const MIZAR: RealData = RealData {
    common_name: "Mizar",
    astronomical_name: "Zeta Ursae Majoris",
    constellation: "Ursa Major",
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
const SCHEDAR: RealData = RealData {
    common_name: "Schedar",
    astronomical_name: "Alpha Cassiopeiae",
    constellation: "Cassiopeia",
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
const ELTANIN: RealData = RealData {
    common_name: "Eltanin",
    astronomical_name: "Gamma Draconis",
    constellation: "Draco",
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
const MINTAKA: RealData = RealData {
    common_name: "Mintaka",
    astronomical_name: "Delta Orionis",
    constellation: "Orion",
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
const CAPH: RealData = RealData {
    common_name: "Caph",
    astronomical_name: "Beta Cassiopeiae",
    constellation: "Cassiopeia",
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
const DSCHUBBA: RealData = RealData {
    common_name: "Dschubba",
    astronomical_name: "Delta Scorpii",
    constellation: "Scorpius",
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
const LARAWAG: RealData = RealData {
    common_name: "Larawag",
    astronomical_name: "Epsilon Scorpii",
    constellation: "Scorpius",
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
const EPSILON_CENTAURI: RealData = RealData {
    common_name: "",
    astronomical_name: "Epsilon Centauri",
    constellation: "Centaurus",
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
const ALPHA_LUPI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Lupi",
    constellation: "Lupus",
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
const ETA_CENTAURI: RealData = RealData {
    common_name: "",
    astronomical_name: "Eta Centauri",
    constellation: "Centaurus",
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
const MERAK: RealData = RealData {
    common_name: "Merak",
    astronomical_name: "Beta Ursae Majoris",
    constellation: "Ursa Major",
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
const IZAR: RealData = RealData {
    common_name: "Izar",
    astronomical_name: "Epsilon Bootis",
    constellation: "Boötes",
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
const ENIF: RealData = RealData {
    common_name: "Enif",
    astronomical_name: "Epsilon Pegasi",
    constellation: "Pegasus",
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
const GIRTAB: RealData = RealData {
    common_name: "Girtab",
    astronomical_name: "Kappa Scorpii",
    constellation: "Scorpius",
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
const ANKAA: RealData = RealData {
    common_name: "Ankaa",
    astronomical_name: "Alpha Phoenicis",
    constellation: "Phoenix",
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
const PHECDA: RealData = RealData {
    common_name: "Phecda",
    astronomical_name: "Gamma Ursae Majoris",
    constellation: "Ursa Major",
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
const SABIK: RealData = RealData {
    common_name: "Sabik",
    astronomical_name: "Eta Ophiuchi",
    constellation: "Ophiuchus",
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
const SCHEAT: RealData = RealData {
    common_name: "Scheat",
    astronomical_name: "Beta Pegasi",
    constellation: "Pegasus",
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
const ALDERAMIN: RealData = RealData {
    common_name: "Alderamin",
    astronomical_name: "Alpha Cephei",
    constellation: "Cepheus",
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
const ALUDRA: RealData = RealData {
    common_name: "Aludra",
    astronomical_name: "Eta Canis Majoris",
    constellation: "Canis Major",
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
const MARKEB: RealData = RealData {
    common_name: "Markeb",
    astronomical_name: "Kappa Velorum",
    constellation: "Vela",
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
const ALJANAH: RealData = RealData {
    common_name: "Aljanah",
    astronomical_name: "Epsilon Cygni",
    constellation: "Cygnus",
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
const MARKAB: RealData = RealData {
    common_name: "Markab",
    astronomical_name: "Alpha Pegasi",
    constellation: "Pegasus",
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
const HAN: RealData = RealData {
    common_name: "Han",
    astronomical_name: "Zeta Ophiuchi",
    constellation: "Ophiuchus",
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
const MENKAR: RealData = RealData {
    common_name: "Menkar",
    astronomical_name: "Alpha Ceti",
    constellation: "Cetus",
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
const ZETA_CENTAURI: RealData = RealData {
    common_name: "",
    astronomical_name: "Zeta Centauri",
    constellation: "Centaurus",
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
const ACRAB: RealData = RealData {
    common_name: "Acrab",
    astronomical_name: "Beta Scorpii",
    constellation: "Scorpius",
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
const ZOSMA: RealData = RealData {
    common_name: "Zosma",
    astronomical_name: "Delta Leonis",
    constellation: "Leo",
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
const MA_WEI: RealData = RealData {
    common_name: "Ma Wei",
    astronomical_name: "Delta Centauri",
    constellation: "Centaurus",
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
const ARNEB: RealData = RealData {
    common_name: "Arneb",
    astronomical_name: "Alpha Leporis",
    constellation: "Lepus",
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
const GHURAB: RealData = RealData {
    common_name: "Ghurab",
    astronomical_name: "Gamma Corvi",
    constellation: "Corvus",
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

const TEJAT: RealData = RealData {
    common_name: "Tejat",
    astronomical_name: "Mu Geminorum",
    constellation: "Gemini",
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

const R_DORADUS: RealData = RealData {
    common_name: "",
    astronomical_name: "R Doradus",
    constellation: "Dorado",
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
    constellation: "Ophiuchus",
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

const GORGONEA_TERTIA: RealData = RealData {
    common_name: "Gorgonea Tertia",
    astronomical_name: "Rho Persei",
    constellation: "Perseus",
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
    constellation: "Sagittarius",
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
    constellation: "Auriga",
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

const PROPUS: RealData = RealData {
    common_name: "Propus",
    astronomical_name: "Eta Geminorum",
    constellation: "Gemini",
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

const ZAURAK: RealData = RealData {
    common_name: "Zaurak",
    astronomical_name: "Gamma Eridani",
    constellation: "Eridanus",
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

const KAUS_MEDIA: RealData = RealData {
    common_name: "Kaus Media",
    astronomical_name: "Delta Sagittarii",
    constellation: "Sagittarius",
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

const BRACHIUM: RealData = RealData {
    common_name: "Brachium",
    astronomical_name: "Sigma Librae",
    constellation: "Libra",
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

const TANIA_AUSTRALIS: RealData = RealData {
    common_name: "Tania Australis",
    astronomical_name: "Mu Ursae Majoris",
    constellation: "Ursa Major",
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

const UNUKALHAI: RealData = RealData {
    common_name: "Unukalhai",
    astronomical_name: "Alpha Serpentis",
    constellation: "Serpens",
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
    constellation: "Lyra",
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

const BETA_ARAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Arae",
    constellation: "Ara",
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

const APLHA_TUCANAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Tucanae",
    constellation: "Tucana",
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

const MINELAUVA: RealData = RealData {
    common_name: "Minelauva",
    astronomical_name: "Delta Virginis",
    constellation: "Virgo",
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

const CEBALRAI: RealData = RealData {
    common_name: "Cebalrai",
    astronomical_name: "Beta Ophiuchi",
    constellation: "Ophiuchus",
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

const KRAZ: RealData = RealData {
    common_name: "Kraz",
    astronomical_name: "Beta Corvi",
    constellation: "Corvus",
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

const ERAKIS: RealData = RealData {
    common_name: "Erakis",
    astronomical_name: "Mu Cephei",
    constellation: "Cepheus",
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

const GAMMA_HYDRI: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Hydri",
    constellation: "Hydrus",
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

const ALPHA_LYNCIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Lyncis",
    constellation: "Lynx",
    radius: Some(Distance {
        m: 54.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.02,
    apparent_magnitude: 3.15,
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

const ATHEBYNE: RealData = RealData {
    common_name: "Athebyne",
    astronomical_name: "Eta Draconis",
    constellation: "Draco",
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

const AHADI: RealData = RealData {
    common_name: "Ahadi",
    astronomical_name: "Pi Puppis",
    constellation: "Puppis",
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

const TARAZED: RealData = RealData {
    common_name: "Tarazed",
    astronomical_name: "Gamma Aquilae",
    constellation: "Aquila",
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

const MEGREZ: RealData = RealData {
    common_name: "Megrez",
    astronomical_name: "Delta Ursae Majoris",
    constellation: "Ursa Major",
    radius: Some(Distance {
        m: 1.4 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.63 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.39,
    apparent_magnitude: 3.312,
    temperature: Some(Temperature { K: 9480. }),
    age: Some(Time {
        s: 0.3 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 15, 26),
    declination: Declination::new(Sgn::Pos, 57, 1, 57),
    distance: Distance {
        m: 80.5 * LIGHT_YEAR.m,
    },
};

const ALPHA_ANTLIAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Antliae",
    constellation: "Antlia",
    radius: Some(Distance {
        m: 41. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.97,
    apparent_magnitude: 4.25,
    temperature: Some(Temperature { K: 4070. }),
    age: None,
    right_ascension: RightAscension::new(10, 27, 9),
    declination: Declination::new(Sgn::Neg, 31, 4, 4),
    distance: Distance {
        m: 320. * LIGHT_YEAR.m,
    },
};

const EPSILON_ANTLIAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Epsilon Antliae",
    constellation: "Antlia",
    radius: Some(Distance {
        m: 56.3 * SOLAR_RADIUS.m,
    }),
    mass: None,
    absolute_magnitude: -2.17,
    apparent_magnitude: 4.51,
    temperature: Some(Temperature { K: 4237. }),
    age: None,
    right_ascension: RightAscension::new(9, 29, 15),
    declination: Declination::new(Sgn::Neg, 35, 57, 5),
    distance: Distance {
        m: 590. * LIGHT_YEAR.m,
    },
};

const IOTA_ANTLIAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Iota Antliae",
    constellation: "Antlia",
    radius: Some(Distance {
        m: 12.1 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.55 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.77,
    apparent_magnitude: 4.60,
    temperature: Some(Temperature { K: 4892. }),
    age: Some(Time {
        s: 3.32 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(10, 56, 43),
    declination: Declination::new(Sgn::Neg, 37, 8, 16),
    distance: Distance {
        m: 202. * LIGHT_YEAR.m,
    },
};

const ALPHA_APODIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Apodis",
    constellation: "Apus",
    radius: Some(Distance {
        m: 48. * SOLAR_RADIUS.m,
    }),
    mass: None,
    absolute_magnitude: -1.67,
    apparent_magnitude: 3.825,
    temperature: Some(Temperature { K: 4312. }),
    age: None,
    right_ascension: RightAscension::new(14, 47, 52),
    declination: Declination::new(Sgn::Neg, 79, 2, 41),
    distance: Distance {
        m: 430. * LIGHT_YEAR.m,
    },
};

const GAMMA_APODIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Apodis",
    constellation: "Apus",
    radius: None,
    mass: None,
    absolute_magnitude: 0.41,
    apparent_magnitude: 3.86,
    temperature: Some(Temperature { K: 5040. }),
    age: None,
    right_ascension: RightAscension::new(16, 33, 27),
    declination: Declination::new(Sgn::Neg, 78, 53, 50),
    distance: Distance {
        m: 150. * LIGHT_YEAR.m,
    },
};

const BETA_APODIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Apodis",
    constellation: "Apus",
    radius: Some(Distance {
        m: 11. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.84 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.819,
    apparent_magnitude: 4.24,
    temperature: Some(Temperature { K: 4900. }),
    age: None,
    right_ascension: RightAscension::new(16, 43, 5),
    declination: Declination::new(Sgn::Neg, 77, 31, 3),
    distance: Distance {
        m: 149. * LIGHT_YEAR.m,
    },
};

const SADALSUUD: RealData = RealData {
    common_name: "Sadalsuud",
    astronomical_name: "Beta Aquarii",
    constellation: "Aquarius",
    radius: Some(Distance {
        m: 47.88 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 4.97 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.04,
    apparent_magnitude: 2.87,
    temperature: Some(Temperature { K: 5608. }),
    age: Some(Time {
        s: 0.110 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(21, 31, 34),
    declination: Declination::new(Sgn::Neg, 5, 34, 16),
    distance: Distance {
        m: 550. * LIGHT_YEAR.m,
    },
};

const SADALMELIK: RealData = RealData {
    common_name: "Sadalmelik",
    astronomical_name: "Alpha Aquarii",
    constellation: "Aquarius",
    radius: Some(Distance {
        m: 53.89 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 5.13 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.882,
    apparent_magnitude: 2.942,
    temperature: Some(Temperature { K: 5383. }),
    age: Some(Time {
        s: 0.053 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(22, 5, 47),
    declination: Declination::new(Sgn::Neg, 0, 19, 11),
    distance: Distance {
        m: 520. * LIGHT_YEAR.m,
    },
};

const SKAT: RealData = RealData {
    common_name: "Skat",
    astronomical_name: "Delta Aquarii",
    constellation: "Aquarius",
    radius: Some(Distance {
        m: 2.4 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.51 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.17,
    apparent_magnitude: 3.28,
    temperature: Some(Temperature { K: 8650. }),
    age: Some(Time {
        s: 0.3 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(22, 54, 39),
    declination: Declination::new(Sgn::Neg, 15, 49, 15),
    distance: Distance {
        m: 113. * LIGHT_YEAR.m,
    },
};

const ALPHA_CAELI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Caeli",
    constellation: "Caelum",
    radius: Some(Distance {
        m: 1.3 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.48 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 3.39,
    apparent_magnitude: 4.456,
    temperature: Some(Temperature { K: 6991. }),
    age: Some(Time {
        s: 0.9 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(4, 40, 34),
    declination: Declination::new(Sgn::Neg, 41, 51, 50),
    distance: Distance {
        m: 65.7 * LIGHT_YEAR.m,
    },
};

const GAMMA1_CAELI: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma1 Caeli",
    constellation: "Caelum",
    radius: Some(Distance {
        m: 14.31 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.4 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.781,
    apparent_magnitude: 4.57,
    temperature: Some(Temperature { K: 4411. }),
    age: None,
    right_ascension: RightAscension::new(5, 4, 24),
    declination: Declination::new(Sgn::Neg, 35, 28, 59),
    distance: Distance {
        m: 185. * LIGHT_YEAR.m,
    },
};

const BETA_CAELI: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Caeli",
    constellation: "Caelum",
    radius: Some(Distance {
        m: 1.3 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.32 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 2.64,
    apparent_magnitude: 5.04,
    temperature: Some(Temperature { K: 6763. }),
    age: Some(Time {
        s: 1.753 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(4, 42, 3),
    declination: Declination::new(Sgn::Neg, 37, 8, 39),
    distance: Distance {
        m: 94. * LIGHT_YEAR.m,
    },
};

const BETA_CAMELOPARDALIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Camelopardalis",
    constellation: "Camelopardalis",
    radius: Some(Distance {
        m: 58. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 6.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.1,
    apparent_magnitude: 4.02,
    temperature: Some(Temperature { K: 5300. }),
    age: Some(Time {
        s: 0.053 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 3, 25),
    declination: Declination::new(Sgn::Pos, 60, 26, 32),
    distance: Distance {
        m: 870. * LIGHT_YEAR.m,
    },
};

const CS_CAMELOPARDALIS: RealData = RealData {
    common_name: "",
    astronomical_name: "CS Camelopardalis",
    constellation: "Camelopardalis",
    radius: Some(Distance {
        m: 85.7 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 19. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -7.1,
    apparent_magnitude: 4.22,
    temperature: Some(Temperature { K: 10_800. }),
    age: Some(Time {
        s: 0.016_5 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(3, 29, 4),
    declination: Declination::new(Sgn::Pos, 59, 56, 25),
    distance: Distance {
        m: 3100. * LIGHT_YEAR.m,
    },
};

const ALPHA_CAMELOPARDALIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Camelopardalis",
    constellation: "Camelopardalis",
    radius: Some(Distance {
        m: 32.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 37.6 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -7.1,
    apparent_magnitude: 4.29,
    temperature: Some(Temperature { K: 29_000. }),
    age: Some(Time {
        s: 0.002 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(4, 54, 3),
    declination: Declination::new(Sgn::Pos, 66, 20, 34),
    distance: Distance {
        m: 6_000. * LIGHT_YEAR.m,
    },
};

const TARF: RealData = RealData {
    common_name: "Tarf",
    astronomical_name: "Beta Cancri",
    constellation: "Cancer",
    radius: Some(Distance {
        m: 47.2 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.7 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.218,
    apparent_magnitude: 3.50,
    temperature: Some(Temperature { K: 4092. }),
    age: Some(Time {
        s: 1.85 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(8, 16, 31),
    declination: Declination::new(Sgn::Pos, 9, 11, 8),
    distance: Distance {
        m: 290. * LIGHT_YEAR.m,
    },
};

const ASELLUS_AUSTRALIS: RealData = RealData {
    common_name: "Asellus Australis",
    astronomical_name: "Delta Cancri",
    constellation: "Cancer",
    radius: Some(Distance {
        m: 11. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.71 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.843,
    apparent_magnitude: 3.94,
    temperature: Some(Temperature { K: 4637. }),
    age: Some(Time {
        s: 2.45 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(8, 44, 41),
    declination: Declination::new(Sgn::Pos, 18, 9, 16),
    distance: Distance {
        m: 131. * LIGHT_YEAR.m,
    },
};

const IOTA_CANCRI: RealData = RealData {
    common_name: "",
    astronomical_name: "Iota Cancri",
    constellation: "Cancer",
    radius: Some(Distance {
        m: 21. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.43 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.79,
    apparent_magnitude: 4.02,
    temperature: Some(Temperature { K: 4954. }),
    age: None,
    right_ascension: RightAscension::new(8, 46, 42),
    declination: Declination::new(Sgn::Pos, 28, 45, 36),
    distance: Distance {
        m: 330. * LIGHT_YEAR.m,
    },
};

const COR_CAROLI: RealData = RealData {
    common_name: "Cor Caroli",
    astronomical_name: "Alpha Canum Venaticorum",
    constellation: "Canes Venatici",
    radius: Some(Distance {
        m: 2.49 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.97 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.16,
    apparent_magnitude: 2.9,
    temperature: Some(Temperature { K: 11_600. }),
    age: Some(Time {
        s: 0.165 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 56, 2),
    declination: Declination::new(Sgn::Pos, 38, 19, 6),
    distance: Distance {
        m: 100. * LIGHT_YEAR.m,
    },
};

const CHARA: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Canum Venaticorum",
    constellation: "Canes Venatici",
    radius: Some(Distance {
        m: 1.123 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 0.97 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 4.64,
    apparent_magnitude: 4.25,
    temperature: Some(Temperature { K: 6043. }),
    age: Some(Time {
        s: 3.4 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 33, 45),
    declination: Declination::new(Sgn::Pos, 41, 21, 27),
    distance: Distance {
        m: 27.63 * LIGHT_YEAR.m,
    },
};

const TWENTYFOUR_CANUM_VENATICORUM: RealData = RealData {
    common_name: "",
    astronomical_name: "24 Canum Venaticorum",
    constellation: "Canes Venatici",
    right_ascension: RightAscension::new(13, 34, 27),
    declination: Declination::new(Sgn::Pos, 49, 0, 58),
    apparent_magnitude: 4.68,
    distance: Distance {
        m: 180. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.85,
    mass: Some(Mass {
        kg: 1.74 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.90 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 8285. }),
    age: Some(Time {
        s: 0.360 * BILLION_YEARS.s,
    }),
};

const BETA_CAPRICORNI: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Capricorni",
    constellation: "Capricornus",
    right_ascension: RightAscension::new(20, 21, 1),
    declination: Declination::new(Sgn::Neg, 14, 46, 53),
    apparent_magnitude: 3.05,
    distance: Distance {
        m: 390. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.03,
    mass: None,
    radius: None,
    temperature: None,
    age: None,
};

const DELTA_CAPRICORNI: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Capricorni",
    constellation: "Capricornus",
    right_ascension: RightAscension::new(21, 47, 2),
    declination: Declination::new(Sgn::Neg, 16, 7, 38),
    apparent_magnitude: 2.81,
    distance: Distance {
        m: 38.7 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.48,
    mass: Some(Mass {
        kg: 2. * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.91 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 7301. }),
    age: None,
};

const OMEGA_CAPRICORNI: RealData = RealData {
    common_name: "",
    astronomical_name: "Omega Capricorni",
    constellation: "Capricornus",
    right_ascension: RightAscension::new(20, 51, 49),
    declination: Declination::new(Sgn::Neg, 26, 55, 9),
    apparent_magnitude: 4.11,
    distance: Distance {
        m: 1000. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.7,
    mass: Some(Mass {
        kg: 6.8 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 172.1 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 3915. }),
    age: Some(Time {
        s: 0.0481 * BILLION_YEARS.s,
    }),
};

const ALPHA_CHAMAELEONTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Chamaeleontis",
    constellation: "Chamaeleon",
    right_ascension: RightAscension::new(8, 18, 32),
    declination: Declination::new(Sgn::Neg, 76, 55, 11),
    apparent_magnitude: 4.06,
    distance: Distance {
        m: 63.8 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.59,
    mass: Some(Mass {
        kg: 1.42 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.11 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 6580. }),
    age: Some(Time {
        s: 1.8 * BILLION_YEARS.s,
    }),
};

const GAMMA_CHAMAELEONTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Chamaeleontis",
    constellation: "Chamaeleon",
    right_ascension: RightAscension::new(10, 35, 28),
    declination: Declination::new(Sgn::Neg, 78, 36, 28),
    apparent_magnitude: 4.12,
    distance: Distance {
        m: 418. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.43,
    mass: None,
    radius: Some(Distance {
        m: 67. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4035. }),
    age: None,
};

const BETA_CHAMAELEONIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Chamaeleontis",
    constellation: "Chaemleon",
    right_ascension: RightAscension::new(12, 18, 21),
    declination: Declination::new(Sgn::Neg, 79, 18, 44),
    apparent_magnitude: 4.24,
    distance: Distance {
        m: 298. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.57,
    mass: Some(Mass {
        kg: 5. * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.84 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 14_495. }),
    age: Some(Time {
        s: 0.0227 * BILLION_YEARS.s,
    }),
};

const ALPHA_CIRCINI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Circini",
    constellation: "Circinus",
    right_ascension: RightAscension::new(14, 42, 30),
    declination: Declination::new(Sgn::Neg, 64, 58, 30),
    apparent_magnitude: 3.18,
    distance: Distance {
        m: 54. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.18,
    mass: Some(Mass {
        kg: 1.6 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.967 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 7500. }),
    age: Some(Time {
        s: 0.012 * BILLION_YEARS.s,
    }),
};

const BETA_CIRCINI: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Circini",
    constellation: "Circinus",
    right_ascension: RightAscension::new(15, 17, 31),
    declination: Declination::new(Sgn::Neg, 58, 48, 4),
    apparent_magnitude: 4.069,
    distance: Distance {
        m: 93. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.64,
    mass: None,
    radius: Some(Distance {
        m: 1.3 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 8676. }),
    age: Some(Time {
        s: 0.4 * BILLION_YEARS.s,
    }),
};

const GAMMA_CIRCINI: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Circini",
    constellation: "Circinus",
    right_ascension: RightAscension::new(15, 23, 23),
    declination: Declination::new(Sgn::Neg, 59, 19, 15),
    apparent_magnitude: 4.51,
    distance: Distance {
        m: 450. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.18,
    mass: Some(Mass {
        kg: 6. * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 15_135. }),
    age: Some(Time {
        s: 0.0631 * BILLION_YEARS.s,
    }),
};

const ALPHA_COLUMBAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Columbae",
    constellation: "Columba",
    right_ascension: RightAscension::new(5, 39, 39),
    declination: Declination::new(Sgn::Neg, 34, 4, 27),
    apparent_magnitude: 2.645,
    distance: Distance {
        m: 261. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.87,
    mass: Some(Mass {
        kg: 4.5 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 5.8 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 12_963. }),
    age: Some(Time {
        s: 0.093 * BILLION_YEARS.s,
    }),
};

const BETA_COLUMBAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Columbae",
    constellation: "Columba",
    right_ascension: RightAscension::new(5, 50, 58),
    declination: Declination::new(Sgn::Neg, 35, 46, 6),
    apparent_magnitude: 3.105,
    distance: Distance {
        m: 87.41 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.01,
    mass: Some(Mass {
        kg: 1.1 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 11.5 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4545. }),
    age: Some(Time {
        s: 2. * BILLION_YEARS.s,
    }),
};

const DELTA_COLUMBAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Columbae",
    constellation: "Columba",
    right_ascension: RightAscension::new(6, 22, 7),
    declination: Declination::new(Sgn::Neg, 33, 26, 11),
    apparent_magnitude: 3.85,
    distance: Distance {
        m: 234. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.32,
    mass: None,
    radius: None,
    temperature: Some(Temperature { K: 5136. }),
    age: None,
};

const EPSILON_COLUMBAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Epsilon Columbae",
    constellation: "Columba",
    right_ascension: RightAscension::new(5, 31, 13),
    declination: Declination::new(Sgn::Neg, 35, 28, 14),
    apparent_magnitude: 3.87,
    distance: Distance {
        m: 262. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.67,
    mass: Some(Mass {
        kg: 2.47 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 25.2 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4575. }),
    age: Some(Time {
        s: 1.53 * BILLION_YEARS.s,
    }),
};

const ALPHA_COMAE_BERENICES: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Comae Berenices",
    constellation: "Coma Berenices",
    right_ascension: RightAscension::new(13, 9, 59),
    declination: Declination::new(Sgn::Pos, 17, 31, 46),
    apparent_magnitude: 4.29,
    distance: Distance {
        m: 58.1 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 3.82,
    mass: Some(Mass {
        kg: 1.237 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 6365. }),
    age: None,
};

const BETA_COMA_BERENICES: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Comae Berenices",
    constellation: "Coma Berenices",
    right_ascension: RightAscension::new(13, 11, 53),
    declination: Declination::new(Sgn::Pos, 27, 52, 41),
    apparent_magnitude: 4.26,
    distance: Distance {
        m: 29.95 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 4.46,
    mass: Some(Mass {
        kg: 1.15 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.106 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5936. }),
    age: Some(Time {
        s: 2. * BILLION_YEARS.s,
    }),
};

const GAMMA_COMA_BERENICES: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Comae Berenices",
    constellation: "Coma Berenices",
    right_ascension: RightAscension::new(12, 26, 56),
    declination: Declination::new(Sgn::Pos, 28, 16, 6),
    apparent_magnitude: 4.36,
    distance: Distance {
        m: 169. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.76,
    mass: Some(Mass {
        kg: 1.65 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 11.76 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4652. }),
    age: Some(Time {
        s: 2.72 * BILLION_YEARS.s,
    }),
};

const ALPHA_CORONAE_AUSTRALIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Coronae Australis",
    constellation: "Corona Australis",
    right_ascension: RightAscension::new(19, 9, 28),
    declination: Declination::new(Sgn::Neg, 37, 54, 16),
    apparent_magnitude: 4.102,
    distance: Distance {
        m: 125. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.11,
    mass: Some(Mass {
        kg: 2.57 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.21 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 9916. }),
    age: Some(Time {
        s: 0.254 * BILLION_YEARS.s,
    }),
};

const BETA_CORONAE_AUSTRALIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Coronae Australis",
    constellation: "Corona Australis",
    right_ascension: RightAscension::new(19, 10, 2),
    declination: Declination::new(Sgn::Neg, 39, 20, 27),
    apparent_magnitude: 4.10,
    distance: Distance {
        m: 470. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.71,
    mass: Some(Mass {
        kg: 5.17 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 38.5 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4575. }),
    age: None,
};

const GAMMA_CORONAE_AUSTRALIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Coronae Australis",
    constellation: "Corona Australis",
    right_ascension: RightAscension::new(19, 6, 25),
    declination: Declination::new(Sgn::Neg, 37, 3, 48),
    apparent_magnitude: 4.2,
    distance: Distance {
        m: 56.4 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 3.73,
    mass: Some(Mass {
        kg: 1.15 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.47 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 6090. }),
    age: Some(Time {
        s: 5. * BILLION_YEARS.s,
    }),
};

const ALPHA_CRATERIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Crateris",
    constellation: "Crater",
    right_ascension: RightAscension::new(10, 59, 46),
    declination: Declination::new(Sgn::Neg, 18, 17, 56),
    apparent_magnitude: 4.07,
    distance: Distance {
        m: 141. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.44,
    mass: Some(Mass {
        kg: 1.81 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 12.32 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4691. }),
    age: Some(Time {
        s: 2.06 * BILLION_YEARS.s,
    }),
};

const BETA_CRATERIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Crateris",
    constellation: "Crater",
    right_ascension: RightAscension::new(11, 11, 39),
    declination: Declination::new(Sgn::Neg, 22, 49, 33),
    apparent_magnitude: 4.46,
    distance: Distance {
        m: 296. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.62,
    mass: None,
    radius: None,
    temperature: Some(Temperature { K: 8830. }),
    age: None,
};

const GAMMA_CRATERIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Crateris",
    constellation: "Crater",
    right_ascension: RightAscension::new(11, 24, 53),
    declination: Declination::new(Sgn::Neg, 17, 41, 2),
    apparent_magnitude: 4.06,
    distance: Distance {
        m: 85.6 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.05,
    mass: Some(Mass {
        kg: 1.81 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.3 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 8020. }),
    age: Some(Time {
        s: 0.757 * BILLION_YEARS.s,
    }),
};

const DELTA_CRATERIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Crateris",
    constellation: "Crater",
    right_ascension: RightAscension::new(11, 19, 20),
    declination: Declination::new(Sgn::Neg, 14, 46, 42),
    apparent_magnitude: 3.56,
    distance: Distance {
        m: 163. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.321,
    mass: Some(Mass {
        kg: 1.56 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 22.44 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4510. }),
    age: Some(Time {
        s: 2.89 * BILLION_YEARS.s,
    }),
};

const ALPHA_DELPHINI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Delphini",
    constellation: "Delphinus",
    right_ascension: RightAscension::new(20, 39, 38),
    declination: Declination::new(Sgn::Pos, 15, 54, 43),
    apparent_magnitude: 3.777,
    distance: Distance {
        m: 254. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.4,
    mass: Some(Mass {
        kg: 3.83 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 3.92 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 11_643. }),
    age: Some(Time {
        s: 0.227 * BILLION_YEARS.s,
    }),
};

const BETA_DELPHINI: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Delphini",
    constellation: "Delphinus",
    right_ascension: RightAscension::new(20, 37, 33),
    declination: Declination::new(Sgn::Pos, 14, 35, 42),
    apparent_magnitude: 3.617,
    distance: Distance {
        m: 101. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.58,
    mass: Some(Mass {
        kg: 1.75 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 6587. }),
    age: Some(Time {
        s: 1.79 * BILLION_YEARS.s,
    }),
};

const GAMMA_DELPHINI: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Delphini",
    constellation: "Delphinus",
    right_ascension: RightAscension::new(20, 46, 39),
    declination: Declination::new(Sgn::Pos, 16, 7, 27),
    apparent_magnitude: 5.14,
    distance: Distance {
        m: 114.8 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.24,
    mass: Some(Mass {
        kg: 1.61 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.6 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 6295. }),
    age: Some(Time {
        s: 1.85 * BILLION_YEARS.s,
    }),
};

const DELTA_DELPHINI: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Delphini",
    constellation: "Delphinus",
    right_ascension: RightAscension::new(20, 43, 28),
    declination: Declination::new(Sgn::Pos, 15, 4, 28),
    apparent_magnitude: 4.43,
    distance: Distance {
        m: 223. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.25,
    mass: Some(Mass {
        kg: 1.78 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 3.43 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 7440. }),
    age: Some(Time {
        s: 0.945 * BILLION_YEARS.s,
    }),
};

const EPSILON_DELPHINI: RealData = RealData {
    common_name: "",
    astronomical_name: "Epsilon Delphini",
    constellation: "Delphinus",
    right_ascension: RightAscension::new(20, 33, 13),
    declination: Declination::new(Sgn::Pos, 11, 18, 12),
    apparent_magnitude: 4.03,
    distance: Distance {
        m: 330. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.06,
    mass: None,
    radius: Some(Distance {
        m: 4.6 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 13_614. }),
    age: Some(Time {
        s: 0.220 * BILLION_YEARS.s,
    }),
};

const ALPHA_EQUULEI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Equulei",
    constellation: "Equuleus",
    right_ascension: RightAscension::new(21, 15, 49),
    declination: Declination::new(Sgn::Pos, 5, 14, 52),
    apparent_magnitude: 3.919,
    distance: Distance {
        m: 190. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.17,
    mass: Some(Mass {
        kg: 2.3 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 9.2 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5100. }),
    age: None,
};

const DELTA_EQUULEI: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Equulei",
    constellation: "Equuleus",
    right_ascension: RightAscension::new(21, 14, 29),
    declination: Declination::new(Sgn::Pos, 10, 0, 25),
    apparent_magnitude: 5.19,
    distance: Distance {
        m: 59.4 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 3.1,
    mass: Some(Mass {
        kg: 1.192 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.30 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 6200. }),
    age: Some(Time {
        s: 3. * BILLION_YEARS.s,
    }),
};

const GAMMA_EQUULEI: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Equulei",
    constellation: "Equuleus",
    right_ascension: RightAscension::new(21, 10, 21),
    declination: Declination::new(Sgn::Pos, 10, 7, 54),
    apparent_magnitude: 4.6,
    distance: Distance {
        m: 118. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.9,
    mass: Some(Mass {
        kg: 1.78 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.11 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 7550. }),
    age: Some(Time {
        s: 1. * BILLION_YEARS.s,
    }),
};

const ALPHA_FORNACIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Fornacis",
    constellation: "Fornax",
    right_ascension: RightAscension::new(3, 12, 5),
    declination: Declination::new(Sgn::Neg, 28, 59, 15),
    apparent_magnitude: 3.85,
    distance: Distance {
        m: 45.66 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 3.08,
    mass: Some(Mass {
        kg: 1.33 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.04 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 6240. }),
    age: Some(Time {
        s: 2.9 * BILLION_YEARS.s,
    }),
};

const BETA_FORNACIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Fornacis",
    constellation: "Fornax",
    right_ascension: RightAscension::new(2, 49, 5),
    declination: Declination::new(Sgn::Neg, 32, 24, 21),
    apparent_magnitude: 4.46,
    distance: Distance {
        m: 178. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.894,
    mass: Some(Mass {
        kg: 1.53 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 11.02 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4820. }),
    age: None,
};

const NU_FORNACIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Nu Fornacis",
    constellation: "Fornax",
    right_ascension: RightAscension::new(2, 4, 29),
    declination: Declination::new(Sgn::Neg, 29, 17, 49),
    apparent_magnitude: 4.69,
    distance: Distance {
        m: 370. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.6,
    mass: Some(Mass {
        kg: 3.65 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 3.44 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 13_400. }),
    age: None,
};

const ALPHA_HERCULIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Herculis",
    constellation: "Hercules",
    right_ascension: RightAscension::new(17, 14, 39),
    declination: Declination::new(Sgn::Pos, 14, 23, 25),
    apparent_magnitude: 3.350,
    distance: Distance {
        m: 360. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.3,
    mass: Some(Mass {
        kg: 2.5 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 284. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 3155. }),
    age: None,
};

const BETA_HERCULIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Herculis",
    constellation: "Herucles",
    right_ascension: RightAscension::new(16, 30, 13),
    declination: Declination::new(Sgn::Pos, 21, 29, 23),
    apparent_magnitude: 2.81,
    distance: Distance {
        m: 139. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.49,
    mass: Some(Mass {
        kg: 2.9 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 17. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4887. }),
    age: None,
};

const DELTA_HERCULIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Herculis",
    constellation: "Herucles",
    right_ascension: RightAscension::new(17, 15, 2),
    declination: Declination::new(Sgn::Pos, 24, 50, 21),
    apparent_magnitude: 3.126,
    distance: Distance {
        m: 75.1 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.31,
    mass: Some(Mass {
        kg: 2.4 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.2 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 9620. }),
    age: Some(Time {
        s: 0.370 * BILLION_YEARS.s,
    }),
};

const ETA_HERCULIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Eta Herculis",
    constellation: "Herucles",
    right_ascension: RightAscension::new(16, 42, 54),
    declination: Declination::new(Sgn::Pos, 38, 55, 20),
    apparent_magnitude: 3.487,
    distance: Distance {
        m: 112. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.84,
    mass: Some(Mass {
        kg: 2.13 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 8.9 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4900. }),
    age: Some(Time {
        s: 1. * BILLION_YEARS.s,
    }),
};

const MU_HERCULIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Mu Herculis",
    constellation: "Herucles",
    right_ascension: RightAscension::new(17, 46, 28),
    declination: Declination::new(Sgn::Pos, 27, 43, 14),
    apparent_magnitude: 3.417,
    distance: Distance {
        m: 27.11 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 3.82,
    mass: Some(Mass {
        kg: 1.11 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.73 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5560. }),
    age: Some(Time {
        s: 7.8 * BILLION_YEARS.s,
    }),
};

const ZETA_HERCULIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Zeta Herculis",
    constellation: "Herucles",
    right_ascension: RightAscension::new(16, 41, 17),
    declination: Declination::new(Sgn::Pos, 31, 36, 10),
    apparent_magnitude: 2.81,
    distance: Distance {
        m: 35. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.65,
    mass: Some(Mass {
        kg: 1.45 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.56 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5820. }),
    age: Some(Time {
        s: 6.2 * BILLION_YEARS.s,
    }),
};

const PI_HERCULIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Pi Herculis",
    constellation: "Herucles",
    right_ascension: RightAscension::new(17, 15, 3),
    declination: Declination::new(Sgn::Pos, 36, 48, 33),
    apparent_magnitude: 3.15,
    distance: Distance {
        m: 377. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.1,
    mass: Some(Mass {
        kg: 4. * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 72. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4170. }),
    age: None,
};

const ALPHA_HOROLOGII: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Horologii",
    constellation: "Horologium",
    right_ascension: RightAscension::new(4, 14, 0),
    declination: Declination::new(Sgn::Neg, 42, 17, 40),
    apparent_magnitude: 3.853,
    distance: Distance {
        m: 115. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.08,
    mass: Some(Mass {
        kg: 1.55 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 8. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5028. }),
    age: None,
};

const R_HOROLOGII: RealData = RealData {
    common_name: "",
    astronomical_name: "R Horologii",
    constellation: "Horologium",
    right_ascension: RightAscension::new(2, 53, 53),
    declination: Declination::new(Sgn::Neg, 49, 53, 23),
    apparent_magnitude: 6.,
    distance: Distance {
        m: 1000. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.,
    mass: None,
    radius: None,
    temperature: Some(Temperature { K: 2200. }),
    age: None,
};

const BETA_HOROLOGII: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Horologii",
    constellation: "Horologium",
    right_ascension: RightAscension::new(2, 58, 48),
    declination: Declination::new(Sgn::Neg, 64, 4, 17),
    apparent_magnitude: 4.979,
    distance: Distance {
        m: 312. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.2,
    mass: None,
    radius: Some(Distance {
        m: 1.4 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 8303. }),
    age: None,
};

const ALPHA_INDI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Indi",
    constellation: "Indus",
    right_ascension: RightAscension::new(20, 37, 34),
    declination: Declination::new(Sgn::Neg, 47, 17, 29),
    apparent_magnitude: 3.11,
    distance: Distance {
        m: 98.3 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.65,
    mass: Some(Mass {
        kg: 2.0 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 12. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4893. }),
    age: Some(Time {
        s: 1. * BILLION_YEARS.s,
    }),
};

const BETA_INDI: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Indi",
    constellation: "Indus",
    right_ascension: RightAscension::new(20, 54, 49),
    declination: Declination::new(Sgn::Neg, 58, 27, 15),
    apparent_magnitude: 3.67,
    distance: Distance {
        m: 600. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.664,
    mass: Some(Mass {
        kg: 6.7 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 55.58 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4541. }),
    age: Some(Time {
        s: 0.0532 * BILLION_YEARS.s,
    }),
};

const ETA_INDI: RealData = RealData {
    common_name: "",
    astronomical_name: "Eta Indi",
    constellation: "Indus",
    right_ascension: RightAscension::new(20, 44, 2),
    declination: Declination::new(Sgn::Neg, 51, 55, 15),
    apparent_magnitude: 4.52,
    distance: Distance {
        m: 78.8 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.59,
    mass: Some(Mass {
        kg: 1.6 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.27 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 7694. }),
    age: Some(Time {
        s: 0.1 * BILLION_YEARS.s,
    }),
};

const ALPHA_LACERTAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Lacertae",
    constellation: "Lacerta",
    right_ascension: RightAscension::new(22, 31, 18),
    declination: Declination::new(Sgn::Pos, 50, 16, 57),
    apparent_magnitude: 3.76,
    distance: Distance {
        m: 102.6 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.27,
    mass: Some(Mass {
        kg: 2.194 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.1432 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 9050. }),
    age: Some(Time {
        s: 0.4 * BILLION_YEARS.s,
    }),
};

const BETA_LACERTAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Lacertae",
    constellation: "Lacerta",
    right_ascension: RightAscension::new(22, 23, 34),
    declination: Declination::new(Sgn::Pos, 52, 13, 45),
    apparent_magnitude: 4.43,
    distance: Distance {
        m: 170. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.67,
    mass: Some(Mass {
        kg: 0.97 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 10.96 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4803. }),
    age: Some(Time {
        s: 6.76 * BILLION_YEARS.s,
    }),
};

const FIVE_LACERTAE: RealData = RealData {
    common_name: "",
    astronomical_name: "5 Lacertae",
    constellation: "Lacerta",
    right_ascension: RightAscension::new(22, 29, 32),
    declination: Declination::new(Sgn::Pos, 47, 42, 25),
    apparent_magnitude: 4.36,
    distance: Distance {
        m: 1600. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -3.42,
    mass: Some(Mass {
        kg: 5.11 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 319.2 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 3713. }),
    age: Some(Time {
        s: 110. * BILLION_YEARS.s,
    }),
};

const FOURTYSIX_LEONIS_MINORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "46 Leonis Minoris",
    constellation: "Leo Minor",
    right_ascension: RightAscension::new(10, 53, 19),
    declination: Declination::new(Sgn::Pos, 34, 12, 54),
    apparent_magnitude: 3.83,
    distance: Distance {
        m: 94.9 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.45,
    mass: Some(Mass {
        kg: 1.69 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 8.22 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4670. }),
    age: Some(Time {
        s: 6.76 * BILLION_YEARS.s,
    }),
};

const BETA_LEONIS_MINORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Leonis Minoris",
    constellation: "Leo Minor",
    right_ascension: RightAscension::new(10, 27, 53),
    declination: Declination::new(Sgn::Pos, 36, 42, 26),
    apparent_magnitude: 4.21,
    distance: Distance {
        m: 154. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.85,
    mass: Some(Mass {
        kg: 2.98 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 9.4 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4097. }),
    age: Some(Time {
        s: 1.2 * BILLION_YEARS.s,
    }),
};

const TWENTYFOUR_LEONIS_MINORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "21 Leonis Minoris",
    constellation: "Leo Minor",
    right_ascension: RightAscension::new(10, 7, 26),
    declination: Declination::new(Sgn::Pos, 35, 14, 41),
    apparent_magnitude: 4.5,
    distance: Distance {
        m: 92.1 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.43,
    mass: Some(Mass {
        kg: 1.75 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.75 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 7839. }),
    age: Some(Time {
        s: 0.390 * BILLION_YEARS.s,
    }),
};

const ALPHA_MENSAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Mensae",
    constellation: "Mensa",
    right_ascension: RightAscension::new(6, 10, 14),
    declination: Declination::new(Sgn::Neg, 74, 45, 11),
    apparent_magnitude: 5.09,
    distance: Distance {
        m: 33.31 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 5.03,
    mass: Some(Mass {
        kg: 0.964 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 0.960 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5569. }),
    age: Some(Time {
        s: 6.2 * BILLION_YEARS.s,
    }),
};

const BETA_MENSAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Mensae",
    constellation: "Mensa",
    right_ascension: RightAscension::new(5, 2, 43),
    declination: Declination::new(Sgn::Neg, 71, 18, 51),
    apparent_magnitude: 5.31,
    distance: Distance {
        m: 660. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.62,
    mass: Some(Mass {
        kg: 3.58 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 25.85 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5088. }),
    age: Some(Time {
        s: 0.270 * BILLION_YEARS.s,
    }),
};

const GAMMA_MENSAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Mensae",
    constellation: "Mensa",
    right_ascension: RightAscension::new(5, 31, 53),
    declination: Declination::new(Sgn::Neg, 76, 20, 27),
    apparent_magnitude: 5.19,
    distance: Distance {
        m: 104.9 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.70,
    mass: Some(Mass {
        kg: 1.04 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 4.99 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4491. }),
    age: Some(Time {
        s: 10.60 * BILLION_YEARS.s,
    }),
};

const GAMMA_MICROSCOPII: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Microscopii",
    constellation: "Microscopium",
    right_ascension: RightAscension::new(21, 1, 17),
    declination: Declination::new(Sgn::Neg, 32, 15, 28),
    apparent_magnitude: 4.680,
    distance: Distance {
        m: 223. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.49,
    mass: Some(Mass {
        kg: 2.5 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 10. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5050. }),
    age: Some(Time {
        s: 0.620 * BILLION_YEARS.s,
    }),
};

const EPSILON_MICROSCOPII: RealData = RealData {
    common_name: "",
    astronomical_name: "Epsilon Microscopii",
    constellation: "Microscopium",
    right_ascension: RightAscension::new(21, 17, 56),
    declination: Declination::new(Sgn::Neg, 32, 10, 21),
    apparent_magnitude: 4.71,
    distance: Distance {
        m: 166. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.97,
    mass: Some(Mass {
        kg: 2.18 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.2 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 9126. }),
    age: Some(Time {
        s: 0.525 * BILLION_YEARS.s,
    }),
};

const THETA1_MICROSCOPII: RealData = RealData {
    common_name: "",
    astronomical_name: "Theta1 Microscopii",
    constellation: "Microscopium",
    right_ascension: RightAscension::new(21, 20, 46),
    declination: Declination::new(Sgn::Neg, 40, 48, 34),
    apparent_magnitude: 4.82,
    distance: Distance {
        m: 179. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.03,
    mass: Some(Mass {
        kg: 2.32 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.35 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 9240. }),
    age: Some(Time {
        s: 0.437 * BILLION_YEARS.s,
    }),
};

const ALPHA_MONOCEROTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Monocerotis",
    constellation: "Monoceros",
    right_ascension: RightAscension::new(7, 41, 15),
    declination: Declination::new(Sgn::Neg, 9, 33, 4),
    apparent_magnitude: 3.94,
    distance: Distance {
        m: 148. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.71,
    mass: Some(Mass {
        kg: 2.02 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 10.1 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4879. }),
    age: Some(Time {
        s: 1.18 * BILLION_YEARS.s,
    }),
};

const GAMMA_MONOCEROTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Monocerotis",
    constellation: "Monoceros",
    right_ascension: RightAscension::new(6, 14, 51),
    declination: Declination::new(Sgn::Neg, 6, 16, 29),
    apparent_magnitude: 3.96,
    distance: Distance {
        m: 500. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.93,
    mass: None,
    radius: None,
    temperature: Some(Temperature { K: 4375. }),
    age: None,
};

const DELTA_MONOCEROTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Monocerotis",
    constellation: "Monoceros",
    right_ascension: RightAscension::new(7, 11, 52),
    declination: Declination::new(Sgn::Neg, 0, 29, 34),
    apparent_magnitude: 4.15,
    distance: Distance {
        m: 384. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.20,
    mass: Some(Mass {
        kg: 2.4 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 9462. }),
    age: Some(Time {
        s: 0.405 * BILLION_YEARS.s,
    }),
};

const ALPHA_MUSCAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Muscae",
    constellation: "Musca",
    right_ascension: RightAscension::new(12, 37, 11),
    declination: Declination::new(Sgn::Neg, 69, 8, 8),
    apparent_magnitude: 2.69,
    distance: Distance {
        m: 315. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.2,
    mass: Some(Mass {
        kg: 8.8 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 4.8 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 21_400. }),
    age: Some(Time {
        s: 18.3 * BILLION_YEARS.s,
    }),
};

const BETA_MUSCAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Muscae",
    constellation: "Musca",
    right_ascension: RightAscension::new(12, 46, 17),
    declination: Declination::new(Sgn::Neg, 68, 6, 29),
    apparent_magnitude: 3.05,
    distance: Distance {
        m: 340. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.06,
    mass: Some(Mass {
        kg: 7.35 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: None,
    age: Some(Time {
        s: 0.0151 * BILLION_YEARS.s,
    }),
};

const DELTA_MUSCAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Muscae",
    constellation: "Musca",
    right_ascension: RightAscension::new(13, 2, 16),
    declination: Declination::new(Sgn::Neg, 71, 32, 56),
    apparent_magnitude: 3.61,
    distance: Distance {
        m: 91. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.38,
    mass: None,
    radius: None,
    temperature: None,
    age: None,
};

const GAMMA2_NORMAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma2 Normae",
    constellation: "Norma",
    right_ascension: RightAscension::new(16, 19, 50),
    declination: Declination::new(Sgn::Neg, 50, 9, 20),
    apparent_magnitude: 4.02,
    distance: Distance {
        m: 129. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.057,
    mass: Some(Mass {
        kg: 2.16 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 4699. }),
    age: None,
};

const EPSILON_NORMAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Epsilon Normae",
    constellation: "Norma",
    right_ascension: RightAscension::new(16, 27, 11),
    declination: Declination::new(Sgn::Neg, 47, 33, 17),
    apparent_magnitude: 4.47,
    distance: Distance {
        m: 530. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.06,
    mass: Some(Mass {
        kg: 6.4 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 10_888. }),
    age: Some(Time {
        s: 0.0501 * BILLION_YEARS.s,
    }),
};

const IOTA1_NORMAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Ioata1 Normae",
    constellation: "Norma",
    right_ascension: RightAscension::new(16, 3, 32),
    declination: Declination::new(Sgn::Neg, 57, 46, 30),
    apparent_magnitude: 4.69,
    distance: Distance {
        m: 128. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.46,
    mass: Some(Mass {
        kg: 1.94 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 7842. }),
    age: Some(Time {
        s: 0.731 * BILLION_YEARS.s,
    }),
};

const NU_OCTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Nu Octantis",
    constellation: "Octans",
    right_ascension: RightAscension::new(21, 41, 29),
    declination: Declination::new(Sgn::Neg, 77, 23, 24),
    apparent_magnitude: 3.73,
    distance: Distance {
        m: 63.3 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.10,
    mass: Some(Mass {
        kg: 1.04 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 5.9 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4860. }),
    age: Some(Time {
        s: 2.5 * BILLION_YEARS.s,
    }),
};

const BETA_OCTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Octantis",
    constellation: "Octans",
    right_ascension: RightAscension::new(22, 46, 4),
    declination: Declination::new(Sgn::Neg, 81, 22, 54),
    apparent_magnitude: 4.13,
    distance: Distance {
        m: 149. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.83,
    mass: Some(Mass {
        kg: 2.27 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 3.2 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 8006. }),
    age: Some(Time {
        s: 496. * BILLION_YEARS.s,
    }),
};

const DELTA_OCTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Octantis",
    constellation: "Octans",
    right_ascension: RightAscension::new(14, 26, 55),
    declination: Declination::new(Sgn::Neg, 83, 40, 4),
    apparent_magnitude: 4.31,
    distance: Distance {
        m: 299. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.35,
    mass: Some(Mass {
        kg: 1.06 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 24.61 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4311. }),
    age: None,
};

const ALPHA_PICTORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Pictoris",
    constellation: "Pictor",
    right_ascension: RightAscension::new(6, 48, 11),
    declination: Declination::new(Sgn::Neg, 61, 56, 29),
    apparent_magnitude: 3.27,
    distance: Distance {
        m: 97. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.86,
    mass: Some(Mass {
        kg: 2.04 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.6 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 7530. }),
    age: Some(Time {
        s: 0.660 * BILLION_YEARS.s,
    }),
};

const BETA_PICTORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Pictoris",
    constellation: "Pictor",
    right_ascension: RightAscension::new(5, 47, 17),
    declination: Declination::new(Sgn::Neg, 51, 3, 59),
    apparent_magnitude: 3.861,
    distance: Distance {
        m: 63.4 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.402,
    mass: Some(Mass {
        kg: 1.75 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.8 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 8052. }),
    age: Some(Time {
        s: 0.023 * BILLION_YEARS.s,
    }),
};

const GAMMA_PICTORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Pictoris",
    constellation: "Pictor",
    right_ascension: RightAscension::new(5, 49, 50),
    declination: Declination::new(Sgn::Neg, 56, 9, 60),
    apparent_magnitude: 4.50,
    distance: Distance {
        m: 177. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.83,
    mass: Some(Mass {
        kg: 1.59 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 11. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4600. }),
    age: None,
};

const ALPHA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(2, 2, 3),
    declination: Declination::new(Sgn::Pos, 2, 45, 50),
    apparent_magnitude: 3.82,
    distance: Distance {
        m: 151. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.5,
    mass: Some(Mass {
        kg: 2.55 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.45 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 10_233. }),
    age: None,
};

const DELTA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(0, 48, 41),
    declination: Declination::new(Sgn::Pos, 7, 35, 6),
    apparent_magnitude: 4.416,
    distance: Distance {
        m: 311. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.46,
    mass: Some(Mass {
        kg: 1.65 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 44. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 3963. }),
    age: Some(Time {
        s: 0.00298 * BILLION_YEARS.s,
    }),
};

const NU_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Nu Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(1, 41, 26),
    declination: Declination::new(Sgn::Pos, 5, 29, 15),
    apparent_magnitude: 4.44,
    distance: Distance {
        m: 363. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.78,
    mass: Some(Mass {
        kg: 1.66 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 34. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4154. }),
    age: Some(Time {
        s: 3.41 * BILLION_YEARS.s,
    }),
};

const IOTA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Ioata Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(23, 39, 57),
    declination: Declination::new(Sgn::Pos, 5, 37, 35),
    apparent_magnitude: 4.13,
    distance: Distance {
        m: 44.73 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 3.43,
    mass: None,
    radius: Some(Distance {
        m: 1.595 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 6288. }),
    age: Some(Time {
        s: 5.2 * BILLION_YEARS.s,
    }),
};

const OMICRON_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Omicron Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(1, 45, 24),
    declination: Declination::new(Sgn::Pos, 9, 9, 28),
    apparent_magnitude: 4.27,
    distance: Distance {
        m: 280. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.22,
    mass: Some(Mass {
        kg: 3.03 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 14.57 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5004. }),
    age: Some(Time {
        s: 0.390 * BILLION_YEARS.s,
    }),
};

const EPSILON_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Epislon Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(1, 2, 57),
    declination: Declination::new(Sgn::Pos, 7, 53, 24),
    apparent_magnitude: 4.27,
    distance: Distance {
        m: 182. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.44,
    mass: Some(Mass {
        kg: 2.27 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 10.9 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4814. }),
    age: Some(Time {
        s: 2.56 * BILLION_YEARS.s,
    }),
};

const THETA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Theta Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(23, 27, 58),
    declination: Declination::new(Sgn::Pos, 6, 22, 44),
    apparent_magnitude: 4.27,
    distance: Distance {
        m: 149. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.83,
    mass: Some(Mass {
        kg: 1.58 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 11. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4684. }),
    age: Some(Time {
        s: 0.00245 * BILLION_YEARS.s,
    }),
};

const ETA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Eta Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(1, 31, 29),
    declination: Declination::new(Sgn::Pos, 15, 20, 45),
    apparent_magnitude: 3.611,
    distance: Distance {
        m: 350. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.52,
    mass: Some(Mass {
        kg: 3.78 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 26.48 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4937. }),
    age: Some(Time {
        s: 0.220 * BILLION_YEARS.s,
    }),
};

const GAMMA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(23, 17, 10),
    declination: Declination::new(Sgn::Pos, 3, 16, 56),
    apparent_magnitude: 3.699,
    distance: Distance {
        m: 135. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.68,
    mass: Some(Mass {
        kg: 0.97 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 11.28 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4833. }),
    age: Some(Time {
        s: 4.58 * BILLION_YEARS.s,
    }),
};

const OMEGA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Omega Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(23, 59, 19),
    declination: Declination::new(Sgn::Pos, 6, 51, 48),
    apparent_magnitude: 4.01,
    distance: Distance {
        m: 104.3 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.51,
    mass: Some(Mass {
        kg: 1.22 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 6641. }),
    age: Some(Time {
        s: 1.337 * BILLION_YEARS.s,
    }),
};

const ALPHA_PYXIDIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Pyxidis",
    constellation: "Pyxis",
    right_ascension: RightAscension::new(8, 43, 36),
    declination: Declination::new(Sgn::Neg, 33, 11, 11),
    apparent_magnitude: 3.67,
    distance: Distance {
        m: 880. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -3.47,
    mass: Some(Mass {
        kg: 10.7 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 6.3 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 24_300. }),
    age: None,
};

const BETA_PYXIDIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Pyxidis",
    constellation: "Pyxis",
    right_ascension: RightAscension::new(8, 40, 6),
    declination: Declination::new(Sgn::Neg, 35, 18, 30),
    apparent_magnitude: 3.954,
    distance: Distance {
        m: 420. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.78,
    mass: Some(Mass {
        kg: 1.2 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 24. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5124. }),
    age: None,
};

const GAMMA_PYXIDIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Pyxidis",
    constellation: "Pyxis",
    right_ascension: RightAscension::new(8, 50, 32),
    declination: Declination::new(Sgn::Neg, 27, 42, 35),
    apparent_magnitude: 4.010,
    distance: Distance {
        m: 207. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.,
    mass: Some(Mass {
        kg: 1.64 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 21.87 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4270. }),
    age: Some(Time {
        s: 4.29 * BILLION_YEARS.s,
    }),
};

const ALPHA_RETICULI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Reticuli",
    constellation: "Reticulum",
    right_ascension: RightAscension::new(4, 14, 25),
    declination: Declination::new(Sgn::Neg, 62, 28, 26),
    apparent_magnitude: 3.315,
    distance: Distance {
        m: 161.6 * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.17,
    mass: Some(Mass {
        kg: 3.11 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 12.8 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5196. }),
    age: Some(Time {
        s: 0.33 * BILLION_YEARS.s,
    }),
};

const BETA_RETICULI: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Reticuli",
    constellation: "Reticulum",
    right_ascension: RightAscension::new(3, 44, 12),
    declination: Declination::new(Sgn::Neg, 64, 48, 25),
    apparent_magnitude: 3.84,
    distance: Distance {
        m: 97. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.46,
    mass: Some(Mass {
        kg: 1.2 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 9.3 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4580. }),
    age: Some(Time {
        s: 5. * BILLION_YEARS.s,
    }),
};

const EPSILON_RETICULI: RealData = RealData {
    common_name: "",
    astronomical_name: "Epsilon Reticuli",
    constellation: "Reticulum",
    right_ascension: RightAscension::new(4, 16, 29),
    declination: Declination::new(Sgn::Neg, 59, 18, 8),
    apparent_magnitude: 4.44,
    distance: Distance {
        m: 60.1 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.87,
    mass: Some(Mass {
        kg: 1.46 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 3.18 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4961. }),
    age: Some(Time {
        s: 2.89 * BILLION_YEARS.s,
    }),
};

const GAMMA_SAGITTAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Sagittae",
    constellation: "Sagitta",
    right_ascension: RightAscension::new(19, 58, 45),
    declination: Declination::new(Sgn::Pos, 19, 29, 32),
    apparent_magnitude: 3.47,
    distance: Distance {
        m: 288. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.11,
    mass: Some(Mass {
        kg: 0.88 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 55.13 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 3862. }),
    age: Some(Time {
        s: 2.35 * BILLION_YEARS.s,
    }),
};

const DELTA_SAGITTAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Sagittae",
    constellation: "Sagitta",
    right_ascension: RightAscension::new(19, 47, 23),
    declination: Declination::new(Sgn::Pos, 18, 32, 4),
    apparent_magnitude: 3.82,
    distance: Distance {
        m: 550. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.58,
    mass: Some(Mass {
        kg: 3.35 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 108. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 3660. }),
    age: None,
};

const ALPHA_SAGITTAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Sagittae",
    constellation: "Sagitta",
    right_ascension: RightAscension::new(19, 40, 6),
    declination: Declination::new(Sgn::Pos, 18, 0, 50),
    apparent_magnitude: 4.38,
    distance: Distance {
        m: 382. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.96,
    mass: Some(Mass {
        kg: 4.11 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 21. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5333. }),
    age: Some(Time {
        s: 0.151 * BILLION_YEARS.s,
    }),
};

const ALPHA_SCULPTORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Sculptoris",
    constellation: "Sculptor",
    right_ascension: RightAscension::new(0, 58, 36),
    declination: Declination::new(Sgn::Neg, 29, 21, 27),
    apparent_magnitude: 4.30,
    distance: Distance {
        m: 780. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.58,
    mass: Some(Mass {
        kg: 5.01 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 7.52 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 13_600. }),
    age: Some(Time {
        s: 0.093 * BILLION_YEARS.s,
    }),
};

const BETA_SCULPTORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Sculptoris",
    constellation: "Sculptor",
    right_ascension: RightAscension::new(23, 32, 58),
    declination: Declination::new(Sgn::Neg, 37, 49, 6),
    apparent_magnitude: 4.37,
    distance: Distance {
        m: 174. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.74,
    mass: Some(Mass {
        kg: 2.98 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.0 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 12_110. }),
    age: None,
};

const GAMMA_SULPTORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Sculptoris",
    constellation: "Sculptor",
    right_ascension: RightAscension::new(23, 18, 49),
    declination: Declination::new(Sgn::Neg, 32, 31, 55),
    apparent_magnitude: 4.41,
    distance: Distance {
        m: 182. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.67,
    mass: Some(Mass {
        kg: 1.6 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 12. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4578. }),
    age: Some(Time {
        s: 2.47 * BILLION_YEARS.s,
    }),
};

const ALPHA_SCUTI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Scuti",
    constellation: "Scutum",
    right_ascension: RightAscension::new(18, 35, 12),
    declination: Declination::new(Sgn::Neg, 8, 14, 39),
    apparent_magnitude: 3.83,
    distance: Distance {
        m: 199. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.08,
    mass: Some(Mass {
        kg: 1.33 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 20. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4315. }),
    age: None,
};

const BETA_SCUTI: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Scuti",
    constellation: "Scutum",
    right_ascension: RightAscension::new(18, 47, 10),
    declination: Declination::new(Sgn::Neg, 4, 44, 52),
    apparent_magnitude: 4.22,
    distance: Distance {
        m: 900. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.99,
    mass: None,
    radius: None,
    temperature: Some(Temperature { K: 4622. }),
    age: None,
};

const ZETA_SCUTI: RealData = RealData {
    common_name: "",
    astronomical_name: "Zeata Scuti",
    constellation: "Scutum",
    right_ascension: RightAscension::new(18, 23, 40),
    declination: Declination::new(Sgn::Neg, 8, 56, 4),
    apparent_magnitude: 4.66,
    distance: Distance {
        m: 210. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.66,
    mass: Some(Mass {
        kg: 1.29 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 9.3 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4750. }),
    age: None,
};

const ALPHA_SEXTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Sextantis",
    constellation: "Sextans",
    right_ascension: RightAscension::new(10, 7, 56),
    declination: Declination::new(Sgn::Neg, 0, 22, 18),
    apparent_magnitude: 4.49,
    distance: Distance {
        m: 280. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.29,
    mass: Some(Mass {
        kg: 2.57 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 3.07 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 9984. }),
    age: Some(Time {
        s: 0.385 * BILLION_YEARS.s,
    }),
};

const GAMMA_SEXTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Sextantis",
    constellation: "Sextans",
    right_ascension: RightAscension::new(9, 52, 30),
    declination: Declination::new(Sgn::Neg, 8, 6, 18),
    apparent_magnitude: 5.05,
    distance: Distance {
        m: 280. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.43,
    mass: Some(Mass {
        kg: 2.60 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 9825. }),
    age: Some(Time {
        s: 0.401 * BILLION_YEARS.s,
    }),
};

const BETA_SEXTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Sextantis",
    constellation: "Sextans",
    right_ascension: RightAscension::new(10, 30, 17),
    declination: Declination::new(Sgn::Neg, 0, 38, 13),
    apparent_magnitude: 5.07,
    distance: Distance {
        m: 364. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.38,
    mass: None,
    radius: Some(Distance {
        m: 3.2 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 14_570. }),
    age: None,
};

const ALPHA_TELESCOPII: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Telescopii",
    constellation: "Telescopium",
    right_ascension: RightAscension::new(18, 26, 58),
    declination: Declination::new(Sgn::Neg, 45, 58, 6),
    apparent_magnitude: 3.51,
    distance: Distance {
        m: 278. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.25,
    mass: Some(Mass {
        kg: 5.2 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 3.3 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 16_700. }),
    age: Some(Time {
        s: 0.0241 * BILLION_YEARS.s,
    }),
};

const ZETA_TELESCOPII: RealData = RealData {
    common_name: "",
    astronomical_name: "Zeata Telescopii",
    constellation: "Telescopium",
    right_ascension: RightAscension::new(18, 28, 50),
    declination: Declination::new(Sgn::Neg, 49, 4, 14),
    apparent_magnitude: 4.13,
    distance: Distance {
        m: 126. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.171,
    mass: Some(Mass {
        kg: 1.53 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 9. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4801. }),
    age: None,
};

const EPSILON_TELESCOPII: RealData = RealData {
    common_name: "",
    astronomical_name: "Epislon Telescopii",
    constellation: "Telescopium",
    right_ascension: RightAscension::new(18, 11, 14),
    declination: Declination::new(Sgn::Neg, 45, 57, 16),
    apparent_magnitude: 4.50,
    distance: Distance {
        m: 410. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.,
    mass: None,
    radius: None,
    temperature: Some(Temperature { K: 4996. }),
    age: None,
};

const BETA_TRIANGULI: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Trianguli",
    constellation: "Triangulum",
    right_ascension: RightAscension::new(2, 9, 33),
    declination: Declination::new(Sgn::Pos, 34, 59, 14),
    apparent_magnitude: 3.,
    distance: Distance {
        m: 127. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.05,
    mass: Some(Mass {
        kg: 3.5 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 8186. }),
    age: Some(Time {
        s: 0.73 * BILLION_YEARS.s,
    }),
};

const ALPHA_TRIANGULI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Trianguli",
    constellation: "Triangulum",
    right_ascension: RightAscension::new(1, 53, 5),
    declination: Declination::new(Sgn::Pos, 29, 34, 44),
    apparent_magnitude: 3.42,
    distance: Distance {
        m: 63.3 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.98,
    mass: Some(Mass {
        kg: 1.70 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 3.22 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 6288. }),
    age: Some(Time {
        s: 1.6 * BILLION_YEARS.s,
    }),
};

const GAMMA_TRIANGULI: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Trianguli",
    constellation: "Triangulum",
    right_ascension: RightAscension::new(2, 17, 19),
    declination: Declination::new(Sgn::Pos, 33, 50, 50),
    apparent_magnitude: 4.01,
    distance: Distance {
        m: 112.3 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.35,
    mass: Some(Mass {
        kg: 2.7 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.96 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 9440. }),
    age: Some(Time {
        s: 0.3 * BILLION_YEARS.s,
    }),
};

const GAMMA1_VOLANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma1 Volantis",
    constellation: "Volans",
    right_ascension: RightAscension::new(7, 8, 42),
    declination: Declination::new(Sgn::Neg, 70, 29, 50),
    apparent_magnitude: 5.704,
    distance: Distance {
        m: 143. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.51,
    mass: Some(Mass {
        kg: 1.69 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 6541. }),
    age: Some(Time {
        s: 1.4 * BILLION_YEARS.s,
    }),
};

const BETA_VOLANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Volantis",
    constellation: "Volans",
    right_ascension: RightAscension::new(8, 25, 44),
    declination: Declination::new(Sgn::Neg, 66, 8, 13),
    apparent_magnitude: 3.75,
    distance: Distance {
        m: 107.5 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.18,
    mass: Some(Mass {
        kg: 1.62 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 4546. }),
    age: None,
};

const ZETA_VOLANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Zeata Volantis",
    constellation: "Volans",
    right_ascension: RightAscension::new(7, 41, 49),
    declination: Declination::new(Sgn::Neg, 72, 36, 22),
    apparent_magnitude: 3.93,
    distance: Distance {
        m: 141. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.75,
    mass: Some(Mass {
        kg: 1.74 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 11. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4721. }),
    age: Some(Time {
        s: 5.27 * BILLION_YEARS.s,
    }),
};

const ALPHA_VULPECULAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Vulpeculae",
    constellation: "Vulpecula",
    right_ascension: RightAscension::new(19, 28, 42),
    declination: Declination::new(Sgn::Pos, 24, 39, 54),
    apparent_magnitude: 4.40,
    distance: Distance {
        m: 291. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.36,
    mass: Some(Mass {
        kg: 0.97 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 43.14 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 3690. }),
    age: Some(Time {
        s: 11.3 * BILLION_YEARS.s,
    }),
};

const TWENTYTHREE_VULPECULAE: RealData = RealData {
    common_name: "",
    astronomical_name: "23 Vulpeculae",
    constellation: "Vulpecula",
    right_ascension: RightAscension::new(20, 15, 46),
    declination: Declination::new(Sgn::Pos, 27, 48, 51),
    apparent_magnitude: 4.52,
    distance: Distance {
        m: 327. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.58,
    mass: Some(Mass {
        kg: 2.4 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 4429. }),
    age: None,
};

const THIRTYONE_VULPECULAE: RealData = RealData {
    common_name: "",
    astronomical_name: "31 Vulpeculae",
    constellation: "Vulpecula",
    right_ascension: RightAscension::new(20, 52, 8),
    declination: Declination::new(Sgn::Pos, 27, 5, 49),
    apparent_magnitude: 4.56,
    distance: Distance {
        m: 228. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.77,
    mass: Some(Mass {
        kg: 2.4 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 8.01 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5261. }),
    age: Some(Time {
        s: 0.7 * BILLION_YEARS.s,
    }),
};

// RealData = RealData {
//     common_name: "",
//     astronomical_name: "",
//     constellation: "",
//     right_ascension: RightAscension::new(),
//     declination: Declination::new(),
//     apparent_magnitude: ,
//     distance: Distance{m:  * LIGHT_YEAR.m},
//     absolute_magnitude: ,
//     mass: Some(Mass{kg: * SOLAR_MASS.kg}),
//     radius: Some(Distance{m: * SOLAR_RADIUS.m}),
//     temperature: Some(Temperature{K:}),
//     age: Some(Time{s:* BILLION_YEARS.s}),
// };

pub const BRIGHTEST_STARS: [RealData; 260] = [
    SIRIUS,
    CANOPUS,
    ARCTURUS,
    RIGEL_KENTAURUS,
    VEGA,
    CAPELLA,
    RIGEL,
    PROCYON,
    BETELGEUSE,
    ACHERNAR,
    HADAR,
    ALTAIR,
    ACRUX,
    ALDEBARAN,
    SPICA,
    ANTARES,
    POLLUX,
    FORMALHAUT,
    DENEB,
    MIMOSA,
    REGULUS,
    ADHARA,
    CASTOR,
    GACRUX,
    SHAULA,
    BELLATRIX,
    ALNATH,
    MIAPLACIDUS,
    ALNILAM,
    ALNAIR,
    ALNITAK,
    REGOR,
    ALIOTH,
    KAUS_AUSTRALIS,
    MIRPHAK,
    DUBHE,
    WEZEN,
    ALKAID,
    SARGAS,
    AVIOR,
    MENKALINAN,
    ATRIA,
    ALSEPHINA,
    ALHENA,
    PEACOCK,
    POLARIS,
    MIRZAM,
    ALPHARD,
    ALGIEBA,
    HAMAL,
    DIPHDA,
    NUNKI,
    MENKENT,
    SAIPH,
    ALPHERATZ,
    TIAKI,
    MIRACH,
    KOCHAB,
    RASALHAGUE,
    ALGOL,
    ALMACH,
    DENEBOLA,
    NAVI,
    MUHLIFAIN,
    NAOS,
    ASPIDISKE,
    ALPHECCA,
    SUHAIL,
    SADIR,
    MIZAR,
    SCHEDAR,
    ELTANIN,
    MINTAKA,
    CAPH,
    DSCHUBBA,
    LARAWAG,
    EPSILON_CENTAURI,
    ALPHA_LUPI,
    ETA_CENTAURI,
    MERAK,
    IZAR,
    ENIF,
    GIRTAB,
    ANKAA,
    PHECDA,
    SABIK,
    SCHEAT,
    ALDERAMIN,
    ALUDRA,
    MARKEB,
    ALJANAH,
    MARKAB,
    HAN,
    MENKAR,
    ZETA_CENTAURI,
    ACRAB,
    ZOSMA,
    MA_WEI,
    ARNEB,
    GHURAB,
    TEJAT,
    R_DORADUS,
    YED_PRIOR,
    GORGONEA_TERTIA,
    NAMALWARID,
    HASSALEH,
    PROPUS,
    ZAURAK,
    KAUS_MEDIA,
    BRACHIUM,
    TANIA_AUSTRALIS,
    UNUKALHAI,
    R_LYRAE_DATE,
    BETA_ARAE,
    APLHA_TUCANAE,
    MINELAUVA,
    CEBALRAI,
    KRAZ,
    ERAKIS,
    GAMMA_HYDRI,
    ALPHA_LYNCIS,
    ATHEBYNE,
    AHADI,
    TARAZED,
    MEGREZ,
    ALPHA_ANTLIAE,
    EPSILON_ANTLIAE,
    IOTA_ANTLIAE,
    ALPHA_APODIS,
    GAMMA_APODIS,
    BETA_APODIS,
    SADALSUUD,
    SADALMELIK,
    SKAT,
    ALPHA_CAELI,
    GAMMA1_CAELI,
    BETA_CAELI,
    BETA_CAMELOPARDALIS,
    CS_CAMELOPARDALIS,
    ALPHA_CAMELOPARDALIS,
    TARF,
    ASELLUS_AUSTRALIS,
    IOTA_CANCRI,
    COR_CAROLI,
    CHARA,
    TWENTYFOUR_CANUM_VENATICORUM,
    BETA_CAPRICORNI,
    DELTA_CAPRICORNI,
    OMEGA_CAPRICORNI,
    ALPHA_CHAMAELEONTIS,
    GAMMA_CHAMAELEONTIS,
    BETA_CHAMAELEONIS,
    ALPHA_CIRCINI,
    BETA_CIRCINI,
    GAMMA_CIRCINI,
    ALPHA_COLUMBAE,
    BETA_COLUMBAE,
    DELTA_COLUMBAE,
    EPSILON_COLUMBAE,
    ALPHA_COMAE_BERENICES,
    BETA_COMA_BERENICES,
    GAMMA_COMA_BERENICES,
    ALPHA_CORONAE_AUSTRALIS,
    BETA_CORONAE_AUSTRALIS,
    GAMMA_CORONAE_AUSTRALIS,
    ALPHA_CRATERIS,
    BETA_CRATERIS,
    GAMMA_CRATERIS,
    DELTA_CRATERIS,
    ALPHA_DELPHINI,
    BETA_DELPHINI,
    GAMMA_DELPHINI,
    DELTA_DELPHINI,
    EPSILON_DELPHINI,
    ALPHA_EQUULEI,
    DELTA_EQUULEI,
    GAMMA_EQUULEI,
    ALPHA_FORNACIS,
    BETA_FORNACIS,
    NU_FORNACIS,
    ALPHA_HERCULIS,
    BETA_HERCULIS,
    DELTA_HERCULIS,
    ETA_HERCULIS,
    MU_HERCULIS,
    ZETA_HERCULIS,
    PI_HERCULIS,
    ALPHA_HOROLOGII,
    R_HOROLOGII,
    BETA_HOROLOGII,
    ALPHA_INDI,
    BETA_INDI,
    ETA_INDI,
    ALPHA_LACERTAE,
    BETA_LACERTAE,
    FIVE_LACERTAE,
    FOURTYSIX_LEONIS_MINORIS,
    BETA_LEONIS_MINORIS,
    TWENTYFOUR_LEONIS_MINORIS,
    ALPHA_MENSAE,
    BETA_MENSAE,
    GAMMA_MENSAE,
    GAMMA_MICROSCOPII,
    EPSILON_MICROSCOPII,
    THETA1_MICROSCOPII,
    ALPHA_MONOCEROTIS,
    GAMMA_MONOCEROTIS,
    DELTA_MONOCEROTIS,
    ALPHA_MUSCAE,
    BETA_MUSCAE,
    DELTA_MUSCAE,
    GAMMA2_NORMAE,
    EPSILON_NORMAE,
    IOTA1_NORMAE,
    NU_OCTANTIS,
    BETA_OCTANTIS,
    DELTA_OCTANTIS,
    ALPHA_PICTORIS,
    BETA_PICTORIS,
    GAMMA_PICTORIS,
    ALPHA_PISCIUM,
    DELTA_PISCIUM,
    NU_PISCIUM,
    IOTA_PISCIUM,
    OMICRON_PISCIUM,
    EPSILON_PISCIUM,
    THETA_PISCIUM,
    ETA_PISCIUM,
    GAMMA_PISCIUM,
    OMEGA_PISCIUM,
    ALPHA_PYXIDIS,
    BETA_PYXIDIS,
    GAMMA_PYXIDIS,
    ALPHA_RETICULI,
    BETA_RETICULI,
    EPSILON_RETICULI,
    GAMMA_SAGITTAE,
    DELTA_SAGITTAE,
    ALPHA_SAGITTAE,
    ALPHA_SCULPTORIS,
    BETA_SCULPTORIS,
    GAMMA_SULPTORIS,
    ALPHA_SCUTI,
    BETA_SCUTI,
    ZETA_SCUTI,
    ALPHA_SEXTANTIS,
    GAMMA_SEXTANTIS,
    BETA_SEXTANTIS,
    ALPHA_TELESCOPII,
    ZETA_TELESCOPII,
    EPSILON_TELESCOPII,
    BETA_TRIANGULI,
    ALPHA_TRIANGULI,
    GAMMA_TRIANGULI,
    GAMMA1_VOLANTIS,
    BETA_VOLANTIS,
    ZETA_VOLANTIS,
    ALPHA_VULPECULAE,
    TWENTYTHREE_VULPECULAE,
    THIRTYONE_VULPECULAE,
];
