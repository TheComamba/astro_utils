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

const BETA_CAPRICORNI: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Capricorni",
    constellation: "Capricornus",
    right_ascension: RightAscension::new(20, 21, 1),
    declination: Declination::new(Sgn::Neg, 14, 46, 53),
    apparent_magnitude: 3.05,
    distance: Distance {
        m: 390. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.03,
    mass: None,
    radius: None,
    temperature: None,
    age: None,
};

const DELTA_CAPRICORNI: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Capricorni",
    constellation: "Capricornus",
    right_ascension: RightAscension::new(21, 47, 2),
    declination: Declination::new(Sgn::Neg, 16, 7, 38),
    apparent_magnitude: 2.81,
    distance: Distance {
        m: 38.7 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.48,
    mass: Some(Mass {
        kg: 2. * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.91 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 7301. }),
    age: None,
};

const OMEGA_CAPRICORNI: RealData = RealData {
    common_name: "",
    astronomical_name: "Omega Capricorni",
    constellation: "Capricornus",
    right_ascension: RightAscension::new(20, 51, 49),
    declination: Declination::new(Sgn::Neg, 26, 55, 9),
    apparent_magnitude: 4.11,
    distance: Distance {
        m: 1000. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.7,
    mass: Some(Mass {
        kg: 6.8 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 172.1 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 3915. }),
    age: Some(Time {
        s: 0.0481 * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 3] = [BETA_CAPRICORNI, DELTA_CAPRICORNI, OMEGA_CAPRICORNI];
