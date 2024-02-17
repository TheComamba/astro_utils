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

const GHURAB: RealData = RealData {
    common_name: "Ghurab",
    astronomical_name: "Gamma Corvi",
    constellation: "Corvus",
    radius: None,
    mass: Some(Mass {
        kg: 4.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.94,
    apparent_magnitude: 2.58,
    temperature: Some(Temperature { K: 12_000. }),
    age: Some(Time {
        s: 0.160 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 15, 48),
    declination: Declination::new(Sgn::Neg, 17, 32, 31),
    distance: Distance {
        m: 165. * LIGHT_YEAR.m,
    },
};

const KRAZ: RealData = RealData {
    common_name: "Kraz",
    astronomical_name: "Beta Corvi",
    constellation: "Corvus",
    radius: Some(Distance {
        m: 16. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.7 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.61,
    apparent_magnitude: 2.65,
    temperature: Some(Temperature { K: 5100. }),
    age: Some(Time {
        s: 0.206 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 34, 23),
    declination: Declination::new(Sgn::Neg, 23, 23, 48),
    distance: Distance {
        m: 146. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
