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

const DIPHDA: RealData = RealData {
    common_name: "Diphda",
    astronomical_name: "Beta Ceti",
    constellation: "Cetus",
    radius: Some(Distance {
        m: 16.78 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.30,
    apparent_magnitude: 2.04,
    temperature: Some(Temperature { K: 4797. }),
    age: Some(Time {
        s: 1. * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(0, 43, 35),
    declination: Declination::new(Sgn::Neg, 17, 59, 12),
    distance: Distance {
        m: 96. * LIGHT_YEAR.m,
    },
};

const MENKAR: RealData = RealData {
    common_name: "Menkar",
    astronomical_name: "Alpha Ceti",
    constellation: "Cetus",
    radius: Some(Distance {
        m: 89. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.3 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.61,
    apparent_magnitude: 2.54,
    temperature: Some(Temperature { K: 3795. }),
    age: None,
    right_ascension: RightAscension::new(3, 2, 17),
    declination: Declination::new(Sgn::Pos, 4, 5, 23),
    distance: Distance {
        m: 220. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
