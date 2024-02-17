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

const NAOS: RealData = RealData {
    common_name: "Naos",
    astronomical_name: "Zeta Puppis",
    constellation: "Puppis",
    radius: Some(Distance {
        m: 20. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 56.1 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -5.95,
    apparent_magnitude: 2.21,
    temperature: Some(Temperature { K: 40_000. }),
    age: Some(Time {
        s: 0.0032 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(8, 3, 35),
    declination: Declination::new(Sgn::Neg, 40, 0, 12),
    distance: Distance {
        m: 1399. * LIGHT_YEAR.m,
    },
};

const AHADI: RealData = RealData {
    common_name: "Ahadi",
    astronomical_name: "Pi Puppis",
    constellation: "Puppis",
    radius: Some(Distance {
        m: 235. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 11.7 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.92,
    apparent_magnitude: 2.71,
    temperature: Some(Temperature { K: 4000. }),
    age: Some(Time {
        s: 0.02 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(7, 17, 9),
    declination: Declination::new(Sgn::Neg, 37, 5, 51),
    distance: Distance {
        m: 1094. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
