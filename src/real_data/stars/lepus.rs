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

const ARNEB: RealData = RealData {
    common_name: "Arneb",
    astronomical_name: "Alpha Leporis",
    constellation: "Lepus",
    radius: Some(Distance {
        m: 75. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 13.9 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -5.40,
    apparent_magnitude: 2.58,
    temperature: Some(Temperature { K: 6_850. }),
    age: Some(Time {
        s: 0.013 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 32, 44),
    declination: Declination::new(Sgn::Neg, 17, 49, 20),
    distance: Distance {
        m: 1283. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
