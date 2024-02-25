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

const ALPHA_SCULPTORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Sculptoris",
    constellation: "Sculptor",
    right_ascension: RightAscension::new(0, 58, 36),
    declination: Declination::new(Sgn::Neg, 29, 21, 27),
    apparent_magnitude: 4.30,
    distance: Distance {
        m: 780. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.58,
    mass: Mass {
        kg: 5.01 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 7.52 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 13_600. },
    age: Some(Time {
        s: 0.093 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.111319448 * BILLION_YEARS.s,
    },
};

const BETA_SCULPTORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Sculptoris",
    constellation: "Sculptor",
    right_ascension: RightAscension::new(23, 32, 58),
    declination: Declination::new(Sgn::Neg, 37, 49, 6),
    apparent_magnitude: 4.37,
    distance: Distance {
        m: 174. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.74,
    mass: Mass {
        kg: 2.98 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 2.0 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 12_110. },
    age: None,
    lifetime: Time {
        s: 0.420724107 * BILLION_YEARS.s,
    },
};

const GAMMA_SULPTORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Sculptoris",
    constellation: "Sculptor",
    right_ascension: RightAscension::new(23, 18, 49),
    declination: Declination::new(Sgn::Neg, 32, 31, 55),
    apparent_magnitude: 4.41,
    distance: Distance {
        m: 182. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.67,
    mass: Mass {
        kg: 1.6 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 12. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4578. },
    age: Some(Time {
        s: 2.47 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 2.08398753 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_SCULPTORIS, BETA_SCULPTORIS, GAMMA_SULPTORIS];
