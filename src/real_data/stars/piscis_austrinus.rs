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

const FORMALHAUT: RealData = RealData {
    common_name: "Formalhaut",
    astronomical_name: "Alpha Piscis Austrini",
    constellation: "Piscis Austrinus",
    radius: Some(Distance {
        m: 1.842 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.92 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.74,
    apparent_magnitude: 1.17,
    temperature: Some(Temperature { K: 8590. }),
    age: Some(Time {
        s: 0.44 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(22, 57, 39),
    declination: Declination::new(Sgn::Neg, 29, 37, 20),
    distance: Distance {
        m: 25. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
