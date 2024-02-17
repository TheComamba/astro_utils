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

const ALPHARD: RealData = RealData {
    common_name: "Alphard",
    astronomical_name: "Alpha Hydrae",
    constellation: "Hydra",
    radius: Some(Distance {
        m: 50.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.03 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.69,
    apparent_magnitude: 1.99,
    temperature: Some(Temperature { K: 4120. }),
    age: Some(Time {
        s: 0.42 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(9, 27, 35),
    declination: Declination::new(Sgn::Neg, 8, 39, 30),
    distance: Distance {
        m: 177. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
