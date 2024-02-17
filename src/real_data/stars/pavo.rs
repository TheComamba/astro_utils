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

const PEACOCK: RealData = RealData {
    common_name: "Peacock",
    astronomical_name: "Alpha Pavonis",
    constellation: "Pavo",
    radius: Some(Distance {
        m: 4.83 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 5.91 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.81,
    apparent_magnitude: 1.94,
    temperature: Some(Temperature { K: 17_711. }),
    age: Some(Time {
        s: 0.048 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(20, 25, 39),
    declination: Declination::new(Sgn::Neg, 56, 44, 6),
    distance: Distance {
        m: 183. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
