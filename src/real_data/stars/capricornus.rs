use astro_coords::ra_and_dec::*;
use simple_si_units::base::{Length, Mass, Temperature, Time};

use crate::{
    stars::real_data::RealData,
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

const DABIH: RealData = RealData {
    common_name: "Dabih",
    astronomical_name: "β Capricorni",
    constellation: "Capricornus",
    right_ascension: RightAscension::new(20, 21, 1.),
    declination: Declination::new(Sgn::Neg, 14, 46, 53.),
    apparent_magnitude: 3.05,
    distance: Length {
        m: 555.4 * LIGHT_YEAR.m,
    },
    absolute_magnitude: -3., // Not literature value
    mass: Mass {
        kg: 3.9 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 4900. },
    age: None,
    lifetime: Time {
        s: 0.220601963 * BILLION_YEARS.s,
    },
};

const DENEB_ALGEDI: RealData = RealData {
    common_name: "Deneb Algedi",
    astronomical_name: "δ Capricorni",
    constellation: "Capricornus",
    right_ascension: RightAscension::new(21, 47, 2.),
    declination: Declination::new(Sgn::Neg, 16, 7, 38.),
    apparent_magnitude: 2.81,
    distance: Length {
        m: 38.7 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.48,
    mass: Mass {
        kg: 2. * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 1.91 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 7301. },
    age: None,
    lifetime: Time {
        s: 1.36020165 * BILLION_YEARS.s,
    },
};

const OMEGA_CAPRICORNI: RealData = RealData {
    common_name: "",
    astronomical_name: "ω Capricorni",
    constellation: "Capricornus",
    right_ascension: RightAscension::new(20, 51, 49.),
    declination: Declination::new(Sgn::Neg, 26, 55, 9.),
    apparent_magnitude: 4.12,
    distance: Length {
        m: 628.1 * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.3,
    mass: Mass {
        kg: 6.8 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 172.1 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 3915. },
    age: Some(Time {
        s: 0.0481 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.052267043 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [DABIH, DENEB_ALGEDI, OMEGA_CAPRICORNI];
