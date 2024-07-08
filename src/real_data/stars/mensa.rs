use astro_coords::ra_and_dec::*;
use simple_si_units::base::{Distance, Mass, Temperature, Time};

use crate::{
    stars::real_data::RealData,
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

const ALPHA_MENSAE: RealData = RealData {
    common_name: "",
    astronomical_name: "α Mensae",
    constellation: "Mensa",
    right_ascension: RightAscension::new(6, 10, 14.),
    declination: Declination::new(Sgn::Neg, 74, 45, 11.),
    apparent_magnitude: 5.09,
    distance: Distance {
        m: 33.31 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 5.03,
    mass: Mass {
        kg: 0.964 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 0.960 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5569. },
    age: Some(Time {
        s: 6.2 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 11.7800188 * BILLION_YEARS.s,
    },
};

const BETA_MENSAE: RealData = RealData {
    common_name: "",
    astronomical_name: "β Mensae",
    constellation: "Mensa",
    right_ascension: RightAscension::new(5, 2, 43.),
    declination: Declination::new(Sgn::Neg, 71, 18, 51.),
    apparent_magnitude: 5.31,
    distance: Distance {
        m: 641.7 * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.17,
    mass: Mass {
        kg: 3.58 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 25.85 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5088. },
    age: Some(Time {
        s: 0.250 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.254814649 * BILLION_YEARS.s,
    },
};

const GAMMA_MENSAE: RealData = RealData {
    common_name: "",
    astronomical_name: "γ Mensae",
    constellation: "Mensa",
    right_ascension: RightAscension::new(5, 31, 53.),
    declination: Declination::new(Sgn::Neg, 76, 20, 27.),
    apparent_magnitude: 5.19,
    distance: Distance {
        m: 104.9 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.70,
    mass: Mass {
        kg: 1.04 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 4.99 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4491. },
    age: Some(Time {
        s: 8. * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 8.24015833 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_MENSAE, BETA_MENSAE, GAMMA_MENSAE];
