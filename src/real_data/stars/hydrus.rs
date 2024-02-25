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

const ALPHA_HYDRI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Hydri",
    constellation: "Hydrus",
    right_ascension: RightAscension::new(1, 58, 46),
    declination: Declination::new(Sgn::Neg, 61, 34, 11),
    apparent_magnitude: 2.9,
    distance: Distance {
        m: 71.8 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.153,
    mass: Mass {
        kg: 2. * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 3.040 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 7087. },
    age: Some(Time {
        s: 0.810 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 3.1 * BILLION_YEARS.s,
    },
};

const BETA_HYDRI: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Hydri",
    constellation: "Hydrus",
    right_ascension: RightAscension::new(0, 25, 45),
    declination: Declination::new(Sgn::Neg, 77, 15, 15),
    apparent_magnitude: 2.8,
    distance: Distance {
        m: 24.33 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 3.45,
    mass: Mass {
        kg: 1.08 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 1.809 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5872. },
    age: Some(Time {
        s: 6.4 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 9.5 * BILLION_YEARS.s,
    },
};

const GAMMA_HYDRI: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Hydri",
    constellation: "Hydrus",
    radius: Some(Distance {
        m: 62. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1. * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.83,
    apparent_magnitude: 3.26,
    temperature: Temperature { K: 3499. },
    age: None,
    lifetime: Time {
        s: 6.3 * BILLION_YEARS.s,
    },
    right_ascension: RightAscension::new(3, 47, 14),
    declination: Declination::new(Sgn::Neg, 74, 14, 20),
    distance: Distance {
        m: 214. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_HYDRI, BETA_HYDRI, GAMMA_HYDRI];
