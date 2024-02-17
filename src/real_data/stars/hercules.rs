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
    constellation: "Hercules",
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
    constellation: "Hercules",
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
    constellation: "Hercules",
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
    constellation: "Hercules",
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
    constellation: "Hercules",
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
    constellation: "Hercules",
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

pub(crate) const STARS: [RealData; 7] = [
    ALPHA_HERCULIS,
    BETA_HERCULIS,
    DELTA_HERCULIS,
    ETA_HERCULIS,
    MU_HERCULIS,
    ZETA_HERCULIS,
    PI_HERCULIS,
];
