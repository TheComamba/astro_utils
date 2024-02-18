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

const PEACOCK: RealData = RealData {
    common_name: "Peacock",
    astronomical_name: "Alpha Pavonis",
    constellation: "Pavo",
    radius: Some(Distance {
        m: 4.83 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 5.91 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.81,
    apparent_magnitude: 1.94,
    temperature: Temperature { K: 17_711. },
    age: Some(Time {
        s: 0.048 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(20, 25, 39),
    declination: Declination::new(Sgn::Neg, 56, 44, 6),
    distance: Distance {
        m: 183. * LIGHT_YEAR.m,
    },
};

const BETA_PAVONIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Pavonis",
    constellation: "Pavo",
    right_ascension: RightAscension::new(20, 44, 57),
    declination: Declination::new(Sgn::Neg, 66, 12, 12),
    apparent_magnitude: 3.42,
    distance: Distance {
        m: 135.1 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.33,
    mass: Some(Mass {
        kg: 2.51 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.3 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 8184. },
    age: Some(Time {
        s: 0.305 * BILLION_YEARS.s,
    }),
};

const DELTA_PAVONIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Pavonis",
    constellation: "Pavo",
    right_ascension: RightAscension::new(20, 8, 44),
    declination: Declination::new(Sgn::Neg, 66, 10, 55),
    apparent_magnitude: 3.56,
    distance: Distance {
        m: 19.89 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 4.62,
    mass: Some(Mass {
        kg: 1.051 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.197 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5571. },
    age: Some(Time {
        s: 6.7 * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 3] = [PEACOCK, BETA_PAVONIS, DELTA_PAVONIS];
