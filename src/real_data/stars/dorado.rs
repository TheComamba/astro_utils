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

const R_DORADUS: RealData = RealData {
    common_name: "",
    astronomical_name: "R Doradus",
    constellation: "Dorado",
    radius: Some(Distance {
        m: 298. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 0.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.61,
    apparent_magnitude: 5.59,
    temperature: Some(Temperature { K: 2710. }),
    age: Some(Time {
        s: 10. * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(4, 36, 46),
    declination: Declination::new(Sgn::Neg, 62, 4, 38),
    distance: Distance {
        m: 203.5 * LIGHT_YEAR.m,
    },
};
pub(crate) const STARS: [RealData; 0] = [];
