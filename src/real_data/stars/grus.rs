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

const ALNAIR: RealData = RealData {
    common_name: "Alnair",
    astronomical_name: "Alpha Gruis",
    constellation: "Grus",
    radius: Some(Distance {
        m: 3.4 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 4. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.73,
    apparent_magnitude: 1.73,
    temperature: Some(Temperature { K: 13_920. }),
    age: Some(Time {
        s: 0.1 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(22, 8, 14),
    declination: Declination::new(Sgn::Neg, 46, 57, 40),
    distance: Distance {
        m: 101. * LIGHT_YEAR.m,
    },
};

const TIAKI: RealData = RealData {
    common_name: "Tiaki",
    astronomical_name: "Beta Gruis",
    constellation: "Grus",
    radius: Some(Distance {
        m: 180. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.4 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.52,
    apparent_magnitude: 2.07,
    temperature: Some(Temperature { K: 3480. }),
    age: None,
    right_ascension: RightAscension::new(22, 42, 40),
    declination: Declination::new(Sgn::Neg, 46, 53, 4),
    distance: Distance {
        m: 170. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
