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

const ALPHA_SEXTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Sextantis",
    constellation: "Sextans",
    right_ascension: RightAscension::new(10, 7, 56),
    declination: Declination::new(Sgn::Neg, 0, 22, 18),
    apparent_magnitude: 4.49,
    distance: Distance {
        m: 280. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.29,
    mass: Some(Mass {
        kg: 2.57 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 3.07 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 9984. }),
    age: Some(Time {
        s: 0.385 * BILLION_YEARS.s,
    }),
};

const GAMMA_SEXTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Sextantis",
    constellation: "Sextans",
    right_ascension: RightAscension::new(9, 52, 30),
    declination: Declination::new(Sgn::Neg, 8, 6, 18),
    apparent_magnitude: 5.05,
    distance: Distance {
        m: 280. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.43,
    mass: Some(Mass {
        kg: 2.60 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 9825. }),
    age: Some(Time {
        s: 0.401 * BILLION_YEARS.s,
    }),
};

const BETA_SEXTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Sextantis",
    constellation: "Sextans",
    right_ascension: RightAscension::new(10, 30, 17),
    declination: Declination::new(Sgn::Neg, 0, 38, 13),
    apparent_magnitude: 5.07,
    distance: Distance {
        m: 364. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.38,
    mass: None,
    radius: Some(Distance {
        m: 3.2 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 14_570. }),
    age: None,
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_SEXTANTIS, GAMMA_SEXTANTIS, BETA_SEXTANTIS];
