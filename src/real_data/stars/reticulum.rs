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

const ALPHA_RETICULI: RealData = RealData {
    common_name: "",
    astronomical_name: "α Reticuli",
    constellation: "Reticulum",
    right_ascension: RightAscension::new(4, 14, 25.),
    declination: Declination::new(Sgn::Neg, 62, 28, 26.),
    apparent_magnitude: 3.315,
    distance: Distance {
        m: 161.6 * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.17,
    mass: Mass {
        kg: 3.11 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 12.8 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5196. },
    age: Some(Time {
        s: 0.33 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.351318702 * BILLION_YEARS.s,
    },
};

const BETA_RETICULI: RealData = RealData {
    common_name: "",
    astronomical_name: "β Reticuli",
    constellation: "Reticulum",
    right_ascension: RightAscension::new(3, 44, 12.),
    declination: Declination::new(Sgn::Neg, 64, 48, 25.),
    apparent_magnitude: 3.84,
    distance: Distance {
        m: 97. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.46,
    mass: Mass {
        kg: 1.2 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 9.3 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4580. },
    age: Some(Time {
        s: 5. * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 5.06543331 * BILLION_YEARS.s,
    },
};

const EPSILON_RETICULI: RealData = RealData {
    common_name: "",
    astronomical_name: "ε Reticuli",
    constellation: "Reticulum",
    right_ascension: RightAscension::new(4, 16, 29.),
    declination: Declination::new(Sgn::Neg, 59, 18, 8.),
    apparent_magnitude: 4.44,
    distance: Distance {
        m: 60.1 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 3.1,
    mass: Mass {
        kg: 1.46 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 3.18 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4961. },
    age: Some(Time {
        s: 2.8 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 2.82957282 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_RETICULI, BETA_RETICULI, EPSILON_RETICULI];
