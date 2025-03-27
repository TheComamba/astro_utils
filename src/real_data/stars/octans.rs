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

const NU_OCTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "ν Octantis",
    constellation: "Octans",
    right_ascension: RightAscension::new(21, 41, 29.),
    declination: Declination::new(Sgn::Neg, 77, 23, 24.),
    apparent_magnitude: 3.73,
    distance: Length {
        m: 63.3 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.10,
    mass: Mass {
        kg: 1.04 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 5.9 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4860. },
    age: Some(Time {
        s: 2.5 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 8.24015833 * BILLION_YEARS.s,
    },
};

const BETA_OCTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "β Octantis",
    constellation: "Octans",
    right_ascension: RightAscension::new(22, 46, 4.),
    declination: Declination::new(Sgn::Neg, 81, 22, 54.),
    apparent_magnitude: 4.13,
    distance: Length {
        m: 149. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.83,
    mass: Mass {
        kg: 2.27 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 3.2 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 8006. },
    age: Some(Time {
        s: 0.496 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.964406929 * BILLION_YEARS.s,
    },
};

const DELTA_OCTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "δ Octantis",
    constellation: "Octans",
    right_ascension: RightAscension::new(14, 26, 55.),
    declination: Declination::new(Sgn::Neg, 83, 40, 4.),
    apparent_magnitude: 4.31,
    distance: Length {
        m: 299. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.35,
    mass: Mass {
        kg: 1.06 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 24.61 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4311. },
    age: None,
    lifetime: Time {
        s: 8.24015833 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [NU_OCTANTIS, BETA_OCTANTIS, DELTA_OCTANTIS];
