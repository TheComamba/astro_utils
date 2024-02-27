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

const ALPHA_PYXIDIS: RealData = RealData {
    common_name: "",
    astronomical_name: "α Pyxidis",
    constellation: "Pyxis",
    right_ascension: RightAscension::new(8, 43, 36),
    declination: Declination::new(Sgn::Neg, 33, 11, 11),
    apparent_magnitude: 3.67,
    distance: Distance {
        m: 880. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -3.47,
    mass: Mass {
        kg: 10.7 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 6.3 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 24_300. },
    age: Some(Time {
        s: 0.026 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.026540021 * BILLION_YEARS.s,
    },
};

const BETA_PYXIDIS: RealData = RealData {
    common_name: "",
    astronomical_name: "β Pyxidis",
    constellation: "Pyxis",
    right_ascension: RightAscension::new(8, 40, 6),
    declination: Declination::new(Sgn::Neg, 35, 18, 30),
    apparent_magnitude: 3.97,
    distance: Distance {
        m: 388.1 * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.41,
    mass: Mass {
        kg: 1.2 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 24. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5124. },
    age: None,
    lifetime: Time {
        s: 5.06543331 * BILLION_YEARS.s,
    },
};

const GAMMA_PYXIDIS: RealData = RealData {
    common_name: "",
    astronomical_name: "γ Pyxidis",
    constellation: "Pyxis",
    right_ascension: RightAscension::new(8, 50, 32),
    declination: Declination::new(Sgn::Neg, 27, 42, 35),
    apparent_magnitude: 4.010,
    distance: Distance {
        m: 207. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.,
    mass: Mass {
        kg: 1.64 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 21.87 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4270. },
    age: Some(Time {
        s: 1.8 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.89665739 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_PYXIDIS, BETA_PYXIDIS, GAMMA_PYXIDIS];
