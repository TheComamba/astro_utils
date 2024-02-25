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

const ALKES: RealData = RealData {
    common_name: "Alkes",
    astronomical_name: "Alpha Crateris",
    constellation: "Crater",
    right_ascension: RightAscension::new(10, 59, 46),
    declination: Declination::new(Sgn::Neg, 18, 17, 56),
    apparent_magnitude: 4.08,
    distance: Distance {
        m: 174.2 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.44,
    mass: Mass {
        kg: 1.81 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 12.32 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4691. },
    age: Some(Time {
        s: 2.06 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.46605285 * BILLION_YEARS.s,
    },
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
    mass: Mass {
        kg: 2.6 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 8830. },
    age: None,
    lifetime: Time {
        s: 0.63513384 * BILLION_YEARS.s,
    },
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
    mass: Mass {
        kg: 1.81 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 1.3 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 8020. },
    age: Some(Time {
        s: 0.757 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.46605285 * BILLION_YEARS.s,
    },
};

const DELTA_CRATERIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Crateris",
    constellation: "Crater",
    right_ascension: RightAscension::new(11, 19, 20),
    declination: Declination::new(Sgn::Neg, 14, 46, 42),
    apparent_magnitude: 3.56,
    distance: Distance {
        m: 194.6 * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.321,
    mass: Mass {
        kg: 1.56 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 22.44 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4510. },
    age: Some(Time {
        s: 2.89 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 2.29668629 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 4] = [ALKES, BETA_CRATERIS, GAMMA_CRATERIS, DELTA_CRATERIS];
