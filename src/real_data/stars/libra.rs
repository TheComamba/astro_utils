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

const BRACHIUM: RealData = RealData {
    common_name: "Brachium",
    astronomical_name: "Sigma Librae",
    constellation: "Libra",
    radius: Some(Distance {
        m: 108. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.5,
    apparent_magnitude: 3.21,
    temperature: Some(Temperature { K: 3596. }),
    age: None,
    right_ascension: RightAscension::new(15, 4, 4),
    declination: Declination::new(Sgn::Neg, 25, 16, 55),
    distance: Distance {
        m: 288. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
