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

const BETA_ARAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Arae",
    constellation: "Ara",
    radius: Some(Distance {
        m: 142. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 8.21 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.49,
    apparent_magnitude: 2.84,
    temperature: Some(Temperature { K: 4197. }),
    age: Some(Time {
        s: 0.05 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(17, 25, 18),
    declination: Declination::new(Sgn::Neg, 55, 31, 48),
    distance: Distance {
        m: 602.6 * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
