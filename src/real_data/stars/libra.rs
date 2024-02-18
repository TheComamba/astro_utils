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

const ZUBENELGENUBI: RealData = RealData {
    common_name: "Zubenelgenubi",
    astronomical_name: "Alpha2 Librae",
    constellation: "Libra",
    right_ascension: RightAscension::new(14, 50, 53),
    declination: Declination::new(Sgn::Neg, 16, 2, 30),
    apparent_magnitude: 2.741,
    distance: Distance {
        m: 75.8 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.879,
    mass: Some(Mass {
        kg: 1.95 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Temperature { K: 8128. },
    age: None,
};

const ZUBENESCHAMALI: RealData = RealData {
    common_name: "Zubeneschamali",
    astronomical_name: "Beta Librae",
    constellation: "Libra",
    right_ascension: RightAscension::new(15, 17, 0),
    declination: Declination::new(Sgn::Neg, 9, 22, 58),
    apparent_magnitude: 2.61,
    distance: Distance {
        m: 185. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.16,
    mass: Some(Mass {
        kg: 3.5 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 4.9 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 12_300. },
    age: Some(Time {
        s: 0.08 * BILLION_YEARS.s,
    }),
};

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
    temperature: Temperature { K: 3596. },
    age: None,
    right_ascension: RightAscension::new(15, 4, 4),
    declination: Declination::new(Sgn::Neg, 25, 16, 55),
    distance: Distance {
        m: 288. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 3] = [ZUBENELGENUBI, ZUBENESCHAMALI, BRACHIUM];
