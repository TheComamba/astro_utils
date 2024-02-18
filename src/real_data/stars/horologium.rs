use crate::{
    coordinates::{
        declination::{Declination, Sgn},
        right_ascension::RightAscension,
    },
    stars::real_data::RealData,
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
    },
};
use simple_si_units::base::{Distance, Mass, Temperature};

const ALPHA_HOROLOGII: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Horologii",
    constellation: "Horologium",
    right_ascension: RightAscension::new(4, 14, 0),
    declination: Declination::new(Sgn::Neg, 42, 17, 40),
    apparent_magnitude: 3.853,
    distance: Distance {
        m: 115. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.08,
    mass: Some(Mass {
        kg: 1.55 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 8. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5028. },
    age: None,
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
    mass: None,
    radius: None,
    temperature: Temperature { K: 2200. },
    age: None,
};

const BETA_HOROLOGII: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Horologii",
    constellation: "Horologium",
    right_ascension: RightAscension::new(2, 58, 48),
    declination: Declination::new(Sgn::Neg, 64, 4, 17),
    apparent_magnitude: 4.979,
    distance: Distance {
        m: 312. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.2,
    mass: None,
    radius: Some(Distance {
        m: 1.4 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 8303. },
    age: None,
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_HOROLOGII, R_HOROLOGII, BETA_HOROLOGII];
