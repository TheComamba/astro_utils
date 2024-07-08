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

const ALPHA_CIRCINI: RealData = RealData {
    common_name: "",
    astronomical_name: "α Circini",
    constellation: "Circinus",
    right_ascension: RightAscension::new(14, 42, 30.),
    declination: Declination::new(Sgn::Neg, 64, 58, 30.),
    apparent_magnitude: 3.18,
    distance: Distance {
        m: 54. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.18,
    mass: Mass {
        kg: 1.6 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 1.967 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 7500. },
    age: Some(Time {
        s: 0.012 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 2.08398753 * BILLION_YEARS.s,
    },
};

const BETA_CIRCINI: RealData = RealData {
    common_name: "",
    astronomical_name: "β Circini",
    constellation: "Circinus",
    right_ascension: RightAscension::new(15, 17, 31.),
    declination: Declination::new(Sgn::Neg, 58, 48, 4.),
    apparent_magnitude: 4.069,
    distance: Distance {
        m: 93. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.64,
    mass: Mass {
        kg: 2.2 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 1.3 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 8676. },
    age: Some(Time {
        s: 0.4 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.03650581 * BILLION_YEARS.s,
    },
};

const GAMMA_CIRCINI: RealData = RealData {
    common_name: "",
    astronomical_name: "γ Circini",
    constellation: "Circinus",
    right_ascension: RightAscension::new(15, 23, 23.),
    declination: Declination::new(Sgn::Neg, 59, 19, 15.),
    apparent_magnitude: 4.51,
    distance: Distance {
        m: 450. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.18,
    mass: Mass {
        kg: 6. * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 15_135. },
    age: Some(Time {
        s: 0.0631 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.073299383 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_CIRCINI, BETA_CIRCINI, GAMMA_CIRCINI];
