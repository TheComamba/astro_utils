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

const ALPHA_HOROLOGII: RealData = RealData {
    common_name: "",
    astronomical_name: "α Horologii",
    constellation: "Horologium",
    right_ascension: RightAscension::new(4, 14, 0),
    declination: Declination::new(Sgn::Neg, 42, 17, 40),
    apparent_magnitude: 3.853,
    distance: Distance {
        m: 115. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.08,
    mass: Mass {
        kg: 1.55 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 8. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5028. },
    age: None,
    lifetime: Time {
        s: 2.29668629 * BILLION_YEARS.s,
    },
};

const R_HOROLOGII: RealData = RealData {
    common_name: "",
    astronomical_name: "R Horologii",
    constellation: "Horologium",
    right_ascension: RightAscension::new(2, 53, 53),
    declination: Declination::new(Sgn::Neg, 49, 53, 23),
    apparent_magnitude: 7.22,
    distance: Distance {
        m: 1003. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.221,
    mass: Mass {
        kg: 3.0 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 2200. },
    age: None,
    lifetime: Time {
        s: 0.420724107 * BILLION_YEARS.s,
    },
};

const BETA_HOROLOGII: RealData = RealData {
    common_name: "",
    astronomical_name: "β Horologii",
    constellation: "Horologium",
    right_ascension: RightAscension::new(2, 58, 48),
    declination: Declination::new(Sgn::Neg, 64, 4, 17),
    apparent_magnitude: 4.979,
    distance: Distance {
        m: 312. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.2,
    mass: Mass {
        kg: 3.3 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 1.4 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 8303. },
    age: None,
    lifetime: Time {
        s: 0.351318702 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_HOROLOGII, R_HOROLOGII, BETA_HOROLOGII];
