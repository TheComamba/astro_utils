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

const ALPHA_SEXTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "α Sextantis",
    constellation: "Sextans",
    right_ascension: RightAscension::new(10, 7, 56.),
    declination: Declination::new(Sgn::Neg, 0, 22, 18.),
    apparent_magnitude: 4.49,
    distance: Length {
        m: 280. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.29,
    mass: Mass {
        kg: 2.57 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 3.07 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 9984. },
    age: Some(Time {
        s: 0.385 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.63513384 * BILLION_YEARS.s,
    },
};

const GAMMA_SEXTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "γ Sextantis",
    constellation: "Sextans",
    right_ascension: RightAscension::new(9, 52, 30.),
    declination: Declination::new(Sgn::Neg, 8, 6, 18.),
    apparent_magnitude: 5.05,
    distance: Length {
        m: 280. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.43,
    mass: Mass {
        kg: 2.60 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 9825. },
    age: Some(Time {
        s: 0.401 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.63513384 * BILLION_YEARS.s,
    },
};

const BETA_SEXTANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "β Sextantis",
    constellation: "Sextans",
    right_ascension: RightAscension::new(10, 30, 17.),
    declination: Declination::new(Sgn::Neg, 0, 38, 13.),
    apparent_magnitude: 5.07,
    distance: Length {
        m: 364. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.38,
    mass: Mass {
        kg: 5.1 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 3.2 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 14_570. },
    age: None,
    lifetime: Time {
        s: 0.111319448 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_SEXTANTIS, GAMMA_SEXTANTIS, BETA_SEXTANTIS];
