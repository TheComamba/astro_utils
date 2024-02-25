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

const NAVI: RealData = RealData {
    common_name: "Navi",
    astronomical_name: "Gamma Cassiopeiae",
    constellation: "Cassiopeia",
    radius: Some(Distance {
        m: 10. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 13. * SOLAR_MASS.kg,
    },
    absolute_magnitude: -4.22,
    apparent_magnitude: 2.20,
    temperature: Temperature { K: 25_000. },
    right_ascension: RightAscension::new(0, 56, 43),
    declination: Declination::new(Sgn::Pos, 60, 43, 0),
    distance: Distance {
        m: 613. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.008 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.019450199 * BILLION_YEARS.s,
    },
};

const SCHEDAR: RealData = RealData {
    common_name: "Schedar",
    astronomical_name: "Alpha Cassiopeiae",
    constellation: "Cassiopeia",
    radius: Some(Distance {
        m: 45.39 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 3.98 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -1.99,
    apparent_magnitude: 2.24,
    temperature: Temperature { K: 4552. },
    right_ascension: RightAscension::new(0, 40, 30),
    declination: Declination::new(Sgn::Pos, 56, 32, 14),
    distance: Distance {
        m: 228. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.19 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.193156929 * BILLION_YEARS.s,
    },
};

const CAPH: RealData = RealData {
    common_name: "Caph",
    astronomical_name: "Beta Cassiopeiae",
    constellation: "Cassiopeia",
    radius: Some(Distance {
        m: 3.5 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1.91 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 1.17,
    apparent_magnitude: 2.28,
    temperature: Temperature { K: 7079. },
    right_ascension: RightAscension::new(0, 9, 11),
    declination: Declination::new(Sgn::Pos, 59, 8, 59),
    distance: Distance {
        m: 54. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 1.1 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.54706939 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [NAVI, SCHEDAR, CAPH];
