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

const ANKAA: RealData = RealData {
    common_name: "Ankaa",
    astronomical_name: "α Phoenicis",
    constellation: "Phoenix",
    radius: Some(Distance {
        m: 15. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1.57 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 0.52,
    apparent_magnitude: 2.4,
    temperature: Temperature { K: 4436. },
    age: None,
    right_ascension: RightAscension::new(0, 26, 17.),
    declination: Declination::new(Sgn::Neg, 42, 18, 21.),
    distance: Distance {
        m: 77. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 2.29668629 * BILLION_YEARS.s,
    },
};

const BETA_PHOENICIS: RealData = RealData {
    common_name: "",
    astronomical_name: "β Phoenicis",
    constellation: "Phoenix",
    right_ascension: RightAscension::new(1, 6, 5.),
    declination: Declination::new(Sgn::Neg, 46, 43, 6.),
    apparent_magnitude: 3.32,
    distance: Distance {
        m: 198.1 * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.598,
    mass: Mass {
        kg: 2.5 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 5090. },
    age: None,
    lifetime: Time {
        s: 0.800458342 * BILLION_YEARS.s,
    },
};

const GAMMA_PHOENICIS: RealData = RealData {
    common_name: "",
    astronomical_name: "γ Phoenicis",
    constellation: "Phoenix",
    right_ascension: RightAscension::new(1, 28, 22.),
    declination: Declination::new(Sgn::Neg, 43, 19, 6.),
    apparent_magnitude: 3.41,
    distance: Distance {
        m: 234. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.86,
    mass: Mass {
        kg: 1.3 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 52. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 3802. },
    age: None,
    lifetime: Time {
        s: 3.9126515 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [ANKAA, BETA_PHOENICIS, GAMMA_PHOENICIS];
