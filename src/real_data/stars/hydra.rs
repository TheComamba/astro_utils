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

const ALPHARD: RealData = RealData {
    common_name: "Alphard",
    astronomical_name: "Alpha Hydrae",
    constellation: "Hydra",
    radius: Some(Distance {
        m: 50.5 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 3.03 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -1.69,
    apparent_magnitude: 1.99,
    temperature: Temperature { K: 4120. },
    age: Some(Time {
        s: 0.42 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.420724107 * BILLION_YEARS.s,
    },
    right_ascension: RightAscension::new(9, 27, 35),
    declination: Declination::new(Sgn::Neg, 8, 39, 30),
    distance: Distance {
        m: 177. * LIGHT_YEAR.m,
    },
};

const GAMMA_HYDRAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Hydrae",
    constellation: "Hydra",
    right_ascension: RightAscension::new(13, 18, 55),
    declination: Declination::new(Sgn::Neg, 23, 10, 17),
    apparent_magnitude: 2.993,
    distance: Distance {
        m: 133.8 * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.15,
    mass: Mass {
        kg: 2.94 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 16. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5087. },
    age: Some(Time {
        s: 0.372 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.420724107 * BILLION_YEARS.s,
    },
};

const ZETA_HYDRAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Zeta Hydrae",
    constellation: "Hydra",
    right_ascension: RightAscension::new(8, 55, 24),
    declination: Declination::new(Sgn::Pos, 5, 56, 44),
    apparent_magnitude: 3.1,
    distance: Distance {
        m: 167. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.24,
    mass: Mass {
        kg: 4.2 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 17.9 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4925. },
    age: Some(Time {
        s: 0.4 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.170765802 * BILLION_YEARS.s,
    },
};

const NU_HYDRAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Nu Hydrae",
    constellation: "Hydra",
    right_ascension: RightAscension::new(10, 49, 37),
    declination: Declination::new(Sgn::Neg, 16, 11, 37),
    apparent_magnitude: 3.115,
    distance: Distance {
        m: 137.1 * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.11,
    mass: Mass {
        kg: 2.0 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 21. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4335. },
    age: None,
    lifetime: Time {
        s: 1.36020165 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 4] = [ALPHARD, GAMMA_HYDRAE, ZETA_HYDRAE, NU_HYDRAE];
