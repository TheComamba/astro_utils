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

const ALPHA_ANTLIAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Antliae",
    constellation: "Antlia",
    radius: Some(Distance {
        m: 41. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.973,
    apparent_magnitude: 4.28,
    temperature: Temperature { K: 4070. },
    age: None,
    right_ascension: RightAscension::new(10, 27, 9),
    declination: Declination::new(Sgn::Neg, 31, 4, 4),
    distance: Distance {
        m: 366.3 * LIGHT_YEAR.m,
    },
};

const EPSILON_ANTLIAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Epsilon Antliae",
    constellation: "Antlia",
    radius: Some(Distance {
        m: 56.3 * SOLAR_RADIUS.m,
    }),
    mass: None,
    absolute_magnitude: -2.15,
    apparent_magnitude: 4.51,
    temperature: Temperature { K: 4237. },
    age: None,
    right_ascension: RightAscension::new(9, 29, 15),
    declination: Declination::new(Sgn::Neg, 35, 57, 5),
    distance: Distance {
        m: 699.6 * LIGHT_YEAR.m,
    },
};

const IOTA_ANTLIAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Iota Antliae",
    constellation: "Antlia",
    radius: Some(Distance {
        m: 12.1 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.55 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.674,
    apparent_magnitude: 4.60,
    temperature: Temperature { K: 4892. },
    age: Some(Time {
        s: 3.32 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(10, 56, 43),
    declination: Declination::new(Sgn::Neg, 37, 8, 16),
    distance: Distance {
        m: 198.8 * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_ANTLIAE, EPSILON_ANTLIAE, IOTA_ANTLIAE];
