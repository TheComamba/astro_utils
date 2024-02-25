use crate::{
    coordinates::{
        declination::{Declination, Sgn},
        right_ascension::RightAscension,
    },
    stars::real_data::RealData,
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};
use simple_si_units::base::{Distance, Mass, Temperature, Time};

const POLARIS: RealData = RealData {
    common_name: "Polaris",
    astronomical_name: "Alpha Ursae Minoris",
    constellation: "Ursa Minor",
    radius: Some(Distance {
        m: 37.5 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 5.4 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -3.64,
    apparent_magnitude: 1.97,
    temperature: Temperature { K: 6015. },
    age: Some(Time {
        s: 0.05 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(2, 31, 49),
    declination: Declination::new(Sgn::Pos, 89, 15, 51),
    distance: Distance {
        m: 431. * LIGHT_YEAR.m,
    },
};

const KOCHAB: RealData = RealData {
    common_name: "Kochab",
    astronomical_name: "Beta Ursae Minoris",
    constellation: "Ursa Minor",
    radius: Some(Distance {
        m: 42.06 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 2.2 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.87,
    apparent_magnitude: 2.07,
    temperature: Temperature { K: 4030. },
    age: None,
    right_ascension: RightAscension::new(14, 50, 42),
    declination: Declination::new(Sgn::Pos, 74, 9, 20),
    distance: Distance {
        m: 126. * LIGHT_YEAR.m,
    },
};

const ZETA_URSAE_MINORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Zeta Ursae Minoris",
    constellation: "Ursa Minor",
    right_ascension: RightAscension::new(15, 44, 4),
    declination: Declination::new(Sgn::Pos, 77, 47, 40),
    apparent_magnitude: 4.29,
    distance: Distance {
        m: 369. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.98,
    mass: Mass {
        kg: 3.4 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 6.15 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 8720. },
    age: Some(Time {
        s: 0.180 * BILLION_YEARS.s,
    }),
};

const DELTA_URSAE_MINORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Ursae Minoris",
    constellation: "Ursa Minor",
    right_ascension: RightAscension::new(17, 32, 13),
    declination: Declination::new(Sgn::Pos, 86, 35, 11),
    apparent_magnitude: 4.36,
    distance: Distance {
        m: 172. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.62,
    mass: Mass {
        kg: 2.35 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 2.8 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 9911. },
    age: Some(Time {
        s: 0.327 * BILLION_YEARS.s,
    }),
};

const ETA_URSAE_MINORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Eta Ursae Minoris",
    constellation: "Ursa Minor",
    right_ascension: RightAscension::new(16, 17, 30),
    declination: Declination::new(Sgn::Pos, 75, 45, 19),
    apparent_magnitude: 4.95,
    distance: Distance {
        m: 97.6 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.61,
    mass: Mass {
        kg: 1.35 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 2.0 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 6858. },
    age: Some(Time {
        s: 1.061 * BILLION_YEARS.s,
    }),
};

const PHERKAD: RealData = RealData {
    common_name: "Pherkad",
    astronomical_name: "Gamma Ursae Minoris",
    constellation: "Ursa Minor",
    right_ascension: RightAscension::new(15, 20, 44),
    declination: Declination::new(Sgn::Pos, 71, 50, 2),
    apparent_magnitude: 3.05,
    distance: Distance {
        m: 487. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.84,
    mass: Mass {
        kg: 9. * SOLAR_MASS.kg, //guessed
    },
    radius: Some(Distance {
        m: 15. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 8280. },
    age: None,
};

const EPSILON_URSAE_MINORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Epsilon Ursae Minoris",
    constellation: "Ursa Minor",
    right_ascension: RightAscension::new(16, 45, 58),
    declination: Declination::new(Sgn::Pos, 82, 2, 14),
    apparent_magnitude: 4.19,
    distance: Distance {
        m: 300. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.922,
    mass: Mass {
        kg: 1.1 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 5215. },
    age: None,
};

pub(crate) const STARS: [RealData; 7] = [
    POLARIS,
    KOCHAB,
    ZETA_URSAE_MINORIS,
    DELTA_URSAE_MINORIS,
    ETA_URSAE_MINORIS,
    PHERKAD,
    EPSILON_URSAE_MINORIS,
];
