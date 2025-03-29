use astro_coords::ra_and_dec::*;
use simple_si_units::base::{Length, Mass, Temperature, Time};

use crate::{
    stars::real_data::RealData,
    units::{
        length::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

const ARNEB: RealData = RealData {
    common_name: "Arneb",
    astronomical_name: "α Leporis",
    constellation: "Lepus",
    radius: Some(Length {
        m: 75. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 13.9 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -5.40,
    apparent_magnitude: 2.58,
    temperature: Temperature { K: 6_850. },
    age: Some(Time {
        s: 0.013 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.015362858 * BILLION_YEARS.s,
    },
    right_ascension: RightAscension::new(5, 32, 44.),
    declination: Declination::new(Sgn::Neg, 17, 49, 20.),
    distance: Length {
        m: 1283. * LIGHT_YEAR.m,
    },
};

const BETA_LEPORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "β Leporis",
    constellation: "Lepus",
    right_ascension: RightAscension::new(5, 28, 15.),
    declination: Declination::new(Sgn::Neg, 20, 45, 34.),
    apparent_magnitude: 2.84,
    distance: Length {
        m: 160. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.65,
    mass: Mass {
        kg: 3.5 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 16. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5450. },
    age: Some(Time {
        s: 0.240 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.297402042 * BILLION_YEARS.s,
    },
};

const EPSILON_LEPORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "ε Leporis",
    constellation: "Lepus",
    right_ascension: RightAscension::new(5, 5, 28.),
    declination: Declination::new(Sgn::Neg, 22, 22, 16.),
    apparent_magnitude: 3.166,
    distance: Length {
        m: 209. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.02,
    mass: Mass {
        kg: 1.7 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 40.1 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4131. },
    age: Some(Time {
        s: 1.72 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.73766023 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [ARNEB, BETA_LEPORIS, EPSILON_LEPORIS];
