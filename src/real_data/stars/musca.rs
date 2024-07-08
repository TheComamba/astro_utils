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

const ALPHA_MUSCAE: RealData = RealData {
    common_name: "",
    astronomical_name: "α Muscae",
    constellation: "Musca",
    right_ascension: RightAscension::new(12, 37, 11.),
    declination: Declination::new(Sgn::Neg, 69, 8, 8.),
    apparent_magnitude: 2.69,
    distance: Distance {
        m: 315. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.2,
    mass: Mass {
        kg: 8.8 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 4.8 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 21_400. },
    age: Some(Time {
        s: 0.0183 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.03224554 * BILLION_YEARS.s,
    },
};

const BETA_MUSCAE: RealData = RealData {
    common_name: "",
    astronomical_name: "β Muscae",
    constellation: "Musca",
    right_ascension: RightAscension::new(12, 46, 17.),
    declination: Declination::new(Sgn::Neg, 68, 6, 29.),
    apparent_magnitude: 3.05,
    distance: Distance {
        m: 340. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.06,
    mass: Mass {
        kg: 7.35 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 21_000. },
    age: Some(Time {
        s: 0.0151 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.052267043 * BILLION_YEARS.s,
    },
};

const DELTA_MUSCAE: RealData = RealData {
    common_name: "",
    astronomical_name: "δ Muscae",
    constellation: "Musca",
    right_ascension: RightAscension::new(13, 2, 16.),
    declination: Declination::new(Sgn::Neg, 71, 32, 56.),
    apparent_magnitude: 3.61,
    distance: Distance {
        m: 91. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.38,
    mass: Mass {
        kg: 1.1 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 4_400. },
    age: None,
    lifetime: Time {
        s: 6.97272616 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_MUSCAE, BETA_MUSCAE, DELTA_MUSCAE];
