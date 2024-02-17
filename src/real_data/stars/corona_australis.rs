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

pub(crate) const STARS: [RealData; 3] = [
    ALPHA_CORONAE_AUSTRALIS,
    BETA_CORONAE_AUSTRALIS,
    GAMMA_CORONAE_AUSTRALIS,
];
