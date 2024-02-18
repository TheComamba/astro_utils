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

const ALPHA_ARAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Arae",
    constellation: "Ara",
    right_ascension: RightAscension::new(17, 31, 50),
    declination: Declination::new(Sgn::Neg, 49, 52, 34),
    apparent_magnitude: 2.93,
    distance: Distance {
        m: 270. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.72,
    mass: Some(Mass {
        kg: 9.6 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 4.5 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 18_044. }),
    age: None,
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

const GAMMA_ARAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Arae",
    constellation: "Ara",
    right_ascension: RightAscension::new(17, 25, 24),
    declination: Declination::new(Sgn::Neg, 56, 22, 40),
    apparent_magnitude: 3.34,
    distance: Distance {
        m: 1110. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -5.8,
    mass: Some(Mass {
        kg: 20. * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 23. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 21_500. }),
    age: Some(Time {
        s: 0.0157 * BILLION_YEARS.s,
    }),
};

const DELTA_ARAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Arae",
    constellation: "Ara",
    right_ascension: RightAscension::new(17, 31, 6),
    declination: Declination::new(Sgn::Neg, 60, 41, 2),
    apparent_magnitude: 3.62,
    distance: Distance {
        m: 198. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.31,
    mass: Some(Mass {
        kg: 3.56 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 3.12 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 11_962. }),
    age: Some(Time {
        s: 0.125 * BILLION_YEARS.s,
    }),
};

const ZETA_ARAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Zeata Arae",
    constellation: "Ara",
    right_ascension: RightAscension::new(16, 58, 37),
    declination: Declination::new(Sgn::Neg, 55, 59, 25),
    apparent_magnitude: 3.13,
    distance: Distance {
        m: 570. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -3.95,
    mass: Some(Mass {
        kg: 7.5 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 114. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4246. }),
    age: Some(Time {
        s: 0.045 * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 5] = [ALPHA_ARAE, BETA_ARAE, GAMMA_ARAE, DELTA_ARAE, ZETA_ARAE];
