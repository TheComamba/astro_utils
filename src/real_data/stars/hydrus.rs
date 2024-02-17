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

const GAMMA_HYDRI: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Hydri",
    constellation: "Hydrus",
    radius: Some(Distance {
        m: 62. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.83,
    apparent_magnitude: 3.26,
    temperature: Some(Temperature { K: 3499. }),
    age: None,
    right_ascension: RightAscension::new(3, 47, 14),
    declination: Declination::new(Sgn::Neg, 74, 14, 20),
    distance: Distance {
        m: 214. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
