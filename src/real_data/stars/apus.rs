use crate::{
    coordinates::{
        declination::{Declination, Sgn},
        right_ascension::RightAscension,
    },
    stars::{fate::StarFate, real_data::RealData},
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};
use simple_si_units::base::{Distance, Mass, Temperature, Time};

const ALPHA_APODIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Apodis",
    constellation: "Apus",
    radius: Some(Distance {
        m: 48. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.67,
    apparent_magnitude: 3.825,
    temperature: Temperature { K: 4312. },
    age: None,
    lifetime: Time {
        s: 6.3 * BILLION_YEARS.s,
    },
    fate: StarFate::WhiteDwarf,
    right_ascension: RightAscension::new(14, 47, 52),
    declination: Declination::new(Sgn::Neg, 79, 2, 41),
    distance: Distance {
        m: 411.1 * LIGHT_YEAR.m,
    },
};

const GAMMA_APODIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Apodis",
    constellation: "Apus",
    radius: None,
    mass: Some(Mass {
        kg: 0.95 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.41,
    apparent_magnitude: 3.86,
    temperature: Temperature { K: 5040. },
    age: None,
    lifetime: Time {
        s: 12. * BILLION_YEARS.s,
    },
    fate: StarFate::WhiteDwarf,
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
    temperature: Temperature { K: 4900. },
    age: None,
    lifetime: Time {
        s: 7.9 * BILLION_YEARS.s,
    },
    fate: StarFate::WhiteDwarf,
    right_ascension: RightAscension::new(16, 43, 5),
    declination: Declination::new(Sgn::Neg, 77, 31, 3),
    distance: Distance {
        m: 149. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_APODIS, GAMMA_APODIS, BETA_APODIS];
