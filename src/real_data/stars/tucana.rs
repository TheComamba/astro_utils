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

const APLHA_TUCANAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Tucanae",
    constellation: "Tucana",
    radius: Some(Distance {
        m: 37. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.05,
    apparent_magnitude: 2.87,
    temperature: Some(Temperature { K: 4300. }),
    age: None,
    right_ascension: RightAscension::new(22, 18, 30),
    declination: Declination::new(Sgn::Neg, 60, 15, 35),
    distance: Distance {
        m: 198.5 * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
