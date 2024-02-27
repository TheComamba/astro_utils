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

const ALPHA_CHAMAELEONTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "α Chamaeleontis",
    constellation: "Chamaeleon",
    right_ascension: RightAscension::new(8, 18, 32),
    declination: Declination::new(Sgn::Neg, 76, 55, 11),
    apparent_magnitude: 4.06,
    distance: Distance {
        m: 63.8 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.59,
    mass: Mass {
        kg: 1.42 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 2.11 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 6580. },
    age: Some(Time {
        s: 1.8 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 3.10253119 * BILLION_YEARS.s,
    },
};

const GAMMA_CHAMAELEONTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "γ Chamaeleontis",
    constellation: "Chamaeleon",
    right_ascension: RightAscension::new(10, 35, 28),
    declination: Declination::new(Sgn::Neg, 78, 36, 28),
    apparent_magnitude: 4.12,
    distance: Distance {
        m: 418. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.43,
    mass: Mass {
        kg: 2.4 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 67. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4035. },
    age: None,
    lifetime: Time {
        s: 0.800458342 * BILLION_YEARS.s,
    },
};

const BETA_CHAMAELEONIS: RealData = RealData {
    common_name: "",
    astronomical_name: "β Chamaeleontis",
    constellation: "Chamaeleon",
    right_ascension: RightAscension::new(12, 18, 21),
    declination: Declination::new(Sgn::Neg, 79, 18, 44),
    apparent_magnitude: 4.24,
    distance: Distance {
        m: 298. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.57,
    mass: Mass {
        kg: 5.9 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 2.84 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 14_495. },
    age: Some(Time {
        s: 0.0227 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.078916095 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] =
    [ALPHA_CHAMAELEONTIS, GAMMA_CHAMAELEONTIS, BETA_CHAMAELEONIS];
