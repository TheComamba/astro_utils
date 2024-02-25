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

const NU_OCTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Nu Octantis",
    constellation: "Octans",
    right_ascension: RightAscension::new(21, 41, 29),
    declination: Declination::new(Sgn::Neg, 77, 23, 24),
    apparent_magnitude: 3.73,
    distance: Distance {
        m: 63.3 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.10,
    mass: Mass {
        kg: 1.04 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 5.9 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4860. },
    age: Some(Time {
        s: 2.5 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 4.294967295 * BILLION_YEARS.s,
    },
};

const BETA_OCTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Octantis",
    constellation: "Octans",
    right_ascension: RightAscension::new(22, 46, 4),
    declination: Declination::new(Sgn::Neg, 81, 22, 54),
    apparent_magnitude: 4.13,
    distance: Distance {
        m: 149. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.83,
    mass: Mass {
        kg: 2.27 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 3.2 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 8006. },
    age: Some(Time {
        s: 496. * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.964406929 * BILLION_YEARS.s,
    },
};

const DELTA_OCTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Octantis",
    constellation: "Octans",
    right_ascension: RightAscension::new(14, 26, 55),
    declination: Declination::new(Sgn::Neg, 83, 40, 4),
    apparent_magnitude: 4.31,
    distance: Distance {
        m: 299. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.35,
    mass: Mass {
        kg: 1.06 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 24.61 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4311. },
    age: None,
    lifetime: Time {
        s: 4.294967295 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [NU_OCTANTIS, BETA_OCTANTIS, DELTA_OCTANTIS];
