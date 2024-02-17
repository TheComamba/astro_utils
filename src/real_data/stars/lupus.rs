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

const ALPHA_LUPI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Lupi",
    constellation: "Lupus",
    radius: None,
    mass: Some(Mass {
        kg: 10.1 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.83,
    apparent_magnitude: 2.30,
    temperature: Some(Temperature { K: 21_820. }),
    age: Some(Time {
        s: 0.018 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(14, 41, 56),
    declination: Declination::new(Sgn::Neg, 47, 23, 18),
    distance: Distance {
        m: 548. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
